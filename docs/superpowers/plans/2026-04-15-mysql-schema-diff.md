# MySQL Schema Diff — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a Rust CLI binary (`mysql-schema-diff`) that compares full MySQL schemas between two databases (tables, indexes, FKs, check constraints, views, procedures, functions, triggers) and writes a structured Markdown diff report.

**Architecture:** Parse CLI args via `clap` → connect to both MySQL instances concurrently via `sqlx` → query `information_schema` for all schema objects → diff two `SchemaSnapshot`s field-by-field → render a `DiffReport` to a Markdown file.

**Tech Stack:** Rust 2021 edition, sqlx 0.7 (MySQL, runtime-tokio-rustls), clap 4 (derive), tokio 1 (full), chrono 0.4, anyhow 1.

---

## File Map

| File | Responsibility |
|------|---------------|
| `Cargo.toml` | Project manifest and dependencies |
| `src/main.rs` | Entry point: parse args, run async main, write output file |
| `src/config.rs` | `Cli` struct (clap derive), `ConnectionConfig`, `Options` |
| `src/schema/mod.rs` | All typed schema structs (`SchemaSnapshot`, `TableDef`, etc.) |
| `src/fetcher.rs` | Async functions to query `information_schema`, returns `SchemaSnapshot` |
| `src/differ.rs` | Diff two `SchemaSnapshot`s → `DiffReport` |
| `src/reporter.rs` | Render `DiffReport` → Markdown `String` |

---

## Task 1: Initialize Cargo project

**Files:**
- Create: `Cargo.toml`
- Create: `src/main.rs` (stub)

- [ ] **Step 1: Initialize the project**

```bash
cd /home/honghaowu/task/converge-neoleap-sqlschema-check
cargo init --name mysql-schema-diff
```

Expected output: `Created binary (application) package`

- [ ] **Step 2: Replace `Cargo.toml` with the full dependency list**

Open `Cargo.toml` and replace its contents entirely with:

```toml
[package]
name = "mysql-schema-diff"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "mysql-schema-diff"
path = "src/main.rs"

[dependencies]
sqlx = { version = "0.7", features = ["mysql", "runtime-tokio-rustls"] }
clap = { version = "4", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
chrono = "0.4"
anyhow = "1"
```

- [ ] **Step 3: Replace `src/main.rs` with a stub that compiles**

```rust
fn main() {
    println!("mysql-schema-diff");
}
```

- [ ] **Step 4: Verify the project compiles**

```bash
cargo build
```

Expected: compiles successfully (downloads dependencies, may take a few minutes on first run).

- [ ] **Step 5: Commit**

```bash
git init
git add Cargo.toml Cargo.lock src/main.rs
git commit -m "feat: initialize Rust project with dependencies"
```

---

## Task 2: Schema types (`src/schema/mod.rs`)

**Files:**
- Create: `src/schema/mod.rs`
- Modify: `src/main.rs` — add `mod schema;`

These are pure data structs. No database access, no logic — just the shapes we use everywhere else.

- [ ] **Step 1: Create `src/schema/` directory and `mod.rs`**

```bash
mkdir -p src/schema
```

- [ ] **Step 2: Write `src/schema/mod.rs`**

```rust
use std::collections::HashMap;

/// Top-level snapshot of all schemas from one MySQL instance.
#[derive(Debug, Clone, PartialEq)]
pub struct SchemaSnapshot {
    /// Key: database (schema) name.
    pub databases: HashMap<String, DatabaseSchema>,
}

/// All objects inside one MySQL database/schema.
#[derive(Debug, Clone, PartialEq)]
pub struct DatabaseSchema {
    pub name: String,
    pub tables: HashMap<String, TableDef>,
    pub views: HashMap<String, ViewDef>,
    pub procedures: HashMap<String, RoutineDef>,
    pub functions: HashMap<String, RoutineDef>,
    pub triggers: HashMap<String, TriggerDef>,
}

/// A single base table (excludes views).
#[derive(Debug, Clone, PartialEq)]
pub struct TableDef {
    pub name: String,
    /// Ordered by ordinal_position.
    pub columns: Vec<ColumnDef>,
    /// Ordered by index name.
    pub indexes: Vec<IndexDef>,
    /// Ordered by constraint name.
    pub foreign_keys: Vec<ForeignKeyDef>,
    /// Ordered by constraint name. Empty on MySQL < 8.0.
    pub checks: Vec<CheckDef>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColumnDef {
    pub name: String,
    /// Full type string from MySQL, e.g. "varchar(255)", "int unsigned".
    pub column_type: String,
    pub is_nullable: bool,
    pub column_default: Option<String>,
    /// e.g. "auto_increment", "on update CURRENT_TIMESTAMP", or empty string.
    pub extra: String,
    pub ordinal_position: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndexDef {
    pub name: String,
    /// BTREE or HASH.
    pub index_type: String,
    pub is_unique: bool,
    /// Ordered by seq_in_index.
    pub columns: Vec<IndexColumn>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndexColumn {
    pub column_name: String,
    pub seq_in_index: u32,
    /// Prefix length for partial indexes (e.g. on TEXT columns). None = full column.
    pub sub_part: Option<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForeignKeyDef {
    pub name: String,
    /// Columns in this table, in key order.
    pub columns: Vec<String>,
    pub ref_table: String,
    /// Corresponding columns in the referenced table.
    pub ref_columns: Vec<String>,
    pub on_delete: String,
    pub on_update: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CheckDef {
    pub name: String,
    pub clause: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ViewDef {
    pub name: String,
    /// Normalized (whitespace-collapsed) SQL definition.
    pub definition: String,
}

/// A stored procedure or function.
#[derive(Debug, Clone, PartialEq)]
pub struct RoutineDef {
    pub name: String,
    /// "PROCEDURE" or "FUNCTION".
    pub routine_type: String,
    /// Normalized body.
    pub definition: String,
    /// "YES" or "NO".
    pub is_deterministic: String,
    /// "CONTAINS SQL", "NO SQL", "READS SQL DATA", or "MODIFIES SQL DATA".
    pub sql_data_access: String,
    /// "DEFINER" or "INVOKER".
    pub security_type: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TriggerDef {
    pub name: String,
    /// "INSERT", "UPDATE", or "DELETE".
    pub event: String,
    /// "BEFORE" or "AFTER".
    pub timing: String,
    pub table_name: String,
    /// Normalized trigger body.
    pub statement: String,
    pub action_order: u32,
}
```

- [ ] **Step 3: Add module declaration to `src/main.rs`**

Replace `src/main.rs` with:

```rust
mod schema;

fn main() {
    println!("mysql-schema-diff");
}
```

- [ ] **Step 4: Verify it compiles**

```bash
cargo build
```

Expected: compiles with no errors.

- [ ] **Step 5: Commit**

```bash
git add src/schema/mod.rs src/main.rs
git commit -m "feat: add schema type definitions"
```

---

## Task 3: CLI & config (`src/config.rs`)

**Files:**
- Create: `src/config.rs`
- Modify: `src/main.rs` — add `mod config;`

- [ ] **Step 1: Write `src/config.rs`**

```rust
use clap::Parser;

/// MySQL schema diff tool — compares two MySQL instances and produces a Markdown report.
#[derive(Parser, Debug)]
#[command(name = "mysql-schema-diff")]
pub struct Cli {
    /// Base database host
    #[arg(long)]
    pub base_host: String,

    /// Base database port
    #[arg(long, default_value = "3306")]
    pub base_port: u16,

    /// Base database username
    #[arg(long)]
    pub base_user: String,

    /// Base database password
    #[arg(long)]
    pub base_password: String,

    /// Check database host
    #[arg(long)]
    pub check_host: String,

    /// Check database port
    #[arg(long, default_value = "3306")]
    pub check_port: u16,

    /// Check database username
    #[arg(long)]
    pub check_user: String,

    /// Check database password
    #[arg(long)]
    pub check_password: String,

    /// Comma-separated list of databases to compare.
    /// If omitted, all non-system databases are compared.
    #[arg(long, value_delimiter = ',')]
    pub databases: Option<Vec<String>>,

    /// Output file path. Defaults to ./schema-diff-YYYYMMDD-HHMMSS.md
    #[arg(long)]
    pub output: Option<String>,
}

/// Connection parameters for one MySQL instance.
#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}

/// Non-connection options parsed from the CLI.
#[derive(Debug)]
pub struct Options {
    /// If Some, only compare these database names. If None, compare all non-system databases.
    pub databases: Option<Vec<String>>,
    /// Output file path (may be None; caller generates the default name).
    pub output: Option<String>,
    /// Human-readable label for the base instance used in the report.
    pub base_label: String,
    /// Human-readable label for the check instance used in the report.
    pub check_label: String,
}

impl Cli {
    pub fn base_config(&self) -> ConnectionConfig {
        ConnectionConfig {
            host: self.base_host.clone(),
            port: self.base_port,
            user: self.base_user.clone(),
            password: self.base_password.clone(),
        }
    }

    pub fn check_config(&self) -> ConnectionConfig {
        ConnectionConfig {
            host: self.check_host.clone(),
            port: self.check_port,
            user: self.check_user.clone(),
            password: self.check_password.clone(),
        }
    }

    pub fn options(&self) -> Options {
        Options {
            databases: self.databases.clone(),
            output: self.output.clone(),
            base_label: format!("{}:{}", self.base_host, self.base_port),
            check_label: format!("{}:{}", self.check_host, self.check_port),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_cli_parses_required_args() {
        let cli = Cli::parse_from([
            "mysql-schema-diff",
            "--base-host", "10.0.0.1",
            "--base-user", "root",
            "--base-password", "secret",
            "--check-host", "10.0.0.2",
            "--check-user", "root",
            "--check-password", "secret2",
        ]);
        assert_eq!(cli.base_host, "10.0.0.1");
        assert_eq!(cli.base_port, 3306);
        assert_eq!(cli.check_host, "10.0.0.2");
        assert_eq!(cli.check_port, 3306);
        assert!(cli.databases.is_none());
        assert!(cli.output.is_none());
    }

    #[test]
    fn test_cli_parses_optional_databases() {
        let cli = Cli::parse_from([
            "mysql-schema-diff",
            "--base-host", "h1", "--base-user", "u", "--base-password", "p",
            "--check-host", "h2", "--check-user", "u", "--check-password", "p",
            "--databases", "db1,db2,db3",
        ]);
        assert_eq!(cli.databases, Some(vec!["db1".into(), "db2".into(), "db3".into()]));
    }

    #[test]
    fn test_labels() {
        let cli = Cli::parse_from([
            "mysql-schema-diff",
            "--base-host", "10.0.0.1", "--base-port", "3307",
            "--base-user", "u", "--base-password", "p",
            "--check-host", "10.0.0.2",
            "--check-user", "u", "--check-password", "p",
        ]);
        let opts = cli.options();
        assert_eq!(opts.base_label, "10.0.0.1:3307");
        assert_eq!(opts.check_label, "10.0.0.2:3306");
    }
}
```

- [ ] **Step 2: Add module declaration to `src/main.rs`**

```rust
mod config;
mod schema;

fn main() {
    println!("mysql-schema-diff");
}
```

- [ ] **Step 3: Run the tests**

```bash
cargo test config
```

Expected:
```
test config::tests::test_cli_parses_required_args ... ok
test config::tests::test_cli_parses_optional_databases ... ok
test config::tests::test_labels ... ok
```

- [ ] **Step 4: Commit**

```bash
git add src/config.rs src/main.rs
git commit -m "feat: add CLI argument parsing with clap"
```

---

## Task 4: Differ (`src/differ.rs`)

**Files:**
- Create: `src/differ.rs`
- Modify: `src/main.rs` — add `mod differ;`

This module takes two `SchemaSnapshot`s and returns a `DiffReport`. No database access.

- [ ] **Step 1: Write `src/differ.rs`**

```rust
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

    let make_obj = |name: &str, val: &dyn std::any::Any| -> DiffStatus { status.clone() };

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
```

- [ ] **Step 2: Add module declaration to `src/main.rs`**

```rust
mod config;
mod differ;
mod schema;

fn main() {
    println!("mysql-schema-diff");
}
```

- [ ] **Step 3: Run the tests**

```bash
cargo test differ
```

Expected:
```
test differ::tests::test_identical_snapshots_produce_same_status ... ok
test differ::tests::test_db_only_in_base ... ok
test differ::tests::test_db_only_in_check ... ok
test differ::tests::test_changed_table_column_type ... ok
test differ::tests::test_table_only_in_base ... ok
```

- [ ] **Step 4: Commit**

```bash
git add src/differ.rs src/main.rs
git commit -m "feat: add diff engine with unit tests"
```

---

## Task 5: Reporter (`src/reporter.rs`)

**Files:**
- Create: `src/reporter.rs`
- Modify: `src/main.rs` — add `mod reporter;`

The reporter turns a `DiffReport` into a Markdown string. No database access, no I/O.

- [ ] **Step 1: Write `src/reporter.rs`**

```rust
use crate::differ::{DatabaseDiff, DiffReport, DiffStatus, ObjectDiff, TableDiff};
use crate::schema::{
    CheckDef, ColumnDef, ForeignKeyDef, IndexDef, RoutineDef, TriggerDef, ViewDef,
};

/// Render a `DiffReport` to a Markdown string.
pub fn render(report: &DiffReport) -> String {
    let mut out = String::new();

    // Header
    out.push_str("# Schema Diff Report\n\n");
    out.push_str(&format!("**Generated:** {}\n\n", report.generated_at));
    out.push_str(&format!("**Base:**  `{}`\n\n", report.base_label));
    out.push_str(&format!("**Check:** `{}`\n\n", report.check_label));
    out.push_str("---\n\n");

    // Summary
    out.push_str("## Summary\n\n");
    let (dbs_compared, dbs_base, dbs_check, dbs_changed) = count_statuses(&report.databases);
    out.push_str(&format!(
        "- **Databases compared:** {}\n",
        dbs_compared
    ));
    out.push_str(&format!(
        "- **Databases only in base:** {}, only in check: {}, changed: {}\n\n",
        dbs_base, dbs_check, dbs_changed
    ));

    // Per-database sections — skip databases with status Same
    for db in &report.databases {
        if db.status == DiffStatus::Same {
            continue;
        }
        out.push_str(&render_database(db));
    }

    out
}

fn render_database(db: &DatabaseDiff) -> String {
    let mut out = String::new();

    match db.status {
        DiffStatus::OnlyInBase => {
            out.push_str(&format!("## ✚ Database only in base: `{}`\n\n", db.name));
        }
        DiffStatus::OnlyInCheck => {
            out.push_str(&format!("## ✖ Database only in check: `{}`\n\n", db.name));
        }
        DiffStatus::Changed | DiffStatus::Same => {
            out.push_str(&format!("## Database: `{}`\n\n", db.name));
        }
    }

    // Tables
    let relevant_tables: Vec<&TableDiff> = db
        .tables
        .iter()
        .filter(|t| t.status != DiffStatus::Same)
        .collect();
    if !relevant_tables.is_empty() {
        out.push_str("### Tables\n\n");
        for table in relevant_tables {
            out.push_str(&render_table(table));
        }
    }

    // Views
    let relevant_views: Vec<&ObjectDiff<ViewDef>> = db
        .views
        .iter()
        .filter(|v| v.status != DiffStatus::Same)
        .collect();
    if !relevant_views.is_empty() {
        out.push_str("### Views\n\n");
        for view in relevant_views {
            out.push_str(&render_simple_object(
                &view.name, &view.status,
                view.base.as_ref().map(|v| v.definition.as_str()),
                view.check.as_ref().map(|v| v.definition.as_str()),
                "SQL Definition",
            ));
        }
    }

    // Procedures
    let relevant_procs: Vec<&ObjectDiff<RoutineDef>> = db
        .procedures
        .iter()
        .filter(|p| p.status != DiffStatus::Same)
        .collect();
    if !relevant_procs.is_empty() {
        out.push_str("### Procedures\n\n");
        for proc in relevant_procs {
            out.push_str(&render_routine_diff(proc));
        }
    }

    // Functions
    let relevant_fns: Vec<&ObjectDiff<RoutineDef>> = db
        .functions
        .iter()
        .filter(|f| f.status != DiffStatus::Same)
        .collect();
    if !relevant_fns.is_empty() {
        out.push_str("### Functions\n\n");
        for func in relevant_fns {
            out.push_str(&render_routine_diff(func));
        }
    }

    // Triggers
    let relevant_triggers: Vec<&ObjectDiff<TriggerDef>> = db
        .triggers
        .iter()
        .filter(|t| t.status != DiffStatus::Same)
        .collect();
    if !relevant_triggers.is_empty() {
        out.push_str("### Triggers\n\n");
        for trigger in relevant_triggers {
            out.push_str(&render_trigger_diff(trigger));
        }
    }

    out
}

fn render_table(table: &TableDiff) -> String {
    let mut out = String::new();

    match table.status {
        DiffStatus::OnlyInBase => {
            out.push_str(&format!("#### ✚ Only in base: `{}`\n\n", table.name));
            out.push_str(&render_column_list(
                table.column_diffs.iter().filter_map(|c| c.base.as_ref()).collect(),
            ));
        }
        DiffStatus::OnlyInCheck => {
            out.push_str(&format!("#### ✖ Only in check: `{}`\n\n", table.name));
            out.push_str(&render_column_list(
                table.column_diffs.iter().filter_map(|c| c.check.as_ref()).collect(),
            ));
        }
        DiffStatus::Changed => {
            out.push_str(&format!("#### ⚠ Changed: `{}`\n\n", table.name));

            // Columns
            let changed_cols: Vec<&ObjectDiff<ColumnDef>> = table
                .column_diffs
                .iter()
                .filter(|c| c.status != DiffStatus::Same)
                .collect();
            if !changed_cols.is_empty() {
                out.push_str("##### Columns\n\n");
                out.push_str("| Column | Status | Base | Check |\n");
                out.push_str("|--------|--------|------|-------|\n");
                for col in changed_cols {
                    let base_str = col.base.as_ref().map(format_column).unwrap_or_default();
                    let check_str = col.check.as_ref().map(format_column).unwrap_or_default();
                    out.push_str(&format!(
                        "| `{}` | {} | {} | {} |\n",
                        col.name,
                        status_icon(&col.status),
                        base_str,
                        check_str,
                    ));
                }
                out.push('\n');
            }

            // Indexes
            let changed_idx: Vec<&ObjectDiff<IndexDef>> = table
                .index_diffs
                .iter()
                .filter(|i| i.status != DiffStatus::Same)
                .collect();
            if !changed_idx.is_empty() {
                out.push_str("##### Indexes\n\n");
                out.push_str("| Index | Status | Base | Check |\n");
                out.push_str("|-------|--------|------|-------|\n");
                for idx in changed_idx {
                    let base_str = idx.base.as_ref().map(format_index).unwrap_or_default();
                    let check_str = idx.check.as_ref().map(format_index).unwrap_or_default();
                    out.push_str(&format!(
                        "| `{}` | {} | {} | {} |\n",
                        idx.name, status_icon(&idx.status), base_str, check_str,
                    ));
                }
                out.push('\n');
            }

            // Foreign Keys
            let changed_fks: Vec<&ObjectDiff<ForeignKeyDef>> = table
                .fk_diffs
                .iter()
                .filter(|fk| fk.status != DiffStatus::Same)
                .collect();
            if !changed_fks.is_empty() {
                out.push_str("##### Foreign Keys\n\n");
                out.push_str("| FK | Status | Base | Check |\n");
                out.push_str("|----|--------|------|-------|\n");
                for fk in changed_fks {
                    let base_str = fk.base.as_ref().map(format_fk).unwrap_or_default();
                    let check_str = fk.check.as_ref().map(format_fk).unwrap_or_default();
                    out.push_str(&format!(
                        "| `{}` | {} | {} | {} |\n",
                        fk.name, status_icon(&fk.status), base_str, check_str,
                    ));
                }
                out.push('\n');
            }

            // Check constraints
            let changed_checks: Vec<&ObjectDiff<CheckDef>> = table
                .check_diffs
                .iter()
                .filter(|c| c.status != DiffStatus::Same)
                .collect();
            if !changed_checks.is_empty() {
                out.push_str("##### Check Constraints\n\n");
                out.push_str("| Constraint | Status | Base | Check |\n");
                out.push_str("|------------|--------|------|-------|\n");
                for ch in changed_checks {
                    let base_str = ch.base.as_ref().map(|c| c.clause.as_str()).unwrap_or("");
                    let check_str = ch.check.as_ref().map(|c| c.clause.as_str()).unwrap_or("");
                    out.push_str(&format!(
                        "| `{}` | {} | `{}` | `{}` |\n",
                        ch.name, status_icon(&ch.status), base_str, check_str,
                    ));
                }
                out.push('\n');
            }
        }
        DiffStatus::Same => {}
    }

    out
}

fn render_simple_object(
    name: &str,
    status: &DiffStatus,
    base_def: Option<&str>,
    check_def: Option<&str>,
    label: &str,
) -> String {
    let mut out = String::new();
    match status {
        DiffStatus::OnlyInBase => {
            out.push_str(&format!("#### ✚ Only in base: `{}`\n\n", name));
            if let Some(def) = base_def {
                out.push_str(&format!("```sql\n{}\n```\n\n", def));
            }
        }
        DiffStatus::OnlyInCheck => {
            out.push_str(&format!("#### ✖ Only in check: `{}`\n\n", name));
            if let Some(def) = check_def {
                out.push_str(&format!("```sql\n{}\n```\n\n", def));
            }
        }
        DiffStatus::Changed => {
            out.push_str(&format!("#### ⚠ Changed: `{}`\n\n", name));
            out.push_str(&format!("**Base {}:**\n\n```sql\n{}\n```\n\n", label, base_def.unwrap_or("")));
            out.push_str(&format!("**Check {}:**\n\n```sql\n{}\n```\n\n", label, check_def.unwrap_or("")));
        }
        DiffStatus::Same => {}
    }
    out
}

fn render_routine_diff(diff: &ObjectDiff<RoutineDef>) -> String {
    render_simple_object(
        &diff.name,
        &diff.status,
        diff.base.as_ref().map(|r| r.definition.as_str()),
        diff.check.as_ref().map(|r| r.definition.as_str()),
        "Definition",
    )
}

fn render_trigger_diff(diff: &ObjectDiff<TriggerDef>) -> String {
    let mut out = String::new();
    match &diff.status {
        DiffStatus::OnlyInBase => {
            out.push_str(&format!("#### ✚ Only in base: `{}`\n\n", diff.name));
            if let Some(t) = &diff.base {
                out.push_str(&format!(
                    "- Event: {} {}\n- Table: `{}`\n\n```sql\n{}\n```\n\n",
                    t.timing, t.event, t.table_name, t.statement
                ));
            }
        }
        DiffStatus::OnlyInCheck => {
            out.push_str(&format!("#### ✖ Only in check: `{}`\n\n", diff.name));
            if let Some(t) = &diff.check {
                out.push_str(&format!(
                    "- Event: {} {}\n- Table: `{}`\n\n```sql\n{}\n```\n\n",
                    t.timing, t.event, t.table_name, t.statement
                ));
            }
        }
        DiffStatus::Changed => {
            out.push_str(&format!("#### ⚠ Changed: `{}`\n\n", diff.name));
            if let Some(t) = &diff.base {
                out.push_str(&format!(
                    "**Base:**\n\n- Event: {} {}\n- Table: `{}`\n\n```sql\n{}\n```\n\n",
                    t.timing, t.event, t.table_name, t.statement
                ));
            }
            if let Some(t) = &diff.check {
                out.push_str(&format!(
                    "**Check:**\n\n- Event: {} {}\n- Table: `{}`\n\n```sql\n{}\n```\n\n",
                    t.timing, t.event, t.table_name, t.statement
                ));
            }
        }
        DiffStatus::Same => {}
    }
    out
}

fn render_column_list(cols: Vec<&ColumnDef>) -> String {
    if cols.is_empty() {
        return String::new();
    }
    let mut out = String::from("| Column | Type |\n|--------|------|\n");
    for col in cols {
        out.push_str(&format!("| `{}` | {} |\n", col.name, format_column(col)));
    }
    out.push('\n');
    out
}

// ── Formatting helpers ────────────────────────────────────────────────────────

fn format_column(col: &ColumnDef) -> String {
    let mut parts = vec![col.column_type.clone()];
    parts.push(if col.is_nullable { "NULL".to_string() } else { "NOT NULL".to_string() });
    if let Some(def) = &col.column_default {
        parts.push(format!("DEFAULT {}", def));
    }
    if !col.extra.is_empty() {
        parts.push(col.extra.clone());
    }
    parts.join(" ")
}

fn format_index(idx: &IndexDef) -> String {
    let cols: Vec<String> = idx.columns.iter().map(|c| {
        if let Some(sp) = c.sub_part {
            format!("{}({})", c.column_name, sp)
        } else {
            c.column_name.clone()
        }
    }).collect();
    format!(
        "{} ({}) [{}{}]",
        if idx.is_unique { "UNIQUE" } else { "" },
        cols.join(", "),
        idx.index_type,
        if idx.name == "PRIMARY" { ", PRIMARY" } else { "" },
    )
}

fn format_fk(fk: &ForeignKeyDef) -> String {
    format!(
        "({}) → `{}`.({}) ON DELETE {} ON UPDATE {}",
        fk.columns.join(", "),
        fk.ref_table,
        fk.ref_columns.join(", "),
        fk.on_delete,
        fk.on_update,
    )
}

fn status_icon(status: &DiffStatus) -> &'static str {
    match status {
        DiffStatus::OnlyInBase => "✚ only in base",
        DiffStatus::OnlyInCheck => "✖ only in check",
        DiffStatus::Changed => "⚠ changed",
        DiffStatus::Same => "✓ same",
    }
}

fn count_statuses<T: HasStatus>(items: &[T]) -> (usize, usize, usize, usize) {
    let total = items.len();
    let base = items.iter().filter(|i| i.status() == &DiffStatus::OnlyInBase).count();
    let check = items.iter().filter(|i| i.status() == &DiffStatus::OnlyInCheck).count();
    let changed = items.iter().filter(|i| i.status() == &DiffStatus::Changed).count();
    (total, base, check, changed)
}

trait HasStatus {
    fn status(&self) -> &DiffStatus;
}

impl HasStatus for DatabaseDiff {
    fn status(&self) -> &DiffStatus { &self.status }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::differ::*;
    use crate::schema::*;
    use std::collections::HashMap;

    fn make_report(databases: Vec<DatabaseDiff>) -> DiffReport {
        DiffReport {
            base_label: "10.0.0.1:3306".into(),
            check_label: "10.0.0.2:3306".into(),
            generated_at: "2026-04-15 10:00:00".into(),
            databases,
        }
    }

    #[test]
    fn test_render_contains_header() {
        let report = make_report(vec![]);
        let md = render(&report);
        assert!(md.contains("# Schema Diff Report"));
        assert!(md.contains("10.0.0.1:3306"));
        assert!(md.contains("10.0.0.2:3306"));
    }

    #[test]
    fn test_render_skips_same_databases() {
        let db_diff = DatabaseDiff {
            name: "mydb".into(),
            status: DiffStatus::Same,
            tables: vec![],
            views: vec![],
            procedures: vec![],
            functions: vec![],
            triggers: vec![],
        };
        let report = make_report(vec![db_diff]);
        let md = render(&report);
        // A Same database should not appear in the body
        assert!(!md.contains("## Database: `mydb`"));
    }

    #[test]
    fn test_render_changed_column() {
        let col_diff = ObjectDiff {
            name: "age".into(),
            status: DiffStatus::Changed,
            base: Some(ColumnDef {
                name: "age".into(),
                column_type: "tinyint".into(),
                is_nullable: false,
                column_default: None,
                extra: "".into(),
                ordinal_position: 2,
            }),
            check: Some(ColumnDef {
                name: "age".into(),
                column_type: "int".into(),
                is_nullable: false,
                column_default: None,
                extra: "".into(),
                ordinal_position: 2,
            }),
        };

        let table_diff = TableDiff {
            name: "users".into(),
            status: DiffStatus::Changed,
            column_diffs: vec![col_diff],
            index_diffs: vec![],
            fk_diffs: vec![],
            check_diffs: vec![],
        };

        let db_diff = DatabaseDiff {
            name: "mydb".into(),
            status: DiffStatus::Changed,
            tables: vec![table_diff],
            views: vec![],
            procedures: vec![],
            functions: vec![],
            triggers: vec![],
        };

        let md = render(&make_report(vec![db_diff]));
        assert!(md.contains("⚠ Changed: `users`"));
        assert!(md.contains("`age`"));
        assert!(md.contains("tinyint"));
        assert!(md.contains("int"));
    }
}
```

- [ ] **Step 2: Add module declaration to `src/main.rs`**

```rust
mod config;
mod differ;
mod reporter;
mod schema;

fn main() {
    println!("mysql-schema-diff");
}
```

- [ ] **Step 3: Run the tests**

```bash
cargo test reporter
```

Expected:
```
test reporter::tests::test_render_contains_header ... ok
test reporter::tests::test_render_skips_same_databases ... ok
test reporter::tests::test_render_changed_column ... ok
```

- [ ] **Step 4: Commit**

```bash
git add src/reporter.rs src/main.rs
git commit -m "feat: add Markdown report renderer with unit tests"
```

---

## Task 6: Fetcher (`src/fetcher.rs`)

**Files:**
- Create: `src/fetcher.rs`
- Modify: `src/main.rs` — add `mod fetcher;`

This is the only module that talks to MySQL. It queries `information_schema` and returns a `SchemaSnapshot`.

- [ ] **Step 1: Write `src/fetcher.rs`**

```rust
use std::collections::HashMap;

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

    // Collect all table names from all sources.
    let mut all_tables: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    all_tables.extend(columns.keys().cloned());

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
        let ordinal: i64 = row.try_get("ORDINAL_POSITION")?;
        let col = ColumnDef {
            name: row.try_get("COLUMN_NAME")?,
            column_type: row.try_get("COLUMN_TYPE")?,
            is_nullable: is_nullable == "YES",
            column_default: row.try_get("COLUMN_DEFAULT")?,
            extra: row.try_get("EXTRA")?,
            ordinal_position: ordinal as u32,
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
        let seq: i64 = row.try_get("SEQ_IN_INDEX")?;
        let non_unique: i64 = row.try_get("NON_UNIQUE")?;
        let index_type: String = row.try_get("INDEX_TYPE")?;
        let sub_part: Option<i64> = row.try_get("SUB_PART")?;
        let column_name: String = row.try_get("COLUMN_NAME")?;

        let entry = raw
            .entry(table_name)
            .or_default()
            .entry(index_name)
            .or_insert_with(|| (non_unique == 0, index_type, vec![]));

        entry.2.push(IndexColumn {
            column_name,
            seq_in_index: seq as u32,
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
        Err(e) => {
            let msg = e.to_string();
            // MySQL < 8.0 doesn't have CHECK_CONSTRAINTS in information_schema.
            if msg.contains("doesn't exist") || msg.contains("Table") {
                return Ok(HashMap::new());
            }
            return Err(e).context("Failed to fetch check constraints");
        }
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
        let action_order: i64 = row.try_get("ACTION_ORDER")?;
        let statement: String = row.try_get("ACTION_STATEMENT")?;
        let trigger = TriggerDef {
            name: name.clone(),
            event: row.try_get("EVENT_MANIPULATION")?,
            timing: row.try_get("ACTION_TIMING")?,
            table_name: row.try_get("EVENT_OBJECT_TABLE")?,
            statement: normalize_sql(&statement),
            action_order: action_order as u32,
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
```

- [ ] **Step 2: Add module declaration to `src/main.rs`**

```rust
mod config;
mod differ;
mod fetcher;
mod reporter;
mod schema;

fn main() {
    println!("mysql-schema-diff");
}
```

- [ ] **Step 3: Verify it compiles**

```bash
cargo build
```

Expected: compiles with no errors. (There are no unit tests for the fetcher since it requires a live MySQL connection.)

- [ ] **Step 4: Commit**

```bash
git add src/fetcher.rs src/main.rs
git commit -m "feat: add async MySQL fetcher for information_schema queries"
```

---

## Task 7: Main (`src/main.rs`) — wire everything together

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Write the final `src/main.rs`**

```rust
mod config;
mod differ;
mod fetcher;
mod reporter;
mod schema;

use anyhow::{Context, Result};
use chrono::Local;
use clap::Parser;

use config::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let base_cfg = cli.base_config();
    let check_cfg = cli.check_config();
    let opts = cli.options();

    let db_filter: Option<Vec<String>> = opts.databases.clone();

    eprintln!("Connecting to base  ({})…", opts.base_label);
    eprintln!("Connecting to check ({})…", opts.check_label);

    // Fetch both snapshots concurrently.
    let (base_result, check_result) = tokio::join!(
        fetcher::fetch_snapshot(&base_cfg, db_filter.as_deref()),
        fetcher::fetch_snapshot(&check_cfg, db_filter.as_deref()),
    );

    let base_snapshot = base_result.context("Failed to fetch base snapshot")?;
    let check_snapshot = check_result.context("Failed to fetch check snapshot")?;

    eprintln!("Diffing schemas…");

    let generated_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let report = differ::diff(
        base_snapshot,
        check_snapshot,
        opts.base_label.clone(),
        opts.check_label.clone(),
        generated_at.clone(),
    );

    let markdown = reporter::render(&report);

    // Determine output path.
    let output_path = opts.output.unwrap_or_else(|| {
        let ts = Local::now().format("%Y%m%d-%H%M%S");
        format!("schema-diff-{}.md", ts)
    });

    std::fs::write(&output_path, &markdown)
        .with_context(|| format!("Failed to write output to {}", output_path))?;

    eprintln!("Report written to: {}", output_path);

    // Exit with code 1 if there are any differences, 0 if everything is identical.
    let has_diff = report
        .databases
        .iter()
        .any(|db| db.status != differ::DiffStatus::Same);

    std::process::exit(if has_diff { 1 } else { 0 });
}
```

- [ ] **Step 2: Build the release binary**

```bash
cargo build --release
```

Expected: produces `target/release/mysql-schema-diff`. No errors.

- [ ] **Step 3: Run all unit tests**

```bash
cargo test
```

Expected:
```
test config::tests::test_cli_parses_required_args ... ok
test config::tests::test_cli_parses_optional_databases ... ok
test config::tests::test_labels ... ok
test differ::tests::test_identical_snapshots_produce_same_status ... ok
test differ::tests::test_db_only_in_base ... ok
test differ::tests::test_db_only_in_check ... ok
test differ::tests::test_changed_table_column_type ... ok
test differ::tests::test_table_only_in_base ... ok
test reporter::tests::test_render_contains_header ... ok
test reporter::tests::test_render_skips_same_databases ... ok
test reporter::tests::test_render_changed_column ... ok
```

- [ ] **Step 4: Test the CLI help output**

```bash
./target/release/mysql-schema-diff --help
```

Expected output includes all flags: `--base-host`, `--base-port`, `--base-user`, `--base-password`, `--check-host`, `--check-port`, `--check-user`, `--check-password`, `--databases`, `--output`.

- [ ] **Step 5: Commit**

```bash
git add src/main.rs
git commit -m "feat: wire up main entry point, exit 1 on diff found"
```

---

## Task 8: Manual integration test (optional, requires MySQL)

**Files:** none

- [ ] **Step 1: Run against two real MySQL instances**

```bash
./target/release/mysql-schema-diff \
  --base-host <BASE_IP> --base-port 3306 \
  --base-user root --base-password <PASS> \
  --check-host <CHECK_IP> --check-port 3306 \
  --check-user root --check-password <PASS>
```

Expected: a file `schema-diff-YYYYMMDD-HHMMSS.md` is created in the current directory.

- [ ] **Step 2: Inspect the report**

Open the `.md` file and verify:
- The header shows the correct hosts and timestamp.
- The summary counts match what you expect.
- A known table difference (e.g., a column type change you made on purpose) appears correctly with base/check values.
- Tables that are identical do not appear in the report.

---

## Self-Review Notes

**Spec coverage check:**
- ✅ Tables + columns (type, nullable, default, extra) — Task 6 `fetch_columns`
- ✅ Indexes (name, type, uniqueness, column list) — Task 6 `fetch_indexes`
- ✅ Foreign keys (name, columns, ref table/columns, ON DELETE/UPDATE) — Task 6 `fetch_foreign_keys`
- ✅ Check constraints (MySQL 8.0+, graceful fallback) — Task 6 `fetch_check_constraints`
- ✅ Views (normalized definition) — Task 6 `fetch_views`
- ✅ Stored procedures — Task 6 `fetch_routines`
- ✅ Functions — Task 6 `fetch_routines`
- ✅ Triggers — Task 6 `fetch_triggers`
- ✅ Concurrent DB fetch — Task 7 `tokio::join!`
- ✅ CLI args: all required + optional flags — Task 3
- ✅ `--databases` filter — Task 6 `list_databases`
- ✅ `--output` override with timestamp default — Task 7
- ✅ Markdown report with summary and per-db sections — Task 5
- ✅ Two-way diff with before/after — Task 4 + Task 5
- ✅ Exit code 1 on diff found — Task 7

**Type consistency check:**
- `SchemaSnapshot`, `DatabaseSchema`, `TableDef`, `ColumnDef`, `IndexDef`, `ForeignKeyDef`, `CheckDef`, `ViewDef`, `RoutineDef`, `TriggerDef` — defined in Task 2, used consistently in Tasks 4, 5, 6.
- `DiffStatus`, `DiffReport`, `DatabaseDiff`, `TableDiff`, `ObjectDiff<T>` — defined in Task 4, used in Tasks 5, 7.
- `fetcher::fetch_snapshot` — defined in Task 6, called in Task 7.
- `differ::diff` — defined in Task 4, called in Task 7.
- `reporter::render` — defined in Task 5, called in Task 7.
