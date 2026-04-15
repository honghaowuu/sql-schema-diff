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
