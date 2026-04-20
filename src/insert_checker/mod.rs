pub mod checker;
pub mod meta_fetcher;
pub mod parser;
pub mod reporter;

use anyhow::{Context, Result};
use chrono::Local;
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use std::time::Duration;

use crate::config::CheckInsertsArgs;
use checker::check_statement;
use meta_fetcher::MetaFetcher;
use parser::parse_inserts;

pub async fn run(args: CheckInsertsArgs) -> Result<i32> {
    let sql_content = std::fs::read_to_string(&args.file)
        .with_context(|| format!("Failed to read file: {}", args.file))?;

    let opts = MySqlConnectOptions::new()
        .host(&args.host)
        .port(args.port)
        .username(&args.user)
        .password(&args.password)
        .database(&args.database);

    let pool = MySqlPoolOptions::new()
        .max_connections(3)
        .acquire_timeout(Duration::from_secs(30))
        .connect_with(opts)
        .await
        .with_context(|| format!("Failed to connect to {}:{}", args.host, args.port))?;

    let (stmts, parse_warnings) = parse_inserts(&sql_content);
    let had_parse_errors = parse_warnings.iter().any(|w| w.is_error);
    let total_stmts = stmts.len();

    let mut fetcher = MetaFetcher::new(pool);
    let mut findings = vec![];

    for (i, stmt) in stmts.iter().enumerate() {
        let finding = check_statement(i + 1, stmt, &args.database, &mut fetcher).await?;
        if !finding.row_findings.is_empty() {
            findings.push(finding);
        }
    }

    let generated_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let markdown = reporter::render(
        &findings,
        &parse_warnings,
        total_stmts,
        &generated_at,
        &args.database,
        &args.host,
        args.port,
        &args.file,
    );

    let output_path = resolve_output_path(args.output);
    std::fs::write(&output_path, &markdown)
        .with_context(|| format!("Failed to write report to {}", output_path))?;
    eprintln!("Report written to: {}", output_path);

    let had_findings = !findings.is_empty();
    Ok(if had_findings || had_parse_errors { 1 } else { 0 })
}

fn resolve_output_path(output: Option<String>) -> String {
    match output {
        Some(p) if p.ends_with(".md") => p,
        Some(p) => format!("{}.md", p),
        None => {
            let ts = Local::now().format("%Y%m%d-%H%M%S");
            format!("insert-check-{}.md", ts)
        }
    }
}
