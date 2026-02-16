use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallerFile {
    pub path: String,
    pub extension: String,
    pub size_bytes: u64,
    pub modified_unix_secs: Option<u64>,
    pub installed_program_match: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanSummary {
    pub root: String,
    pub scanned_at_unix_secs: u64,
    pub include_archives: bool,
    pub files: Vec<InstallerFile>,
    pub warnings: Vec<String>,
}

impl ScanSummary {
    pub fn total_size_bytes(&self) -> u64 {
        self.files.iter().map(|f| f.size_bytes).sum()
    }
}
