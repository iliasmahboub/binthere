use clap::{ArgAction, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "binthere",
    version,
    about = "Scan and safely purge installer files",
    long_about = "BinThere scans folders for installer/setup files and supports safe dry-run or confirmed purge."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Recursively scan a directory for installer files
    Scan {
        /// Target path to scan (defaults to Downloads folder)
        path: Option<PathBuf>,

        /// Include archive installers (.zip, .7z)
        #[arg(long, action = ArgAction::SetTrue)]
        include_archives: bool,
    },

    /// Show found files and summary from latest scan
    Report,

    /// Display or delete files found in latest scan
    Purge {
        /// Explicit dry-run output (default behavior if --confirm is not set)
        #[arg(long, action = ArgAction::SetTrue, conflicts_with = "confirm")]
        dry_run: bool,

        /// Actually delete files after interactive confirmation
        #[arg(long, action = ArgAction::SetTrue, conflicts_with = "dry_run")]
        confirm: bool,
    },
}
