use std::collections::HashMap;

use std::time::Duration;

use anyhow::{Context, Result};
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use sqlx::{MySqlPool, Row};

use crate::config::ConnectionConfig;
use crate::schema::{
    CheckDef, ColumnDef, DatabaseSchema, ForeignKeyDef, IndexColumn, IndexDef,
    RoutineDef, SchemaSnapshot, TableDef, TriggerDef, ViewDef,
};

/// System databases that are always excluded from comparison.
const SYSTEM_SCHEMAS: &[&str] = &[
    "information_schema",
    "mysql",
    "performance_schema",
    "sys",
];

/// Connect to the given MySQL instance, discover all relevant databases,
/// and return a complete `SchemaSnapshot`.
pub async fn fetch_snapshot(
    config: &ConnectionConfig,
    db_filter: Option<&[String]>,
) -> Result<SchemaSnapshot> {
    let pool = connect(config)
        .await
        .with_context(|| format!("Failed to connect to {}:{}", config.host, config.port))?;

    let databases = list_databases(&pool, db_filter).await?;

    let mut db_schemas = HashMap::new();
    for db_name in databases {
        let schema = fetch_database_schema(&pool, &db_name)
            .await
            .with_context(|| format!("Failed to fetch schema for database `{}`", db_name))?;
        db_schemas.insert(db_name, schema);
    }

    Ok(SchemaSnapshot { databases: db_schemas })
}

// ── Connection ────────────────────────────────────────────────────────────────

async fn connect(config: &ConnectionConfig) -> Result<MySqlPool> {
    let opts = MySqlConnectOptions::new()
        .host(&config.host)
        .port(config.port)
        .username(&config.user)
        .password(&config.password);

    let pool = MySqlPoolOptions::new()
        .max_connections(3)
        .acquire_timeout(Duration::from_secs(30))
        .connect_with(opts)
        .await?;

    Ok(pool)
}

// ── Database discovery ────────────────────────────────────────────────────────

async fn list_databases(pool: &MySqlPool, filter: Option<&[String]>) -> Result<Vec<String>> {
    let rows = sqlx::query("SHOW DATABASES")
        .fetch_all(pool)
        .await
        .context("Failed to list databases")?;

    let mut names: Vec<String> = rows
        .into_iter()
        .filter_map(|row| row.try_get::<String, _>(0).ok())
        .filter(|name| !SYSTEM_SCHEMAS.contains(&name.as_str()))
        .collect();

    if let Some(filter) = filter {
        names.retain(|name| filter.contains(name));
    }

    names.sort();
    Ok(names)
}

// ── Full database schema fetch ────────────────────────────────────────────────

async fn fetch_database_schema(pool: &MySqlPool, schema: &str) -> Result<DatabaseSchema> {
    let tables = fetch_tables(pool, schema).await?;
    let views = fetch_views(pool, schema).await?;
    let (procedures, functions) = fetch_routines(pool, schema).await?;
    let triggers = fetch_triggers(pool, schema).await?;

    Ok(DatabaseSchema {
        name: schema.to_string(),
        tables,
        views,
        procedures,
        functions,
        triggers,
    })
}

// ── Tables ────────────────────────────────────────────────────────────────────

async fn fetch_tables(pool: &MySqlPool, schema: &str) -> Result<HashMap<String, TableDef>> {
    let columns = fetch_columns(pool, schema).await?;
    let indexes = fetch_indexes(pool, schema).await?;
    let foreign_keys = fetch_foreign_keys(pool, schema).await?;
    let checks = fetch_check_constraints(pool, schema).await?;

    // Collect all table names from all sources so tables with no columns
    // (contrived but possible during migrations) are not silently dropped.
    let mut all_tables: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    all_tables.extend(columns.keys().cloned());
    all_tables.extend(indexes.keys().cloned());
    all_tables.extend(foreign_keys.keys().cloned());
    all_tables.extend(checks.keys().cloned());

    let mut tables = HashMap::new();
    for table_name in all_tables {
        let mut table_columns = columns.get(&table_name).cloned().unwrap_or_default();
        table_columns.sort_by_key(|c| c.ordinal_position);

        let mut table_indexes = indexes.get(&table_name).cloned().unwrap_or_default();
        table_indexes.sort_by(|a, b| a.name.cmp(&b.name));

        let mut table_fks = foreign_keys.get(&table_name).cloned().unwrap_or_default();
        table_fks.sort_by(|a, b| a.name.cmp(&b.name));

        let mut table_checks = checks.get(&table_name).cloned().unwrap_or_default();
        table_checks.sort_by(|a, b| a.name.cmp(&b.name));

        tables.insert(
            table_name.clone(),
            TableDef {
                name: table_name,
                columns: table_columns,
                indexes: table_indexes,
                foreign_keys: table_fks,
                checks: table_checks,
            },
        );
    }

    Ok(tables)
}

/// Returns: table_name → Vec<ColumnDef>
async fn fetch_columns(
    pool: &MySqlPool,
    schema: &str,
) -> Result<HashMap<String, Vec<ColumnDef>>> {
    let rows = sqlx::query(
        "SELECT c.TABLE_NAME, c.COLUMN_NAME, c.ORDINAL_POSITION, \
                c.COLUMN_DEFAULT, c.IS_NULLABLE, c.COLUMN_TYPE, c.EXTRA \
         FROM information_schema.COLUMNS c \
         JOIN information_schema.TABLES t \
             ON t.TABLE_SCHEMA = c.TABLE_SCHEMA AND t.TABLE_NAME = c.TABLE_NAME \
         WHERE c.TABLE_SCHEMA = ? \
           AND t.TABLE_TYPE = 'BASE TABLE' \
         ORDER BY c.TABLE_NAME, c.ORDINAL_POSITION",
    )
    .bind(schema)
    .fetch_all(pool)
    .await
    .context("Failed to fetch columns")?;

    let mut result: HashMap<String, Vec<ColumnDef>> = HashMap::new();
    for row in rows {
        let table_name: String = row.try_get("TABLE_NAME")?;
        let is_nullable: String = row.try_get("IS_NULLABLE")?;
        let ordinal: u32 = row.try_get("ORDINAL_POSITION")?;
        let col = ColumnDef {
            name: row.try_get("COLUMN_NAME")?,
            column_type: row.try_get("COLUMN_TYPE")?,
            is_nullable: is_nullable == "YES",
            column_default: row.try_get("COLUMN_DEFAULT")?,
            extra: row.try_get("EXTRA")?,
            ordinal_position: ordinal,
        };
        result.entry(table_name).or_default().push(col);
    }
    Ok(result)
}

/// Returns: table_name → Vec<IndexDef>
async fn fetch_indexes(
    pool: &MySqlPool,
    schema: &str,
) -> Result<HashMap<String, Vec<IndexDef>>> {
    let rows = sqlx::query(
        "SELECT TABLE_NAME, INDEX_NAME, SEQ_IN_INDEX, COLUMN_NAME, \
                NON_UNIQUE, INDEX_TYPE, SUB_PART \
         FROM information_schema.STATISTICS \
         WHERE TABLE_SCHEMA = ? \
         ORDER BY TABLE_NAME, INDEX_NAME, SEQ_IN_INDEX",
    )
    .bind(schema)
    .fetch_all(pool)
    .await
    .context("Failed to fetch indexes")?;

    // table_name → index_name → (is_unique, index_type, columns)
    let mut raw: HashMap<String, HashMap<String, (bool, String, Vec<IndexColumn>)>> = HashMap::new();

    for row in rows {
        let table_name: String = row.try_get("TABLE_NAME")?;
        let index_name: String = row.try_get("INDEX_NAME")?;
        let seq: u32 = row.try_get("SEQ_IN_INDEX")?;
        let non_unique: i32 = row.try_get("NON_UNIQUE")?;
        let index_type: String = row.try_get("INDEX_TYPE")?;
        let sub_part: Option<i64> = row.try_get("SUB_PART")?;
        let column_name: String = row.try_get("COLUMN_NAME")?;

        let entry = raw
            .entry(table_name)
            .or_default()
            .entry(index_name)
            .or_insert_with(|| (non_unique == 0i32, index_type, vec![]));

        entry.2.push(IndexColumn {
            column_name,
            seq_in_index: seq,
            sub_part,
        });
    }

    let mut result: HashMap<String, Vec<IndexDef>> = HashMap::new();
    for (table_name, index_map) in raw {
        let mut indexes: Vec<IndexDef> = index_map
            .into_iter()
            .map(|(name, (is_unique, index_type, mut cols))| {
                cols.sort_by_key(|c| c.seq_in_index);
                IndexDef { name, index_type, is_unique, columns: cols }
            })
            .collect();
        indexes.sort_by(|a, b| a.name.cmp(&b.name));
        result.insert(table_name, indexes);
    }

    Ok(result)
}

/// Returns: table_name → Vec<ForeignKeyDef>
async fn fetch_foreign_keys(
    pool: &MySqlPool,
    schema: &str,
) -> Result<HashMap<String, Vec<ForeignKeyDef>>> {
    let rows = sqlx::query(
        "SELECT kcu.CONSTRAINT_NAME, kcu.TABLE_NAME, kcu.COLUMN_NAME, \
                kcu.REFERENCED_TABLE_NAME, kcu.REFERENCED_COLUMN_NAME, \
                kcu.ORDINAL_POSITION, rc.DELETE_RULE, rc.UPDATE_RULE \
         FROM information_schema.KEY_COLUMN_USAGE kcu \
         JOIN information_schema.REFERENTIAL_CONSTRAINTS rc \
             ON rc.CONSTRAINT_NAME = kcu.CONSTRAINT_NAME \
            AND rc.CONSTRAINT_SCHEMA = kcu.TABLE_SCHEMA \
         WHERE kcu.TABLE_SCHEMA = ? \
           AND kcu.REFERENCED_TABLE_NAME IS NOT NULL \
         ORDER BY kcu.TABLE_NAME, kcu.CONSTRAINT_NAME, kcu.ORDINAL_POSITION",
    )
    .bind(schema)
    .fetch_all(pool)
    .await
    .context("Failed to fetch foreign keys")?;

    // table → fk_name → (ref_table, on_delete, on_update, columns[], ref_columns[])
    let mut raw: HashMap<String, HashMap<String, (String, String, String, Vec<String>, Vec<String>)>> =
        HashMap::new();

    for row in rows {
        let table_name: String = row.try_get("TABLE_NAME")?;
        let constraint_name: String = row.try_get("CONSTRAINT_NAME")?;
        let column_name: String = row.try_get("COLUMN_NAME")?;
        let ref_table: String = row.try_get("REFERENCED_TABLE_NAME")?;
        let ref_column: String = row.try_get("REFERENCED_COLUMN_NAME")?;
        let delete_rule: String = row.try_get("DELETE_RULE")?;
        let update_rule: String = row.try_get("UPDATE_RULE")?;

        let entry = raw
            .entry(table_name)
            .or_default()
            .entry(constraint_name)
            .or_insert_with(|| (ref_table, delete_rule, update_rule, vec![], vec![]));

        entry.3.push(column_name);
        entry.4.push(ref_column);
    }

    let mut result: HashMap<String, Vec<ForeignKeyDef>> = HashMap::new();
    for (table_name, fk_map) in raw {
        let mut fks: Vec<ForeignKeyDef> = fk_map
            .into_iter()
            .map(|(name, (ref_table, on_delete, on_update, columns, ref_columns))| {
                ForeignKeyDef { name, columns, ref_table, ref_columns, on_delete, on_update }
            })
            .collect();
        fks.sort_by(|a, b| a.name.cmp(&b.name));
        result.insert(table_name, fks);
    }

    Ok(result)
}

/// Returns: table_name → Vec<CheckDef>
/// Returns empty map gracefully on MySQL < 8.0 where CHECK_CONSTRAINTS doesn't exist.
async fn fetch_check_constraints(
    pool: &MySqlPool,
    schema: &str,
) -> Result<HashMap<String, Vec<CheckDef>>> {
    let result = sqlx::query(
        "SELECT cc.CONSTRAINT_NAME, tc.TABLE_NAME, cc.CHECK_CLAUSE \
         FROM information_schema.CHECK_CONSTRAINTS cc \
         JOIN information_schema.TABLE_CONSTRAINTS tc \
             ON tc.CONSTRAINT_NAME = cc.CONSTRAINT_NAME \
            AND tc.CONSTRAINT_SCHEMA = cc.CONSTRAINT_SCHEMA \
         WHERE cc.CONSTRAINT_SCHEMA = ? \
         ORDER BY tc.TABLE_NAME, cc.CONSTRAINT_NAME",
    )
    .bind(schema)
    .fetch_all(pool)
    .await;

    let rows = match result {
        Ok(rows) => rows,
        // SQLSTATE 42S02 = ER_NO_SUCH_TABLE: MySQL < 8.0 doesn't have
        // CHECK_CONSTRAINTS in information_schema. Treat as empty.
        Err(sqlx::Error::Database(ref dberr)) if dberr.code().as_deref() == Some("42S02") => {
            return Ok(HashMap::new());
        }
        Err(e) => return Err(e).context("Failed to fetch check constraints"),
    };

    let mut result: HashMap<String, Vec<CheckDef>> = HashMap::new();
    for row in rows {
        let table_name: String = row.try_get("TABLE_NAME")?;
        let check = CheckDef {
            name: row.try_get("CONSTRAINT_NAME")?,
            clause: row.try_get("CHECK_CLAUSE")?,
        };
        result.entry(table_name).or_default().push(check);
    }

    Ok(result)
}

// ── Views ─────────────────────────────────────────────────────────────────────

async fn fetch_views(pool: &MySqlPool, schema: &str) -> Result<HashMap<String, ViewDef>> {
    let rows = sqlx::query(
        "SELECT TABLE_NAME, VIEW_DEFINITION \
         FROM information_schema.VIEWS \
         WHERE TABLE_SCHEMA = ? \
         ORDER BY TABLE_NAME",
    )
    .bind(schema)
    .fetch_all(pool)
    .await
    .context("Failed to fetch views")?;

    let mut result = HashMap::new();
    for row in rows {
        let name: String = row.try_get("TABLE_NAME")?;
        let definition: String = row.try_get("VIEW_DEFINITION")?;
        result.insert(
            name.clone(),
            ViewDef {
                name,
                definition: normalize_sql(&definition),
            },
        );
    }

    Ok(result)
}

// ── Routines (procedures + functions) ────────────────────────────────────────

/// Returns (procedures_map, functions_map).
async fn fetch_routines(
    pool: &MySqlPool,
    schema: &str,
) -> Result<(HashMap<String, RoutineDef>, HashMap<String, RoutineDef>)> {
    let rows = sqlx::query(
        "SELECT ROUTINE_NAME, ROUTINE_TYPE, ROUTINE_DEFINITION, \
                IS_DETERMINISTIC, SQL_DATA_ACCESS, SECURITY_TYPE \
         FROM information_schema.ROUTINES \
         WHERE ROUTINE_SCHEMA = ? \
         ORDER BY ROUTINE_TYPE, ROUTINE_NAME",
    )
    .bind(schema)
    .fetch_all(pool)
    .await
    .context("Failed to fetch routines")?;

    let mut procedures = HashMap::new();
    let mut functions = HashMap::new();

    for row in rows {
        let name: String = row.try_get("ROUTINE_NAME")?;
        let routine_type: String = row.try_get("ROUTINE_TYPE")?;
        let definition: Option<String> = row.try_get("ROUTINE_DEFINITION")?;
        let routine = RoutineDef {
            name: name.clone(),
            routine_type: routine_type.clone(),
            definition: normalize_sql(&definition.unwrap_or_default()),
            is_deterministic: row.try_get("IS_DETERMINISTIC")?,
            sql_data_access: row.try_get("SQL_DATA_ACCESS")?,
            security_type: row.try_get("SECURITY_TYPE")?,
        };
        if routine_type == "PROCEDURE" {
            procedures.insert(name, routine);
        } else {
            functions.insert(name, routine);
        }
    }

    Ok((procedures, functions))
}

// ── Triggers ──────────────────────────────────────────────────────────────────

async fn fetch_triggers(
    pool: &MySqlPool,
    schema: &str,
) -> Result<HashMap<String, TriggerDef>> {
    let rows = sqlx::query(
        "SELECT TRIGGER_NAME, EVENT_MANIPULATION, EVENT_OBJECT_TABLE, \
                ACTION_STATEMENT, ACTION_TIMING, ACTION_ORDER \
         FROM information_schema.TRIGGERS \
         WHERE TRIGGER_SCHEMA = ? \
         ORDER BY EVENT_OBJECT_TABLE, TRIGGER_NAME",
    )
    .bind(schema)
    .fetch_all(pool)
    .await
    .context("Failed to fetch triggers")?;

    let mut result = HashMap::new();
    for row in rows {
        let name: String = row.try_get("TRIGGER_NAME")?;
        let action_order: u32 = row.try_get("ACTION_ORDER")?;
        let statement: String = row.try_get("ACTION_STATEMENT")?;
        let trigger = TriggerDef {
            name: name.clone(),
            event: row.try_get("EVENT_MANIPULATION")?,
            timing: row.try_get("ACTION_TIMING")?,
            table_name: row.try_get("EVENT_OBJECT_TABLE")?,
            statement: normalize_sql(&statement),
            action_order,
        };
        result.insert(name, trigger);
    }

    Ok(result)
}

// ── Utilities ─────────────────────────────────────────────────────────────────

/// Collapse all whitespace sequences to a single space and trim.
/// Used to normalize SQL body text so that cosmetic whitespace differences
/// don't show up as false-positive changes.
fn normalize_sql(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}
