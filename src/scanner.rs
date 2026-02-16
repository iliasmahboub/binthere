use crate::installed::{find_program_match, installed_program_names};
use crate::models::{InstallerFile, ScanSummary};
use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use walkdir::WalkDir;

pub fn scan_path(root: &Path, include_archives: bool) -> Result<ScanSummary> {
    if !root.exists() {
        anyhow::bail!("Path does not exist: {}", root.display());
    }

    if !root.is_dir() {
        anyhow::bail!("Path is not a directory: {}", root.display());
    }

    let mut warnings = Vec::new();
    let mut files = Vec::new();

    let allowed = allowed_extensions(include_archives);
    let installed = installed_program_names();

    for entry in WalkDir::new(root).follow_links(false) {
        match entry {
            Ok(entry) => {
                if !entry.file_type().is_file() {
                    continue;
                }

                let path = entry.path();
                let ext = path
                    .extension()
                    .and_then(|v| v.to_str())
                    .map(|s| s.to_ascii_lowercase());

                let Some(ext) = ext else {
                    continue;
                };

                if !allowed.contains(ext.as_str()) {
                    continue;
                }

                let metadata = fs::metadata(path)
                    .with_context(|| format!("Failed to read metadata for {}", path.display()))?;

                let modified_unix_secs = metadata
                    .modified()
                    .ok()
                    .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                    .map(|d| d.as_secs());

                let file_stem = path
                    .file_stem()
                    .and_then(|v| v.to_str())
                    .unwrap_or_default();

                files.push(InstallerFile {
                    path: path.to_string_lossy().to_string(),
                    extension: ext,
                    size_bytes: metadata.len(),
                    modified_unix_secs,
                    installed_program_match: find_program_match(file_stem, &installed),
                });
            }
            Err(err) => warnings.push(format!("Traversal warning: {err}")),
        }
    }

    let scanned_at_unix_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("System time appears to be before UNIX_EPOCH")?
        .as_secs();

    Ok(ScanSummary {
        root: root.to_string_lossy().to_string(),
        scanned_at_unix_secs,
        include_archives,
        files,
        warnings,
    })
}

fn allowed_extensions(include_archives: bool) -> HashSet<&'static str> {
    let mut set = HashSet::from(["exe", "msi", "dmg", "pkg"]);

    if include_archives {
        set.insert("zip");
        set.insert("7z");
    }

    set
}
