use crate::models::{InstallerFile, ScanSummary};
use colored::Colorize;
use std::cmp::Reverse;
use std::time::{Duration, UNIX_EPOCH};

pub fn print_report(summary: &ScanSummary) {
    println!("{} {}", "[REPORT]".blue().bold(), summary.root.bold());
    println!(
        "{} {}",
        "Last scan:".cyan(),
        format_timestamp(summary.scanned_at_unix_secs).bold()
    );
    println!("{} {}", "Include archives:".cyan(), summary.include_archives);
    println!("{} {}", "Total installers:".cyan(), summary.files.len().to_string().bold());
    println!(
        "{} {}",
        "Total reclaimable:".cyan(),
        human_size(summary.total_size_bytes()).yellow().bold()
    );

    if summary.files.is_empty() {
        println!("{}", "No installer candidates found in the latest scan.".green());
        return;
    }

    println!("\n{}", "Largest installers".bold().underline());
    for (idx, f) in largest_files(summary, 5).iter().enumerate() {
        println!(
            "{}. {} [{}] {}{}",
            idx + 1,
            f.path,
            f.extension,
            human_size(f.size_bytes).yellow(),
            format_match_suffix(f)
        );
    }

    println!("\n{}", "All detected installers".bold().underline());
    for f in &summary.files {
        println!(
            "- {} | {} | {} | {}{}",
            f.path,
            f.extension,
            human_size(f.size_bytes),
            format_optional_timestamp(f.modified_unix_secs),
            format_match_suffix(f)
        );
    }

    if !summary.warnings.is_empty() {
        println!("\n{}", "Scan warnings".yellow().bold());
        for warning in &summary.warnings {
            println!("- {}", warning.yellow());
        }
    }
}

fn largest_files(summary: &ScanSummary, max: usize) -> Vec<&InstallerFile> {
    let mut files: Vec<&InstallerFile> = summary.files.iter().collect();
    files.sort_by_key(|f| Reverse(f.size_bytes));
    files.into_iter().take(max).collect()
}

fn format_optional_timestamp(unix_secs: Option<u64>) -> String {
    unix_secs
        .map(format_timestamp)
        .unwrap_or_else(|| "unknown-modified-time".to_string())
}

fn format_timestamp(unix_secs: u64) -> String {
    let dt = UNIX_EPOCH + Duration::from_secs(unix_secs);
    humantime::format_rfc3339_seconds(dt).to_string()
}

fn format_match_suffix(file: &InstallerFile) -> String {
    file.installed_program_match
        .as_ref()
        .map(|name| format!(" {} {}", "| matches installed:".green(), name.green().bold()))
        .unwrap_or_default()
}

pub fn human_size(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];

    if bytes == 0 {
        return "0 B".to_string();
    }

    let mut size = bytes as f64;
    let mut unit = 0;

    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    format!("{size:.2} {}", UNITS[unit])
}
