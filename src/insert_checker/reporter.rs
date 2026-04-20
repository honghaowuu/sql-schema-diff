use super::checker::{FailureDetail, FailureKind, Finding, Suggestion};
use super::parser::ParseWarning;

pub fn render(
    findings: &[Finding],
    parse_warnings: &[ParseWarning],
    total_stmts: usize,
    generated_at: &str,
    database: &str,
    host: &str,
    port: u16,
    file: &str,
) -> String {
    let mut out = String::new();

    let skipped = parse_warnings.len();
    let with_issues = findings.iter().filter(|f| !f.row_findings.is_empty()).count();
    let clean = total_stmts.saturating_sub(skipped).saturating_sub(with_issues);

    out.push_str("# INSERT Check Report\n");
    out.push_str(&format!("Generated: {}\n", generated_at));
    out.push_str(&format!("Database:  {} @ {}:{}\n", database, host, port));
    out.push_str(&format!("File:      {}\n", file));
    out.push('\n');
    out.push_str("## Summary\n");
    out.push_str(&format!("- Total INSERT statements parsed: {}\n", total_stmts));
    out.push_str(&format!(
        "- Statements skipped (parse warning or INSERT-SELECT): {}\n",
        skipped
    ));
    out.push_str(&format!("- Statements with issues: {}\n", with_issues));
    out.push_str(&format!("- Clean statements: {}\n", clean));

    if !parse_warnings.is_empty() {
        out.push_str("\n---\n\n## Parse Warnings\n\n");
        for w in parse_warnings {
            if w.stmt_index == 0 {
                out.push_str(&format!("- {}\n", w.message));
            } else {
                out.push_str(&format!("- Statement {}: {}\n", w.stmt_index, w.message));
            }
        }
    }

    for (issue_num, finding) in findings.iter().enumerate() {
        if finding.row_findings.is_empty() {
            continue;
        }
        out.push_str(&format!(
            "\n---\n\n## Issue #{} — Statement {}\n\n",
            issue_num + 1,
            finding.stmt_index
        ));
        out.push_str("**Original SQL:**\n");
        out.push_str("```sql\n");
        out.push_str(&finding.original_sql);
        if !finding.original_sql.ends_with('\n') {
            out.push('\n');
        }
        out.push_str("```\n");

        for row_finding in &finding.row_findings {
            out.push_str(&format!("\n### Row {}\n", row_finding.row_index));
            for (fail_num, detail) in row_finding.failures.iter().enumerate() {
                out.push_str(&render_failure(fail_num + 1, detail));
            }
        }
    }

    out
}

fn render_failure(num: usize, detail: &FailureDetail) -> String {
    let mut out = String::new();
    let title = failure_title(&detail.kind);
    out.push_str(&format!("\n#### Failure {}: {}\n", num, title));
    out.push_str(&format!("{}\n", detail.message));

    if !detail.suggestions.is_empty() {
        out.push_str("\n**Suggestions:**\n");
        for s in &detail.suggestions {
            out.push_str(&render_suggestion(s));
        }
    }

    out
}

fn failure_title(kind: &FailureKind) -> String {
    match kind {
        FailureKind::PrimaryKeyConflict => "Primary Key Conflict".to_string(),
        FailureKind::UniqueKeyConflict { index_name } => {
            format!("Unique Key Conflict ({})", index_name)
        }
        FailureKind::ForeignKeyViolation { fk_columns } => {
            format!("Foreign Key Violation ({})", fk_columns.join(", "))
        }
        FailureKind::NotNullViolation { column } => format!("NOT NULL Violation ({})", column),
        FailureKind::ColumnLengthExceeded { column, .. } => {
            format!("Column Length Exceeded ({})", column)
        }
        FailureKind::TypeMismatch { column, .. } => format!("Type Mismatch ({})", column),
        FailureKind::UnknownTable => "Unknown Table".to_string(),
        FailureKind::UnknownColumn { column } => format!("Unknown Column ({})", column),
        FailureKind::SkippedExpression { column } => {
            format!("Note: Expression Value — Column `{}`", column)
        }
    }
}

fn render_suggestion(s: &Suggestion) -> String {
    match s {
        Suggestion::InsertIgnore => "- Use `INSERT IGNORE` to skip silently.\n".to_string(),
        Suggestion::OnDuplicateKeyUpdate { rendered_sql, .. } => {
            format!("- Or use `ON DUPLICATE KEY UPDATE`:\n```sql\n{}\n```\n", rendered_sql)
        }
        Suggestion::FixValue { column, hint } => {
            format!("- Fix column `{}`: {}\n", column, hint)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::insert_checker::checker::{RowFinding};

    fn render_simple(findings: &[Finding], total: usize) -> String {
        render(findings, &[], total, "2026-04-20 00:00:00", "mydb", "localhost", 3306, "test.sql")
    }

    fn make_finding(stmt_index: usize, failures: Vec<FailureDetail>) -> Finding {
        Finding {
            stmt_index,
            original_sql: "INSERT INTO t (id) VALUES (1);".to_string(),
            row_findings: if failures.is_empty() {
                vec![]
            } else {
                vec![RowFinding { row_index: 1, failures }]
            },
        }
    }

    #[test]
    fn test_header_fields() {
        let out = render_simple(&[], 0);
        assert!(out.contains("# INSERT Check Report"));
        assert!(out.contains("mydb @ localhost:3306"));
        assert!(out.contains("test.sql"));
    }

    #[test]
    fn test_summary_counts_no_issues() {
        let out = render_simple(&[], 10);
        assert!(out.contains("Total INSERT statements parsed: 10"));
        assert!(out.contains("Statements with issues: 0"));
        assert!(out.contains("Clean statements: 10"));
    }

    #[test]
    fn test_summary_counts_with_issues() {
        let finding = make_finding(3, vec![FailureDetail {
            kind: FailureKind::UnknownTable,
            message: "Table does not exist".to_string(),
            suggestions: vec![],
        }]);
        let out = render_simple(&[finding], 5);
        assert!(out.contains("Statements with issues: 1"));
        assert!(out.contains("Clean statements: 4"));
    }

    #[test]
    fn test_parse_warnings_section() {
        use crate::insert_checker::parser::ParseWarning;
        let warns = vec![ParseWarning {
            stmt_index: 2,
            message: "Skipped: INSERT-SELECT not checkable".to_string(),
            is_error: false,
        }];
        let out = render(&[], &warns, 3, "2026-04-20", "db", "host", 3306, "f.sql");
        assert!(out.contains("## Parse Warnings"));
        assert!(out.contains("Statement 2: Skipped: INSERT-SELECT"));
        assert!(out.contains("Statements skipped (parse warning or INSERT-SELECT): 1"));
    }

    #[test]
    fn test_finding_renders_original_sql() {
        let finding = make_finding(7, vec![FailureDetail {
            kind: FailureKind::PrimaryKeyConflict,
            message: "`id = 1` already exists".to_string(),
            suggestions: vec![],
        }]);
        let out = render_simple(&[finding], 10);
        assert!(out.contains("## Issue #1 — Statement 7"));
        assert!(out.contains("```sql\nINSERT INTO t (id) VALUES (1);"));
    }

    #[test]
    fn test_pk_conflict_with_suggestions() {
        let finding = make_finding(1, vec![FailureDetail {
            kind: FailureKind::PrimaryKeyConflict,
            message: "`id = 42` already exists in `users`".to_string(),
            suggestions: vec![
                Suggestion::InsertIgnore,
                Suggestion::OnDuplicateKeyUpdate {
                    columns: vec!["email".to_string()],
                    rendered_sql: "INSERT INTO `users` VALUES (42) AS new_row\nON DUPLICATE KEY UPDATE `email` = new_row.`email`;".to_string(),
                },
            ],
        }]);
        let out = render_simple(&[finding], 1);
        assert!(out.contains("Primary Key Conflict"));
        assert!(out.contains("INSERT IGNORE"));
        assert!(out.contains("ON DUPLICATE KEY UPDATE"));
        assert!(out.contains("new_row.`email`"));
    }

    #[test]
    fn test_not_null_violation_fix_suggestion() {
        let finding = make_finding(1, vec![FailureDetail {
            kind: FailureKind::NotNullViolation { column: "name".to_string() },
            message: "Column `name` is NOT NULL".to_string(),
            suggestions: vec![Suggestion::FixValue {
                column: "name".to_string(),
                hint: "provide a non-NULL value".to_string(),
            }],
        }]);
        let out = render_simple(&[finding], 1);
        assert!(out.contains("NOT NULL Violation (name)"));
        assert!(out.contains("Fix column `name`"));
    }

    #[test]
    fn test_fk_violation_title() {
        let finding = make_finding(1, vec![FailureDetail {
            kind: FailureKind::ForeignKeyViolation { fk_columns: vec!["role_id".to_string()] },
            message: "No matching row in roles".to_string(),
            suggestions: vec![],
        }]);
        let out = render_simple(&[finding], 1);
        assert!(out.contains("Foreign Key Violation (role_id)"));
    }

    #[test]
    fn test_skipped_expression_note() {
        let finding = make_finding(1, vec![FailureDetail {
            kind: FailureKind::SkippedExpression { column: "created_at".to_string() },
            message: "Column `created_at`: value is an expression; dynamic checks skipped".to_string(),
            suggestions: vec![],
        }]);
        let out = render_simple(&[finding], 1);
        assert!(out.contains("Note: Expression Value — Column `created_at`"));
        assert!(!out.contains("**Suggestions:**"));
    }

    #[test]
    fn test_unknown_table_no_suggestions() {
        let finding = make_finding(1, vec![FailureDetail {
            kind: FailureKind::UnknownTable,
            message: "Table does not exist".to_string(),
            suggestions: vec![],
        }]);
        let out = render_simple(&[finding], 1);
        assert!(out.contains("Unknown Table"));
        assert!(!out.contains("**Suggestions:**"));
    }

    #[test]
    fn test_multiple_issues_numbered() {
        let f1 = make_finding(1, vec![FailureDetail {
            kind: FailureKind::UnknownTable,
            message: "nope".to_string(),
            suggestions: vec![],
        }]);
        let f2 = make_finding(5, vec![FailureDetail {
            kind: FailureKind::PrimaryKeyConflict,
            message: "conflict".to_string(),
            suggestions: vec![],
        }]);
        let out = render_simple(&[f1, f2], 10);
        assert!(out.contains("## Issue #1 — Statement 1"));
        assert!(out.contains("## Issue #2 — Statement 5"));
    }
}
