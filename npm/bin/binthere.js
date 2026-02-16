#!/usr/bin/env node
const fs = require("node:fs");
const path = require("node:path");
const cp = require("node:child_process");

const isWin = process.platform === "win32";
const exeName = isWin ? "binthere.exe" : "binthere";
const binPath = path.join(__dirname, exeName);

if (!fs.existsSync(binPath)) {
  console.error(
    "BinThere binary is missing. Reinstall package or run: npm rebuild binthere-cli"
  );
  process.exit(1);
}

const child = cp.spawn(binPath, process.argv.slice(2), { stdio: "inherit" });
child.on("exit", (code, signal) => {
  if (signal) {
    process.kill(process.pid, signal);
    return;
  }
  process.exit(code ?? 1);
});
