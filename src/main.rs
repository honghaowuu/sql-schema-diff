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

    // Fetch both snapshots concurrently.
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
        false,
    );

    let markdown = reporter::render(&report);

    // Determine output path.
    let output_path = opts.output.unwrap_or_else(|| {
        let ts = Local::now().format("%Y%m%d-%H%M%S");
        format!("schema-diff-{}.md", ts)
    });

    std::fs::write(&output_path, &markdown)
        .with_context(|| format!("Failed to write output to {}", output_path))?;

    eprintln!("Report written to: {}", output_path);

    // Exit with code 1 if there are any differences, 0 if everything is identical.
    let has_diff = report
        .databases
        .iter()
        .any(|db| db.status != differ::DiffStatus::Same);

    std::process::exit(if has_diff { 1 } else { 0 });
}
