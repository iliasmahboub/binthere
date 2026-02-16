# BinThere

BinThere is a safety-first CLI for cleaning up installer files.

It scans folders (usually `Downloads`) for setup files like `.exe` and `.msi`, shows reclaimable space, and only deletes files when you explicitly confirm.

## Install

If `cargo install binthere` or `npm i -g binthere-cli` is not available yet, use source install first.

### Option 1: Build from source

```bash
git clone https://github.com/iliasmahboub/binthere.git
cd binthere
cargo build --release
```

Run it:

```bash
./target/release/binthere --help
```

Windows PowerShell:

```powershell
.\target\release\binthere.exe --help
```

### Option 2: Cargo install

```bash
cargo install binthere
```

### Option 3: npm global install

```bash
npm i -g binthere-cli
binthere --help
```

## Commands

```bash
binthere <COMMAND>
```

### `scan [PATH]`

Recursively scan a directory for installer files.

```bash
binthere scan
binthere scan "C:\\Users\\ilyas\\Downloads"
binthere scan --include-archives
```

Arguments:
- `[PATH]` target path to scan (defaults to Downloads folder)

Options:
- `--include-archives` include archive installers (`.zip`, `.7z`)

### `report`

Show found files and summary from the latest saved scan.

```bash
binthere report
```

Note:
- Run `binthere scan` first so there is saved scan data to report.

### `purge`

Display or delete files found in the latest scan.

```bash
binthere purge --dry-run
binthere purge --confirm
```

Options:
- `--dry-run` explicit dry-run output (safe preview)
- `--confirm` actually delete files after interactive confirmation

Note:
- Run `binthere scan` first so there is saved scan data to purge.

## Safety

- Deletion never happens unless `--confirm` is passed.
- `--confirm` still prompts for confirmation before deleting.
- Missing or locked files are skipped and reported.

## License

MIT
