# BinThere

<p align="center">
  <img src="./assets/binthere-logo.png" alt="BinThere logo" width="220" />
</p>

BinThere is a Rust CLI that finds leftover installer files and helps you remove them safely.

It is built for the classic `Downloads` problem: setup files pile up, disk space disappears, and nobody remembers what can be deleted.

## What It Does

- Recursively scans directories for installer files
- Detects `.exe`, `.msi`, `.dmg`, `.pkg`
- Optionally includes `.zip` and `.7z`
- Shows file-level details and summary stats
- Defaults to dry-run behavior
- Requires explicit confirmation before deletion
- On Windows, can tag likely matches to installed programs via registry lookup

## Command Overview

```bash
binthere <COMMAND>
```

Available commands:
- `scan [PATH]`
- `report`
- `purge --dry-run`
- `purge --confirm`

### Scan

```bash
binthere scan
binthere scan "C:\\Users\\ilyas\\Downloads"
binthere scan --include-archives
```

Notes:
- If `PATH` is omitted, BinThere uses your Downloads directory when available.
- Scan results are saved and reused by `report` and `purge`.

### Report

```bash
binthere report
```

Shows:
- total installers found
- total reclaimable size
- largest installer files
- per-file list with metadata

### Purge

```bash
binthere purge --dry-run
binthere purge --confirm
```

Behavior:
- `--dry-run`: prints what would be deleted
- `--confirm`: asks for interactive confirmation, then deletes

## Install Locally

```bash
git clone <your-repo-url>
cd BinThere
cargo build --release
./target/release/binthere scan
```

Windows PowerShell:

```powershell
.\target\release\binthere.exe scan
```

## Distribution

### GitHub Releases

Workflow: `.github/workflows/release.yml`

Tag format: `v*` (example: `v0.1.0`)

```bash
git tag v0.1.0
git push origin v0.1.0
```

Builds release binaries for:
- Windows x64
- Linux x64
- macOS x64
- macOS arm64

### crates.io

For Rust users:

```bash
cargo install binthere
```

Publish:

```bash
cargo login
cargo publish
```

### npm (global `binthere` command)

The `npm/` package installs the correct prebuilt binary from GitHub Releases.

Before publishing:
- set `npm/package.json -> binthereBinary.repo` to `owner/repo`
- keep npm version aligned with git tag (`0.1.0` -> `v0.1.0`)

Publish:

```bash
cd npm
npm publish --access public
```

Install:

```bash
npm i -g binthere-cli
binthere --help
```

## Safety

- No file is deleted unless `--confirm` is passed.
- `--confirm` still requires an interactive yes/no prompt.
- Missing or locked files are skipped and reported.
- Invalid paths and permission issues return user-friendly errors.

## Example

```text
$ binthere scan
[SCAN] Root: C:\Users\ilyas\Downloads
[OK] Found installer candidates: 12
[OK] Total reclaimable: 1.87 GB
[INFO] Saved scan state: C:\Users\ilyas\AppData\Local\BinThere\last_scan.json

$ binthere report
[REPORT] C:\Users\ilyas\Downloads
Last scan: 2026-02-16T18:10:41Z
Total installers: 12
Total reclaimable: 1.87 GB

$ binthere purge --dry-run
[DRY RUN] Files that would be deleted:
- C:\Users\ilyas\Downloads\old-setup.exe (402.10 MB)
Potential reclaimable: 1.87 GB
```

## Branding

Logo path used by this README:
- `assets/binthere-logo.png`

## Project Layout

- `src/main.rs` entrypoint and command routing
- `src/cli.rs` Clap args and subcommands
- `src/scanner.rs` filesystem traversal and detection
- `src/report.rs` output formatting and summaries
- `src/purge.rs` dry-run and confirmed deletion flow
- `src/state.rs` persisted latest scan state
- `src/installed.rs` Windows installed-program matching
- `src/models.rs` shared data models

## License

MIT
