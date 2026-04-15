# MySQL Schema Diff Tool — Design Spec

**Date:** 2026-04-15  
**Language:** Rust  
**Status:** Approved

---

## 1. Purpose

A command-line tool that compares the full database schema between two MySQL instances (base vs. check) and produces a structured Markdown diff report. The report shows:

- Objects present in base but missing in check
- Objects present in check but missing in base
- Objects present in both but with differences (before/after field-level detail)

---

## 2. Scope of Comparison

| Object Type         | What is compared                                              |
|---------------------|---------------------------------------------------------------|
| Tables              | Column definitions (name, type, nullable, default, extra)     |
| Indexes             | Name, type (BTREE/HASH), uniqueness, column list, order       |
| Foreign Keys        | Name, referenced table/columns, ON DELETE/UPDATE rules        |
| Check Constraints   | Name, expression (MySQL 8.0+)                                 |
| Views               | Full `CREATE VIEW` definition (normalized)                    |
| Stored Procedures   | Full `CREATE PROCEDURE` body                                  |
| Triggers            | Full `CREATE TRIGGER` body                                    |

System schemas (`information_schema`, `mysql`, `performance_schema`, `sys`) are always excluded.

---

## 3. CLI Interface

```
mysql-schema-diff \
  --base-host <HOST>     --check-host <HOST> \
  --base-port <PORT>     --check-port <PORT> \
  --base-user <USER>     --check-user <USER> \
  --base-password <PASS> --check-password <PASS> \
  [--databases db1,db2]  \   # optional: limit to specific schema names
  [--output report.md]       # optional: override default output path
```

Defaults:
- `--base-port` / `--check-port`: `3306`
- `--output`: `./schema-diff-YYYY-MM-DD-HH-MM-SS.md` in the current working directory

---

## 4. Module Architecture

```
src/
├── main.rs        — Entry point: parse args, orchestrate pipeline, write output file
├── config.rs      — ConnectionConfig struct (host, port, user, password, db filter)
├── fetcher.rs     — Async: connect to one DB, discover schemas, fetch all objects
├── schema/
│   └── mod.rs     — Typed structs: SchemaSnapshot, TableDef, ColumnDef, IndexDef,
│                    ForeignKeyDef, CheckDef, ViewDef, ProcedureDef, TriggerDef
├── differ.rs      — Compare two SchemaSnapshots → DiffReport
└── reporter.rs    — Render DiffReport → Markdown string
```

### Data Flow

```
CLI args
  → config.rs       : parse into (BaseConfig, CheckConfig, Options)
  → fetcher.rs      : two tokio tasks run concurrently
                      each returns SchemaSnapshot { databases: HashMap<String, DatabaseSchema> }
  → differ.rs       : diff(base_snapshot, check_snapshot) → DiffReport
  → reporter.rs     : render(diff_report) → String
  → main.rs         : write String to output file
```

---

## 5. Schema Fetching Strategy

All structural data is queried from `information_schema` tables (portable, no superuser required):

| Data                | Source                                              |
|---------------------|-----------------------------------------------------|
| Tables & columns    | `information_schema.COLUMNS`                        |
| Indexes             | `information_schema.STATISTICS`                     |
| Foreign keys        | `information_schema.KEY_COLUMN_USAGE` + `REFERENTIAL_CONSTRAINTS` |
| Check constraints   | `information_schema.CHECK_CONSTRAINTS`              |
| Views               | `information_schema.VIEWS` (VIEW_DEFINITION column) |
| Procedures          | `information_schema.ROUTINES`                       |
| Triggers            | `information_schema.TRIGGERS`                       |

Both databases are fetched concurrently via `tokio::join!`.

---

## 6. Diff Logic

The differ operates on three categories for each object type:

1. **Only in base** — present in base snapshot, absent in check snapshot
2. **Only in check** — present in check snapshot, absent in base snapshot
3. **In both, but different** — present in both; field-by-field comparison with before/after values

For tables, the diff is recursive: the table itself is compared first, then its columns, indexes, foreign keys, and check constraints are each diffed independently.

---

## 7. Report Format (Markdown)

```markdown
# Schema Diff Report
Generated: 2026-04-15 10:30:00
Base:  host1:3306
Check: host2:3306

## Summary
- Databases compared: 3
- Tables: 2 added, 1 removed, 5 changed
- ...

## Database: `mydb`

### Tables

#### ⚠ Changed: `users`
##### Columns
| Column | Base | Check |
|--------|------|-------|
| age    | TINYINT NOT NULL | INT NOT NULL |

##### Indexes
...

#### ✚ Only in base: `orders`
...

#### ✖ Only in check: `shipments`
...

### Views
...

### Procedures
...

### Triggers
...
```

---

## 8. Error Handling

- Connection failures: print clear error message with which side (base/check) failed, exit with code 1
- Schema not found: listed as "only in base" or "only in check" in the report
- MySQL 5.x vs 8.x: check constraints query gracefully returns empty if table doesn't exist
- Insufficient privileges: surface the MySQL error message directly

---

## 9. Key Dependencies

| Crate             | Purpose                                  |
|-------------------|------------------------------------------|
| `sqlx`            | Async MySQL queries, offline mode        |
| `clap`            | CLI argument parsing (derive feature)    |
| `tokio`           | Async runtime                            |
| `chrono`          | Timestamp for default output filename    |
| `anyhow`          | Ergonomic error propagation              |

---

## 10. Out of Scope

- Data comparison (row-level diffs)
- Schema migration script generation (e.g., ALTER TABLE statements)
- Non-MySQL databases
- Authentication via TLS client certificates (future)
