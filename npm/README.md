# npm wrapper

This folder contains an npm package that installs the prebuilt BinThere binary from GitHub Releases and exposes the `binthere` command.

## Setup before publish

1. Edit `package.json` and set `binthereBinary.repo` to your repository, for example:
   - `ilyas/binthere`
2. Ensure a matching GitHub release exists for the npm version, for example:
   - npm version `0.1.0` -> git tag/release `v0.1.0`

## Publish

```bash
cd npm
npm publish --access public
```

## Install for users

```bash
npm i -g binthere-cli
binthere --help
```
