# mysql-schema-diff

A Rust CLI tool that compares full MySQL schemas between two database instances and produces a structured Markdown diff report.

## What it compares

- Tables — columns (type, nullability, default, extra), indexes, foreign keys, check constraints (MySQL 8.0+)
- Views
- Stored procedures and functions
- Triggers

System databases (`information_schema`, `mysql`, `performance_schema`, `sys`) are always excluded.

## Build

Requires Rust 1.70+ and Cargo.

```bash
cargo build --release
# binary at: target/release/mysql-schema-diff
```

## Usage

```
mysql-schema-diff \
  --base-host <HOST>     --base-port <PORT>  --base-user <USER>  --base-password <PASS> \
  --check-host <HOST>    --check-port <PORT> --check-user <USER> --check-password <PASS> \
  [--databases db1,db2]  [--output report.md]
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
mysql-schema-diff \
  --base-host 10.0.0.1  --base-user root  --base-password secret \
  --check-host 10.0.0.2 --check-user root --check-password secret \
  --databases myapp,myapp_audit \
  --output diff.md
```

Both instances are queried concurrently. Progress is printed to stderr; two output files are written:

- `<base>.md` — human-readable Markdown diff report
- `<base>.sql` — SQL sync script to bring the check instance in line with base

## Exit codes

| Code | Meaning |
|------|---------|
| `0` | Schemas are identical |
| `1` | At least one difference found |
| non-zero (other) | Fatal error (connection failure, I/O error, etc.) |

This makes the tool CI-friendly — you can fail a pipeline on any detected schema drift.

## Output format

### Markdown report (`<base>.md`)

- A header showing the two instance labels and the timestamp
- A summary of how many databases were compared and how many differ
- Per-database sections (databases with no differences are omitted)
  - Tables with column-level, index-level, foreign key, and check constraint diffs
  - Views, procedures, functions, and triggers with before/after definitions

Objects that are identical between the two instances are not listed.

### SQL sync script (`<base>.sql`)

An executable SQL file that, when applied to the **check** instance, brings it in line with **base**:

- `ALTER TABLE` statements for column, index, and foreign key differences
- `CREATE TABLE` for tables present only in base
- `CREATE OR REPLACE` for views, routines, and triggers present only in base
- Lines prefixed with `-- [WARN]` are commented out — they describe objects present in check but not in base (destructive drops require manual review)
- Wrapped in `SET FOREIGN_KEY_CHECKS = 0/1`
- `DELIMITER` directives are included for the `mysql` CLI; strip them when using programmatic APIs

## Architecture

```
CLI args (clap)
    └─► config.rs          — Cli struct, ConnectionConfig, Options
         └─► fetcher.rs    — async queries against information_schema → SchemaSnapshot
              └─► differ.rs       — diff two SchemaSnapshots → DiffReport
                   ├─► reporter.rs      — render DiffReport → Markdown string
                   └─► sql_generator.rs — render DiffReport → SQL sync script
                        └─► main.rs — write .md + .sql files, exit with code
```

## Requirements

- The connecting user needs `SELECT` privilege on `information_schema` and `SHOW DATABASES`.
- For stored routine bodies, the user needs the `SHOW_ROUTINE` privilege (MySQL 8.0+) or `SELECT` on `mysql.proc` (MySQL 5.x).
- Check constraint diffing requires MySQL 8.0+. On older versions the tool falls back to an empty constraint list without error.
