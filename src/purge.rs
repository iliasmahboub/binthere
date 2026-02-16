use crate::models::ScanSummary;
use crate::report::human_size;
use anyhow::Result;
use colored::Colorize;
use dialoguer::Confirm;
use std::fs;
use std::path::Path;

pub fn run_purge(summary: &ScanSummary, confirm: bool) -> Result<()> {
    if !confirm {
        print_dry_run(summary);
        return Ok(());
    }

    if summary.files.is_empty() {
        println!("{}", "No files to delete from the latest scan.".green());
        return Ok(());
    }

    print_dry_run(summary);

    let proceed = Confirm::new()
        .with_prompt(format!(
            "Delete {} files and reclaim up to {}?",
            summary.files.len(),
            human_size(summary.total_size_bytes())
        ))
        .default(false)
        .interact()?;

    if !proceed {
        println!("{}", "Purge aborted. No files were deleted.".yellow());
        return Ok(());
    }

    let mut deleted_count = 0_u64;
    let mut deleted_bytes = 0_u64;
    let mut failures = Vec::new();

    for file in &summary.files {
        let path = Path::new(&file.path);
        if !path.exists() {
            failures.push(format!("Missing: {}", file.path));
            continue;
        }

        match fs::remove_file(path) {
            Ok(_) => {
                deleted_count += 1;
                deleted_bytes += file.size_bytes;
                println!("{} {}", "Deleted".green().bold(), file.path);
            }
            Err(err) => failures.push(format!("{} ({err})", file.path)),
        }
    }

    println!(
        "\n{} {} files, {}",
        "Deleted:".cyan().bold(),
        deleted_count,
        human_size(deleted_bytes).yellow().bold()
    );

    if !failures.is_empty() {
        println!("{}", "Some files could not be deleted:".yellow().bold());
        for failure in failures {
            println!("- {}", failure.yellow());
        }
    }

    Ok(())
}

fn print_dry_run(summary: &ScanSummary) {
    println!("{}", "[DRY RUN] Files that would be deleted:".blue().bold());

    if summary.files.is_empty() {
        println!("{}", "(none)".green());
        return;
    }

    for file in &summary.files {
        println!("- {} ({})", file.path, human_size(file.size_bytes));
    }

    println!(
        "{} {}",
        "Potential reclaimable:".cyan(),
        human_size(summary.total_size_bytes()).yellow().bold()
    );
}
