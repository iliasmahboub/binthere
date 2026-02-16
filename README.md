# BinThere

BinThere is a CLI that finds leftover installer files and helps you remove them safely.

It scans folders (usually `Downloads`) for files like `.exe`, `.msi`, `.dmg`, and `.pkg`, then shows reclaimable space before any deletion.

## Install

### Users (npm)

```bash
npm i -g binthere-cli
binthere --help
```

### Developers (from source)

```bash
git clone https://github.com/iliasmahboub/binthere.git
cd binthere
cargo build --release
```

Run:

```bash
./target/release/binthere --help
```

Windows PowerShell:

```powershell
.\target\release\binthere.exe --help
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
- `--include-archives` include `.zip` and `.7z`

### `report`

Show files and summary from the latest scan.

```bash
binthere report
```

### `purge`

Preview or delete files from the latest scan.

```bash
binthere purge --dry-run
binthere purge --confirm
```

Options:
- `--dry-run` preview what would be deleted
- `--confirm` delete after interactive confirmation

## Typical Flow

```bash
binthere scan
binthere report
binthere purge --dry-run
binthere purge --confirm
```

## Safety

- Nothing is deleted unless `--confirm` is passed.
- `--confirm` still asks for interactive confirmation.
- Missing or locked files are skipped and reported.

## License

MIT
