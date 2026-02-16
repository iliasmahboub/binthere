use crate::models::ScanSummary;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

const STATE_FILE: &str = "last_scan.json";

pub fn save_scan(summary: &ScanSummary) -> Result<PathBuf> {
    let path = state_path()?;
    let parent = path
        .parent()
        .context("Failed to derive parent directory for scan state")?;

    fs::create_dir_all(parent)
        .with_context(|| format!("Failed to create state directory: {}", parent.display()))?;

    let bytes = serde_json::to_vec_pretty(summary).context("Failed to serialize scan summary")?;
    fs::write(&path, bytes).with_context(|| format!("Failed to write state file: {}", path.display()))?;

    Ok(path)
}

pub fn load_scan() -> Result<ScanSummary> {
    let path = state_path()?;

    if !path.exists() {
        anyhow::bail!(
            "No saved scan found. Run `binthere scan` first (expected state file at {}).",
            path.display()
        );
    }

    let bytes = fs::read(&path).with_context(|| format!("Failed to read state file: {}", path.display()))?;
    let summary: ScanSummary = serde_json::from_slice(&bytes).context("Failed to parse saved scan JSON")?;

    Ok(summary)
}

pub fn state_path() -> Result<PathBuf> {
    let base = dirs::data_local_dir().context("Could not resolve local application data directory")?;
    Ok(base.join("BinThere").join(STATE_FILE))
}
