#![allow(dead_code)]
use std::collections::HashMap;

use anyhow::Result;

use super::meta_fetcher::MetaFetcher;
use super::parser::{InsertStmt, SqlValue};

pub struct Finding {
    pub stmt_index: usize,
    pub original_sql: String,
    pub row_findings: Vec<RowFinding>,
}

pub struct RowFinding {
    pub row_index: usize,
    pub failures: Vec<FailureDetail>,
}

pub struct FailureDetail {
    pub kind: FailureKind,
    pub message: String,
    pub suggestions: Vec<Suggestion>,
}

pub enum FailureKind {
    PrimaryKeyConflict,
    UniqueKeyConflict { index_name: String },
    ForeignKeyViolation { fk_columns: Vec<String> },
    NotNullViolation { column: String },
    ColumnLengthExceeded { column: String, max_chars: u64, actual_chars: usize },
    TypeMismatch { column: String, expected: String },
    UnknownTable,
    UnknownColumn { column: String },
    SkippedExpression { column: String },
}

pub enum Suggestion {
    InsertIgnore,
    OnDuplicateKeyUpdate { columns: Vec<String>, rendered_sql: String },
    FixValue { column: String, hint: String },
}

const NUMERIC_TYPES: &[&str] = &[
    "tinyint", "smallint", "mediumint", "int", "bigint", "decimal", "float", "double",
    "numeric", "real",
];

pub async fn check_statement(
    stmt_index: usize,
    stmt: &InsertStmt,
    default_db: &str,
    fetcher: &mut MetaFetcher,
) -> Result<Finding> {
    let db = stmt.database.as_deref().unwrap_or(default_db);

    // 5.1 Table existence
    if !fetcher.table_exists(db, &stmt.table).await? {
        return Ok(Finding {
            stmt_index,
            original_sql: stmt.original_sql.clone(),
            row_findings: vec![RowFinding {
                row_index: 1,
                failures: vec![FailureDetail {
                    kind: FailureKind::UnknownTable,
                    message: format!("Table `{}`.`{}` does not exist", db, stmt.table),
                    suggestions: vec![],
                }],
            }],
        });
    }

    let meta = fetcher.get_meta(db, &stmt.table).await?.clone();

    // 5.2 Column existence (named INSERTs only)
    let known_col_names: std::collections::HashSet<&str> =
        meta.columns.iter().map(|c| c.name.as_str()).collect();
    let unknown_columns: Vec<String> = if !stmt.columns.is_empty() {
        stmt.columns
            .iter()
            .filter(|c| !known_col_names.contains(c.as_str()))
            .cloned()
            .collect()
    } else {
        vec![]
    };

    let table_ref = qualified_table(stmt.database.as_deref(), &stmt.table);
    let mut row_findings: Vec<RowFinding> = vec![];

    for (row_idx, row) in stmt.rows.iter().enumerate() {
        let row_num = row_idx + 1;
        let mut failures: Vec<FailureDetail> = vec![];

        // Unknown column failures (once per row)
        for col in &unknown_columns {
            failures.push(FailureDetail {
                kind: FailureKind::UnknownColumn { column: col.clone() },
                message: format!(
                    "Column `{}` does not exist in table `{}`",
                    col, stmt.table
                ),
                suggestions: vec![],
            });
        }

        // Build value map for this row
        let value_map: HashMap<String, SqlValue>;
        if stmt.columns.is_empty() {
            // 5.3 Positional INSERT
            if row.len() > meta.columns.len() {
                failures.push(FailureDetail {
                    kind: FailureKind::UnknownColumn {
                        column: format!("(position {})", row.len()),
                    },
                    message: format!(
                        "Row has {} values but table has only {} columns",
                        row.len(),
                        meta.columns.len()
                    ),
                    suggestions: vec![],
                });
                row_findings.push(RowFinding { row_index: row_num, failures });
                continue;
            }
            value_map = meta
                .columns
                .iter()
                .zip(row.iter())
                .map(|(col, val)| (col.name.clone(), val.clone()))
                .collect();
        } else {
            value_map = stmt
                .columns
                .iter()
                .zip(row.iter())
                .filter(|(col, _)| !unknown_columns.contains(col))
                .map(|(col, val)| (col.clone(), val.clone()))
                .collect();
        }

        // 5.4 Static checks
        for col_meta in &meta.columns {
            let val = value_map.get(&col_meta.name);

            // NOT NULL
            if !col_meta.is_nullable {
                match val {
                    Some(SqlValue::Null) => {
                        failures.push(FailureDetail {
                            kind: FailureKind::NotNullViolation { column: col_meta.name.clone() },
                            message: format!(
                                "Column `{}` is NOT NULL but explicit NULL was provided",
                                col_meta.name
                            ),
                            suggestions: vec![Suggestion::FixValue {
                                column: col_meta.name.clone(),
                                hint: "provide a non-NULL value".to_string(),
                            }],
                        });
                    }
                    None if !stmt.columns.is_empty() && !col_meta.has_default => {
                        // Named INSERT: column absent with no default
                        failures.push(FailureDetail {
                            kind: FailureKind::NotNullViolation { column: col_meta.name.clone() },
                            message: format!(
                                "Column `{}` is NOT NULL and has no default but is not in INSERT",
                                col_meta.name
                            ),
                            suggestions: vec![Suggestion::FixValue {
                                column: col_meta.name.clone(),
                                hint: "add this column to the INSERT with a non-NULL value"
                                    .to_string(),
                            }],
                        });
                    }
                    _ => {}
                }
            }

            // Column length exceeded
            if let (Some(max_len), Some(SqlValue::Str(s))) =
                (col_meta.char_max_length, val.as_ref())
            {
                let actual = s.chars().count();
                if actual > max_len as usize {
                    failures.push(FailureDetail {
                        kind: FailureKind::ColumnLengthExceeded {
                            column: col_meta.name.clone(),
                            max_chars: max_len,
                            actual_chars: actual,
                        },
                        message: format!(
                            "Column `{}` max {} chars but value has {} chars",
                            col_meta.name, max_len, actual
                        ),
                        suggestions: vec![Suggestion::FixValue {
                            column: col_meta.name.clone(),
                            hint: format!("truncate to {} characters", max_len),
                        }],
                    });
                }
            }

            // Type mismatch
            if NUMERIC_TYPES.contains(&col_meta.data_type.to_lowercase().as_str()) {
                if let Some(SqlValue::Str(s)) = val {
                    if s.parse::<f64>().is_err() {
                        failures.push(FailureDetail {
                            kind: FailureKind::TypeMismatch {
                                column: col_meta.name.clone(),
                                expected: col_meta.data_type.clone(),
                            },
                            message: format!(
                                "Column `{}` expects `{}` but got string '{}'",
                                col_meta.name, col_meta.data_type, s
                            ),
                            suggestions: vec![Suggestion::FixValue {
                                column: col_meta.name.clone(),
                                hint: format!(
                                    "provide a numeric value for type {}",
                                    col_meta.data_type
                                ),
                            }],
                        });
                    }
                }
            }
        }

        // SkippedExpression notes
        for (col, val) in &value_map {
            if matches!(val, SqlValue::Other(_)) {
                failures.push(FailureDetail {
                    kind: FailureKind::SkippedExpression { column: col.clone() },
                    message: format!(
                        "Column `{}`: value is an expression; dynamic checks skipped",
                        col
                    ),
                    suggestions: vec![],
                });
            }
        }

        // 5.5 PK conflict
        if !meta.primary_key.is_empty() {
            let pk_conditions: Vec<(&str, &SqlValue)> = meta
                .primary_key
                .iter()
                .filter_map(|col| {
                    value_map.get(col.as_str()).and_then(|v| {
                        if matches!(v, SqlValue::Other(_)) { None } else { Some((col.as_str(), v)) }
                    })
                })
                .collect();

            if pk_conditions.len() == meta.primary_key.len() {
                if fetcher.check_row_exists(db, &stmt.table, &pk_conditions).await? {
                    let update_cols: Vec<String> = value_map
                        .keys()
                        .filter(|c| !meta.primary_key.contains(c))
                        .cloned()
                        .collect();
                    let rendered = render_odku(stmt, row, &update_cols, &table_ref);
                    let pk_desc: Vec<String> = pk_conditions
                        .iter()
                        .map(|(col, val)| format!("`{}` = {}", col, render_value(val)))
                        .collect();
                    failures.push(FailureDetail {
                        kind: FailureKind::PrimaryKeyConflict,
                        message: format!(
                            "{} already exists in `{}`",
                            pk_desc.join(" AND "),
                            stmt.table
                        ),
                        suggestions: vec![
                            Suggestion::InsertIgnore,
                            Suggestion::OnDuplicateKeyUpdate {
                                columns: update_cols,
                                rendered_sql: rendered,
                            },
                        ],
                    });
                }
            }
        }

        // 5.6 Unique key conflicts
        for (index_name, uk_cols) in &meta.unique_keys {
            let uk_conditions: Vec<(&str, &SqlValue)> = uk_cols
                .iter()
                .filter_map(|col| {
                    value_map.get(col.as_str()).and_then(|v| {
                        if matches!(v, SqlValue::Other(_)) { None } else { Some((col.as_str(), v)) }
                    })
                })
                .collect();

            if uk_conditions.len() == uk_cols.len() {
                if fetcher.check_row_exists(db, &stmt.table, &uk_conditions).await? {
                    let update_cols: Vec<String> = value_map
                        .keys()
                        .filter(|c| !uk_cols.contains(c))
                        .cloned()
                        .collect();
                    let rendered = render_odku(stmt, row, &update_cols, &table_ref);
                    let uk_desc: Vec<String> = uk_conditions
                        .iter()
                        .map(|(col, val)| format!("`{}` = {}", col, render_value(val)))
                        .collect();
                    failures.push(FailureDetail {
                        kind: FailureKind::UniqueKeyConflict { index_name: index_name.clone() },
                        message: format!(
                            "Unique key `{}`: {} already exists in `{}`",
                            index_name,
                            uk_desc.join(" AND "),
                            stmt.table
                        ),
                        suggestions: vec![
                            Suggestion::InsertIgnore,
                            Suggestion::OnDuplicateKeyUpdate {
                                columns: update_cols,
                                rendered_sql: rendered,
                            },
                        ],
                    });
                }
            }
        }

        // 5.7 Foreign key violations
        for fk in &meta.foreign_keys {
            let fk_conditions: Vec<(String, &SqlValue)> = fk
                .columns
                .iter()
                .zip(fk.referenced_columns.iter())
                .filter_map(|(col, ref_col)| {
                    value_map.get(col.as_str()).and_then(|v| {
                        if matches!(v, SqlValue::Other(_)) {
                            None
                        } else {
                            Some((ref_col.clone(), v))
                        }
                    })
                })
                .collect();

            if fk_conditions.len() == fk.columns.len() {
                let ref_conds: Vec<(&str, &SqlValue)> =
                    fk_conditions.iter().map(|(c, v)| (c.as_str(), *v)).collect();
                if !fetcher
                    .check_row_exists(&fk.referenced_database, &fk.referenced_table, &ref_conds)
                    .await?
                {
                    let col_list = fk.columns.join("`, `");
                    failures.push(FailureDetail {
                        kind: FailureKind::ForeignKeyViolation {
                            fk_columns: fk.columns.clone(),
                        },
                        message: format!(
                            "Column(s) `{}` reference `{}.{}` but no matching row exists",
                            col_list, fk.referenced_database, fk.referenced_table
                        ),
                        suggestions: vec![Suggestion::FixValue {
                            column: fk.columns.join(", "),
                            hint: "insert the referenced row first, or use an existing value"
                                .to_string(),
                        }],
                    });
                }
            }
        }

        if !failures.is_empty() {
            row_findings.push(RowFinding { row_index: row_num, failures });
        }
    }

    Ok(Finding { stmt_index, original_sql: stmt.original_sql.clone(), row_findings })
}

fn qualified_table(db: Option<&str>, table: &str) -> String {
    match db {
        Some(d) => format!("`{}`.`{}`", d, table),
        None => format!("`{}`", table),
    }
}

pub fn render_value(val: &SqlValue) -> String {
    match val {
        SqlValue::Null => "NULL".to_string(),
        SqlValue::Number(s) => s.clone(),
        SqlValue::Str(s) => format!("'{}'", s.replace('\'', "''")),
        SqlValue::Bool(true) => "TRUE".to_string(),
        SqlValue::Bool(false) => "FALSE".to_string(),
        SqlValue::Other(s) => s.clone(),
    }
}

fn render_odku(
    stmt: &InsertStmt,
    row: &[SqlValue],
    update_cols: &[String],
    table_ref: &str,
) -> String {
    let col_list = if stmt.columns.is_empty() {
        String::new()
    } else {
        format!(" ({})", stmt.columns.iter().map(|c| format!("`{}`", c)).collect::<Vec<_>>().join(", "))
    };

    let val_list = row.iter().map(render_value).collect::<Vec<_>>().join(", ");

    let update_parts: Vec<String> = update_cols
        .iter()
        .map(|c| format!("`{}` = new_row.`{}`", c, c))
        .collect();

    if update_parts.is_empty() {
        format!("INSERT INTO {}{} VALUES ({}) AS new_row\nON DUPLICATE KEY UPDATE `id` = new_row.`id`;", table_ref, col_list, val_list)
    } else {
        format!(
            "INSERT INTO {}{} VALUES ({}) AS new_row\nON DUPLICATE KEY UPDATE {};",
            table_ref,
            col_list,
            val_list,
            update_parts.join(", ")
        )
    }
}
