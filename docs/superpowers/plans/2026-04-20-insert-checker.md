# sqltool INSERT Checker Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Rename the binary to `sqltool` with two subcommands (`diff` and `check-inserts`), where `check-inserts` parses a `.sql` file and validates each INSERT against a live MySQL instance using read-only SELECT queries, producing a Markdown report.

**Architecture:** Add a `sqlparser`-based `insert_checker` module under `src/insert_checker/` with five focused files (parser, meta_fetcher, checker, reporter, orchestrator). Refactor `config.rs` to use clap's `Subcommand` derive, and `main.rs` to dispatch on the subcommand. All existing `diff` behavior is unchanged.

**Tech Stack:** Rust, sqlx (MySQL, async), clap (subcommand derive), tokio (async), sqlparser 0.54 (MySQL dialect), anyhow, chrono.

**Spec:** `docs/superpowers/specs/2026-04-20-insert-checker-design.md`

---

## File Map

| Path | Action | Responsibility |
|------|--------|----------------|
| `Cargo.toml` | Modify | Rename binary to `sqltool`; add `sqlparser = "0.54"` |
| `src/config.rs` | Modify | Add `Commands` enum + `DiffArgs` + `CheckInsertsArgs`; keep all existing logic |
| `src/main.rs` | Modify | Dispatch on `Commands::Diff` or `Commands::CheckInserts` |
| `src/insert_checker/mod.rs` | Create | Orchestrator: connect → parse → check → report |
| `src/insert_checker/parser.rs` | Create | Parse `.sql` file → `Vec<ParseResult>` using sqlparser |
| `src/insert_checker/meta_fetcher.rs` | Create | Fetch + cache `TableMeta` from `information_schema` |
| `src/insert_checker/checker.rs` | Create | All check logic → `Vec<Finding>` |
| `src/insert_checker/reporter.rs` | Create | Render `Vec<Finding>` + skips → Markdown file |

Unchanged: `src/fetcher.rs`, `src/differ.rs`, `src/reporter.rs`, `src/sql_generator.rs`, `src/schema/mod.rs`.

---

## Task 1: Add `sqlparser` and rename binary

**Files:**
- Modify: `Cargo.toml`

- [ ] **Step 1: Edit `Cargo.toml`**

Change the `[[bin]]` name and add the new dependency:

```toml
[[bin]]
name = "sqltool"
path = "src/main.rs"

[dependencies]
sqlx = { version = "0.7", features = ["mysql", "runtime-tokio-rustls"] }
clap = { version = "4", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
chrono = "0.4"
anyhow = "1"
sqlparser = { version = "0.54" }
```

- [ ] **Step 2: Verify the project still compiles**

```bash
cargo build 2>&1 | head -20
```

Expected: no errors (binary name change is harmless at compile time).

- [ ] **Step 3: Commit**

```bash
git add Cargo.toml Cargo.lock
git commit -m "chore: rename binary to sqltool, add sqlparser dependency"
```

---

## Task 2: Refactor `config.rs` — subcommand structure

**Files:**
- Modify: `src/config.rs`

The existing `Cli` struct has all `diff` flags at the top level. We need to restructure so clap dispatches between `diff` and `check-inserts`. The existing `ConnectionConfig` and `Options` structs stay intact — only the clap-facing structs change.

- [ ] **Step 1: Write the failing test first**

Add at the bottom of `src/config.rs` (inside the existing `#[cfg(test)]` block):

```rust
#[test]
fn test_subcommand_diff_parses() {
    let cli = Cli::parse_from([
        "sqltool", "diff",
        "--base-host", "h1", "--base-user", "u", "--base-password", "p",
        "--check-host", "h2", "--check-user", "u", "--check-password", "p",
    ]);
    match cli.command {
        Commands::Diff(args) => {
            assert_eq!(args.base_host, "h1");
            assert_eq!(args.check_host, "h2");
        }
        _ => panic!("expected Diff"),
    }
}

#[test]
fn test_subcommand_check_inserts_parses() {
    let cli = Cli::parse_from([
        "sqltool", "check-inserts",
        "--host", "localhost",
        "--user", "root",
        "--password", "secret",
        "--database", "mydb",
        "--file", "inserts.sql",
    ]);
    match cli.command {
        Commands::CheckInserts(args) => {
            assert_eq!(args.host, "localhost");
            assert_eq!(args.port, 3306);
            assert_eq!(args.database, "mydb");
            assert_eq!(args.file, "inserts.sql");
            assert!(args.output.is_none());
        }
        _ => panic!("expected CheckInserts"),
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

```bash
cargo test test_subcommand 2>&1 | tail -10
```

Expected: compile error — `cli.command`, `Commands`, `DiffArgs`, `CheckInsertsArgs` do not exist yet.

- [ ] **Step 3: Rewrite `src/config.rs`**

Replace the entire file content:

```rust
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "sqltool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Compare schemas between two MySQL instances
    Diff(DiffArgs),
    /// Check INSERT statements in a .sql file against a live MySQL instance
    CheckInserts(CheckInsertsArgs),
}

/// Arguments for the `diff` subcommand (identical to old top-level flags).
#[derive(clap::Args, Debug)]
pub struct DiffArgs {
    #[arg(long)]
    pub base_host: String,
    #[arg(long, default_value = "3306")]
    pub base_port: u16,
    #[arg(long)]
    pub base_user: String,
    #[arg(long)]
    pub base_password: String,
    #[arg(long)]
    pub check_host: String,
    #[arg(long, default_value = "3306")]
    pub check_port: u16,
    #[arg(long)]
    pub check_user: String,
    #[arg(long)]
    pub check_password: String,
    #[arg(long, value_delimiter = ',')]
    pub databases: Option<Vec<String>>,
    #[arg(long)]
    pub output: Option<String>,
    #[arg(long, default_value_t = false)]
    pub ignore_base_only_dbs: bool,
}

/// Arguments for the `check-inserts` subcommand.
#[derive(clap::Args, Debug)]
pub struct CheckInsertsArgs {
    #[arg(long)]
    pub host: String,
    #[arg(long, default_value = "3306")]
    pub port: u16,
    #[arg(long)]
    pub user: String,
    #[arg(long)]
    pub password: String,
    /// Target database name (used when INSERT has no `db.table` qualifier)
    #[arg(long)]
    pub database: String,
    /// Path to the .sql file containing INSERT statements
    #[arg(long)]
    pub file: String,
    /// Output report path. `.md` is appended if not present. Defaults to ./insert-check-YYYYMMDD-HHMMSS.md
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

/// Non-connection options for the diff subcommand.
#[derive(Debug)]
pub struct Options {
    pub databases: Option<Vec<String>>,
    pub output: Option<String>,
    pub base_label: String,
    pub check_label: String,
    pub ignore_base_only_dbs: bool,
}

impl DiffArgs {
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
            ignore_base_only_dbs: self.ignore_base_only_dbs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_subcommand_diff_parses() {
        let cli = Cli::parse_from([
            "sqltool", "diff",
            "--base-host", "h1", "--base-user", "u", "--base-password", "p",
            "--check-host", "h2", "--check-user", "u", "--check-password", "p",
        ]);
        match cli.command {
            Commands::Diff(args) => {
                assert_eq!(args.base_host, "h1");
                assert_eq!(args.check_host, "h2");
            }
            _ => panic!("expected Diff"),
        }
    }

    #[test]
    fn test_subcommand_check_inserts_parses() {
        let cli = Cli::parse_from([
            "sqltool", "check-inserts",
            "--host", "localhost",
            "--user", "root",
            "--password", "secret",
            "--database", "mydb",
            "--file", "inserts.sql",
        ]);
        match cli.command {
            Commands::CheckInserts(args) => {
                assert_eq!(args.host, "localhost");
                assert_eq!(args.port, 3306);
                assert_eq!(args.database, "mydb");
                assert_eq!(args.file, "inserts.sql");
                assert!(args.output.is_none());
            }
            _ => panic!("expected CheckInserts"),
        }
    }

    #[test]
    fn test_diff_labels() {
        let cli = Cli::parse_from([
            "sqltool", "diff",
            "--base-host", "10.0.0.1", "--base-port", "3307",
            "--base-user", "u", "--base-password", "p",
            "--check-host", "10.0.0.2",
            "--check-user", "u", "--check-password", "p",
        ]);
        match cli.command {
            Commands::Diff(args) => {
                let opts = args.options();
                assert_eq!(opts.base_label, "10.0.0.1:3307");
                assert_eq!(opts.check_label, "10.0.0.2:3306");
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_diff_databases_flag() {
        let cli = Cli::parse_from([
            "sqltool", "diff",
            "--base-host", "h1", "--base-user", "u", "--base-password", "p",
            "--check-host", "h2", "--check-user", "u", "--check-password", "p",
            "--databases", "db1,db2,db3",
        ]);
        match cli.command {
            Commands::Diff(args) => {
                assert_eq!(args.databases, Some(vec!["db1".into(), "db2".into(), "db3".into()]));
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_diff_ignore_base_only_dbs() {
        let cli = Cli::parse_from([
            "sqltool", "diff",
            "--base-host", "h1", "--base-user", "u", "--base-password", "p",
            "--check-host", "h2", "--check-user", "u", "--check-password", "p",
            "--ignore-base-only-dbs",
        ]);
        match cli.command {
            Commands::Diff(args) => assert!(args.options().ignore_base_only_dbs),
            _ => panic!(),
        }
    }
}
```

- [ ] **Step 4: Fix the compile error in `main.rs`**

`main.rs` calls `cli.base_config()`, `cli.check_config()`, `cli.options()` directly on `Cli`. Update `main.rs` to extract `DiffArgs` first (full rewrite in Task 3, but for now just make it compile):

Temporary stub — wrap the existing body in a `Commands::Diff(args)` match. Replace the top of `main()`:

```rust
let cli = Cli::parse();
let args = match cli.command {
    config::Commands::Diff(a) => a,
    config::Commands::CheckInserts(_) => {
        eprintln!("check-inserts not yet implemented");
        std::process::exit(1);
    }
};
let base_cfg = args.base_config();
let check_cfg = args.check_config();
let opts = args.options();
```

Also update the `use config::Cli;` import to `use config::{Cli, Commands};` — or just use the fully qualified path `config::Commands` as shown above.

- [ ] **Step 5: Run tests**

```bash
cargo test 2>&1 | tail -20
```

Expected: all tests pass (new subcommand tests + all old tests ported).

- [ ] **Step 6: Commit**

```bash
git add src/config.rs src/main.rs
git commit -m "refactor: restructure CLI into diff/check-inserts subcommands"
```

---

## Task 3: Rewrite `main.rs` — proper subcommand dispatch

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Rewrite `main.rs`**

```rust
mod config;
mod differ;
mod fetcher;
mod insert_checker;
mod reporter;
mod schema;
mod sql_generator;

use anyhow::{Context, Result};
use chrono::Local;
use clap::Parser;

use config::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Diff(args) => run_diff(args).await,
        Commands::CheckInserts(args) => insert_checker::run(args).await,
    }
}

async fn run_diff(args: config::DiffArgs) -> Result<()> {
    let base_cfg = args.base_config();
    let check_cfg = args.check_config();
    let opts = args.options();

    let db_filter: Option<Vec<String>> = opts.databases.clone();

    eprintln!("Connecting to base  ({})…", opts.base_label);
    eprintln!("Connecting to check ({})…", opts.check_label);

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
        opts.ignore_base_only_dbs,
    );

    let markdown = reporter::render(&report);

    let base_path = opts.output.unwrap_or_else(|| {
        let ts = Local::now().format("%Y%m%d-%H%M%S");
        format!("schema-diff-{}", ts)
    });
    let md_path = if base_path.ends_with(".md") {
        base_path.clone()
    } else {
        format!("{}.md", base_path)
    };
    let sql_path = if base_path.ends_with(".md") {
        format!("{}.sql", &base_path[..base_path.len() - 3])
    } else {
        format!("{}.sql", base_path)
    };

    std::fs::write(&md_path, &markdown)
        .with_context(|| format!("Failed to write output to {}", md_path))?;
    eprintln!("Report written to: {}", md_path);

    let (sql_content, warnings) = sql_generator::generate(&report);
    for w in &warnings {
        eprintln!("{}", w);
    }
    std::fs::write(&sql_path, &sql_content)
        .with_context(|| format!("Failed to write SQL to {}", sql_path))?;
    eprintln!("SQL sync file written to: {}", sql_path);

    let has_diff = report
        .databases
        .iter()
        .any(|db| db.status != differ::DiffStatus::Same);

    std::process::exit(if has_diff { 1 } else { 0 });
}
```

The `insert_checker::run` function doesn't exist yet — the compiler will warn but the file needs a stub module. Create `src/insert_checker/mod.rs` with just:

```rust
use anyhow::Result;
use crate::config::CheckInsertsArgs;

pub async fn run(_args: CheckInsertsArgs) -> Result<()> {
    todo!("insert_checker not yet implemented")
}
```

- [ ] **Step 2: Build**

```bash
cargo build 2>&1 | tail -10
```

Expected: compiles successfully (todo! panics at runtime only).

- [ ] **Step 3: Commit**

```bash
git add src/main.rs src/insert_checker/mod.rs
git commit -m "refactor: main.rs dispatches on subcommand; insert_checker stub"
```

---

## Task 4: `parser.rs` — parse INSERT statements

**Files:**
- Create: `src/insert_checker/parser.rs`

This module reads a SQL file as a string, uses sqlparser with the MySQL dialect to produce an AST, then extracts `InsertStmt` values. Non-INSERT statements and `INSERT INTO ... SELECT` are noted as skipped items.

- [ ] **Step 1: Write the tests first**

Create `src/insert_checker/parser.rs` with tests only:

```rust
#[derive(Debug, PartialEq)]
pub struct InsertStmt {
    pub database: Option<String>,
    pub table: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<SqlValue>>,
    pub original_sql: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SqlValue {
    Null,
    Number(String),
    Str(String),
    Bool(bool),
    Other(String),
}

/// One item produced per SQL statement in the file.
#[derive(Debug)]
pub enum ParseResult {
    Insert(InsertStmt),
    Skipped { reason: String, sql: String },
}

pub fn parse_sql_file(sql: &str) -> Vec<ParseResult> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_named_insert_single_row() {
        let sql = "INSERT INTO `users` (`id`, `email`) VALUES (1, 'alice@example.com');";
        let results = parse_sql_file(sql);
        assert_eq!(results.len(), 1);
        let ParseResult::Insert(stmt) = &results[0] else { panic!("expected Insert") };
        assert_eq!(stmt.database, None);
        assert_eq!(stmt.table, "users");
        assert_eq!(stmt.columns, vec!["id", "email"]);
        assert_eq!(stmt.rows.len(), 1);
        assert_eq!(stmt.rows[0][0], SqlValue::Number("1".into()));
        assert_eq!(stmt.rows[0][1], SqlValue::Str("alice@example.com".into()));
    }

    #[test]
    fn test_qualified_table_name() {
        let sql = "INSERT INTO mydb.users (id) VALUES (1);";
        let results = parse_sql_file(sql);
        let ParseResult::Insert(stmt) = &results[0] else { panic!() };
        assert_eq!(stmt.database, Some("mydb".into()));
        assert_eq!(stmt.table, "users");
    }

    #[test]
    fn test_positional_insert() {
        let sql = "INSERT INTO t VALUES (42, NULL, 'hello');";
        let results = parse_sql_file(sql);
        let ParseResult::Insert(stmt) = &results[0] else { panic!() };
        assert_eq!(stmt.columns, Vec::<String>::new());
        assert_eq!(stmt.rows[0][0], SqlValue::Number("42".into()));
        assert_eq!(stmt.rows[0][1], SqlValue::Null);
        assert_eq!(stmt.rows[0][2], SqlValue::Str("hello".into()));
    }

    #[test]
    fn test_multi_row_insert() {
        let sql = "INSERT INTO t (a, b) VALUES (1, 2), (3, 4);";
        let results = parse_sql_file(sql);
        let ParseResult::Insert(stmt) = &results[0] else { panic!() };
        assert_eq!(stmt.rows.len(), 2);
        assert_eq!(stmt.rows[1][0], SqlValue::Number("3".into()));
    }

    #[test]
    fn test_insert_select_is_skipped() {
        let sql = "INSERT INTO t SELECT * FROM other;";
        let results = parse_sql_file(sql);
        assert_eq!(results.len(), 1);
        assert!(matches!(results[0], ParseResult::Skipped { .. }));
    }

    #[test]
    fn test_parse_error_is_skipped() {
        let sql = "INSERT INTO @@#@# garbage;";
        let results = parse_sql_file(sql);
        assert_eq!(results.len(), 1);
        assert!(matches!(results[0], ParseResult::Skipped { .. }));
    }

    #[test]
    fn test_non_insert_is_ignored() {
        // Non-INSERT statements (CREATE TABLE, UPDATE, etc.) are silently ignored (not in output)
        let sql = "CREATE TABLE t (id INT); INSERT INTO t (id) VALUES (1);";
        let results = parse_sql_file(sql);
        assert_eq!(results.len(), 1);
        assert!(matches!(results[0], ParseResult::Insert(_)));
    }

    #[test]
    fn test_expression_value_is_other() {
        let sql = "INSERT INTO t (created_at) VALUES (NOW());";
        let results = parse_sql_file(sql);
        let ParseResult::Insert(stmt) = &results[0] else { panic!() };
        assert!(matches!(stmt.rows[0][0], SqlValue::Other(_)));
    }

    #[test]
    fn test_bool_values() {
        let sql = "INSERT INTO t (active) VALUES (TRUE);";
        let results = parse_sql_file(sql);
        let ParseResult::Insert(stmt) = &results[0] else { panic!() };
        assert_eq!(stmt.rows[0][0], SqlValue::Bool(true));
    }
}
```

- [ ] **Step 2: Add `mod parser;` to `src/insert_checker/mod.rs`** (needed before tests can compile)

Append to the existing `src/insert_checker/mod.rs`:

```rust
mod parser;
pub use parser::{ParseResult, InsertStmt, SqlValue};
```

- [ ] **Step 3: Run tests to confirm they fail (red bar)**

```bash
cargo test insert_checker::parser 2>&1 | tail -10
```

Expected: tests run but panic with `not yet implemented` (the `todo!()`). That is the expected red bar.

- [ ] **Step 4: Implement `parse_sql_file`**

Replace `pub fn parse_sql_file(sql: &str) -> Vec<ParseResult> { todo!() }` and add all helpers:

```rust
use sqlparser::ast::{Expr, SetExpr, Statement, TableObject, Value};
use sqlparser::dialect::MySqlDialect;
use sqlparser::parser::Parser as SqlParser;

pub fn parse_sql_file(sql: &str) -> Vec<ParseResult> {
    let dialect = MySqlDialect {};

    // Try parsing the whole file; if any statement is malformed the whole parse
    // fails, so fall back to per-statement mode.
    match SqlParser::parse_sql(&dialect, sql) {
        Ok(stmts) => process_statements(stmts),
        Err(_) => parse_per_statement(&dialect, sql),
    }
}

/// Split on `;` and parse each chunk independently so one bad statement
/// does not discard the rest. Note: splitting on `;` is a heuristic and
/// will misfire if a string value contains a semicolon — acceptable for
/// typical SQL dump files.
fn parse_per_statement(dialect: &MySqlDialect, sql: &str) -> Vec<ParseResult> {
    let mut results = Vec::new();
    for chunk in sql.split(';') {
        let chunk = chunk.trim();
        if chunk.is_empty() {
            continue;
        }
        let chunk_with_semi = format!("{};", chunk);
        match SqlParser::parse_sql(dialect, &chunk_with_semi) {
            Ok(stmts) => results.extend(process_statements(stmts)),
            Err(e) => results.push(ParseResult::Skipped {
                reason: format!("parse error: {}", e),
                sql: chunk_with_semi,
            }),
        }
    }
    results
}

fn process_statements(statements: Vec<Statement>) -> Vec<ParseResult> {
    let mut results = Vec::new();
    for stmt in statements {
        match stmt {
            Statement::Insert(insert) => {
                // INSERT INTO ... SELECT is skipped
                if let Some(source) = &insert.source {
                    if matches!(source.body.as_ref(), SetExpr::Select(_)) {
                        results.push(ParseResult::Skipped {
                            reason: "INSERT-SELECT not checkable".into(),
                            sql: insert.to_string(),
                        });
                        continue;
                    }
                }

                let (database, table) = extract_table_parts(&insert.table);

                let columns: Vec<String> = insert
                    .columns
                    .iter()
                    .map(|id| id.value.clone())
                    .collect();

                let rows = match extract_rows(&insert) {
                    Ok(r) => r,
                    Err(e) => {
                        results.push(ParseResult::Skipped {
                            reason: format!("could not extract values: {}", e),
                            sql: insert.to_string(),
                        });
                        continue;
                    }
                };

                let original_sql = insert.to_string();
                results.push(ParseResult::Insert(InsertStmt {
                    database,
                    table,
                    columns,
                    rows,
                    original_sql,
                }));
            }
            // All other statement types are silently ignored
            _ => {}
        }
    }
    results
}

/// In sqlparser 0.54, `Insert.table` is a `TableObject` enum.
fn extract_table_parts(table: &TableObject) -> (Option<String>, String) {
    let obj_name = match table {
        TableObject::TableName(n) => n,
        _ => return (None, table.to_string()),
    };
    let parts: Vec<&str> = obj_name.0.iter().map(|id| id.value.as_str()).collect();
    match parts.as_slice() {
        [db, tbl] => (Some(db.to_string()), tbl.to_string()),
        [tbl] => (None, tbl.to_string()),
        _ => (None, obj_name.to_string()),
    }
}

fn extract_rows(insert: &sqlparser::ast::Insert) -> Result<Vec<Vec<SqlValue>>, String> {
    let source = insert.source.as_ref().ok_or("no source")?;
    let values = match source.body.as_ref() {
        SetExpr::Values(v) => &v.rows,
        _ => return Err("not a VALUES clause".into()),
    };
    Ok(values.iter().map(|row| row.iter().map(expr_to_value).collect()).collect())
}

fn expr_to_value(expr: &Expr) -> SqlValue {
    match expr {
        Expr::Value(v) => match v {
            Value::Null => SqlValue::Null,
            Value::Number(n, _) => SqlValue::Number(n.clone()),
            Value::SingleQuotedString(s) | Value::DoubleQuotedString(s) => SqlValue::Str(s.clone()),
            Value::Boolean(b) => SqlValue::Bool(*b),
            _ => SqlValue::Other(expr.to_string()),
        },
        Expr::UnaryOp { op, expr } => {
            if matches!(op, sqlparser::ast::UnaryOperator::Minus) {
                if let Expr::Value(Value::Number(n, _)) = expr.as_ref() {
                    return SqlValue::Number(format!("-{}", n));
                }
            }
            SqlValue::Other(format!("{} {}", op, expr))
        }
        _ => SqlValue::Other(expr.to_string()),
    }
}
```

- [ ] **Step 5: Run tests**

```bash
cargo test insert_checker::parser 2>&1 | tail -20
```

Expected: all 9 tests pass.

- [ ] **Step 6: Commit**

```bash
git add src/insert_checker/parser.rs src/insert_checker/mod.rs
git commit -m "feat(insert-checker): parser — extract InsertStmt from .sql files"
```

> **Note on parse error scope:** When the whole file has no bad statements, sqlparser parses it in one pass. When any statement is malformed, the per-statement fallback parses each chunk individually (splitting by `;`). This means one bad statement will not prevent valid INSERT statements before or after it from being checked. Strings containing literal semicolons are a known edge case not handled by the splitter.

---

## Task 5: `meta_fetcher.rs` — fetch table metadata from information_schema

**Files:**
- Create: `src/insert_checker/meta_fetcher.rs`

This module fetches and caches `TableMeta` per `(database, table)` from `information_schema`. The queries are read-only SELECTs. Tests verify struct construction and the cache key logic — DB queries are verified by integration testing only.

- [ ] **Step 1: Write tests first**

Create `src/insert_checker/meta_fetcher.rs`:

```rust
use std::collections::HashMap;
use anyhow::Result;
use sqlx::MySqlPool;

#[derive(Debug, Clone)]
pub struct TableMeta {
    pub columns: Vec<ColumnMeta>,
    pub primary_key: Vec<String>,
    pub unique_keys: HashMap<String, Vec<String>>,
    pub foreign_keys: Vec<FkMeta>,
}

#[derive(Debug, Clone)]
pub struct ColumnMeta {
    pub name: String,
    pub data_type: String,
    pub is_nullable: bool,
    pub has_default: bool,
    pub char_max_length: Option<u64>,
    pub numeric_precision: Option<u64>,
    pub numeric_scale: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct FkMeta {
    pub columns: Vec<String>,
    pub referenced_database: String,
    pub referenced_table: String,
    pub referenced_columns: Vec<String>,
}

pub struct MetaCache {
    cache: HashMap<(String, String), Option<TableMeta>>,
}

impl MetaCache {
    pub fn new() -> Self {
        Self { cache: HashMap::new() }
    }

    /// Returns `None` if the table does not exist.
    pub async fn get(
        &mut self,
        pool: &MySqlPool,
        database: &str,
        table: &str,
    ) -> Result<Option<&TableMeta>> {
        let key = (database.to_string(), table.to_string());
        if !self.cache.contains_key(&key) {
            let meta = fetch_table_meta(pool, database, table).await?;
            self.cache.insert(key.clone(), meta);
        }
        Ok(self.cache[&key].as_ref())
    }
}

async fn fetch_table_meta(pool: &MySqlPool, database: &str, table: &str) -> Result<Option<TableMeta>> {
    use sqlx::Row;

    // Check table existence (use COUNT rather than query_scalar to avoid type inference issues)
    let count_row = sqlx::query(
        "SELECT COUNT(*) FROM information_schema.TABLES \
         WHERE TABLE_SCHEMA = ? AND TABLE_NAME = ?"
    )
    .bind(database)
    .bind(table)
    .fetch_one(pool)
    .await?;
    let count: i64 = count_row.try_get(0)?;
    if count == 0 {
        return Ok(None);
    }

    // Fetch columns ordered by ORDINAL_POSITION
    let column_rows = sqlx::query(
        "SELECT COLUMN_NAME, DATA_TYPE, IS_NULLABLE, COLUMN_DEFAULT, \
                CHARACTER_MAXIMUM_LENGTH, NUMERIC_PRECISION, NUMERIC_SCALE \
         FROM information_schema.COLUMNS \
         WHERE TABLE_SCHEMA = ? AND TABLE_NAME = ? \
         ORDER BY ORDINAL_POSITION"
    )
    .bind(database)
    .bind(table)
    .fetch_all(pool)
    .await?;

    let columns: Vec<ColumnMeta> = column_rows.iter().map(|r| ColumnMeta {
        name: r.try_get("COLUMN_NAME").unwrap_or_default(),
        data_type: r.try_get("DATA_TYPE").unwrap_or_default(),
        is_nullable: r.try_get::<String, _>("IS_NULLABLE").unwrap_or_default() == "YES",
        has_default: r.try_get::<Option<String>, _>("COLUMN_DEFAULT").unwrap_or(None).is_some(),
        char_max_length: r.try_get::<Option<u64>, _>("CHARACTER_MAXIMUM_LENGTH").unwrap_or(None),
        numeric_precision: r.try_get::<Option<u64>, _>("NUMERIC_PRECISION").unwrap_or(None),
        numeric_scale: r.try_get::<Option<u64>, _>("NUMERIC_SCALE").unwrap_or(None),
    }).collect();

    // Fetch PKs and unique keys from STATISTICS
    let index_rows = sqlx::query(
        "SELECT INDEX_NAME, NON_UNIQUE, COLUMN_NAME \
         FROM information_schema.STATISTICS \
         WHERE TABLE_SCHEMA = ? AND TABLE_NAME = ? \
         ORDER BY INDEX_NAME, SEQ_IN_INDEX"
    )
    .bind(database)
    .bind(table)
    .fetch_all(pool)
    .await?;

    let mut primary_key: Vec<String> = Vec::new();
    let mut unique_keys: HashMap<String, Vec<String>> = HashMap::new();

    for row in &index_rows {
        let idx_name: String = row.try_get("INDEX_NAME").unwrap_or_default();
        let col_name: String = row.try_get("COLUMN_NAME").unwrap_or_default();
        let non_unique: i64 = row.try_get("NON_UNIQUE").unwrap_or(1);

        if idx_name == "PRIMARY" {
            primary_key.push(col_name);
        } else if non_unique == 0 {
            unique_keys.entry(idx_name).or_default().push(col_name);
        }
    }

    // Fetch foreign keys
    let fk_rows = sqlx::query(
        "SELECT kcu.CONSTRAINT_NAME, kcu.COLUMN_NAME, \
                kcu.REFERENCED_TABLE_SCHEMA, kcu.REFERENCED_TABLE_NAME, \
                kcu.REFERENCED_COLUMN_NAME \
         FROM information_schema.KEY_COLUMN_USAGE kcu \
         JOIN information_schema.REFERENTIAL_CONSTRAINTS rc \
           ON rc.CONSTRAINT_SCHEMA = kcu.CONSTRAINT_SCHEMA \
          AND rc.CONSTRAINT_NAME = kcu.CONSTRAINT_NAME \
         WHERE kcu.TABLE_SCHEMA = ? AND kcu.TABLE_NAME = ? \
         ORDER BY kcu.CONSTRAINT_NAME, kcu.ORDINAL_POSITION"
    )
    .bind(database)
    .bind(table)
    .fetch_all(pool)
    .await?;

    let mut fk_map: HashMap<String, FkMeta> = HashMap::new();
    for row in &fk_rows {
        let name: String = row.try_get("CONSTRAINT_NAME").unwrap_or_default();
        let col: String = row.try_get("COLUMN_NAME").unwrap_or_default();
        let ref_db: String = row.try_get("REFERENCED_TABLE_SCHEMA").unwrap_or_default();
        let ref_tbl: String = row.try_get("REFERENCED_TABLE_NAME").unwrap_or_default();
        let ref_col: String = row.try_get("REFERENCED_COLUMN_NAME").unwrap_or_default();

        let entry = fk_map.entry(name).or_insert(FkMeta {
            columns: Vec::new(),
            referenced_database: ref_db,
            referenced_table: ref_tbl,
            referenced_columns: Vec::new(),
        });
        entry.columns.push(col);
        entry.referenced_columns.push(ref_col);
    }

    let foreign_keys: Vec<FkMeta> = fk_map.into_values().collect();

    Ok(Some(TableMeta { columns, primary_key, unique_keys, foreign_keys }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meta_cache_key_is_db_and_table() {
        // Verify the cache key type compiles and is constructed correctly
        let key1: (String, String) = ("mydb".into(), "users".into());
        let key2: (String, String) = ("mydb".into(), "orders".into());
        let key3: (String, String) = ("otherdb".into(), "users".into());
        let mut map: HashMap<(String, String), u8> = HashMap::new();
        map.insert(key1.clone(), 1);
        map.insert(key2, 2);
        map.insert(key3, 3);
        assert_eq!(map[&key1], 1);
        assert_eq!(map.len(), 3);
    }

    #[test]
    fn test_column_meta_has_default_logic() {
        let col_with_default = ColumnMeta {
            name: "created_at".into(),
            data_type: "datetime".into(),
            is_nullable: false,
            has_default: true,
            char_max_length: None,
            numeric_precision: None,
            numeric_scale: None,
        };
        assert!(col_with_default.has_default);
        assert!(!col_with_default.is_nullable);
    }
}
```

- [ ] **Step 2: Run tests**

```bash
cargo test insert_checker::meta_fetcher 2>&1 | tail -10
```

Expected: 2 unit tests pass (no DB needed).

- [ ] **Step 3: Add `mod meta_fetcher;` to `src/insert_checker/mod.rs`**

```rust
mod meta_fetcher;
pub use meta_fetcher::{MetaCache, TableMeta, ColumnMeta, FkMeta};
```

- [ ] **Step 4: Build**

```bash
cargo build 2>&1 | tail -10
```

Expected: compiles. The `sqlx::query!` macros require `DATABASE_URL` env var or offline mode. If you see `sqlx` compile errors about offline mode, run:

```bash
cargo build --features sqlx/offline 2>&1 | tail -10
```

Or set `SQLX_OFFLINE=true cargo build`. If the project already has a `sqlx-data.json` / `.sqlx/` folder, you may need to add the new queries. For now, if the macros fail, use `sqlx::query_as` with explicit type annotations instead (see note below).

> **Note on sqlx macros:** The existing codebase uses `sqlx::query!` which requires compile-time DB verification. If compile-time checks fail in CI/offline, switch `fetch_table_meta` to use `sqlx::query_as::<_, (String, String, ...)>()` with explicit row types. The behavior is identical.

- [ ] **Step 5: Commit**

```bash
git add src/insert_checker/meta_fetcher.rs src/insert_checker/mod.rs
git commit -m "feat(insert-checker): meta_fetcher — fetch and cache TableMeta from information_schema"
```

---

## Task 6: `checker.rs` — check logic

**Files:**
- Create: `src/insert_checker/checker.rs`

This is the core of the feature. Static checks (NOT NULL, length, type) are fully unit-testable. DB-query checks (PK, UK, FK) require a live MySQL connection — unit tests cover the static path only; DB paths are verified by integration/manual testing.

- [ ] **Step 1: Write the type definitions and static-check tests first**

Create `src/insert_checker/checker.rs`:

```rust
use std::collections::HashMap;
use anyhow::Result;
use sqlx::MySqlPool;

use crate::insert_checker::meta_fetcher::{ColumnMeta, FkMeta, MetaCache, TableMeta};
use crate::insert_checker::parser::{InsertStmt, SqlValue};

// ── Output types ──────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct Finding {
    pub stmt_index: usize,
    pub original_sql: String,
    pub row_findings: Vec<RowFinding>,
}

#[derive(Debug)]
pub struct RowFinding {
    pub row_index: usize,
    pub failures: Vec<FailureDetail>,
}

#[derive(Debug)]
pub struct FailureDetail {
    pub kind: FailureKind,
    pub message: String,
    pub suggestions: Vec<Suggestion>,
}

#[derive(Debug)]
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

#[derive(Debug, Clone)]
pub enum Suggestion {
    Skip,
    InsertIgnore,
    OnDuplicateKeyUpdate { columns: Vec<String> },
    FixValue { column: String, hint: String },
}

// ── Numeric data types for type-mismatch check ────────────────────────────────

const NUMERIC_TYPES: &[&str] = &[
    "tinyint", "smallint", "mediumint", "int", "bigint",
    "decimal", "float", "double", "numeric", "real",
];

// ── Public API ────────────────────────────────────────────────────────────────

/// Run all checks for a single INSERT statement.
/// `stmt_index` is 1-based.
pub async fn check_stmt(
    pool: &MySqlPool,
    cache: &mut MetaCache,
    stmt: &InsertStmt,
    default_db: &str,
    stmt_index: usize,
) -> Result<Option<Finding>> {
    let db = stmt.database.as_deref().unwrap_or(default_db);

    // 5.1 — Table existence
    let meta = match cache.get(pool, db, &stmt.table).await? {
        Some(m) => m,
        None => {
            return Ok(Some(Finding {
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
            }));
        }
    };

    // 5.2 — Column existence (named INSERT only)
    let mut unknown_cols: Vec<String> = Vec::new();
    if !stmt.columns.is_empty() {
        let known: std::collections::HashSet<&str> =
            meta.columns.iter().map(|c| c.name.as_str()).collect();
        for col in &stmt.columns {
            if !known.contains(col.as_str()) {
                unknown_cols.push(col.clone());
            }
        }
    }

    // Statement-level failures (UnknownTable was already handled; UnknownColumn is statement-level)
    if !unknown_cols.is_empty() {
        let failures: Vec<FailureDetail> = unknown_cols.iter().map(|col| FailureDetail {
            kind: FailureKind::UnknownColumn { column: col.clone() },
            message: format!("Column `{}` does not exist in `{}`", col, stmt.table),
            suggestions: vec![],
        }).collect();
        // Return early — downstream checks cannot reliably run with unknown columns
        return Ok(Some(Finding {
            stmt_index,
            original_sql: stmt.original_sql.clone(),
            row_findings: vec![RowFinding { row_index: 1, failures }],
        }));
    }

    // Per-row checks
    let mut row_findings: Vec<RowFinding> = Vec::new();

    for (row_idx, row) in stmt.rows.iter().enumerate() {
        // 5.3 — Build effective column-to-value mapping
        let col_map = build_col_map(stmt, row, meta);

        let mut failures: Vec<FailureDetail> = Vec::new();

        // Note SkippedExpression for any Other values
        for (col_name, val) in &col_map {
            if matches!(val, SqlValue::Other(_)) {
                failures.push(FailureDetail {
                    kind: FailureKind::SkippedExpression { column: col_name.clone() },
                    message: format!("Column `{}`: value is an expression; dynamic checks skipped", col_name),
                    suggestions: vec![],
                });
            }
        }

        // 5.4 — Static checks
        failures.extend(check_not_null(stmt, &col_map, meta));
        failures.extend(check_lengths(&col_map, meta));
        failures.extend(check_types(&col_map, meta));

        // 5.5 — PK conflict
        if let Some(f) = check_pk_conflict(pool, db, stmt, &col_map, meta).await? {
            failures.push(f);
        }

        // 5.6 — UK conflicts
        failures.extend(check_uk_conflicts(pool, db, stmt, &col_map, meta).await?);

        // 5.7 — FK violations
        failures.extend(check_fk_violations(pool, &col_map, meta).await?);

        if !failures.is_empty() {
            row_findings.push(RowFinding { row_index: row_idx + 1, failures });
        }
    }

    if row_findings.is_empty() {
        Ok(None)
    } else {
        Ok(Some(Finding { stmt_index, original_sql: stmt.original_sql.clone(), row_findings }))
    }
}

// ── Internal helpers ──────────────────────────────────────────────────────────

/// Build a map from column name → &SqlValue for one row.
/// For positional INSERTs, maps by ORDINAL_POSITION order.
/// If there are more values than columns, extra values are ignored and a warning is noted
/// (handled upstream in the orchestrator; here we just truncate).
fn build_col_map<'a>(
    stmt: &InsertStmt,
    row: &'a [SqlValue],
    meta: &TableMeta,
) -> HashMap<String, &'a SqlValue> {
    let mut map = HashMap::new();
    if stmt.columns.is_empty() {
        // positional
        for (i, col) in meta.columns.iter().enumerate() {
            if let Some(val) = row.get(i) {
                map.insert(col.name.clone(), val);
            }
        }
    } else {
        for (col, val) in stmt.columns.iter().zip(row.iter()) {
            map.insert(col.clone(), val);
        }
    }
    map
}

fn check_not_null<'a>(
    stmt: &InsertStmt,
    col_map: &HashMap<String, &'a SqlValue>,
    meta: &TableMeta,
) -> Vec<FailureDetail> {
    let mut failures = Vec::new();
    for col in &meta.columns {
        if col.is_nullable {
            continue;
        }
        match col_map.get(&col.name) {
            None => {
                // Column absent from named INSERT
                if !stmt.columns.is_empty() && !col.has_default {
                    failures.push(FailureDetail {
                        kind: FailureKind::NotNullViolation { column: col.name.clone() },
                        message: format!(
                            "Column `{}` is NOT NULL with no default but is missing from the INSERT",
                            col.name
                        ),
                        suggestions: vec![Suggestion::FixValue {
                            column: col.name.clone(),
                            hint: "provide a non-null value".into(),
                        }],
                    });
                }
            }
            Some(SqlValue::Null) => {
                failures.push(FailureDetail {
                    kind: FailureKind::NotNullViolation { column: col.name.clone() },
                    message: format!("Column `{}` is NOT NULL but value is NULL", col.name),
                    suggestions: vec![Suggestion::FixValue {
                        column: col.name.clone(),
                        hint: "provide a non-null value".into(),
                    }],
                });
            }
            Some(SqlValue::Other(_)) => {}  // skipped (expression)
            _ => {}
        }
    }
    failures
}

fn check_lengths<'a>(
    col_map: &HashMap<String, &'a SqlValue>,
    meta: &TableMeta,
) -> Vec<FailureDetail> {
    let mut failures = Vec::new();
    for col in &meta.columns {
        let Some(max) = col.char_max_length else { continue };
        let Some(val) = col_map.get(&col.name) else { continue };
        if let SqlValue::Str(s) = val {
            let actual = s.chars().count();
            if actual as u64 > max {
                failures.push(FailureDetail {
                    kind: FailureKind::ColumnLengthExceeded {
                        column: col.name.clone(),
                        max_chars: max,
                        actual_chars: actual,
                    },
                    message: format!(
                        "Column `{}`: value is {} chars, max is {}",
                        col.name, actual, max
                    ),
                    suggestions: vec![Suggestion::FixValue {
                        column: col.name.clone(),
                        hint: format!("truncate to {} characters", max),
                    }],
                });
            }
        }
    }
    failures
}

fn check_types<'a>(
    col_map: &HashMap<String, &'a SqlValue>,
    meta: &TableMeta,
) -> Vec<FailureDetail> {
    let mut failures = Vec::new();
    for col in &meta.columns {
        if !NUMERIC_TYPES.contains(&col.data_type.to_lowercase().as_str()) {
            continue;
        }
        let Some(val) = col_map.get(&col.name) else { continue };
        if let SqlValue::Str(s) = val {
            if s.parse::<f64>().is_err() {
                failures.push(FailureDetail {
                    kind: FailureKind::TypeMismatch {
                        column: col.name.clone(),
                        expected: col.data_type.clone(),
                    },
                    message: format!(
                        "Column `{}` expects numeric type `{}` but got string `{}`",
                        col.name, col.data_type, s
                    ),
                    suggestions: vec![Suggestion::FixValue {
                        column: col.name.clone(),
                        hint: format!("provide a numeric value for type `{}`", col.data_type),
                    }],
                });
            }
        }
    }
    failures
}

async fn check_pk_conflict(
    pool: &MySqlPool,
    db: &str,
    stmt: &InsertStmt,
    col_map: &HashMap<String, &SqlValue>,
    meta: &TableMeta,
) -> Result<Option<FailureDetail>> {
    if meta.primary_key.is_empty() {
        return Ok(None);
    }
    let (where_clause, binds) = build_where(&meta.primary_key, col_map);
    let where_clause = match where_clause {
        Some(w) => w,
        None => return Ok(None),  // some PK columns missing or Other
    };

    let table_ref = format!("`{}`.`{}`", db, stmt.table);
    let sql = format!("SELECT 1 FROM {} WHERE {} LIMIT 1", table_ref, where_clause);

    let exists = run_exists_query(pool, &sql, &binds).await?;
    if exists {
        let non_pk_cols: Vec<String> = stmt.columns.iter()
            .filter(|c| !meta.primary_key.contains(c))
            .cloned()
            .collect();
        Ok(Some(FailureDetail {
            kind: FailureKind::PrimaryKeyConflict,
            message: format!(
                "Primary key conflict: a row with {} already exists in `{}`",
                format_pk_values(&meta.primary_key, col_map),
                stmt.table
            ),
            suggestions: vec![
                Suggestion::InsertIgnore,
                Suggestion::OnDuplicateKeyUpdate { columns: non_pk_cols },
            ],
        }))
    } else {
        Ok(None)
    }
}

async fn check_uk_conflicts(
    pool: &MySqlPool,
    db: &str,
    stmt: &InsertStmt,
    col_map: &HashMap<String, &SqlValue>,
    meta: &TableMeta,
) -> Result<Vec<FailureDetail>> {
    let mut failures = Vec::new();
    for (index_name, uk_cols) in &meta.unique_keys {
        let (where_clause, binds) = build_where(uk_cols, col_map);
        let where_clause = match where_clause {
            Some(w) => w,
            None => continue,
        };
        let table_ref = format!("`{}`.`{}`", db, stmt.table);
        let sql = format!("SELECT 1 FROM {} WHERE {} LIMIT 1", table_ref, where_clause);
        if run_exists_query(pool, &sql, &binds).await? {
            let non_uk_cols: Vec<String> = stmt.columns.iter()
                .filter(|c| !uk_cols.contains(c))
                .cloned()
                .collect();
            failures.push(FailureDetail {
                kind: FailureKind::UniqueKeyConflict { index_name: index_name.clone() },
                message: format!(
                    "Unique key conflict on index `{}`: duplicate value exists in `{}`",
                    index_name, stmt.table
                ),
                suggestions: vec![
                    Suggestion::InsertIgnore,
                    Suggestion::OnDuplicateKeyUpdate { columns: non_uk_cols },
                ],
            });
        }
    }
    Ok(failures)
}

async fn check_fk_violations(
    pool: &MySqlPool,
    col_map: &HashMap<String, &SqlValue>,
    meta: &TableMeta,
) -> Result<Vec<FailureDetail>> {
    let mut failures = Vec::new();
    for fk in &meta.foreign_keys {
        let (where_clause, binds) = build_where(&fk.referenced_columns, &{
            // remap: fk.columns → fk.referenced_columns mapping
            let mut ref_map: HashMap<String, &SqlValue> = HashMap::new();
            for (local_col, ref_col) in fk.columns.iter().zip(fk.referenced_columns.iter()) {
                if let Some(val) = col_map.get(local_col) {
                    ref_map.insert(ref_col.clone(), val);
                }
            }
            ref_map
        });
        let where_clause = match where_clause {
            Some(w) => w,
            None => continue,
        };
        let table_ref = format!("`{}`.`{}`", fk.referenced_database, fk.referenced_table);
        let sql = format!("SELECT 1 FROM {} WHERE {} LIMIT 1", table_ref, where_clause);
        if !run_exists_query(pool, &sql, &binds).await? {
            failures.push(FailureDetail {
                kind: FailureKind::ForeignKeyViolation { fk_columns: fk.columns.clone() },
                message: format!(
                    "Foreign key violation: column(s) `{}` reference `{}`.`{}` but no matching row exists",
                    fk.columns.join(", "),
                    fk.referenced_database,
                    fk.referenced_table
                ),
                suggestions: vec![Suggestion::FixValue {
                    column: fk.columns.join(", "),
                    hint: "insert the referenced row first, or use an existing value".into(),
                }],
            });
        }
    }
    Ok(failures)
}

/// Build a `col1 = ? AND col2 = ?` clause + the bound values.
/// Returns None if any column is missing from col_map or is SqlValue::Other (skip check).
fn build_where(cols: &[String], col_map: &HashMap<String, &SqlValue>) -> (Option<String>, Vec<String>) {
    let mut parts = Vec::new();
    let mut binds = Vec::new();
    for col in cols {
        match col_map.get(col) {
            Some(SqlValue::Other(_)) | None => return (None, vec![]),
            Some(val) => {
                parts.push(format!("`{}` = ?", col));
                binds.push(render_sql_literal(val));
            }
        }
    }
    (Some(parts.join(" AND ")), binds)
}

fn format_pk_values(pk_cols: &[String], col_map: &HashMap<String, &SqlValue>) -> String {
    pk_cols.iter()
        .filter_map(|c| col_map.get(c).map(|v| format!("`{}` = {}", c, render_sql_literal(v))))
        .collect::<Vec<_>>()
        .join(", ")
}

async fn run_exists_query(pool: &MySqlPool, sql: &str, binds: &[String]) -> Result<bool> {
    let mut q = sqlx::query(sql);
    for b in binds {
        q = q.bind(b);
    }
    let row = q.fetch_optional(pool).await?;
    Ok(row.is_some())
}

pub fn render_sql_literal(val: &SqlValue) -> String {
    match val {
        SqlValue::Null => "NULL".into(),
        SqlValue::Number(n) => n.clone(),
        SqlValue::Str(s) => format!("'{}'", s.replace('\'', "''")),
        SqlValue::Bool(true) => "TRUE".into(),
        SqlValue::Bool(false) => "FALSE".into(),
        SqlValue::Other(s) => s.clone(),
    }
}

// ── Unit tests (static checks only, no DB) ───────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_col(name: &str, data_type: &str, nullable: bool, has_default: bool, char_max: Option<u64>) -> ColumnMeta {
        ColumnMeta {
            name: name.into(),
            data_type: data_type.into(),
            is_nullable: nullable,
            has_default,
            char_max_length: char_max,
            numeric_precision: None,
            numeric_scale: None,
        }
    }

    fn make_meta(cols: Vec<ColumnMeta>) -> TableMeta {
        TableMeta {
            columns: cols,
            primary_key: vec![],
            unique_keys: HashMap::new(),
            foreign_keys: vec![],
        }
    }

    fn col_map<'a>(pairs: Vec<(&str, &'a SqlValue)>) -> HashMap<String, &'a SqlValue> {
        pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect()
    }

    // render_sql_literal tests
    #[test]
    fn test_render_null() {
        assert_eq!(render_sql_literal(&SqlValue::Null), "NULL");
    }

    #[test]
    fn test_render_number() {
        assert_eq!(render_sql_literal(&SqlValue::Number("42".into())), "42");
        assert_eq!(render_sql_literal(&SqlValue::Number("-1".into())), "-1");
    }

    #[test]
    fn test_render_str_escapes_single_quote() {
        assert_eq!(render_sql_literal(&SqlValue::Str("it's".into())), "'it''s'");
    }

    #[test]
    fn test_render_bool() {
        assert_eq!(render_sql_literal(&SqlValue::Bool(true)), "TRUE");
        assert_eq!(render_sql_literal(&SqlValue::Bool(false)), "FALSE");
    }

    // check_not_null tests
    #[test]
    fn test_not_null_absent_no_default_flagged() {
        let meta = make_meta(vec![
            make_col("id", "int", false, false, None),
            make_col("email", "varchar", false, false, Some(255)),
        ]);
        let stmt = InsertStmt {
            database: None, table: "t".into(),
            columns: vec!["id".into()],  // email missing
            rows: vec![vec![SqlValue::Number("1".into())]],
            original_sql: "".into(),
        };
        let map = col_map(vec![("id", &SqlValue::Number("1".into()))]);
        let failures = check_not_null(&stmt, &map, &meta);
        assert_eq!(failures.len(), 1);
        assert!(matches!(failures[0].kind, FailureKind::NotNullViolation { ref column } if column == "email"));
    }

    #[test]
    fn test_not_null_absent_with_default_ok() {
        let meta = make_meta(vec![
            make_col("id", "int", false, false, None),
            make_col("created_at", "datetime", false, true, None),  // has_default = true
        ]);
        let stmt = InsertStmt {
            database: None, table: "t".into(),
            columns: vec!["id".into()],
            rows: vec![vec![SqlValue::Number("1".into())]],
            original_sql: "".into(),
        };
        let map = col_map(vec![("id", &SqlValue::Number("1".into()))]);
        let failures = check_not_null(&stmt, &map, &meta);
        assert!(failures.is_empty(), "column with default should not fail NOT NULL check");
    }

    #[test]
    fn test_not_null_explicit_null_flagged() {
        let meta = make_meta(vec![make_col("name", "varchar", false, false, Some(100))]);
        let stmt = InsertStmt {
            database: None, table: "t".into(),
            columns: vec!["name".into()],
            rows: vec![vec![SqlValue::Null]],
            original_sql: "".into(),
        };
        let map = col_map(vec![("name", &SqlValue::Null)]);
        let failures = check_not_null(&stmt, &map, &meta);
        assert_eq!(failures.len(), 1);
    }

    #[test]
    fn test_not_null_nullable_col_ok() {
        let meta = make_meta(vec![make_col("bio", "text", true, false, None)]);
        let stmt = InsertStmt {
            database: None, table: "t".into(),
            columns: vec!["bio".into()],
            rows: vec![vec![SqlValue::Null]],
            original_sql: "".into(),
        };
        let map = col_map(vec![("bio", &SqlValue::Null)]);
        let failures = check_not_null(&stmt, &map, &meta);
        assert!(failures.is_empty());
    }

    // check_lengths tests
    #[test]
    fn test_length_exceeded() {
        let meta = make_meta(vec![make_col("code", "varchar", true, false, Some(3))]);
        let val = SqlValue::Str("toolong".into());
        let map = col_map(vec![("code", &val)]);
        let failures = check_lengths(&map, &meta);
        assert_eq!(failures.len(), 1);
        assert!(matches!(failures[0].kind, FailureKind::ColumnLengthExceeded { max_chars: 3, actual_chars: 7, .. }));
    }

    #[test]
    fn test_length_ok() {
        let meta = make_meta(vec![make_col("code", "varchar", true, false, Some(10))]);
        let val = SqlValue::Str("ok".into());
        let map = col_map(vec![("code", &val)]);
        assert!(check_lengths(&map, &meta).is_empty());
    }

    #[test]
    fn test_number_not_length_checked() {
        let meta = make_meta(vec![make_col("code", "varchar", true, false, Some(3))]);
        let val = SqlValue::Number("999999".into());
        let map = col_map(vec![("code", &val)]);
        assert!(check_lengths(&map, &meta).is_empty(), "Numbers are not length-checked");
    }

    // check_types tests
    #[test]
    fn test_type_mismatch_string_in_int() {
        let meta = make_meta(vec![make_col("age", "int", false, false, None)]);
        let val = SqlValue::Str("not_a_number".into());
        let map = col_map(vec![("age", &val)]);
        let failures = check_types(&map, &meta);
        assert_eq!(failures.len(), 1);
        assert!(matches!(failures[0].kind, FailureKind::TypeMismatch { .. }));
    }

    #[test]
    fn test_type_ok_numeric_string_in_int() {
        let meta = make_meta(vec![make_col("age", "int", false, false, None)]);
        let val = SqlValue::Str("42".into());
        let map = col_map(vec![("age", &val)]);
        assert!(check_types(&map, &meta).is_empty());
    }

    // build_where tests
    #[test]
    fn test_build_where_simple() {
        let val = SqlValue::Number("1".into());
        let map = col_map(vec![("id", &val)]);
        let (clause, binds) = build_where(&["id".to_string()], &map);
        assert_eq!(clause, Some("`id` = ?".to_string()));
        assert_eq!(binds, vec!["1"]);
    }

    #[test]
    fn test_build_where_returns_none_for_other() {
        let val = SqlValue::Other("NOW()".into());
        let map = col_map(vec![("created_at", &val)]);
        let (clause, _) = build_where(&["created_at".to_string()], &map);
        assert!(clause.is_none());
    }

    #[test]
    fn test_build_where_returns_none_for_missing() {
        let map: HashMap<String, &SqlValue> = HashMap::new();
        let (clause, _) = build_where(&["id".to_string()], &map);
        assert!(clause.is_none());
    }
}
```

- [ ] **Step 2: Run the unit tests**

```bash
cargo test insert_checker::checker 2>&1 | tail -30
```

Expected: all unit tests pass. The async functions are not tested here (no DB).

- [ ] **Step 3: Add `mod checker;` to `src/insert_checker/mod.rs`**

```rust
mod checker;
pub use checker::{check_stmt, Finding, RowFinding, FailureDetail, FailureKind, Suggestion, render_sql_literal};
```

- [ ] **Step 4: Build**

```bash
cargo build 2>&1 | tail -10
```

Expected: compiles.

- [ ] **Step 5: Commit**

```bash
git add src/insert_checker/checker.rs src/insert_checker/mod.rs
git commit -m "feat(insert-checker): checker — static + DB-query check logic"
```

---

## Task 7: `reporter.rs` — Markdown rendering

**Files:**
- Create: `src/insert_checker/reporter.rs`

- [ ] **Step 1: Write tests first**

Create `src/insert_checker/reporter.rs`:

```rust
use crate::insert_checker::checker::{
    FailureDetail, FailureKind, Finding, RowFinding, Suggestion,
};
use crate::insert_checker::parser::SqlValue;

pub struct ReportInput<'a> {
    pub database: &'a str,
    pub host: &'a str,
    pub port: u16,
    pub file: &'a str,
    pub generated_at: &'a str,
    pub total_stmts: usize,
    pub skipped_stmts: usize,
    pub findings: &'a [Finding],
}

pub fn render(input: &ReportInput) -> String {
    let mut out = String::new();

    // Header
    out.push_str("# INSERT Check Report\n");
    out.push_str(&format!("Generated: {}\n", input.generated_at));
    out.push_str(&format!("Database:  {} @ {}:{}\n", input.database, input.host, input.port));
    out.push_str(&format!("File:      {}\n\n", input.file));

    // Summary
    let with_issues = input.findings.len();
    let clean = input.total_stmts.saturating_sub(input.skipped_stmts).saturating_sub(with_issues);
    out.push_str("## Summary\n");
    out.push_str(&format!("- Total INSERT statements parsed: {}\n", input.total_stmts));
    out.push_str(&format!("- Statements skipped (parse warning or INSERT-SELECT): {}\n", input.skipped_stmts));
    out.push_str(&format!("- Statements with issues: {}\n", with_issues));
    out.push_str(&format!("- Clean statements: {}\n", clean));

    if input.findings.is_empty() {
        out.push_str("\nAll INSERT statements look clean!\n");
        return out;
    }

    out.push_str("\n---\n");

    for (issue_idx, finding) in input.findings.iter().enumerate() {
        out.push_str(&format!("\n## Issue #{} — Statement {}\n\n", issue_idx + 1, finding.stmt_index));
        out.push_str("**Original SQL:**\n");
        out.push_str("```sql\n");
        out.push_str(&finding.original_sql);
        out.push_str("\n```\n");

        for rf in &finding.row_findings {
            out.push_str(&format!("\n### Row {}\n", rf.row_index));

            for (fail_idx, detail) in rf.failures.iter().enumerate() {
                out.push_str(&format!("\n#### {} {}\n", failure_label(fail_idx + 1, &detail.kind), failure_title(&detail.kind)));
                out.push_str(&format!("{}\n", detail.message));

                if !detail.suggestions.is_empty() {
                    out.push_str("\n**Suggestions:**\n");
                    for suggestion in &detail.suggestions {
                        render_suggestion(&mut out, suggestion, finding);
                    }
                }
            }
        }

        out.push_str("\n---\n");
    }

    out
}

fn failure_label(idx: usize, kind: &FailureKind) -> String {
    if matches!(kind, FailureKind::SkippedExpression { .. }) {
        "Note:".into()
    } else {
        format!("Failure {}:", idx)
    }
}

fn failure_title(kind: &FailureKind) -> String {
    match kind {
        FailureKind::PrimaryKeyConflict => "Primary Key Conflict".into(),
        FailureKind::UniqueKeyConflict { index_name } => format!("Unique Key Conflict (`{}`)", index_name),
        FailureKind::ForeignKeyViolation { .. } => "Foreign Key Violation".into(),
        FailureKind::NotNullViolation { .. } => "NOT NULL Violation".into(),
        FailureKind::ColumnLengthExceeded { .. } => "Column Length Exceeded".into(),
        FailureKind::TypeMismatch { .. } => "Type Mismatch".into(),
        FailureKind::UnknownTable => "Unknown Table".into(),
        FailureKind::UnknownColumn { .. } => "Unknown Column".into(),
        FailureKind::SkippedExpression { column } => format!("Expression Value — Column `{}`", column),
    }
}

fn render_suggestion(out: &mut String, suggestion: &Suggestion, finding: &Finding) {
    match suggestion {
        Suggestion::Skip => {
            out.push_str("- Comment out this statement to skip it.\n");
        }
        Suggestion::InsertIgnore => {
            out.push_str("- Use `INSERT IGNORE` to skip silently.\n");
        }
        Suggestion::OnDuplicateKeyUpdate { columns } => {
            if !columns.is_empty() {
                let alias = "new_row";
                let update_clause = columns.iter()
                    .map(|c| format!("`{}` = {}.`{}`", c, alias, c))
                    .collect::<Vec<_>>()
                    .join(", ");
                let rewrite = format!(
                    "{} AS {}\nON DUPLICATE KEY UPDATE {};",
                    finding.original_sql.trim_end_matches(';'),
                    alias,
                    update_clause
                );
                out.push_str("- Or use `ON DUPLICATE KEY UPDATE`:\n");
                out.push_str("```sql\n");
                out.push_str(&rewrite);
                out.push_str("\n```\n");
            }
        }
        Suggestion::FixValue { hint, .. } => {
            out.push_str(&format!("- {}\n", hint));
        }
    }
}

/// Resolve output path: append `.md` if not already present.
pub fn resolve_output_path(output: Option<&str>, default_stem: &str) -> String {
    match output {
        Some(p) if p.ends_with(".md") => p.to_string(),
        Some(p) => format!("{}.md", p),
        None => format!("{}.md", default_stem),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::insert_checker::checker::{FailureDetail, FailureKind, Finding, RowFinding, Suggestion};

    fn make_finding(stmt_index: usize, kind: FailureKind, suggestions: Vec<Suggestion>) -> Finding {
        Finding {
            stmt_index,
            original_sql: "INSERT INTO `t` (`id`) VALUES (1);".into(),
            row_findings: vec![RowFinding {
                row_index: 1,
                failures: vec![FailureDetail {
                    kind,
                    message: "test message".into(),
                    suggestions,
                }],
            }],
        }
    }

    fn base_input<'a>(findings: &'a [Finding]) -> ReportInput<'a> {
        ReportInput {
            database: "mydb",
            host: "localhost",
            port: 3306,
            file: "inserts.sql",
            generated_at: "2026-04-20 12:00:00",
            total_stmts: 10,
            skipped_stmts: 1,
            findings,
        }
    }

    #[test]
    fn test_render_header() {
        let findings = vec![];
        let report = render(&base_input(&findings));
        assert!(report.contains("# INSERT Check Report"));
        assert!(report.contains("Database:  mydb @ localhost:3306"));
        assert!(report.contains("File:      inserts.sql"));
    }

    #[test]
    fn test_render_summary_counts() {
        let findings = vec![make_finding(1, FailureKind::PrimaryKeyConflict, vec![])];
        let report = render(&base_input(&findings));
        assert!(report.contains("Total INSERT statements parsed: 10"));
        assert!(report.contains("Statements skipped"));
        assert!(report.contains(": 1"));
        assert!(report.contains("Statements with issues: 1"));
        assert!(report.contains("Clean statements: 8"));
    }

    #[test]
    fn test_render_no_issues_message() {
        let findings = vec![];
        let report = render(&base_input(&findings));
        assert!(report.contains("All INSERT statements look clean!"));
    }

    #[test]
    fn test_render_pk_conflict_shows_insert_ignore() {
        let findings = vec![make_finding(
            3,
            FailureKind::PrimaryKeyConflict,
            vec![Suggestion::InsertIgnore],
        )];
        let report = render(&base_input(&findings));
        assert!(report.contains("Issue #1 — Statement 3"));
        assert!(report.contains("Primary Key Conflict"));
        assert!(report.contains("INSERT IGNORE"));
    }

    #[test]
    fn test_render_on_duplicate_key_update_snippet() {
        let findings = vec![make_finding(
            1,
            FailureKind::PrimaryKeyConflict,
            vec![Suggestion::OnDuplicateKeyUpdate { columns: vec!["email".into(), "name".into()] }],
        )];
        let report = render(&base_input(&findings));
        assert!(report.contains("ON DUPLICATE KEY UPDATE"));
        assert!(report.contains("`email` = new_row.`email`"));
        assert!(report.contains("`name` = new_row.`name`"));
        assert!(report.contains("AS new_row"));
    }

    #[test]
    fn test_resolve_output_path_appends_md() {
        assert_eq!(resolve_output_path(Some("report"), "default"), "report.md");
        assert_eq!(resolve_output_path(Some("report.md"), "default"), "report.md");
        assert_eq!(resolve_output_path(None, "default"), "default.md");
    }

    #[test]
    fn test_render_skipped_expression_uses_note_label() {
        let findings = vec![make_finding(
            1,
            FailureKind::SkippedExpression { column: "created_at".into() },
            vec![],
        )];
        let report = render(&base_input(&findings));
        assert!(report.contains("Note:"));
        assert!(report.contains("Expression Value"));
    }
}
```

- [ ] **Step 2: Run tests**

```bash
cargo test insert_checker::reporter 2>&1 | tail -20
```

Expected: all tests pass.

- [ ] **Step 3: Add `mod reporter;` to `src/insert_checker/mod.rs`**

```rust
mod reporter;
pub use reporter::{render, resolve_output_path, ReportInput};
```

- [ ] **Step 4: Build**

```bash
cargo build 2>&1 | tail -5
```

- [ ] **Step 5: Commit**

```bash
git add src/insert_checker/reporter.rs src/insert_checker/mod.rs
git commit -m "feat(insert-checker): reporter — render findings to Markdown"
```

---

## Task 8: `mod.rs` — orchestrator (wire everything together)

**Files:**
- Modify: `src/insert_checker/mod.rs`

- [ ] **Step 1: Replace the stub with the full orchestrator**

```rust
mod checker;
mod meta_fetcher;
mod parser;
mod reporter;

pub use checker::{Finding, RowFinding, FailureDetail, FailureKind, Suggestion, render_sql_literal};
pub use meta_fetcher::{MetaCache, TableMeta, ColumnMeta, FkMeta};
pub use parser::{ParseResult, InsertStmt, SqlValue};
pub use reporter::{render, resolve_output_path, ReportInput};

use anyhow::{Context, Result};
use chrono::Local;
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};

use crate::config::CheckInsertsArgs;

pub async fn run(args: CheckInsertsArgs) -> Result<()> {
    // Connect
    let opts = MySqlConnectOptions::new()
        .host(&args.host)
        .port(args.port)
        .username(&args.user)
        .password(&args.password);

    let pool = MySqlPoolOptions::new()
        .max_connections(3)
        .acquire_timeout(std::time::Duration::from_secs(30))
        .connect_with(opts)
        .await
        .with_context(|| format!("Failed to connect to {}:{}", args.host, args.port))?;

    eprintln!("Connected to {}:{}", args.host, args.port);

    // Read file
    let sql_content = std::fs::read_to_string(&args.file)
        .with_context(|| format!("Failed to read file: {}", args.file))?;

    // Parse
    let parse_results = parser::parse_sql_file(&sql_content);

    let total_stmts = parse_results.iter().filter(|r| matches!(r, ParseResult::Insert(_))).count();
    let skipped: Vec<&ParseResult> = parse_results.iter().filter(|r| matches!(r, ParseResult::Skipped { .. })).collect();
    let skipped_count = skipped.len();

    eprintln!("Parsed {} INSERT statements, {} skipped", total_stmts, skipped_count);

    // Check
    let mut cache = MetaCache::new();
    let mut findings: Vec<Finding> = Vec::new();
    let mut had_parse_errors = false;

    let mut stmt_counter = 0;
    for result in &parse_results {
        match result {
            ParseResult::Skipped { reason, .. } => {
                if reason.starts_with("parse error") {
                    had_parse_errors = true;
                }
            }
            ParseResult::Insert(stmt) => {
                stmt_counter += 1;
                match checker::check_stmt(&pool, &mut cache, stmt, &args.database, stmt_counter).await {
                    Ok(Some(finding)) => findings.push(finding),
                    Ok(None) => {}
                    Err(e) => {
                        eprintln!("Warning: error checking statement {}: {}", stmt_counter, e);
                    }
                }
            }
        }
    }

    // Resolve output path
    let generated_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let default_stem = format!("insert-check-{}", Local::now().format("%Y%m%d-%H%M%S"));
    let out_path = resolve_output_path(args.output.as_deref(), &default_stem);

    // Render report
    let report = render(&ReportInput {
        database: &args.database,
        host: &args.host,
        port: args.port,
        file: &args.file,
        generated_at: &generated_at,
        total_stmts: total_stmts + skipped_count,
        skipped_stmts: skipped_count,
        findings: &findings,
    });

    std::fs::write(&out_path, &report)
        .with_context(|| format!("Failed to write report to {}", out_path))?;
    eprintln!("Report written to: {}", out_path);

    let had_findings = !findings.is_empty();
    std::process::exit(if had_findings || had_parse_errors { 1 } else { 0 });
}
```

- [ ] **Step 2: Build**

```bash
cargo build 2>&1 | tail -10
```

Expected: compiles with no errors.

- [ ] **Step 3: Run all tests**

```bash
cargo test 2>&1 | tail -30
```

Expected: all unit tests pass.

- [ ] **Step 4: Commit**

```bash
git add src/insert_checker/mod.rs
git commit -m "feat(insert-checker): orchestrator — wire parser, checker, reporter"
```

---

## Task 9: Manual integration smoke test

No automated DB tests are included (requires a live MySQL instance). These are manual verification steps.

- [ ] **Step 1: Build release binary**

```bash
cargo build --release 2>&1 | tail -5
ls -lh target/release/sqltool
```

- [ ] **Step 2: Test `diff` subcommand still works**

```bash
./target/release/sqltool diff --help
```

Expected: shows all the old flags under the `diff` subcommand.

- [ ] **Step 3: Test `check-inserts` help**

```bash
./target/release/sqltool check-inserts --help
```

Expected: shows `--host`, `--port`, `--user`, `--password`, `--database`, `--file`, `--output`.

- [ ] **Step 4: Create a test SQL file**

```bash
cat > /tmp/test_inserts.sql << 'EOF'
-- A valid insert (assuming table exists with id=999 not present)
INSERT INTO `test_table` (`id`, `name`) VALUES (999, 'hello');
-- A potentially conflicting insert
INSERT INTO `test_table` (`id`, `name`) VALUES (1, 'world');
-- An INSERT-SELECT (should be skipped)
INSERT INTO `test_table` SELECT * FROM other_table;
EOF
```

- [ ] **Step 5: Run against a live MySQL instance**

```bash
./target/release/sqltool check-inserts \
  --host <your-host> \
  --user <user> \
  --password <pass> \
  --database <dbname> \
  --file /tmp/test_inserts.sql \
  --output /tmp/insert-report
```

Expected: `/tmp/insert-report.md` is created. Report shows summary, any PK/UK/FK/static failures found, and skipped INSERT-SELECT.

- [ ] **Step 6: Commit final state**

```bash
git add .
git commit -m "feat: sqltool check-inserts — validate INSERT statements against live MySQL"
```

---

## Summary

| Task | What it does |
|------|-------------|
| 1 | Rename binary, add sqlparser |
| 2 | Restructure config.rs for subcommands |
| 3 | Dispatch main.rs on subcommand |
| 4 | Parse SQL file → InsertStmt (unit tested) |
| 5 | Fetch table metadata from information_schema (unit tested struct logic) |
| 6 | All check logic — static fully tested, DB checks verified manually |
| 7 | Markdown report renderer (unit tested) |
| 8 | Orchestrator wires everything |
| 9 | Integration smoke test |
