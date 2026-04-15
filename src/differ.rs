use std::collections::{BTreeSet, HashMap};

use crate::schema::{
    CheckDef, ColumnDef, DatabaseSchema, ForeignKeyDef, IndexDef,
    RoutineDef, SchemaSnapshot, TableDef, TriggerDef, ViewDef,
};

// ── Status ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum DiffStatus {
    OnlyInBase,
    OnlyInCheck,
    Changed,
    Same,
}

// ── Report types ─────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct DiffReport {
    pub base_label: String,
    pub check_label: String,
    pub generated_at: String,
    pub databases: Vec<DatabaseDiff>,
}

#[derive(Debug)]
pub struct DatabaseDiff {
    pub name: String,
    pub status: DiffStatus,
    pub tables: Vec<TableDiff>,
    pub views: Vec<ObjectDiff<ViewDef>>,
    pub procedures: Vec<ObjectDiff<RoutineDef>>,
    pub functions: Vec<ObjectDiff<RoutineDef>>,
    pub triggers: Vec<ObjectDiff<TriggerDef>>,
}

/// Diff for a single named object where we just compare the whole value.
#[derive(Debug)]
pub struct ObjectDiff<T: std::fmt::Debug + Clone> {
    pub name: String,
    pub status: DiffStatus,
    pub base: Option<T>,
    pub check: Option<T>,
}

/// Diff for a table — structured recursively into column/index/fk/check diffs.
#[derive(Debug)]
pub struct TableDiff {
    pub name: String,
    pub status: DiffStatus,
    pub column_diffs: Vec<ObjectDiff<ColumnDef>>,
    pub index_diffs: Vec<ObjectDiff<IndexDef>>,
    pub fk_diffs: Vec<ObjectDiff<ForeignKeyDef>>,
    pub check_diffs: Vec<ObjectDiff<CheckDef>>,
}

// ── Entry point ───────────────────────────────────────────────────────────────

/// Diff two snapshots and return a report. `base_label`, `check_label`, and
/// `generated_at` are purely for display in the rendered report.
pub fn diff(
    base: SchemaSnapshot,
    check: SchemaSnapshot,
    base_label: String,
    check_label: String,
    generated_at: String,
) -> DiffReport {
    let mut all_dbs: BTreeSet<String> = BTreeSet::new();
    all_dbs.extend(base.databases.keys().cloned());
    all_dbs.extend(check.databases.keys().cloned());

    let databases = all_dbs
        .into_iter()
        .map(|name| {
            let b = base.databases.get(&name);
            let c = check.databases.get(&name);
            match (b, c) {
                (Some(b), None) => db_only_in(b, DiffStatus::OnlyInBase),
                (None, Some(c)) => db_only_in(c, DiffStatus::OnlyInCheck),
                (Some(b), Some(c)) => diff_databases(b, c),
                (None, None) => unreachable!(),
            }
        })
        .collect();

    DiffReport {
        base_label,
        check_label,
        generated_at,
        databases,
    }
}

// ── Internal helpers ──────────────────────────────────────────────────────────

/// Build a DatabaseDiff when the entire database exists on only one side.
fn db_only_in(db: &DatabaseSchema, status: DiffStatus) -> DatabaseDiff {
    let is_base = status == DiffStatus::OnlyInBase;

    let tables = db
        .tables
        .values()
        .map(|t| table_only_in(t, status.clone()))
        .collect();

    let views = db.views.values().map(|v| ObjectDiff {
        name: v.name.clone(),
        status: status.clone(),
        base: if is_base { Some(v.clone()) } else { None },
        check: if is_base { None } else { Some(v.clone()) },
    }).collect();

    let procedures = db.procedures.values().map(|r| ObjectDiff {
        name: r.name.clone(),
        status: status.clone(),
        base: if is_base { Some(r.clone()) } else { None },
        check: if is_base { None } else { Some(r.clone()) },
    }).collect();

    let functions = db.functions.values().map(|r| ObjectDiff {
        name: r.name.clone(),
        status: status.clone(),
        base: if is_base { Some(r.clone()) } else { None },
        check: if is_base { None } else { Some(r.clone()) },
    }).collect();

    let triggers = db.triggers.values().map(|t| ObjectDiff {
        name: t.name.clone(),
        status: status.clone(),
        base: if is_base { Some(t.clone()) } else { None },
        check: if is_base { None } else { Some(t.clone()) },
    }).collect();

    DatabaseDiff {
        name: db.name.clone(),
        status,
        tables,
        views,
        procedures,
        functions,
        triggers,
    }
}

fn table_only_in(t: &TableDef, status: DiffStatus) -> TableDiff {
    let is_base = status == DiffStatus::OnlyInBase;

    let column_diffs = t.columns.iter().map(|c| ObjectDiff {
        name: c.name.clone(),
        status: status.clone(),
        base: if is_base { Some(c.clone()) } else { None },
        check: if is_base { None } else { Some(c.clone()) },
    }).collect();

    let index_diffs = t.indexes.iter().map(|i| ObjectDiff {
        name: i.name.clone(),
        status: status.clone(),
        base: if is_base { Some(i.clone()) } else { None },
        check: if is_base { None } else { Some(i.clone()) },
    }).collect();

    let fk_diffs = t.foreign_keys.iter().map(|fk| ObjectDiff {
        name: fk.name.clone(),
        status: status.clone(),
        base: if is_base { Some(fk.clone()) } else { None },
        check: if is_base { None } else { Some(fk.clone()) },
    }).collect();

    let check_diffs = t.checks.iter().map(|ch| ObjectDiff {
        name: ch.name.clone(),
        status: status.clone(),
        base: if is_base { Some(ch.clone()) } else { None },
        check: if is_base { None } else { Some(ch.clone()) },
    }).collect();

    TableDiff {
        name: t.name.clone(),
        status,
        column_diffs,
        index_diffs,
        fk_diffs,
        check_diffs,
    }
}

/// Diff two databases that exist on both sides.
fn diff_databases(base: &DatabaseSchema, check: &DatabaseSchema) -> DatabaseDiff {
    let tables = diff_tables_map(&base.tables, &check.tables);
    let views = diff_obj_map(&base.views, &check.views);
    let procedures = diff_obj_map(&base.procedures, &check.procedures);
    let functions = diff_obj_map(&base.functions, &check.functions);
    let triggers = diff_obj_map(&base.triggers, &check.triggers);

    let has_changes = tables.iter().any(|t| t.status != DiffStatus::Same)
        || views.iter().any(|v| v.status != DiffStatus::Same)
        || procedures.iter().any(|p| p.status != DiffStatus::Same)
        || functions.iter().any(|f| f.status != DiffStatus::Same)
        || triggers.iter().any(|t| t.status != DiffStatus::Same);

    DatabaseDiff {
        name: base.name.clone(),
        status: if has_changes { DiffStatus::Changed } else { DiffStatus::Same },
        tables,
        views,
        procedures,
        functions,
        triggers,
    }
}

/// Generic diff for a HashMap of named objects that implement PartialEq + Clone.
fn diff_obj_map<T>(
    base: &HashMap<String, T>,
    check: &HashMap<String, T>,
) -> Vec<ObjectDiff<T>>
where
    T: Clone + PartialEq + std::fmt::Debug,
{
    let mut all_keys: BTreeSet<String> = BTreeSet::new();
    all_keys.extend(base.keys().cloned());
    all_keys.extend(check.keys().cloned());

    all_keys
        .into_iter()
        .map(|key| {
            let b = base.get(&key);
            let c = check.get(&key);
            let status = match (b, c) {
                (Some(_), None) => DiffStatus::OnlyInBase,
                (None, Some(_)) => DiffStatus::OnlyInCheck,
                (Some(bv), Some(cv)) => {
                    if bv == cv { DiffStatus::Same } else { DiffStatus::Changed }
                }
                (None, None) => unreachable!(),
            };
            ObjectDiff {
                name: key,
                status,
                base: b.cloned(),
                check: c.cloned(),
            }
        })
        .collect()
}

/// Diff a map of TableDefs — produces a TableDiff per table with recursive column/index/fk/check diffs.
fn diff_tables_map(
    base: &HashMap<String, TableDef>,
    check: &HashMap<String, TableDef>,
) -> Vec<TableDiff> {
    let mut all_keys: BTreeSet<String> = BTreeSet::new();
    all_keys.extend(base.keys().cloned());
    all_keys.extend(check.keys().cloned());

    all_keys
        .into_iter()
        .map(|key| {
            let b = base.get(&key);
            let c = check.get(&key);
            match (b, c) {
                (Some(b), None) => table_only_in(b, DiffStatus::OnlyInBase),
                (None, Some(c)) => table_only_in(c, DiffStatus::OnlyInCheck),
                (Some(b), Some(c)) => diff_table(b, c),
                (None, None) => unreachable!(),
            }
        })
        .collect()
}

/// Diff two tables that exist on both sides.
fn diff_table(base: &TableDef, check: &TableDef) -> TableDiff {
    let base_cols: HashMap<String, &ColumnDef> =
        base.columns.iter().map(|c| (c.name.clone(), c)).collect();
    let check_cols: HashMap<String, &ColumnDef> =
        check.columns.iter().map(|c| (c.name.clone(), c)).collect();

    let base_idx: HashMap<String, &IndexDef> =
        base.indexes.iter().map(|i| (i.name.clone(), i)).collect();
    let check_idx: HashMap<String, &IndexDef> =
        check.indexes.iter().map(|i| (i.name.clone(), i)).collect();

    let base_fks: HashMap<String, &ForeignKeyDef> =
        base.foreign_keys.iter().map(|fk| (fk.name.clone(), fk)).collect();
    let check_fks: HashMap<String, &ForeignKeyDef> =
        check.foreign_keys.iter().map(|fk| (fk.name.clone(), fk)).collect();

    let base_checks: HashMap<String, &CheckDef> =
        base.checks.iter().map(|ch| (ch.name.clone(), ch)).collect();
    let check_checks: HashMap<String, &CheckDef> =
        check.checks.iter().map(|ch| (ch.name.clone(), ch)).collect();

    let column_diffs = diff_ref_map(&base_cols, &check_cols);
    let index_diffs = diff_ref_map(&base_idx, &check_idx);
    let fk_diffs = diff_ref_map(&base_fks, &check_fks);
    let check_diffs = diff_ref_map(&base_checks, &check_checks);

    let has_changes = column_diffs.iter().any(|d| d.status != DiffStatus::Same)
        || index_diffs.iter().any(|d| d.status != DiffStatus::Same)
        || fk_diffs.iter().any(|d| d.status != DiffStatus::Same)
        || check_diffs.iter().any(|d| d.status != DiffStatus::Same);

    TableDiff {
        name: base.name.clone(),
        status: if has_changes { DiffStatus::Changed } else { DiffStatus::Same },
        column_diffs,
        index_diffs,
        fk_diffs,
        check_diffs,
    }
}

/// Like diff_obj_map but works with reference values (avoids cloning the whole HashMap).
fn diff_ref_map<T>(
    base: &HashMap<String, &T>,
    check: &HashMap<String, &T>,
) -> Vec<ObjectDiff<T>>
where
    T: Clone + PartialEq + std::fmt::Debug,
{
    let mut all_keys: BTreeSet<String> = BTreeSet::new();
    all_keys.extend(base.keys().cloned());
    all_keys.extend(check.keys().cloned());

    all_keys
        .into_iter()
        .map(|key| {
            let b = base.get(&key).copied();
            let c = check.get(&key).copied();
            let status = match (b, c) {
                (Some(_), None) => DiffStatus::OnlyInBase,
                (None, Some(_)) => DiffStatus::OnlyInCheck,
                (Some(bv), Some(cv)) => {
                    if bv == cv { DiffStatus::Same } else { DiffStatus::Changed }
                }
                (None, None) => unreachable!(),
            };
            ObjectDiff {
                name: key,
                status,
                base: b.cloned(),
                check: c.cloned(),
            }
        })
        .collect()
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::*;
    use std::collections::HashMap;

    fn make_snapshot(databases: Vec<DatabaseSchema>) -> SchemaSnapshot {
        SchemaSnapshot {
            databases: databases.into_iter().map(|db| (db.name.clone(), db)).collect(),
        }
    }

    fn empty_db(name: &str) -> DatabaseSchema {
        DatabaseSchema {
            name: name.to_string(),
            tables: HashMap::new(),
            views: HashMap::new(),
            procedures: HashMap::new(),
            functions: HashMap::new(),
            triggers: HashMap::new(),
        }
    }

    fn simple_table(name: &str, col_type: &str) -> TableDef {
        TableDef {
            name: name.to_string(),
            columns: vec![ColumnDef {
                name: "id".to_string(),
                column_type: col_type.to_string(),
                is_nullable: false,
                column_default: None,
                extra: "auto_increment".to_string(),
                ordinal_position: 1,
            }],
            indexes: vec![],
            foreign_keys: vec![],
            checks: vec![],
        }
    }

    #[test]
    fn test_identical_snapshots_produce_same_status() {
        let db = empty_db("mydb");
        let base = make_snapshot(vec![db.clone()]);
        let check = make_snapshot(vec![db]);
        let report = diff(base, check, "base".into(), "check".into(), "now".into());
        assert_eq!(report.databases.len(), 1);
        assert_eq!(report.databases[0].status, DiffStatus::Same);
    }

    #[test]
    fn test_db_only_in_base() {
        let base = make_snapshot(vec![empty_db("mydb")]);
        let check = make_snapshot(vec![]);
        let report = diff(base, check, "base".into(), "check".into(), "now".into());
        assert_eq!(report.databases[0].status, DiffStatus::OnlyInBase);
    }

    #[test]
    fn test_db_only_in_check() {
        let base = make_snapshot(vec![]);
        let check = make_snapshot(vec![empty_db("mydb")]);
        let report = diff(base, check, "base".into(), "check".into(), "now".into());
        assert_eq!(report.databases[0].status, DiffStatus::OnlyInCheck);
    }

    #[test]
    fn test_changed_table_column_type() {
        let mut base_db = empty_db("mydb");
        base_db.tables.insert("users".into(), simple_table("users", "tinyint"));

        let mut check_db = empty_db("mydb");
        check_db.tables.insert("users".into(), simple_table("users", "int"));

        let report = diff(
            make_snapshot(vec![base_db]),
            make_snapshot(vec![check_db]),
            "base".into(), "check".into(), "now".into(),
        );

        let db_diff = &report.databases[0];
        assert_eq!(db_diff.status, DiffStatus::Changed);

        let table_diff = db_diff.tables.iter().find(|t| t.name == "users").unwrap();
        assert_eq!(table_diff.status, DiffStatus::Changed);

        let col_diff = table_diff.column_diffs.iter().find(|c| c.name == "id").unwrap();
        assert_eq!(col_diff.status, DiffStatus::Changed);
        assert_eq!(col_diff.base.as_ref().unwrap().column_type, "tinyint");
        assert_eq!(col_diff.check.as_ref().unwrap().column_type, "int");
    }

    #[test]
    fn test_table_only_in_base() {
        let mut base_db = empty_db("mydb");
        base_db.tables.insert("orders".into(), simple_table("orders", "int"));
        let check_db = empty_db("mydb");

        let report = diff(
            make_snapshot(vec![base_db]),
            make_snapshot(vec![check_db]),
            "base".into(), "check".into(), "now".into(),
        );

        let db_diff = &report.databases[0];
        let table_diff = db_diff.tables.iter().find(|t| t.name == "orders").unwrap();
        assert_eq!(table_diff.status, DiffStatus::OnlyInBase);
    }
}
