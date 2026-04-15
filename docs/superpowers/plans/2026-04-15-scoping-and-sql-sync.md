# Scoping & SQL Sync Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add `--ignore-base-only-dbs` flag and generate a `.sql` sync file alongside the Markdown diff report.

**Architecture:** Feature 1 is a one-liner in `differ::diff()` guarded by a new CLI flag. Feature 2 adds `src/sql_generator.rs` following the same pipeline pattern as `reporter.rs` — it consumes a `&DiffReport` and returns `(String, Vec<String>)` (SQL content + warnings). `main.rs` wires both outputs together.

**Tech Stack:** Rust, clap (CLI parsing), no new dependencies.

---

## File Map

| File | Action | Responsibility |
|---|---|---|
| `src/config.rs` | Modify | Add `--ignore-base-only-dbs` bool flag to `Cli` and `Options` |
| `src/differ.rs` | Modify | Accept `ignore_base_only: bool` param in `diff()`, scope `all_dbs` accordingly |
| `src/sql_generator.rs` | **Create** | `generate(report) -> (String, Vec<String>)` — full SQL sync file |
| `src/main.rs` | Modify | Pass flag to `diff()`, call `sql_generator::generate()`, write `.sql`, print warnings |

---

## Task 1: Add `--ignore-base-only-dbs` flag to config

**Files:**
- Modify: `src/config.rs`

- [ ] **Step 1: Write the failing test**

Add inside the existing `#[cfg(test)] mod tests` block in `src/config.rs`:

```rust
#[test]
fn test_ignore_base_only_dbs_flag() {
    let cli = Cli::parse_from([
        "mysql-schema-diff",
        "--base-host", "h1", "--base-user", "u", "--base-password", "p",
        "--check-host", "h2", "--check-user", "u", "--check-password", "p",
        "--ignore-base-only-dbs",
    ]);
    assert!(cli.options().ignore_base_only_dbs);
}

#[test]
fn test_ignore_base_only_dbs_default_false() {
    let cli = Cli::parse_from([
        "mysql-schema-diff",
        "--base-host", "h1", "--base-user", "u", "--base-password", "p",
        "--check-host", "h2", "--check-user", "u", "--check-password", "p",
    ]);
    assert!(!cli.options().ignore_base_only_dbs);
}
```

- [ ] **Step 2: Run to verify it fails**

```bash
cargo test -q 2>&1 | grep -E "FAILED|error"
```

Expected: compile error — `ignore_base_only_dbs` field doesn't exist yet.

- [ ] **Step 3: Add the flag to `Cli` in `src/config.rs`**

After the existing `output` field in the `Cli` struct, add:

```rust
/// When set, databases present only in the base instance are silently
/// excluded from the diff. Databases present only in check are still reported.
#[arg(long, default_value_t = false)]
pub ignore_base_only_dbs: bool,
```

Add `ignore_base_only_dbs` to the `Options` struct:

```rust
pub struct Options {
    pub databases: Option<Vec<String>>,
    pub output: Option<String>,
    pub base_label: String,
    pub check_label: String,
    pub ignore_base_only_dbs: bool,   // ← add this
}
```

Update `options()` method to set it:

```rust
pub fn options(&self) -> Options {
    Options {
        databases: self.databases.clone(),
        output: self.output.clone(),
        base_label: format!("{}:{}", self.base_host, self.base_port),
        check_label: format!("{}:{}", self.check_host, self.check_port),
        ignore_base_only_dbs: self.ignore_base_only_dbs,
    }
}
```

- [ ] **Step 4: Run to verify tests pass**

```bash
cargo test -q 2>&1 | grep -E "FAILED|error|ok"
```

Expected: all tests pass.

- [ ] **Step 5: Commit**

```bash
git add src/config.rs
git commit -m "feat: add --ignore-base-only-dbs CLI flag"
```

---

## Task 2: Update `differ::diff()` to respect the scoping flag

**Files:**
- Modify: `src/differ.rs`

- [ ] **Step 1: Write the failing test**

Add inside the existing `#[cfg(test)] mod tests` block in `src/differ.rs`:

```rust
#[test]
fn test_ignore_base_only_dbs_excludes_base_only_db() {
    // base has "mydb", check does not — with flag on, mydb is excluded
    let base = make_snapshot(vec![empty_db("mydb")]);
    let check = make_snapshot(vec![]);
    let report = diff(base, check, "base".into(), "check".into(), "now".into(), true);
    assert!(report.databases.is_empty(), "base-only db should be excluded");
}

#[test]
fn test_ignore_base_only_dbs_keeps_check_only_db() {
    // check has "mydb", base does not — check-only is still reported
    let base = make_snapshot(vec![]);
    let check = make_snapshot(vec![empty_db("mydb")]);
    let report = diff(base, check, "base".into(), "check".into(), "now".into(), true);
    assert_eq!(report.databases.len(), 1);
    assert_eq!(report.databases[0].status, DiffStatus::OnlyInCheck);
}

#[test]
fn test_flag_off_still_reports_base_only_db() {
    // flag off: existing behaviour — base-only db is reported
    let base = make_snapshot(vec![empty_db("mydb")]);
    let check = make_snapshot(vec![]);
    let report = diff(base, check, "base".into(), "check".into(), "now".into(), false);
    assert_eq!(report.databases[0].status, DiffStatus::OnlyInBase);
}
```

- [ ] **Step 2: Run to verify it fails**

```bash
cargo test -q 2>&1 | grep -E "FAILED|error"
```

Expected: compile error — `diff()` signature doesn't accept the new param yet.

- [ ] **Step 3: Update `diff()` signature and scoping logic in `src/differ.rs`**

Change the function signature:

```rust
pub fn diff(
    base: SchemaSnapshot,
    check: SchemaSnapshot,
    base_label: String,
    check_label: String,
    generated_at: String,
    ignore_base_only: bool,    // ← add this
) -> DiffReport {
```

Replace the `all_dbs` construction at the top of `diff()`:

```rust
let all_dbs: BTreeSet<String> = if ignore_base_only {
    check.databases.keys().cloned().collect()
} else {
    let mut s = BTreeSet::new();
    s.extend(base.databases.keys().cloned());
    s.extend(check.databases.keys().cloned());
    s
};
```

- [ ] **Step 4: Fix call sites**

In `src/main.rs`, the `diff()` call needs the new param. Temporarily pass `false` to keep existing behaviour until Task 5 wires it properly:

```rust
let report = differ::diff(
    base_snapshot,
    check_snapshot,
    opts.base_label.clone(),
    opts.check_label.clone(),
    generated_at.clone(),
    false,   // ← temporary; Task 5 replaces with opts.ignore_base_only_dbs
);
```

Also update the existing `diff()` calls in the test module's existing tests — they need a `false` as the last arg (pass `false` to all existing test call sites to preserve their semantics).

- [ ] **Step 5: Run to verify tests pass**

```bash
cargo test -q 2>&1 | grep -E "FAILED|error|ok"
```

Expected: all tests pass including the three new ones.

- [ ] **Step 6: Commit**

```bash
git add src/differ.rs src/main.rs
git commit -m "feat: scope differ to check-side databases when flag is set"
```

---

## Task 3: Scaffold `sql_generator.rs` — file header + empty/same database cases

**Files:**
- Create: `src/sql_generator.rs`
- Modify: `src/main.rs` (add `mod sql_generator;`)

- [ ] **Step 1: Create the file with a failing test**

Create `src/sql_generator.rs`:

```rust
use crate::differ::{DatabaseDiff, DiffReport, DiffStatus, ObjectDiff, TableDiff};
use crate::schema::{
    CheckDef, ColumnDef, ForeignKeyDef, IndexDef, RoutineDef, TriggerDef, ViewDef,
};

/// Generate a SQL sync file from a diff report.
///
/// Returns `(sql_content, warnings)`.
/// Warnings should be printed to stderr by the caller.
pub fn generate(report: &DiffReport) -> (String, Vec<String>) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::differ::*;

    fn empty_report() -> DiffReport {
        DiffReport {
            base_label: "base".into(),
            check_label: "check".into(),
            generated_at: "2026-01-01".into(),
            databases: vec![],
        }
    }

    fn db_diff(name: &str, status: DiffStatus) -> DatabaseDiff {
        DatabaseDiff {
            name: name.into(),
            status,
            tables: vec![],
            views: vec![],
            procedures: vec![],
            functions: vec![],
            triggers: vec![],
        }
    }

    #[test]
    fn test_empty_report_produces_header_only() {
        let (sql, warnings) = generate(&empty_report());
        assert!(sql.contains("Generated by mysql-schema-diff"));
        assert!(sql.contains("base"));
        assert!(sql.contains("check"));
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_same_database_is_skipped() {
        let mut report = empty_report();
        report.databases.push(db_diff("mydb", DiffStatus::Same));
        let (sql, _) = generate(&report);
        assert!(!sql.contains("USE `mydb`"));
    }
}
```

- [ ] **Step 2: Add `mod sql_generator;` to `src/main.rs`**

Add alongside the other `mod` declarations at the top of `src/main.rs`:

```rust
mod sql_generator;
```

- [ ] **Step 3: Run to verify it fails**

```bash
cargo test sql_generator 2>&1 | grep -E "FAILED|error|panicked"
```

Expected: panics with "not yet implemented" (todo!()).

- [ ] **Step 4: Implement the header and database iteration skeleton**

Replace `todo!()` in `generate()`:

```rust
pub fn generate(report: &DiffReport) -> (String, Vec<String>) {
    let mut out = String::new();
    let mut warnings: Vec<String> = Vec::new();

    // File header
    out.push_str("-- Generated by mysql-schema-diff\n");
    out.push_str(&format!("-- Base:  {}\n", report.base_label));
    out.push_str(&format!("-- Check: {}\n", report.check_label));
    out.push_str(&format!("-- Date:  {}\n", report.generated_at));
    out.push_str("--\n");
    out.push_str("-- Apply this file to the CHECK instance to sync it with BASE.\n");
    out.push_str("-- Lines with -- [WARN] are commented out: object exists in check but not base.\n");
    out.push_str("-- DELIMITER directives are for mysql CLI; strip them for programmatic APIs.\n");
    out.push('\n');
    out.push_str("SET FOREIGN_KEY_CHECKS = 0;\n");

    for db in &report.databases {
        if db.status == DiffStatus::Same {
            continue;
        }
        let (db_sql, db_warns) = generate_database(db);
        out.push_str(&db_sql);
        warnings.extend(db_warns);
    }

    out.push_str("\nSET FOREIGN_KEY_CHECKS = 1;\n");

    (out, warnings)
}

fn generate_database(db: &DatabaseDiff) -> (String, Vec<String>) {
    let mut out = String::new();
    let mut warnings: Vec<String> = Vec::new();

    out.push_str(&format!(
        "\n-- ============================================================\n\
         -- Database: {}\n\
         -- ============================================================\n",
        db.name
    ));

    match db.status {
        DiffStatus::OnlyInCheck => {
            let msg = format!(
                "database `{}` exists in check but not in base",
                db.name
            );
            warnings.push(format!("[WARN] {} — review commented DROP in sync SQL", msg));
            out.push_str(&format!("-- [WARN] {}\n", msg));
            out.push_str(&format!("-- DROP DATABASE IF EXISTS `{}`;\n", db.name));
        }
        DiffStatus::OnlyInBase => {
            out.push_str(&format!("CREATE DATABASE IF NOT EXISTS `{}`;\n", db.name));
            out.push_str(&format!("USE `{}`;\n", db.name));
            // Full object generation handled in later tasks via Changed path reuse
        }
        DiffStatus::Changed => {
            out.push_str(&format!("USE `{}`;\n", db.name));
        }
        DiffStatus::Same => {}
    }

    (out, warnings)
}
```

- [ ] **Step 5: Run to verify tests pass**

```bash
cargo test sql_generator 2>&1 | grep -E "FAILED|error|ok"
```

Expected: both tests pass.

- [ ] **Step 6: Commit**

```bash
git add src/sql_generator.rs src/main.rs
git commit -m "feat: scaffold sql_generator with header and database skeleton"
```

---

## Task 4: Column SQL generation

**Files:**
- Modify: `src/sql_generator.rs`

Columns within a `Changed` table: `OnlyInBase` → ADD COLUMN, `OnlyInCheck` → warn + commented DROP, `Changed` → MODIFY COLUMN.

- [ ] **Step 1: Write the failing tests**

Add to the `tests` module in `src/sql_generator.rs`:

```rust
fn col(name: &str, col_type: &str, nullable: bool, default: Option<&str>, extra: &str, pos: u32) -> ColumnDef {
    ColumnDef {
        name: name.into(),
        column_type: col_type.into(),
        is_nullable: nullable,
        column_default: default.map(|s| s.into()),
        extra: extra.into(),
        ordinal_position: pos,
    }
}

fn col_diff(name: &str, status: DiffStatus, base: Option<ColumnDef>, check: Option<ColumnDef>) -> ObjectDiff<ColumnDef> {
    ObjectDiff { name: name.into(), status, base, check }
}

fn changed_table_with_col_diff(col_diffs: Vec<ObjectDiff<ColumnDef>>) -> DatabaseDiff {
    let table = TableDiff {
        name: "users".into(),
        status: DiffStatus::Changed,
        column_diffs: col_diffs,
        index_diffs: vec![],
        fk_diffs: vec![],
        check_diffs: vec![],
    };
    DatabaseDiff {
        name: "mydb".into(),
        status: DiffStatus::Changed,
        tables: vec![table],
        views: vec![], procedures: vec![], functions: vec![], triggers: vec![],
    }
}

#[test]
fn test_add_column_only_in_base() {
    let c = col("email", "varchar(255)", false, None, "", 2);
    let db = changed_table_with_col_diff(vec![
        col_diff("email", DiffStatus::OnlyInBase, Some(c), None),
    ]);
    let report = DiffReport {
        base_label: "b".into(), check_label: "c".into(),
        generated_at: "t".into(), databases: vec![db],
    };
    let (sql, warnings) = generate(&report);
    assert!(sql.contains("ALTER TABLE `users` ADD COLUMN `email` varchar(255) NOT NULL"), "got: {}", sql);
    assert!(warnings.is_empty());
}

#[test]
fn test_modify_column() {
    let base_col = col("age", "tinyint", false, None, "", 3);
    let check_col = col("age", "int", false, None, "", 3);
    let db = changed_table_with_col_diff(vec![
        col_diff("age", DiffStatus::Changed, Some(base_col), Some(check_col)),
    ]);
    let report = DiffReport {
        base_label: "b".into(), check_label: "c".into(),
        generated_at: "t".into(), databases: vec![db],
    };
    let (sql, warnings) = generate(&report);
    assert!(sql.contains("ALTER TABLE `users` MODIFY COLUMN `age` tinyint NOT NULL"), "got: {}", sql);
    assert!(warnings.is_empty());
}

#[test]
fn test_warn_column_only_in_check() {
    let c = col("old_col", "text", true, None, "", 5);
    let db = changed_table_with_col_diff(vec![
        col_diff("old_col", DiffStatus::OnlyInCheck, None, Some(c)),
    ]);
    let report = DiffReport {
        base_label: "b".into(), check_label: "c".into(),
        generated_at: "t".into(), databases: vec![db],
    };
    let (sql, warnings) = generate(&report);
    assert!(sql.contains("-- [WARN]"), "got: {}", sql);
    assert!(sql.contains("-- ALTER TABLE `users` DROP COLUMN `old_col`;"), "got: {}", sql);
    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].contains("old_col"));
}

#[test]
fn test_add_column_with_default_and_extra() {
    let c = col("created_at", "timestamp", false, Some("CURRENT_TIMESTAMP"), "on update CURRENT_TIMESTAMP", 4);
    let db = changed_table_with_col_diff(vec![
        col_diff("created_at", DiffStatus::OnlyInBase, Some(c), None),
    ]);
    let report = DiffReport {
        base_label: "b".into(), check_label: "c".into(),
        generated_at: "t".into(), databases: vec![db],
    };
    let (sql, _) = generate(&report);
    assert!(sql.contains("DEFAULT CURRENT_TIMESTAMP"), "got: {}", sql);
    assert!(sql.contains("on update CURRENT_TIMESTAMP"), "got: {}", sql);
}
```

- [ ] **Step 2: Run to verify they fail**

```bash
cargo test sql_generator::tests::test_add_column 2>&1 | grep -E "FAILED|panicked|error"
```

Expected: test failures (no ADD COLUMN output yet).

- [ ] **Step 3: Implement column helpers and wire into table generation**

Add these helper functions to `src/sql_generator.rs` (above `generate_database`):

```rust
/// Format a column definition clause: `type [NOT NULL|NULL] [DEFAULT x] [extra]`
fn format_col_def(col: &ColumnDef) -> String {
    let mut parts = vec![col.column_type.clone()];
    if col.is_nullable {
        parts.push("NULL".into());
    } else {
        parts.push("NOT NULL".into());
    }
    if let Some(ref d) = col.column_default {
        parts.push(format!("DEFAULT {}", d));
    }
    if !col.extra.is_empty() {
        parts.push(col.extra.clone());
    }
    parts.join(" ")
}

/// Generate column diff SQL for one table. Returns (sql, warnings).
fn generate_col_diffs(
    table_name: &str,
    col_diffs: &[ObjectDiff<ColumnDef>],
) -> (String, Vec<String>) {
    let mut out = String::new();
    let mut warnings: Vec<String> = Vec::new();

    // Build ordinal → name map from base side (for AFTER clause)
    let mut base_cols_by_ord: Vec<(u32, &str)> = col_diffs
        .iter()
        .filter_map(|d| d.base.as_ref().map(|c| (c.ordinal_position, c.name.as_str())))
        .collect();
    base_cols_by_ord.sort_by_key(|(ord, _)| *ord);

    for diff in col_diffs {
        match diff.status {
            DiffStatus::Same => {}
            DiffStatus::OnlyInBase => {
                let col = diff.base.as_ref().unwrap();
                // Determine AFTER clause from ordinal position
                let after = base_cols_by_ord
                    .iter()
                    .filter(|(ord, _)| *ord < col.ordinal_position)
                    .max_by_key(|(ord, _)| *ord)
                    .map(|(_, name)| format!(" AFTER `{}`", name))
                    .unwrap_or_default();
                out.push_str(&format!(
                    "ALTER TABLE `{}` ADD COLUMN `{}` {}{};\n",
                    table_name, col.name, format_col_def(col), after
                ));
            }
            DiffStatus::OnlyInCheck => {
                let col = diff.check.as_ref().unwrap();
                let msg = format!(
                    "column `{}` on `{}` exists in check but not in base",
                    col.name, table_name
                );
                warnings.push(format!("[WARN] {} — review commented DROP in sync SQL", msg));
                out.push_str(&format!("-- [WARN] {}\n", msg));
                out.push_str(&format!(
                    "-- ALTER TABLE `{}` DROP COLUMN `{}`;\n",
                    table_name, col.name
                ));
            }
            DiffStatus::Changed => {
                let col = diff.base.as_ref().unwrap();
                out.push_str(&format!(
                    "ALTER TABLE `{}` MODIFY COLUMN `{}` {};\n",
                    table_name, col.name, format_col_def(col)
                ));
            }
        }
    }

    (out, warnings)
}
```

Add a `generate_table_diffs` function that calls `generate_col_diffs` (index/FK/check diffs handled in later tasks — stub them for now):

```rust
fn generate_table_diffs(tables: &[TableDiff], db_name: &str) -> (String, Vec<String>) {
    let mut out = String::new();
    let mut warnings: Vec<String> = Vec::new();

    for table in tables {
        match table.status {
            DiffStatus::Same => {}
            DiffStatus::OnlyInCheck => {
                let msg = format!("table `{}` in `{}` exists in check but not in base", table.name, db_name);
                warnings.push(format!("[WARN] {} — review commented DROP in sync SQL", msg));
                out.push_str(&format!("-- [WARN] {}\n", msg));
                out.push_str(&format!("-- DROP TABLE IF EXISTS `{}`;\n\n", table.name));
            }
            DiffStatus::OnlyInBase => {
                // Full CREATE TABLE handled in Task 6
                out.push_str(&format!("-- TODO: CREATE TABLE `{}` (see Task 6)\n\n", table.name));
            }
            DiffStatus::Changed => {
                let (col_sql, col_warns) = generate_col_diffs(&table.name, &table.column_diffs);
                out.push_str(&col_sql);
                warnings.extend(col_warns);
                // index/FK/check stubs filled in Tasks 5, 7, 8
            }
        }
    }

    (out, warnings)
}
```

Update `generate_database()` to call `generate_table_diffs` for `Changed` and `OnlyInBase` status:

```rust
DiffStatus::Changed => {
    out.push_str(&format!("USE `{}`;\n\n", db.name));
    let (t_sql, t_warns) = generate_table_diffs(&db.tables, &db.name);
    out.push_str(&t_sql);
    warnings.extend(t_warns);
}
DiffStatus::OnlyInBase => {
    out.push_str(&format!("CREATE DATABASE IF NOT EXISTS `{}`;\n", db.name));
    out.push_str(&format!("USE `{}`;\n\n", db.name));
    let (t_sql, t_warns) = generate_table_diffs(&db.tables, &db.name);
    out.push_str(&t_sql);
    warnings.extend(t_warns);
}
```

- [ ] **Step 4: Run to verify tests pass**

```bash
cargo test sql_generator 2>&1 | grep -E "FAILED|error|ok"
```

Expected: all tests pass.

- [ ] **Step 5: Commit**

```bash
git add src/sql_generator.rs
git commit -m "feat: column ADD/MODIFY/WARN-DROP SQL generation"
```

---

## Task 5: Index SQL generation

**Files:**
- Modify: `src/sql_generator.rs`

- [ ] **Step 1: Write the failing tests**

Add to the test module:

```rust
fn idx(name: &str, unique: bool, idx_type: &str, cols: Vec<(&str, Option<i64>)>) -> IndexDef {
    IndexDef {
        name: name.into(),
        index_type: idx_type.into(),
        is_unique: unique,
        columns: cols.into_iter().enumerate().map(|(i, (col, sp))| IndexColumn {
            column_name: col.into(),
            seq_in_index: (i + 1) as u32,
            sub_part: sp,
        }).collect(),
    }
}

fn idx_diff(name: &str, status: DiffStatus, base: Option<IndexDef>, check: Option<IndexDef>) -> ObjectDiff<IndexDef> {
    ObjectDiff { name: name.into(), status, base, check }
}

fn changed_table_with_idx_diff(idx_diffs: Vec<ObjectDiff<IndexDef>>) -> DatabaseDiff {
    let table = TableDiff {
        name: "users".into(),
        status: DiffStatus::Changed,
        column_diffs: vec![],
        index_diffs: idx_diffs,
        fk_diffs: vec![],
        check_diffs: vec![],
    };
    DatabaseDiff {
        name: "mydb".into(), status: DiffStatus::Changed,
        tables: vec![table], views: vec![], procedures: vec![], functions: vec![], triggers: vec![],
    }
}

#[test]
fn test_add_regular_index() {
    let i = idx("idx_email", false, "BTREE", vec![("email", None)]);
    let db = changed_table_with_idx_diff(vec![idx_diff("idx_email", DiffStatus::OnlyInBase, Some(i), None)]);
    let report = DiffReport { base_label: "b".into(), check_label: "c".into(), generated_at: "t".into(), databases: vec![db] };
    let (sql, _) = generate(&report);
    assert!(sql.contains("ALTER TABLE `users` ADD INDEX `idx_email` (`email`)"), "got: {}", sql);
}

#[test]
fn test_add_unique_index() {
    let i = idx("uq_email", true, "BTREE", vec![("email", None)]);
    let db = changed_table_with_idx_diff(vec![idx_diff("uq_email", DiffStatus::OnlyInBase, Some(i), None)]);
    let report = DiffReport { base_label: "b".into(), check_label: "c".into(), generated_at: "t".into(), databases: vec![db] };
    let (sql, _) = generate(&report);
    assert!(sql.contains("ADD UNIQUE INDEX `uq_email`"), "got: {}", sql);
}

#[test]
fn test_add_primary_key() {
    let i = idx("PRIMARY", false, "BTREE", vec![("id", None)]);
    let db = changed_table_with_idx_diff(vec![idx_diff("PRIMARY", DiffStatus::OnlyInBase, Some(i), None)]);
    let report = DiffReport { base_label: "b".into(), check_label: "c".into(), generated_at: "t".into(), databases: vec![db] };
    let (sql, _) = generate(&report);
    assert!(sql.contains("ADD PRIMARY KEY (`id`)"), "got: {}", sql);
}

#[test]
fn test_partial_index_uses_sub_part() {
    let i = idx("idx_bio", false, "BTREE", vec![("bio", Some(100))]);
    let db = changed_table_with_idx_diff(vec![idx_diff("idx_bio", DiffStatus::OnlyInBase, Some(i), None)]);
    let report = DiffReport { base_label: "b".into(), check_label: "c".into(), generated_at: "t".into(), databases: vec![db] };
    let (sql, _) = generate(&report);
    assert!(sql.contains("`bio`(100)"), "got: {}", sql);
}

#[test]
fn test_hash_index_emits_using_clause() {
    let i = idx("idx_hash", false, "HASH", vec![("token", None)]);
    let db = changed_table_with_idx_diff(vec![idx_diff("idx_hash", DiffStatus::OnlyInBase, Some(i), None)]);
    let report = DiffReport { base_label: "b".into(), check_label: "c".into(), generated_at: "t".into(), databases: vec![db] };
    let (sql, _) = generate(&report);
    assert!(sql.contains("USING HASH"), "got: {}", sql);
}

#[test]
fn test_warn_index_only_in_check() {
    let i = idx("old_idx", false, "BTREE", vec![("x", None)]);
    let db = changed_table_with_idx_diff(vec![idx_diff("old_idx", DiffStatus::OnlyInCheck, None, Some(i))]);
    let report = DiffReport { base_label: "b".into(), check_label: "c".into(), generated_at: "t".into(), databases: vec![db] };
    let (sql, warnings) = generate(&report);
    assert!(sql.contains("-- ALTER TABLE `users` DROP INDEX `old_idx`;"), "got: {}", sql);
    assert_eq!(warnings.len(), 1);
}

#[test]
fn test_changed_index_drops_then_adds() {
    let base_i = idx("idx_x", false, "BTREE", vec![("x", None)]);
    let check_i = idx("idx_x", false, "BTREE", vec![("y", None)]);
    let db = changed_table_with_idx_diff(vec![idx_diff("idx_x", DiffStatus::Changed, Some(base_i), Some(check_i))]);
    let report = DiffReport { base_label: "b".into(), check_label: "c".into(), generated_at: "t".into(), databases: vec![db] };
    let (sql, _) = generate(&report);
    assert!(sql.contains("DROP INDEX"), "got: {}", sql);
    assert!(sql.contains("ADD INDEX"), "got: {}", sql);
}
```

- [ ] **Step 2: Run to verify they fail**

```bash
cargo test sql_generator::tests::test_add_regular_index 2>&1 | grep -E "FAILED|panicked"
```

- [ ] **Step 3: Implement index generation**

Add `format_index_cols` helper and `generate_idx_diffs` function:

```rust
fn format_index_cols(cols: &[IndexColumn]) -> String {
    let mut sorted = cols.to_vec();
    sorted.sort_by_key(|c| c.seq_in_index);
    sorted.iter().map(|c| {
        if let Some(sp) = c.sub_part {
            format!("`{}`({})", c.column_name, sp)
        } else {
            format!("`{}`", c.column_name)
        }
    }).collect::<Vec<_>>().join(", ")
}

fn format_add_index(table_name: &str, idx: &IndexDef) -> String {
    let cols = format_index_cols(&idx.columns);
    let using = if idx.index_type == "HASH" { " USING HASH" } else { "" };
    if idx.name == "PRIMARY" {
        format!("ALTER TABLE `{}` ADD PRIMARY KEY ({}){};\n", table_name, cols, using)
    } else if idx.is_unique {
        format!("ALTER TABLE `{}` ADD UNIQUE INDEX `{}` ({}){};\n", table_name, idx.name, cols, using)
    } else {
        format!("ALTER TABLE `{}` ADD INDEX `{}` ({}){};\n", table_name, idx.name, cols, using)
    }
}

fn generate_idx_diffs(
    table_name: &str,
    idx_diffs: &[ObjectDiff<IndexDef>],
) -> (String, Vec<String>) {
    let mut out = String::new();
    let mut warnings: Vec<String> = Vec::new();

    for diff in idx_diffs {
        match diff.status {
            DiffStatus::Same => {}
            DiffStatus::OnlyInBase => {
                out.push_str(&format_add_index(table_name, diff.base.as_ref().unwrap()));
            }
            DiffStatus::OnlyInCheck => {
                let idx = diff.check.as_ref().unwrap();
                let msg = format!(
                    "index `{}` on `{}` exists in check but not in base",
                    idx.name, table_name
                );
                warnings.push(format!("[WARN] {} — review commented DROP in sync SQL", msg));
                out.push_str(&format!("-- [WARN] {}\n", msg));
                if idx.name == "PRIMARY" {
                    out.push_str(&format!("-- ALTER TABLE `{}` DROP PRIMARY KEY;\n", table_name));
                } else {
                    out.push_str(&format!(
                        "-- ALTER TABLE `{}` DROP INDEX `{}`;\n",
                        table_name, idx.name
                    ));
                }
            }
            DiffStatus::Changed => {
                let base_idx = diff.base.as_ref().unwrap();
                // Drop (commented with info, non-destructive approach: actually do the drop since
                // the index definition changed — this is non-data-destructive)
                if base_idx.name == "PRIMARY" {
                    out.push_str(&format!("ALTER TABLE `{}` DROP PRIMARY KEY;\n", table_name));
                } else {
                    out.push_str(&format!(
                        "ALTER TABLE `{}` DROP INDEX `{}`;\n",
                        table_name, base_idx.name
                    ));
                }
                out.push_str(&format_add_index(table_name, base_idx));
            }
        }
    }

    (out, warnings)
}
```

Wire `generate_idx_diffs` into `generate_table_diffs` inside the `DiffStatus::Changed` arm (after the col_sql lines):

```rust
DiffStatus::Changed => {
    let (col_sql, col_warns) = generate_col_diffs(&table.name, &table.column_diffs);
    out.push_str(&col_sql);
    warnings.extend(col_warns);
    let (idx_sql, idx_warns) = generate_idx_diffs(&table.name, &table.index_diffs);
    out.push_str(&idx_sql);
    warnings.extend(idx_warns);
    // FK/check stubs filled in Tasks 7, 8
}
```

- [ ] **Step 4: Run to verify tests pass**

```bash
cargo test sql_generator 2>&1 | grep -E "FAILED|error|ok"
```

- [ ] **Step 5: Commit**

```bash
git add src/sql_generator.rs
git commit -m "feat: index ADD/DROP/WARN SQL generation with partial index and HASH support"
```

---

## Task 6: Foreign key SQL generation

**Files:**
- Modify: `src/sql_generator.rs`

- [ ] **Step 1: Write the failing tests**

```rust
fn fk(name: &str, cols: Vec<&str>, ref_table: &str, ref_cols: Vec<&str>, on_delete: &str, on_update: &str) -> ForeignKeyDef {
    ForeignKeyDef {
        name: name.into(),
        columns: cols.into_iter().map(|s| s.into()).collect(),
        ref_table: ref_table.into(),
        ref_columns: ref_cols.into_iter().map(|s| s.into()).collect(),
        on_delete: on_delete.into(),
        on_update: on_update.into(),
    }
}

fn fk_diff(name: &str, status: DiffStatus, base: Option<ForeignKeyDef>, check: Option<ForeignKeyDef>) -> ObjectDiff<ForeignKeyDef> {
    ObjectDiff { name: name.into(), status, base, check }
}

fn changed_table_with_fk_diff(fk_diffs: Vec<ObjectDiff<ForeignKeyDef>>) -> DatabaseDiff {
    let table = TableDiff {
        name: "orders".into(), status: DiffStatus::Changed,
        column_diffs: vec![], index_diffs: vec![], fk_diffs, check_diffs: vec![],
    };
    DatabaseDiff {
        name: "mydb".into(), status: DiffStatus::Changed,
        tables: vec![table], views: vec![], procedures: vec![], functions: vec![], triggers: vec![],
    }
}

#[test]
fn test_add_foreign_key() {
    let f = fk("fk_user", vec!["user_id"], "users", vec!["id"], "CASCADE", "RESTRICT");
    let db = changed_table_with_fk_diff(vec![fk_diff("fk_user", DiffStatus::OnlyInBase, Some(f), None)]);
    let report = DiffReport { base_label: "b".into(), check_label: "c".into(), generated_at: "t".into(), databases: vec![db] };
    let (sql, warnings) = generate(&report);
    assert!(sql.contains("ADD CONSTRAINT `fk_user` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE ON UPDATE RESTRICT"), "got: {}", sql);
    assert!(warnings.is_empty());
}

#[test]
fn test_warn_fk_only_in_check() {
    let f = fk("fk_old", vec!["x_id"], "x", vec!["id"], "NO ACTION", "NO ACTION");
    let db = changed_table_with_fk_diff(vec![fk_diff("fk_old", DiffStatus::OnlyInCheck, None, Some(f))]);
    let report = DiffReport { base_label: "b".into(), check_label: "c".into(), generated_at: "t".into(), databases: vec![db] };
    let (sql, warnings) = generate(&report);
    assert!(sql.contains("-- ALTER TABLE `orders` DROP FOREIGN KEY `fk_old`;"), "got: {}", sql);
    assert_eq!(warnings.len(), 1);
}
```

- [ ] **Step 2: Run to verify they fail**

```bash
cargo test sql_generator::tests::test_add_foreign_key 2>&1 | grep -E "FAILED|panicked"
```

- [ ] **Step 3: Implement `generate_fk_diffs`**

```rust
fn generate_fk_diffs(
    table_name: &str,
    fk_diffs: &[ObjectDiff<ForeignKeyDef>],
) -> (String, Vec<String>) {
    let mut out = String::new();
    let mut warnings: Vec<String> = Vec::new();

    for diff in fk_diffs {
        match diff.status {
            DiffStatus::Same => {}
            DiffStatus::OnlyInBase => {
                let fk = diff.base.as_ref().unwrap();
                let cols = fk.columns.iter().map(|c| format!("`{}`", c)).collect::<Vec<_>>().join(", ");
                let ref_cols = fk.ref_columns.iter().map(|c| format!("`{}`", c)).collect::<Vec<_>>().join(", ");
                out.push_str(&format!(
                    "ALTER TABLE `{}` ADD CONSTRAINT `{}` FOREIGN KEY ({}) REFERENCES `{}` ({}) ON DELETE {} ON UPDATE {};\n",
                    table_name, fk.name, cols, fk.ref_table, ref_cols, fk.on_delete, fk.on_update
                ));
            }
            DiffStatus::OnlyInCheck => {
                let fk = diff.check.as_ref().unwrap();
                let msg = format!(
                    "foreign key `{}` on `{}` exists in check but not in base",
                    fk.name, table_name
                );
                warnings.push(format!("[WARN] {} — review commented DROP in sync SQL", msg));
                out.push_str(&format!("-- [WARN] {}\n", msg));
                out.push_str(&format!(
                    "-- ALTER TABLE `{}` DROP FOREIGN KEY `{}`;\n",
                    table_name, fk.name
                ));
            }
            DiffStatus::Changed => {
                let base_fk = diff.base.as_ref().unwrap();
                let check_fk = diff.check.as_ref().unwrap();
                out.push_str(&format!(
                    "ALTER TABLE `{}` DROP FOREIGN KEY `{}`;\n",
                    table_name, check_fk.name
                ));
                let cols = base_fk.columns.iter().map(|c| format!("`{}`", c)).collect::<Vec<_>>().join(", ");
                let ref_cols = base_fk.ref_columns.iter().map(|c| format!("`{}`", c)).collect::<Vec<_>>().join(", ");
                out.push_str(&format!(
                    "ALTER TABLE `{}` ADD CONSTRAINT `{}` FOREIGN KEY ({}) REFERENCES `{}` ({}) ON DELETE {} ON UPDATE {};\n",
                    table_name, base_fk.name, cols, base_fk.ref_table, ref_cols, base_fk.on_delete, base_fk.on_update
                ));
            }
        }
    }

    (out, warnings)
}
```

Wire into `generate_table_diffs` `Changed` arm, after index lines:

```rust
let (fk_sql, fk_warns) = generate_fk_diffs(&table.name, &table.fk_diffs);
out.push_str(&fk_sql);
warnings.extend(fk_warns);
```

- [ ] **Step 4: Run to verify tests pass**

```bash
cargo test sql_generator 2>&1 | grep -E "FAILED|error|ok"
```

- [ ] **Step 5: Commit**

```bash
git add src/sql_generator.rs
git commit -m "feat: foreign key ADD/DROP/WARN SQL generation"
```

---

## Task 7: CREATE TABLE for base-only tables

**Files:**
- Modify: `src/sql_generator.rs`

A base-only table means all its `column_diffs`, `index_diffs`, `fk_diffs`, `check_diffs` have `OnlyInBase` status. Reconstruct the full DDL from the base side of each diff.

- [ ] **Step 1: Write the failing test**

```rust
#[test]
fn test_create_table_only_in_base() {
    use crate::schema::IndexColumn;
    let col_diffs = vec![
        col_diff("id", DiffStatus::OnlyInBase,
            Some(col("id", "int unsigned", false, None, "auto_increment", 1)), None),
        col_diff("name", DiffStatus::OnlyInBase,
            Some(col("name", "varchar(100)", false, None, "", 2)), None),
    ];
    let idx_diffs = vec![
        idx_diff("PRIMARY", DiffStatus::OnlyInBase,
            Some(idx("PRIMARY", false, "BTREE", vec![("id", None)])), None),
    ];
    let table = TableDiff {
        name: "products".into(), status: DiffStatus::OnlyInBase,
        column_diffs: col_diffs, index_diffs: idx_diffs, fk_diffs: vec![], check_diffs: vec![],
    };
    let db = DatabaseDiff {
        name: "mydb".into(), status: DiffStatus::Changed,
        tables: vec![table], views: vec![], procedures: vec![], functions: vec![], triggers: vec![],
    };
    let report = DiffReport { base_label: "b".into(), check_label: "c".into(), generated_at: "t".into(), databases: vec![db] };
    let (sql, warnings) = generate(&report);
    assert!(sql.contains("CREATE TABLE IF NOT EXISTS `products`"), "got: {}", sql);
    assert!(sql.contains("`id` int unsigned NOT NULL auto_increment"), "got: {}", sql);
    assert!(sql.contains("`name` varchar(100) NOT NULL"), "got: {}", sql);
    assert!(sql.contains("PRIMARY KEY (`id`)"), "got: {}", sql);
    assert!(warnings.is_empty());
}

#[test]
fn test_create_table_with_check_constraint() {
    let col_diffs = vec![
        col_diff("id", DiffStatus::OnlyInBase,
            Some(col("id", "int", false, None, "", 1)), None),
    ];
    let check_diffs = vec![ObjectDiff {
        name: "chk_positive".into(),
        status: DiffStatus::OnlyInBase,
        base: Some(CheckDef { name: "chk_positive".into(), clause: "`id` > 0".into() }),
        check: None,
    }];
    let table = TableDiff {
        name: "items".into(), status: DiffStatus::OnlyInBase,
        column_diffs: col_diffs, index_diffs: vec![], fk_diffs: vec![], check_diffs,
    };
    let db = DatabaseDiff {
        name: "mydb".into(), status: DiffStatus::Changed,
        tables: vec![table], views: vec![], procedures: vec![], functions: vec![], triggers: vec![],
    };
    let report = DiffReport { base_label: "b".into(), check_label: "c".into(), generated_at: "t".into(), databases: vec![db] };
    let (sql, _) = generate(&report);
    assert!(sql.contains("CONSTRAINT `chk_positive` CHECK (`id` > 0)"), "got: {}", sql);
}

#[test]
fn test_warn_table_only_in_check() {
    let table = TableDiff {
        name: "legacy".into(), status: DiffStatus::OnlyInCheck,
        column_diffs: vec![], index_diffs: vec![], fk_diffs: vec![], check_diffs: vec![],
    };
    let db = DatabaseDiff {
        name: "mydb".into(), status: DiffStatus::Changed,
        tables: vec![table], views: vec![], procedures: vec![], functions: vec![], triggers: vec![],
    };
    let report = DiffReport { base_label: "b".into(), check_label: "c".into(), generated_at: "t".into(), databases: vec![db] };
    let (sql, warnings) = generate(&report);
    assert!(sql.contains("-- DROP TABLE IF EXISTS `legacy`;"), "got: {}", sql);
    assert_eq!(warnings.len(), 1);
}
```

- [ ] **Step 2: Run to verify they fail**

```bash
cargo test sql_generator::tests::test_create_table 2>&1 | grep -E "FAILED|panicked"
```

- [ ] **Step 3: Implement `generate_create_table`**

Replace the TODO stub in `generate_table_diffs` `OnlyInBase` arm with:

```rust
DiffStatus::OnlyInBase => {
    let (ct_sql, ct_warns) = generate_create_table(&table);
    out.push_str(&ct_sql);
    warnings.extend(ct_warns);
}
```

Add the function:

```rust
fn generate_create_table(table: &TableDiff) -> (String, Vec<String>) {
    let mut parts: Vec<String> = Vec::new();

    // Columns (sorted by ordinal_position)
    let mut base_cols: Vec<&ColumnDef> = table.column_diffs.iter()
        .filter_map(|d| d.base.as_ref())
        .collect();
    base_cols.sort_by_key(|c| c.ordinal_position);

    for col in &base_cols {
        parts.push(format!("  `{}` {}", col.name, format_col_def(col)));
    }

    // Indexes
    for idx_diff in &table.index_diffs {
        if let Some(idx) = &idx_diff.base {
            let cols = format_index_cols(&idx.columns);
            let using = if idx.index_type == "HASH" { " USING HASH" } else { "" };
            if idx.name == "PRIMARY" {
                parts.push(format!("  PRIMARY KEY ({}){}", cols, using));
            } else if idx.is_unique {
                parts.push(format!("  UNIQUE KEY `{}` ({}){}", idx.name, cols, using));
            } else {
                parts.push(format!("  KEY `{}` ({}){}", idx.name, cols, using));
            }
        }
    }

    // Foreign keys
    for fk_diff in &table.fk_diffs {
        if let Some(fk) = &fk_diff.base {
            let cols = fk.columns.iter().map(|c| format!("`{}`", c)).collect::<Vec<_>>().join(", ");
            let ref_cols = fk.ref_columns.iter().map(|c| format!("`{}`", c)).collect::<Vec<_>>().join(", ");
            parts.push(format!(
                "  CONSTRAINT `{}` FOREIGN KEY ({}) REFERENCES `{}` ({}) ON DELETE {} ON UPDATE {}",
                fk.name, cols, fk.ref_table, ref_cols, fk.on_delete, fk.on_update
            ));
        }
    }

    // Check constraints (MySQL 8.0+ only; empty on 5.x)
    for ch_diff in &table.check_diffs {
        if let Some(ch) = &ch_diff.base {
            parts.push(format!("  CONSTRAINT `{}` CHECK ({})", ch.name, ch.clause));
        }
    }

    let body = parts.join(",\n");
    let sql = format!(
        "CREATE TABLE IF NOT EXISTS `{}` (\n{}\n);\n\n",
        table.name, body
    );

    (sql, vec![])
}
```

- [ ] **Step 4: Run to verify tests pass**

```bash
cargo test sql_generator 2>&1 | grep -E "FAILED|error|ok"
```

- [ ] **Step 5: Commit**

```bash
git add src/sql_generator.rs
git commit -m "feat: CREATE TABLE generation for base-only tables"
```

---

## Task 8: View, routine, and trigger SQL generation

> **Known Limitation — Routine Parameters:** `RoutineDef.definition` contains only the routine body (from `information_schema.ROUTINES.ROUTINE_DEFINITION`), not the parameter list. Parameters are stored in `information_schema.PARAMETERS`, which the fetcher does not query. As a result, generated `CREATE PROCEDURE/FUNCTION` statements will always include `( /* add parameters here */ )` as a placeholder. This is intentional — the generated SQL is a starting point that requires manual parameter completion before execution.

**Files:**
- Modify: `src/sql_generator.rs`

- [ ] **Step 1: Write the failing tests**

```rust
fn view(name: &str, def: &str) -> ViewDef { ViewDef { name: name.into(), definition: def.into() } }
fn view_diff(name: &str, status: DiffStatus, base: Option<ViewDef>, check: Option<ViewDef>) -> ObjectDiff<ViewDef> {
    ObjectDiff { name: name.into(), status, base, check }
}
fn db_with_view(v: ObjectDiff<ViewDef>) -> DiffReport {
    DiffReport {
        base_label: "b".into(), check_label: "c".into(), generated_at: "t".into(),
        databases: vec![DatabaseDiff {
            name: "mydb".into(), status: DiffStatus::Changed,
            tables: vec![], views: vec![v], procedures: vec![], functions: vec![], triggers: vec![],
        }],
    }
}

#[test]
fn test_create_or_replace_view_only_in_base() {
    let (sql, w) = generate(&db_with_view(view_diff("v_users", DiffStatus::OnlyInBase,
        Some(view("v_users", "SELECT id FROM users")), None)));
    assert!(sql.contains("CREATE OR REPLACE VIEW `v_users` AS SELECT id FROM users;"), "got: {}", sql);
    assert!(w.is_empty());
}

#[test]
fn test_create_or_replace_view_changed() {
    let (sql, _) = generate(&db_with_view(view_diff("v_users", DiffStatus::Changed,
        Some(view("v_users", "SELECT id, name FROM users")),
        Some(view("v_users", "SELECT id FROM users")))));
    assert!(sql.contains("CREATE OR REPLACE VIEW `v_users` AS SELECT id, name FROM users;"), "got: {}", sql);
}

#[test]
fn test_warn_view_only_in_check() {
    let (sql, warnings) = generate(&db_with_view(view_diff("v_old", DiffStatus::OnlyInCheck,
        None, Some(view("v_old", "SELECT 1")))));
    assert!(sql.contains("-- DROP VIEW IF EXISTS `v_old`;"), "got: {}", sql);
    assert_eq!(warnings.len(), 1);
}

fn routine(name: &str, rtype: &str, def: &str) -> RoutineDef {
    RoutineDef {
        name: name.into(), routine_type: rtype.into(),
        definition: def.into(), is_deterministic: "NO".into(),
        sql_data_access: "MODIFIES SQL DATA".into(), security_type: "DEFINER".into(),
    }
}
fn proc_diff(name: &str, status: DiffStatus, base: Option<RoutineDef>, check: Option<RoutineDef>) -> ObjectDiff<RoutineDef> {
    ObjectDiff { name: name.into(), status, base, check }
}
fn db_with_proc(p: ObjectDiff<RoutineDef>) -> DiffReport {
    DiffReport {
        base_label: "b".into(), check_label: "c".into(), generated_at: "t".into(),
        databases: vec![DatabaseDiff {
            name: "mydb".into(), status: DiffStatus::Changed,
            tables: vec![], views: vec![], procedures: vec![p], functions: vec![], triggers: vec![],
        }],
    }
}

#[test]
fn test_create_procedure_only_in_base() {
    let r = routine("sp_foo", "PROCEDURE", "BEGIN SELECT 1; END");
    let (sql, w) = generate(&db_with_proc(proc_diff("sp_foo", DiffStatus::OnlyInBase, Some(r), None)));
    assert!(sql.contains("DROP PROCEDURE IF EXISTS `sp_foo`;"), "got: {}", sql);
    assert!(sql.contains("CREATE PROCEDURE `sp_foo`"), "got: {}", sql);
    assert!(sql.contains("MODIFIES SQL DATA"), "got: {}", sql);
    assert!(sql.contains("SQL SECURITY DEFINER"), "got: {}", sql);
    assert!(sql.contains("BEGIN SELECT 1; END"), "got: {}", sql);
    assert!(w.is_empty());
}

#[test]
fn test_warn_procedure_only_in_check() {
    let r = routine("sp_old", "PROCEDURE", "BEGIN END");
    let (sql, warnings) = generate(&db_with_proc(proc_diff("sp_old", DiffStatus::OnlyInCheck, None, Some(r))));
    assert!(sql.contains("-- DROP PROCEDURE IF EXISTS `sp_old`;"), "got: {}", sql);
    assert_eq!(warnings.len(), 1);
}

fn trig(name: &str) -> TriggerDef {
    TriggerDef {
        name: name.into(), event: "INSERT".into(), timing: "BEFORE".into(),
        table_name: "orders".into(), statement: "BEGIN SET NEW.ts = NOW(); END".into(),
        action_order: 1,
    }
}
fn trig_diff(name: &str, status: DiffStatus, base: Option<TriggerDef>, check: Option<TriggerDef>) -> ObjectDiff<TriggerDef> {
    ObjectDiff { name: name.into(), status, base, check }
}
fn db_with_trigger(t: ObjectDiff<TriggerDef>) -> DiffReport {
    DiffReport {
        base_label: "b".into(), check_label: "c".into(), generated_at: "t".into(),
        databases: vec![DatabaseDiff {
            name: "mydb".into(), status: DiffStatus::Changed,
            tables: vec![], views: vec![], procedures: vec![], functions: vec![], triggers: vec![t],
        }],
    }
}

#[test]
fn test_create_trigger_only_in_base() {
    let t = trig("trg_orders_bi");
    let (sql, w) = generate(&db_with_trigger(trig_diff("trg_orders_bi", DiffStatus::OnlyInBase, Some(t), None)));
    assert!(sql.contains("DROP TRIGGER IF EXISTS `trg_orders_bi`;"), "got: {}", sql);
    assert!(sql.contains("CREATE TRIGGER `trg_orders_bi`"), "got: {}", sql);
    assert!(sql.contains("BEFORE INSERT ON `orders`"), "got: {}", sql);
    assert!(w.is_empty());
}

#[test]
fn test_warn_trigger_only_in_check() {
    let t = trig("trg_old");
    let (sql, warnings) = generate(&db_with_trigger(trig_diff("trg_old", DiffStatus::OnlyInCheck, None, Some(t))));
    assert!(sql.contains("-- DROP TRIGGER IF EXISTS `trg_old`;"), "got: {}", sql);
    assert_eq!(warnings.len(), 1);
}
```

- [ ] **Step 2: Run to verify they fail**

```bash
cargo test sql_generator::tests::test_create_or_replace_view 2>&1 | grep -E "FAILED|panicked"
```

- [ ] **Step 3: Implement view, routine, trigger generation**

Add these functions to `src/sql_generator.rs`:

```rust
fn generate_view_diffs(db_name: &str, views: &[ObjectDiff<ViewDef>]) -> (String, Vec<String>) {
    let mut out = String::new();
    let mut warnings: Vec<String> = Vec::new();

    for diff in views {
        match diff.status {
            DiffStatus::Same => {}
            DiffStatus::OnlyInBase | DiffStatus::Changed => {
                let v = diff.base.as_ref().unwrap();
                out.push_str(&format!(
                    "CREATE OR REPLACE VIEW `{}` AS {};\n",
                    v.name, v.definition
                ));
            }
            DiffStatus::OnlyInCheck => {
                let v = diff.check.as_ref().unwrap();
                let msg = format!("view `{}` in `{}` exists in check but not in base", v.name, db_name);
                warnings.push(format!("[WARN] {} — review commented DROP in sync SQL", msg));
                out.push_str(&format!("-- [WARN] {}\n", msg));
                out.push_str(&format!("-- DROP VIEW IF EXISTS `{}`;\n", v.name));
            }
        }
    }

    (out, warnings)
}

fn generate_routine_diffs(db_name: &str, routines: &[ObjectDiff<RoutineDef>]) -> (String, Vec<String>) {
    let mut out = String::new();
    let mut warnings: Vec<String> = Vec::new();

    for diff in routines {
        match diff.status {
            DiffStatus::Same => {}
            DiffStatus::OnlyInBase | DiffStatus::Changed => {
                let r = diff.base.as_ref().unwrap();
                let deterministic = if r.is_deterministic == "YES" { "DETERMINISTIC" } else { "NOT DETERMINISTIC" };
                out.push_str(&format!("DROP {} IF EXISTS `{}`;\n", r.routine_type, r.name));
                out.push_str("DELIMITER $$\n");
                out.push_str(&format!(
                    "-- NOTE: parameter list not captured — add parameters manually\n\
                     CREATE {} `{}`( /* add parameters here */ )\n\
                     {}\n\
                     {}\n\
                     SQL SECURITY {}\n\
                     {}\n\
                     $$\n",
                    r.routine_type, r.name,
                    deterministic,
                    r.sql_data_access,
                    r.security_type,
                    r.definition
                ));
                out.push_str("DELIMITER ;\n\n");
            }
            DiffStatus::OnlyInCheck => {
                let r = diff.check.as_ref().unwrap();
                let msg = format!(
                    "{} `{}` in `{}` exists in check but not in base",
                    r.routine_type.to_lowercase(), r.name, db_name
                );
                warnings.push(format!("[WARN] {} — review commented DROP in sync SQL", msg));
                out.push_str(&format!("-- [WARN] {}\n", msg));
                out.push_str(&format!("-- DROP {} IF EXISTS `{}`;\n", r.routine_type, r.name));
            }
        }
    }

    (out, warnings)
}

fn generate_trigger_diffs(db_name: &str, triggers: &[ObjectDiff<TriggerDef>]) -> (String, Vec<String>) {
    let mut out = String::new();
    let mut warnings: Vec<String> = Vec::new();

    for diff in triggers {
        match diff.status {
            DiffStatus::Same => {}
            DiffStatus::OnlyInBase | DiffStatus::Changed => {
                let t = diff.base.as_ref().unwrap();
                out.push_str(&format!("DROP TRIGGER IF EXISTS `{}`;\n", t.name));
                out.push_str("DELIMITER $$\n");
                out.push_str(&format!(
                    "CREATE TRIGGER `{}` {} {} ON `{}` FOR EACH ROW\n{}\n$$\n",
                    t.name, t.timing, t.event, t.table_name, t.statement
                ));
                out.push_str("DELIMITER ;\n\n");
            }
            DiffStatus::OnlyInCheck => {
                let t = diff.check.as_ref().unwrap();
                let msg = format!("trigger `{}` in `{}` exists in check but not in base", t.name, db_name);
                warnings.push(format!("[WARN] {} — review commented DROP in sync SQL", msg));
                out.push_str(&format!("-- [WARN] {}\n", msg));
                out.push_str(&format!("-- DROP TRIGGER IF EXISTS `{}`;\n", t.name));
            }
        }
    }

    (out, warnings)
}
```

Wire all three into `generate_database()` for `Changed` and `OnlyInBase` arms. After the table diff section add:

```rust
let (v_sql, v_warns) = generate_view_diffs(&db.name, &db.views);
out.push_str(&v_sql);
warnings.extend(v_warns);

let (p_sql, p_warns) = generate_routine_diffs(&db.name, &db.procedures);
out.push_str(&p_sql);
warnings.extend(p_warns);

let (f_sql, f_warns) = generate_routine_diffs(&db.name, &db.functions);
out.push_str(&f_sql);
warnings.extend(f_warns);

let (t_sql, t_warns) = generate_trigger_diffs(&db.name, &db.triggers);
out.push_str(&t_sql);
warnings.extend(t_warns);
```

- [ ] **Step 4: Run to verify tests pass**

```bash
cargo test sql_generator 2>&1 | grep -E "FAILED|error|ok"
```

- [ ] **Step 5: Commit**

```bash
git add src/sql_generator.rs
git commit -m "feat: view, routine, trigger SQL generation"
```

---

## Task 9: Wire everything into `main.rs`

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Replace the temporary `false` with the real flag and add sql output**

Update `src/main.rs`. The final file should look like:

```rust
mod config;
mod differ;
mod fetcher;
mod reporter;
mod schema;
mod sql_generator;

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

    // Derive output paths (strip .md suffix and add both extensions)
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

- [ ] **Step 2: Run all tests**

```bash
cargo test 2>&1 | grep -E "FAILED|error\[|ok"
```

Expected: all tests pass.

- [ ] **Step 3: Build the release binary**

```bash
cargo build --release 2>&1 | grep -E "error|warning.*unused"
```

Expected: clean build, no errors.

- [ ] **Step 4: Commit**

```bash
git add src/main.rs
git commit -m "feat: wire ignore-base-only-dbs flag and SQL sync output into main"
```

---

## Task 10: Final verification

- [ ] **Step 1: Run full test suite**

```bash
cargo test 2>&1
```

Expected: all tests pass, zero failures.

- [ ] **Step 2: Verify CLI help shows new flag**

```bash
./target/release/mysql-schema-diff --help 2>&1 | grep -A2 "ignore"
```

Expected output contains `--ignore-base-only-dbs`.

- [ ] **Step 3: Commit any final cleanup, then tag**

```bash
git add -p   # review any stragglers
git status   # should be clean
```
