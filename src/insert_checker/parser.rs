use sqlparser::ast::{Expr, SetExpr, Statement, TableObject, UnaryOperator, Value as AstValue};
use sqlparser::dialect::MySqlDialect;
use sqlparser::parser::Parser as SqlParser;

#[derive(Debug, Clone)]
pub struct InsertStmt {
    pub database: Option<String>,
    pub table: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<SqlValue>>,
    pub original_sql: String,
}

#[derive(Debug, Clone)]
pub enum SqlValue {
    Null,
    Number(String),
    Str(String),
    Bool(bool),
    Other(String),
}

pub struct ParseWarning {
    pub stmt_index: usize,
    pub message: String,
    pub is_error: bool,
}

pub fn parse_inserts(sql: &str) -> (Vec<InsertStmt>, Vec<ParseWarning>) {
    let dialect = MySqlDialect {};
    let statements = match SqlParser::parse_sql(&dialect, sql) {
        Ok(stmts) => stmts,
        Err(e) => {
            return (
                vec![],
                vec![ParseWarning {
                    stmt_index: 0,
                    message: format!("Failed to parse SQL file: {}", e),
                    is_error: true,
                }],
            );
        }
    };

    let mut results = vec![];
    let mut warnings = vec![];
    let mut stmt_num = 0;

    for stmt in statements {
        stmt_num += 1;
        if let Statement::Insert(insert) = stmt {
            let original = format!("{}", Statement::Insert(insert.clone()));

            let idents = match &insert.table {
                TableObject::TableName(obj) => &obj.0,
                TableObject::TableFunction(_) => {
                    warnings.push(ParseWarning {
                        stmt_index: stmt_num,
                        message: "Skipped: INSERT with table function not supported".to_string(),
                        is_error: false,
                    });
                    continue;
                }
            };
            let (database, table) = match idents.len() {
                0 => {
                    warnings.push(ParseWarning {
                        stmt_index: stmt_num,
                        message: "Parse warning: empty table name".to_string(),
                        is_error: true,
                    });
                    continue;
                }
                1 => (None, idents[0].value.clone()),
                _ => (
                    Some(idents[idents.len() - 2].value.clone()),
                    idents[idents.len() - 1].value.clone(),
                ),
            };

            let source = match &insert.source {
                Some(q) => q,
                None => {
                    warnings.push(ParseWarning {
                        stmt_index: stmt_num,
                        message: "Skipped: INSERT with no VALUES".to_string(),
                        is_error: false,
                    });
                    continue;
                }
            };

            let rows = match source.body.as_ref() {
                SetExpr::Values(values) => {
                    let mut parsed_rows = vec![];
                    for row in &values.rows {
                        parsed_rows.push(row.iter().map(expr_to_value).collect());
                    }
                    parsed_rows
                }
                _ => {
                    warnings.push(ParseWarning {
                        stmt_index: stmt_num,
                        message: "Skipped: INSERT-SELECT not checkable".to_string(),
                        is_error: false,
                    });
                    continue;
                }
            };

            let columns: Vec<String> = insert.columns.iter().map(|c| c.value.clone()).collect();

            results.push(InsertStmt {
                database,
                table,
                columns,
                rows,
                original_sql: original,
            });
        }
    }

    (results, warnings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_named_insert_single_row() {
        let (stmts, warns) =
            parse_inserts("INSERT INTO users (id, email) VALUES (1, 'alice@example.com');");
        assert_eq!(warns.len(), 0);
        assert_eq!(stmts.len(), 1);
        let s = &stmts[0];
        assert_eq!(s.table, "users");
        assert!(s.database.is_none());
        assert_eq!(s.columns, vec!["id", "email"]);
        assert_eq!(s.rows.len(), 1);
        assert!(matches!(s.rows[0][0], SqlValue::Number(_)));
        assert!(matches!(s.rows[0][1], SqlValue::Str(_)));
    }

    #[test]
    fn test_qualified_table_name() {
        let (stmts, warns) =
            parse_inserts("INSERT INTO mydb.users (id) VALUES (42);");
        assert_eq!(warns.len(), 0);
        assert_eq!(stmts.len(), 1);
        assert_eq!(stmts[0].database, Some("mydb".to_string()));
        assert_eq!(stmts[0].table, "users");
    }

    #[test]
    fn test_positional_insert() {
        let (stmts, warns) = parse_inserts("INSERT INTO t VALUES (1, 'hello', NULL);");
        assert_eq!(warns.len(), 0);
        assert_eq!(stmts.len(), 1);
        assert!(stmts[0].columns.is_empty());
        assert_eq!(stmts[0].rows[0].len(), 3);
        assert!(matches!(stmts[0].rows[0][2], SqlValue::Null));
    }

    #[test]
    fn test_multi_row_insert() {
        let (stmts, warns) =
            parse_inserts("INSERT INTO t (a, b) VALUES (1, 2), (3, 4), (5, 6);");
        assert_eq!(warns.len(), 0);
        assert_eq!(stmts.len(), 1);
        assert_eq!(stmts[0].rows.len(), 3);
    }

    #[test]
    fn test_insert_select_emits_warning() {
        let (stmts, warns) = parse_inserts("INSERT INTO t (a) SELECT a FROM other;");
        assert_eq!(stmts.len(), 0);
        assert_eq!(warns.len(), 1);
        assert!(warns[0].message.contains("INSERT-SELECT"));
        assert!(!warns[0].is_error);
    }

    #[test]
    fn test_null_value() {
        let (stmts, _) = parse_inserts("INSERT INTO t (a) VALUES (NULL);");
        assert!(matches!(stmts[0].rows[0][0], SqlValue::Null));
    }

    #[test]
    fn test_bool_value() {
        let (stmts, _) = parse_inserts("INSERT INTO t (a, b) VALUES (TRUE, FALSE);");
        assert!(matches!(stmts[0].rows[0][0], SqlValue::Bool(true)));
        assert!(matches!(stmts[0].rows[0][1], SqlValue::Bool(false)));
    }

    #[test]
    fn test_negative_number() {
        let (stmts, _) = parse_inserts("INSERT INTO t (a) VALUES (-42);");
        if let SqlValue::Number(n) = &stmts[0].rows[0][0] {
            assert_eq!(n, "-42");
        } else {
            panic!("expected Number");
        }
    }

    #[test]
    fn test_expression_becomes_other() {
        let (stmts, _) = parse_inserts("INSERT INTO t (a) VALUES (NOW());");
        assert!(matches!(stmts[0].rows[0][0], SqlValue::Other(_)));
    }

    #[test]
    fn test_multiple_statements_skips_non_insert() {
        let sql = "SELECT 1; INSERT INTO t (a) VALUES (1); UPDATE t SET a=2;";
        let (stmts, warns) = parse_inserts(sql);
        assert_eq!(stmts.len(), 1);
        assert_eq!(warns.len(), 0);
    }

    #[test]
    fn test_invalid_sql_returns_error_warning() {
        let (stmts, warns) = parse_inserts("INSERT INTO GARBAGE $$$ BROKEN");
        assert_eq!(stmts.len(), 0);
        assert_eq!(warns.len(), 1);
        assert!(warns[0].is_error);
    }

    #[test]
    fn test_original_sql_is_populated() {
        let (stmts, _) = parse_inserts("INSERT INTO t (a) VALUES (1);");
        assert!(!stmts[0].original_sql.is_empty());
    }
}

fn expr_to_value(expr: &Expr) -> SqlValue {
    match expr {
        Expr::Value(v) => match v {
            AstValue::Null => SqlValue::Null,
            AstValue::Number(n, _) => SqlValue::Number(n.clone()),
            AstValue::SingleQuotedString(s) | AstValue::DoubleQuotedString(s) => {
                SqlValue::Str(s.clone())
            }
            AstValue::Boolean(b) => SqlValue::Bool(*b),
            other => SqlValue::Other(format!("{}", other)),
        },
        Expr::UnaryOp { op: UnaryOperator::Minus, expr } => {
            if let Expr::Value(AstValue::Number(n, _)) = expr.as_ref() {
                SqlValue::Number(format!("-{}", n))
            } else {
                SqlValue::Other(format!("{}", expr))
            }
        }
        other => SqlValue::Other(format!("{}", other)),
    }
}
