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
