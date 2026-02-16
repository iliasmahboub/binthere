mod cli;
mod installed;
mod models;
mod purge;
mod report;
mod scanner;
mod state;

use crate::cli::{Cli, Commands};
use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use std::env;
use std::path::PathBuf;

fn main() {
    if let Err(err) = run() {
        eprintln!("{} {}", "Error:".red().bold(), err);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan {
            path,
            include_archives,
        } => {
            let target = resolve_scan_path(path)?;

            println!("{} {}", "[SCAN] Root:".blue().bold(), target.display());
            let summary = scanner::scan_path(&target, include_archives)?;

            println!(
                "{} {}",
                "[OK] Found installer candidates:".green().bold(),
                summary.files.len()
            );
            println!(
                "{} {}",
                "[OK] Total reclaimable:".green().bold(),
                report::human_size(summary.total_size_bytes()).yellow().bold()
            );

            if !summary.warnings.is_empty() {
                println!("{}", "[WARN] Scan encountered non-fatal issues:".yellow().bold());
                for warning in &summary.warnings {
                    println!("- {}", warning.yellow());
                }
            }

            let state_file = state::save_scan(&summary)?;
            println!(
                "{} {}",
                "[INFO] Saved scan state:".cyan().bold(),
                state_file.display()
            );
        }
        Commands::Report => {
            let summary = state::load_scan()?;
            report::print_report(&summary);
        }
        Commands::Purge {
            dry_run: _,
            confirm,
        } => {
            let summary = state::load_scan()?;
            purge::run_purge(&summary, confirm)?;
        }
    }

    Ok(())
}

fn resolve_scan_path(path: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(p) = path {
        return Ok(p);
    }

    if let Some(downloads) = dirs::download_dir() {
        return Ok(downloads);
    }

    Ok(env::current_dir()?)
}
