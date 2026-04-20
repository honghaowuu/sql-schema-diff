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
- `check-inserts --output`: if the value ends with `.md` it is used as-is; otherwise `.md` is appended. Defaults to `./insert-check-YYYYMMDD-HHMMSS.md`.
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
// config.rs
struct CheckInsertsArgs {
    host: String,
    port: u16,          // default 3306
    user: String,
    password: String,
    database: String,
    file: String,       // path to .sql file
    output: Option<String>,
}

// parser.rs
struct InsertStmt {
    database: Option<String>,   // if qualified: `db.table`
    table: String,
    columns: Vec<String>,       // explicit column list; empty means positional (INSERT INTO t VALUES (...))
    rows: Vec<Vec<SqlValue>>,   // each row is a list of values
    original_sql: String,       // raw text of the statement, for reporting
}

// `INSERT INTO t SELECT ...` statements are recognized and skipped (emitted as a warning in
// the report, no checks performed, same treatment as a parse warning).

enum SqlValue {
    Null,
    Number(String),
    Str(String),
    Bool(bool),
    Other(String),  // expressions, functions — unknown at parse time
}

// meta_fetcher.rs
struct TableMeta {
    columns: Vec<ColumnMeta>,                       // ordered by ORDINAL_POSITION
    primary_key: Vec<String>,                       // column names
    unique_keys: HashMap<String, Vec<String>>,      // index_name → column names
    foreign_keys: Vec<FkMeta>,
}

struct ColumnMeta {
    name: String,
    data_type: String,          // raw value from information_schema.COLUMNS.DATA_TYPE
    is_nullable: bool,
    has_default: bool,
    char_max_length: Option<u64>,   // CHARACTER_MAXIMUM_LENGTH (character count, not bytes)
    numeric_precision: Option<u64>,
    numeric_scale: Option<u64>,
}

struct FkMeta {
    columns: Vec<String>,
    referenced_database: String,    // REFERENCED_TABLE_SCHEMA from KEY_COLUMN_USAGE
    referenced_table: String,
    referenced_columns: Vec<String>,
}

// checker.rs
struct Finding {
    stmt_index: usize,              // 1-based statement number in the .sql file
    original_sql: String,
    row_findings: Vec<RowFinding>,  // one entry per problematic row
}

struct RowFinding {
    row_index: usize,               // 1-based row number within the INSERT (always 1 for single-row INSERTs)
    failures: Vec<FailureDetail>,
}

struct FailureDetail {
    kind: FailureKind,
    message: String,                // human-readable explanation
    suggestions: Vec<Suggestion>,   // one or more applicable suggestions
}

enum FailureKind {
    PrimaryKeyConflict,
    UniqueKeyConflict { index_name: String },
    ForeignKeyViolation { fk_columns: Vec<String> },
    NotNullViolation { column: String },
    ColumnLengthExceeded { column: String, max_chars: u64, actual_chars: usize },
    TypeMismatch { column: String, expected: String },
    UnknownTable,
    UnknownColumn { column: String },
}

enum Suggestion {
    Skip,
    InsertIgnore,
    OnDuplicateKeyUpdate { columns: Vec<String> },  // rendered using alias form (MySQL 8.0+)
    FixValue { column: String, hint: String },
}
```

---

## 5. Check Logic

For each `InsertStmt`, the checker performs the following in order. Checks within a step apply per-row unless otherwise noted. If a column's value is `SqlValue::Other`, **all checks for that column are skipped** (both static and DB-query checks); the report notes "value is an expression — skipped".

### 5.1 Table existence (statement-level)

```sql
SELECT 1 FROM information_schema.TABLES
WHERE TABLE_SCHEMA = '<db>' AND TABLE_NAME = '<table>' LIMIT 1
```

If no row returned: emit `UnknownTable` at statement level, skip all remaining checks for this statement.

### 5.2 Column existence (statement-level, named INSERTs only)

For each column in `InsertStmt.columns`, verify it exists in `TableMeta.columns`. Emit `UnknownColumn` for each missing column. Unknown columns are excluded from all downstream checks (do not attempt to look up their values).

### 5.3 Positional INSERT column mapping

When `InsertStmt.columns` is empty (positional INSERT: `INSERT INTO t VALUES (...)`), map row values to columns by `ORDINAL_POSITION` order from `TableMeta.columns`. If a row has more values than the table has columns, emit a parse-level warning and skip that row. Static checks (5.4) then proceed using this mapped column list.

### 5.4 Static checks (per row, no DB query)

- **NOT NULL violation**: Emit `NotNullViolation` in either of these cases:
  - Named INSERT: the column name does not appear in `InsertStmt.columns` **and** `has_default = false`, **or** the column is listed but a row supplies `SqlValue::Null` as its value. When `has_default = true`, the absence-based check is suppressed (MySQL will use the default); the explicit-null check still applies.
  - Positional INSERT: the mapped value for the column is `SqlValue::Null`.
  Suggestion: `FixValue`.
- **Column length exceeded**: For a `Str(s)` value in a column with `char_max_length = Some(n)`: if `s.chars().count() > n`, emit `ColumnLengthExceeded { max_chars: n, actual_chars: s.chars().count() }`. `SqlValue::Number` values are **not** length-checked against `char_max_length` (MySQL coerces numbers into strings implicitly). Suggestion: `FixValue`.
- **Type mismatch**: For a column whose `data_type` is one of `tinyint, smallint, mediumint, int, bigint, decimal, float, double, numeric, real`, if the value is `Str(s)` and `s` cannot be parsed as a number (`s.parse::<f64>()` fails), emit `TypeMismatch`. Suggestion: `FixValue`.

### 5.5 PK conflict (SELECT query, per row)

If all PK columns are present and have non-`Other` values:

```sql
SELECT 1 FROM `<db>`.`<table>` WHERE `pk_col1` = <v1> [AND `pk_col2` = <v2>] LIMIT 1
```

If a row is returned: emit `PrimaryKeyConflict`. Suggestions: `[InsertIgnore, OnDuplicateKeyUpdate { columns: <all non-pk columns present in the INSERT> }]`.

### 5.6 Unique key conflict (SELECT query, per unique index, per row)

For each unique index where all its columns are present in the INSERT and have non-`Other` values, run:

```sql
SELECT 1 FROM `<db>`.`<table>` WHERE `uk_col1` = <v1> [AND ...] LIMIT 1
```

If a row is returned: emit `UniqueKeyConflict { index_name }`. Suggestions: `[InsertIgnore, OnDuplicateKeyUpdate { columns: <all non-uk columns present in the INSERT> }]`.

**Conflict between PK and UK on the same row:** If both a `PrimaryKeyConflict` and one or more `UniqueKeyConflict`s are detected on the same row, each is emitted as a separate `FailureDetail`. The `OnDuplicateKeyUpdate` suggestion in the `PrimaryKeyConflict` entry takes precedence for the rendered SQL snippet (since MySQL `ON DUPLICATE KEY UPDATE` fires on any unique constraint violation, a single `ON DUPLICATE KEY UPDATE` covering non-PK columns is the correct unified fix). UK conflict entries still list `OnDuplicateKeyUpdate` in their `suggestions` field but their rendered SQL is identical to the PK conflict's snippet.

**Rendered suggestion SQL table reference:** When generating the suggestion SQL snippet, use the qualified form `` `<db>`.`<table>` `` if `InsertStmt.database` is `Some(db)`, or the unqualified form `` `<table>` `` if `InsertStmt.database` is `None` (the reader's session is assumed to be set to `--database`).

### 5.7 Foreign key violation (SELECT query, per FK, per row)

For each FK where all FK columns are present and have non-`Other` values:

```sql
SELECT 1 FROM `<ref_db>`.`<ref_table>` WHERE `ref_col1` = <v1> [AND ...] LIMIT 1
```

`<ref_db>` comes from `FkMeta.referenced_database`. If **no** row is returned: emit `ForeignKeyViolation`. Suggestion: `[FixValue { column: <fk_columns joined>, hint: "insert the referenced row first, or use an existing value" }]`.

### 5.8 Database resolution

For every query in 5.1–5.7, the target database is resolved as follows:
- If `InsertStmt.database` is `Some(db)`, use `db`.
- If `InsertStmt.database` is `None`, use `CheckInsertsArgs.database` (the `--database` flag value).

### 5.9 Query volume note

Checks are issued one SELECT per constraint per row (no batching). For large INSERTs this may be slow; batching is out of scope and deferred to a future improvement.

---

## 6. SqlValue → SQL Literal Rendering

When generating suggestion SQL snippets in the report (e.g., the `ON DUPLICATE KEY UPDATE` rewrite), `SqlValue` variants are rendered as follows:

| Variant | Rendered SQL literal |
|---------|----------------------|
| `Null` | `NULL` |
| `Number(s)` | `s` (unquoted; value is already a valid numeric literal from `sqlparser`) |
| `Str(s)` | `'<s>'` with single quotes escaped as `''` (standard SQL escaping) |
| `Bool(true)` | `TRUE` |
| `Bool(false)` | `FALSE` |
| `Other(s)` | `s` (raw expression, unquoted — this case does not arise in suggestion SQL since `Other` values skip all checks, but the renderer handles it defensively) |

---

## 7. RowFinding Grouping

- A `Finding` is emitted for a statement if **at least one row** has one or more failures.
- `Finding.row_findings` contains only rows that have at least one failure (rows with no failures are omitted).
- A statement is counted as "with issues" in the summary if its `Finding.row_findings` is non-empty.
- The `SqlValue::Other` skipped-check note is emitted as a `FailureDetail` entry within the relevant `RowFinding`, with `kind: TypeMismatch` replaced by a dedicated `SkippedExpression { column: String }` variant — **add this to `FailureKind`**:

```rust
enum FailureKind {
    // ... existing variants ...
    SkippedExpression { column: String },  // value is an expression; dynamic checks skipped
}
```

The `SkippedExpression` detail has no suggestion (`suggestions: vec![]`). In the Markdown report it renders as:

```
#### Note: Expression Value — Column `<col>`
Value is an expression; dynamic checks skipped for this column.
```

---

## 9. Table Metadata Caching

`meta_fetcher` caches `TableMeta` per `(database, table)` key in a `HashMap`. Each table's metadata is fetched at most once per `check-inserts` run.

---

## 10. Report Format

```markdown
# INSERT Check Report
Generated: 2026-04-20 14:30:00
Database:  mydb @ localhost:3306
File:      inserts.sql

## Summary
- Total INSERT statements parsed: 42
- Statements skipped (parse warning or INSERT-SELECT): 1
- Statements with issues: 5
- Clean statements: 36

---

## Issue #1 — Statement 7

**Original SQL:**
```sql
INSERT INTO `users` (`id`, `email`, `role_id`) VALUES (42, 'alice@example.com', 99);
```

### Row 1

#### Failure 1: Primary Key Conflict
`id = 42` already exists in `users`.

**Suggestions:**
- Use `INSERT IGNORE` to skip silently.
- Or use `ON DUPLICATE KEY UPDATE`:
```sql
INSERT INTO `users` (`id`, `email`, `role_id`) VALUES (42, 'alice@example.com', 99) AS new_row
ON DUPLICATE KEY UPDATE `email` = new_row.`email`, `role_id` = new_row.`role_id`;
```

#### Failure 2: Foreign Key Violation
Column `role_id = 99` references `roles.id` in database `mydb`, but no such row exists.

**Suggestions:**
- Insert the referenced row in `roles` first, or change `role_id` to an existing value.

---

## Issue #2 — Statement 15
...
```

---

## 11. Error Handling

| Situation | Behavior | Exit code contribution |
|-----------|----------|------------------------|
| Connection failure | Print error with host/port, exit immediately | 1 |
| File not found / unreadable | Print error, exit immediately | 1 |
| SQL parse error for a statement | Warn in report, skip that statement, continue | 1 |
| INSERT-SELECT statement | Warn in report ("skipped: INSERT-SELECT not checkable"), continue | 0 (not a tool error) |
| `SqlValue::Other` in a value | Skip checks for that column, note in report | 0 |
| No issues found | Normal completion | 0 |
| Any issues found | Normal completion | 1 |

Exit code policy: the final exit code is `1` if **any** row in the table above contributed exit code `1`; otherwise `0`. Specifically: `std::process::exit(if had_error || had_findings || had_parse_errors { 1 } else { 0 })`. Callers that need to distinguish tool errors from finding issues should inspect the report file.

---

## 12. Key Dependencies

| Crate        | Purpose                                          |
|--------------|--------------------------------------------------|
| `sqlparser`  | Parse INSERT statements into AST (MySQL dialect) |
| `sqlx`       | Async MySQL queries (existing)                   |
| `clap`       | Subcommand CLI parsing (existing)                |
| `tokio`      | Async runtime (existing)                         |
| `chrono`     | Timestamp for default output filename (existing) |
| `anyhow`     | Error propagation (existing)                     |

---

## 13. Out of Scope

- Checking non-INSERT statements (UPDATE, DELETE, DDL).
- Executing or dry-running INSERTs (write access not assumed).
- Automatic rewriting of the `.sql` file.
- Non-MySQL databases.
- Batching SELECT queries for large multi-row INSERTs (deferred).
