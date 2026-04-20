# sqltool — INSERT Checker Feature Design Spec

**Date:** 2026-04-20
**Language:** Rust
**Status:** Approved

---

## 1. Purpose

Extend the existing `mysql-schema-diff` tool into a unified CLI binary named `sqltool` with two subcommands:

- `sqltool diff` — the existing schema diff feature (unchanged behavior)
- `sqltool check-inserts` — new feature: parse an `.sql` file, check each INSERT statement against a live MySQL instance using only read-only SELECT queries, and produce a Markdown report identifying statements that would fail and why, with concrete fix suggestions.

---

## 2. CLI Interface

### 2.1 Binary rename

The binary is renamed from `mysql-schema-diff` to `sqltool` in `Cargo.toml`.

### 2.2 Subcommands

```
sqltool diff \
  --base-host <HOST> --base-user <USER> --base-password <PASS> \
  --check-host <HOST> --check-user <USER> --check-password <PASS> \
  [--base-port 3306] [--check-port 3306] \
  [--databases db1,db2] \
  [--output report] \
  [--ignore-base-only-dbs]

sqltool check-inserts \
  --host <HOST> [--port 3306] \
  --user <USER> \
  --password <PASS> \
  --database <DB> \
  --file inserts.sql \
  [--output report]
```

- `diff` flags are identical to the current `mysql-schema-diff` flags.
- `check-inserts --output` defaults to `./insert-check-YYYYMMDD-HHMMSS.md`.
- The MySQL connection for `check-inserts` is **read-only**: no INSERT, UPDATE, DELETE, or DDL is executed.

---

## 3. Module Architecture

```
src/
├── main.rs                  — parse subcommand enum, dispatch to diff pipeline or check_inserts pipeline
├── config.rs                — SubCommand enum, CheckInsertsArgs struct, existing diff structs unchanged
├── fetcher.rs               — unchanged
├── differ.rs                — unchanged
├── reporter.rs              — unchanged
├── sql_generator.rs         — unchanged
├── schema/mod.rs            — unchanged
└── insert_checker/
    ├── mod.rs               — orchestrator: reads .sql file, drives parser → checker → reporter
    ├── parser.rs            — sqlparser-based extraction of InsertStmt { table, database, columns, rows }
    ├── meta_fetcher.rs      — fetches & caches TableMeta per table from information_schema
    ├── checker.rs           — runs checks per InsertStmt, returns Vec<Finding>
    └── reporter.rs          — renders Vec<Finding> → Markdown string, writes to file
```

### New dependency

```toml
sqlparser = { version = "0.54" }
```

Pure Rust, no C dependencies, supports MySQL dialect.

---

## 4. Data Structures

```rust
// parser.rs
struct InsertStmt {
    database: Option<String>,   // if qualified: `db.table`
    table: String,
    columns: Vec<String>,       // explicit column list, or empty if `INSERT INTO t VALUES (...)`
    rows: Vec<Vec<SqlValue>>,   // each row is a list of values
}

enum SqlValue {
    Null,
    Number(String),
    Str(String),
    Bool(bool),
    Other(String),              // expressions, functions — treated as unknown
}

// meta_fetcher.rs
struct TableMeta {
    columns: Vec<ColumnMeta>,
    primary_key: Vec<String>,                    // column names
    unique_keys: HashMap<String, Vec<String>>,   // index_name → column names
    foreign_keys: Vec<FkMeta>,
}

struct ColumnMeta {
    name: String,
    data_type: String,
    is_nullable: bool,
    has_default: bool,
    char_max_length: Option<u64>,
    numeric_precision: Option<u64>,
    numeric_scale: Option<u64>,
}

struct FkMeta {
    columns: Vec<String>,
    referenced_table: String,
    referenced_columns: Vec<String>,
}

// checker.rs
struct Finding {
    stmt_index: usize,          // 1-based position in the .sql file
    original_sql: String,
    failures: Vec<FailureDetail>,
}

struct FailureDetail {
    kind: FailureKind,
    message: String,            // human-readable explanation
    suggestion: Suggestion,
}

enum FailureKind {
    PrimaryKeyConflict,
    UniqueKeyConflict { index_name: String },
    ForeignKeyViolation { fk_columns: Vec<String> },
    NotNullViolation { column: String },
    ColumnLengthExceeded { column: String, max: u64, actual: usize },
    TypeMismatch { column: String, expected: String },
    UnknownTable,
    UnknownColumn { column: String },
}

enum Suggestion {
    Skip,
    InsertIgnore,
    OnDuplicateKeyUpdate { columns: Vec<String> },
    FixValue { column: String, hint: String },
}
```

---

## 5. Check Logic

For each `InsertStmt`, the checker performs the following in order:

### 5.1 Table existence
Query `information_schema.TABLES` to confirm the table exists in the target database. If not: emit `UnknownTable`, skip remaining checks for this statement.

### 5.2 Column existence
Verify every column in the INSERT's column list exists in `TableMeta.columns`. Emit `UnknownColumn` for each missing column.

### 5.3 Static checks (no DB query needed)
- **NOT NULL violation**: if a NOT NULL column with no default is absent from the column list, or its value is `SqlValue::Null`.
- **Column length exceeded**: if a string value's byte length exceeds `char_max_length`.
- **Type mismatch**: if a numeric column receives a non-numeric string value (best-effort).

### 5.4 PK conflict (SELECT query)
```sql
SELECT 1 FROM `<db>`.`<table>` WHERE `pk_col1` = <v1> [AND `pk_col2` = <v2>] LIMIT 1
```
If a row is returned: emit `PrimaryKeyConflict`. Suggestion: `InsertIgnore` or `OnDuplicateKeyUpdate`.

### 5.5 Unique key conflict (SELECT query, per unique index)
Same pattern as PK, one query per unique index that has all its columns present in the INSERT. Suggestion: `InsertIgnore` or `OnDuplicateKeyUpdate`.

### 5.6 Foreign key violation (SELECT query, per FK)
```sql
SELECT 1 FROM `<ref_db>`.`<ref_table>` WHERE `ref_col` = <val> LIMIT 1
```
If NO row is returned: emit `ForeignKeyViolation`. Suggestion: `FixValue`.

### 5.7 Per-row processing
For multi-row INSERTs (`INSERT INTO t VALUES (...), (...)`), each row is checked independently. Findings are tagged with row index within the statement.

---

## 6. Table Metadata Caching

`meta_fetcher` caches `TableMeta` per `(database, table)` key using a `HashMap`. Each table is queried at most once per `check-inserts` run, regardless of how many INSERT statements target it.

---

## 7. Report Format

```markdown
# INSERT Check Report
Generated: 2026-04-20 14:30:00
Database:  mydb @ localhost:3306
File:      inserts.sql

## Summary
- Total INSERT statements: 42
- Statements with issues: 5
- Clean statements: 37

---

## Issue #1 — Statement 7

**Original SQL:**
```sql
INSERT INTO `users` (`id`, `email`, `role_id`) VALUES (42, 'alice@example.com', 99);
```

### Failure 1: Primary Key Conflict
`id = 42` already exists in `users`.

**Suggestion:** Use `INSERT IGNORE` to skip silently, or:
```sql
INSERT INTO `users` (`id`, `email`, `role_id`) VALUES (42, 'alice@example.com', 99)
ON DUPLICATE KEY UPDATE `email` = VALUES(`email`), `role_id` = VALUES(`role_id`);
```

### Failure 2: Foreign Key Violation
Column `role_id = 99` references `roles.id`, but no such row exists.

**Suggestion:** Insert the referenced row first, or change `role_id` to an existing value.

---

## Issue #2 — Statement 15
...
```

---

## 8. Error Handling

- **Connection failure**: print error with host/port, exit code 1.
- **File not found / unreadable**: print error, exit code 1.
- **SQL parse error**: warn about the unparseable statement, skip it, continue.
- **`SqlValue::Other`** (expressions, functions): skip DB-query checks for that column/row and note "value is an expression — skipped dynamic checks" in the report.
- **Exit code**: 0 if no issues found, 1 if any issues found or errors occurred.

---

## 9. Key Dependencies

| Crate        | Purpose                                      |
|--------------|----------------------------------------------|
| `sqlparser`  | Parse INSERT statements into AST (MySQL dialect) |
| `sqlx`       | Async MySQL queries (existing)               |
| `clap`       | Subcommand CLI parsing (existing)            |
| `tokio`      | Async runtime (existing)                     |
| `chrono`     | Timestamp for default output filename (existing) |
| `anyhow`     | Error propagation (existing)                 |

---

## 10. Out of Scope

- Checking non-INSERT statements (UPDATE, DELETE, DDL).
- Executing or dry-running INSERTs (write access not assumed).
- Automatic rewriting of the `.sql` file.
- Non-MySQL databases.
