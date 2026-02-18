use crate::models::ScanSummary;
use crate::report::human_size;
use anyhow::Result;
use colored::Colorize;
use dialoguer::{Confirm, Input};
use std::collections::BTreeSet;
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

    let selected_indices = prompt_file_selection(summary)?;
    if selected_indices.is_empty() {
        println!("{}", "No files selected. Purge aborted.".yellow());
        return Ok(());
    }

    print_selected_dry_run(summary, &selected_indices);

    let selected_count = selected_indices.len();
    let selected_bytes = selected_indices
        .iter()
        .map(|idx| summary.files[*idx].size_bytes)
        .sum::<u64>();

    let proceed = Confirm::new()
        .with_prompt(format!(
            "Delete {} files and reclaim up to {}?",
            selected_count,
            human_size(selected_bytes)
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

    for idx in selected_indices {
        let file = &summary.files[idx];
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

fn print_selected_dry_run(summary: &ScanSummary, selected_indices: &[usize]) {
    println!("{}", "[DRY RUN] Selected files to delete:".blue().bold());

    for idx in selected_indices {
        let file = &summary.files[*idx];
        println!("[{}] {} ({})", idx + 1, file.path, human_size(file.size_bytes));
    }

    let selected_bytes = selected_indices
        .iter()
        .map(|idx| summary.files[*idx].size_bytes)
        .sum::<u64>();
    println!(
        "{} {}",
        "Potential reclaimable:".cyan(),
        human_size(selected_bytes).yellow().bold()
    );
}

fn prompt_file_selection(summary: &ScanSummary) -> Result<Vec<usize>> {
    println!("{}", "[SELECT] Choose files to delete:".cyan().bold());
    for (idx, file) in summary.files.iter().enumerate() {
        println!("[{}] {} ({})", idx + 1, file.path, human_size(file.size_bytes));
    }

    let max = summary.files.len();
    loop {
        let prompt = "Delete which files? (e.g. 1,3-5 | all | none)";
        let input: String = Input::new().with_prompt(prompt).interact_text()?;
        match parse_selection(&input, max) {
            Ok(indices) => return Ok(indices),
            Err(err) => println!("{} {}", "Invalid selection:".red().bold(), err),
        }
    }
}

fn parse_selection(input: &str, max: usize) -> Result<Vec<usize>, String> {
    let trimmed = input.trim();
    if trimmed.eq_ignore_ascii_case("all") {
        return Ok((0..max).collect());
    }
    if trimmed.eq_ignore_ascii_case("none") {
        return Ok(Vec::new());
    }
    if trimmed.is_empty() {
        return Err("enter indices, ranges, `all`, or `none`".to_string());
    }

    let mut selected = BTreeSet::new();
    for raw_part in trimmed.split(',') {
        let part = raw_part.trim();
        if part.is_empty() {
            return Err("empty item in list".to_string());
        }

        if let Some((start_raw, end_raw)) = part.split_once('-') {
            let start = parse_one_based(start_raw.trim(), max)?;
            let end = parse_one_based(end_raw.trim(), max)?;
            if start > end {
                return Err(format!("range start must be <= end: `{part}`"));
            }
            for idx in start..=end {
                selected.insert(idx - 1);
            }
            continue;
        }

        let idx = parse_one_based(part, max)?;
        selected.insert(idx - 1);
    }

    Ok(selected.into_iter().collect())
}

fn parse_one_based(value: &str, max: usize) -> Result<usize, String> {
    let parsed = value
        .parse::<usize>()
        .map_err(|_| format!("not a valid number: `{value}`"))?;
    if parsed == 0 || parsed > max {
        return Err(format!("index out of range: `{parsed}` (valid: 1-{max})"));
    }
    Ok(parsed)
}
