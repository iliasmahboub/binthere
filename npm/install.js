#!/usr/bin/env node
const fs = require("node:fs");
const path = require("node:path");
const https = require("node:https");
const os = require("node:os");
const AdmZip = require("adm-zip");
const tar = require("tar");

const pkg = require("./package.json");
const outDir = path.join(__dirname, "bin");

const map = {
  "win32-x64": {
    asset: "binthere-x86_64-pc-windows-msvc.zip",
    binary: "binthere.exe",
    kind: "zip",
  },
  "linux-x64": {
    asset: "binthere-x86_64-unknown-linux-gnu.tar.gz",
    binary: "binthere",
    kind: "tar",
  },
  "darwin-x64": {
    asset: "binthere-x86_64-apple-darwin.tar.gz",
    binary: "binthere",
    kind: "tar",
  },
  "darwin-arm64": {
    asset: "binthere-aarch64-apple-darwin.tar.gz",
    binary: "binthere",
    kind: "tar",
  },
};

const key = `${process.platform}-${process.arch}`;
const target = map[key];

if (!target) {
  console.error(`Unsupported platform: ${key}`);
  process.exit(1);
}

const repo = process.env.BINTHERE_REPO || pkg.binthereBinary?.repo;
if (!repo || repo.includes("REPLACE_WITH")) {
  console.error(
    "npm package is missing GitHub repo config. Set package.json -> binthereBinary.repo."
  );
  process.exit(1);
}

const version = pkg.version;
const tagPrefix = pkg.binthereBinary?.tagPrefix || "v";
const tag = `${tagPrefix}${version}`;
const url = `https://github.com/${repo}/releases/download/${tag}/${target.asset}`;

fs.mkdirSync(outDir, { recursive: true });
const archivePath = path.join(os.tmpdir(), target.asset);

console.log(`Downloading BinThere ${version} for ${key}...`);
download(url, archivePath)
  .then(async () => {
    if (target.kind === "zip") {
      const zip = new AdmZip(archivePath);
      zip.extractAllTo(outDir, true);
    } else {
      await tar.x({ file: archivePath, cwd: outDir });
    }

    const binPath = path.join(outDir, target.binary);
    if (!fs.existsSync(binPath)) {
      throw new Error(`Binary not found after extraction: ${binPath}`);
    }

    if (process.platform !== "win32") {
      fs.chmodSync(binPath, 0o755);
    }

    console.log(`Installed ${target.binary}`);
  })
  .catch((err) => {
    console.error(`Failed to install binary: ${err.message}`);
    process.exit(1);
  });

function download(url, dest) {
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(dest);
    https
      .get(url, (res) => {
        if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
          file.close();
          fs.unlink(dest, () => {
            download(res.headers.location, dest).then(resolve).catch(reject);
          });
          return;
        }

        if (res.statusCode !== 200) {
          file.close();
          fs.unlink(dest, () => reject(new Error(`HTTP ${res.statusCode} from ${url}`)));
          return;
        }

        res.pipe(file);
        file.on("finish", () => {
          file.close(resolve);
        });
      })
      .on("error", (err) => {
        file.close();
        fs.unlink(dest, () => reject(err));
      });
  });
}
