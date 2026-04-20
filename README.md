# sqltool

A Rust CLI with two subcommands for MySQL schema management:

- **`sqltool diff`** — compare full MySQL schemas between two instances and produce a Markdown diff report and SQL sync script
- **`sqltool check-inserts`** — parse a `.sql` file, validate each INSERT against a live MySQL instance using read-only queries, and produce a Markdown report with failure details and fix suggestions

## Build

Requires Rust 1.70+ and Cargo.

```bash
cargo build --release
# binary at: target/release/sqltool
```

---

## `sqltool diff`

Compares two MySQL instances across all (or selected) databases and reports schema differences.

### What it compares

- Tables — columns (type, nullability, default, extra), indexes, foreign keys, check constraints (MySQL 8.0+)
- Views
- Stored procedures and functions
- Triggers

System databases (`information_schema`, `mysql`, `performance_schema`, `sys`) are always excluded.

### Usage

```
sqltool diff \
  --base-host <HOST>     --base-port <PORT>  --base-user <USER>  --base-password <PASS> \
  --check-host <HOST>    --check-port <PORT> --check-user <USER> --check-password <PASS> \
  [--databases db1,db2]  [--output report.md] [--ignore-base-only-dbs]
```

### Options

| Flag | Default | Description |
|------|---------|-------------|
| `--base-host` | *(required)* | Hostname of the base (reference) MySQL instance |
| `--base-port` | `3306` | Port of the base instance |
| `--base-user` | *(required)* | Username for the base instance |
| `--base-password` | *(required)* | Password for the base instance |
| `--check-host` | *(required)* | Hostname of the instance to check |
| `--check-port` | `3306` | Port of the check instance |
| `--check-user` | *(required)* | Username for the check instance |
| `--check-password` | *(required)* | Password for the check instance |
| `--databases` | all non-system DBs | Comma-separated list of databases to compare |
| `--output` | `schema-diff-YYYYMMDD-HHMMSS` | Base path for output files (`.md` and `.sql` are appended) |
| `--ignore-base-only-dbs` | `false` | Silently skip databases present only in the base instance |

### Example

```bash
sqltool diff \
  --base-host 10.0.0.1  --base-user root  --base-password secret \
  --check-host 10.0.0.2 --check-user root --check-password secret \
  --databases myapp,myapp_audit \
  --output diff.md
```

Both instances are queried concurrently. Progress is printed to stderr; two output files are written:

- `<output>.md` — human-readable Markdown diff report
- `<output>.sql` — SQL sync script to bring the check instance in line with base

### Output format

**Markdown report** (`<output>.md`):
- Header showing the two instance labels and timestamp
- Summary of databases compared and how many differ
- Per-database sections (identical databases are omitted) with column-, index-, FK-, and check-constraint-level diffs

**SQL sync script** (`<output>.sql`):
- `ALTER TABLE` for column, index, and foreign key differences
- `CREATE TABLE` for tables present only in base
- `CREATE OR REPLACE` for views, routines, and triggers present only in base
- Lines prefixed `-- [WARN]` are commented out — they describe objects present only in check (destructive drops require manual review)
- Wrapped in `SET FOREIGN_KEY_CHECKS = 0/1`; includes `DELIMITER` directives for the `mysql` CLI

### Exit codes

| Code | Meaning |
|------|---------|
| `0` | Schemas are identical |
| `1` | At least one difference found |
| non-zero (other) | Fatal error (connection failure, I/O error, etc.) |

### Required privileges

- `SELECT` on `information_schema` and `SHOW DATABASES`
- `SHOW_ROUTINE` (MySQL 8.0+) or `SELECT` on `mysql.proc` (MySQL 5.x) for stored routine bodies
- Check constraint diffing requires MySQL 8.0+; older versions fall back to an empty list without error

---

## `sqltool check-inserts`

Reads a `.sql` file, parses every `INSERT` statement, and validates it against a live MySQL instance using **read-only** `SELECT` queries. Produces a Markdown report listing statements that would fail and why, with concrete fix suggestions.

No data is written to the database.

### Usage

```
sqltool check-inserts \
  --host <HOST> [--port 3306] \
  --user <USER> \
  --password <PASS> \
  --database <DB> \
  --file inserts.sql \
  [--output report.md]
```

### Options

| Flag | Default | Description |
|------|---------|-------------|
| `--host` | *(required)* | MySQL host |
| `--port` | `3306` | MySQL port |
| `--user` | *(required)* | Username |
| `--password` | *(required)* | Password |
| `--database` | *(required)* | Default database (used when INSERT has no `db.table` qualifier) |
| `--file` | *(required)* | Path to the `.sql` file to check |
| `--output` | `insert-check-YYYYMMDD-HHMMSS.md` | Output report path (`.md` appended if omitted) |

### Checks performed

For each `INSERT` statement, in order:

1. **Table existence** — confirms the target table exists
2. **Column existence** — confirms every named column exists (named INSERTs only)
3. **NOT NULL** — flags columns that are NOT NULL, have no default, and are missing from the INSERT, or are explicitly set to `NULL`
4. **Column length** — flags string values that exceed `CHARACTER_MAXIMUM_LENGTH`
5. **Type mismatch** — flags string values in numeric columns that cannot be parsed as a number
6. **Primary key conflict** — SELECT query to detect duplicate PK rows
7. **Unique key conflict** — SELECT query per unique index
8. **Foreign key violation** — SELECT query to confirm referenced rows exist

`INSERT … SELECT` statements and expressions (e.g. `NOW()`) are noted and skipped rather than erroring.

### Suggestions

For each failure the report includes one or more fix suggestions:

- `INSERT IGNORE` — skip silently on conflict
- `ON DUPLICATE KEY UPDATE … AS new_row` — MySQL 8.0+ alias form
- `FixValue` — concrete hint for type/length/NULL issues
- `FixValue` with FK hint — insert the referenced row first or use an existing value

### Example

```bash
sqltool check-inserts \
  --host localhost --user root --password secret \
  --database myapp \
  --file migrations/seed.sql \
  --output seed-check.md
```

### Exit codes

| Code | Meaning |
|------|---------|
| `0` | No issues found |
| `1` | At least one issue or parse error found |
| non-zero (other) | Fatal error (connection failure, file not found, etc.) |

---

## Architecture

```
src/
├── main.rs              — parse SubCommand enum, dispatch to diff or check-inserts pipeline
├── config.rs            — SubCommand enum, DiffArgs, CheckInsertsArgs
├── fetcher.rs           — async information_schema queries → SchemaSnapshot  (diff)
├── differ.rs            — diff two SchemaSnapshots → DiffReport              (diff)
├── reporter.rs          — DiffReport → Markdown string                       (diff)
├── sql_generator.rs     — DiffReport → SQL sync script                       (diff)
└── insert_checker/
    ├── mod.rs           — orchestrator: file → parse → check → report
    ├── parser.rs        — sqlparser-based extraction of InsertStmt
    ├── meta_fetcher.rs  — fetch & cache TableMeta from information_schema
    ├── checker.rs       — per-statement check logic → Vec<Finding>
    └── reporter.rs      — Vec<Finding> → Markdown report
```
