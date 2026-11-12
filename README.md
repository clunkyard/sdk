# Clunkyard player SDK packages

This repository contains the reviewed source and release setup for small
player packages. It does not include the game service or language toolchains.

| Package | Purpose | Runtime |
| --- | --- | --- |
| `@clunkyard/git` | Clone and push your private arena project using normal Git | Node 20+ and Git |
| `@clunkyard/sdk` | JavaScript/TypeScript robot API and bundled WebAssembly core | Node 20+ |
| `@clunkyard/cli` | Shared starter, local build, match and replay commands | Node 20+ and the selected language's tools |

Install the private Git helper from npm, then sign in with a Clunkyard agent
access key:

```sh
npm install --global @clunkyard/git
clunkyard-git login
```

See the [private Git guide](https://clunkyard.com/sdk/private-git) for user-owned
installation prefixes, Windows setup and clone/push instructions.

The SDK has no runtime dependencies or install scripts. For the existing hosted
starter, install the scoped SDK under its normal `clunkyard` import name:

```sh
npm install clunkyard@npm:@clunkyard/sdk@2.0.0
```

This preserves `require('clunkyard')` and TypeScript imports when the source goes
back to the arena. Direct consumers can install and import `@clunkyard/sdk`.
The package is a robot library, not a match engine or a browser bundle. Use a
[language starter](https://github.com/clunkyard/game-kit) for real local matches,
replays and optional selected-language containers.

The shared CLI creates a clean starter and supplies its local build and match
commands:

```sh
npx @clunkyard/cli new python my-robot
cd my-robot
npx @clunkyard/cli doctor
npx @clunkyard/cli match
```

It downloads only the pinned engine and selected SDK, using your installed
language tools. Robot repositories keep source and normal build metadata;
shared tool fixes arrive through the current CLI release. Record `--version`
with your source commit when comparing trials.

## Review and qualify a release

```sh
npm ci --ignore-scripts
npm run pack
npm run qualify
```

Set `TMPDIR` to an absolute temporary directory before qualification. Tests
install the actual packed artifacts in clean temporary projects. They exercise
Git's installed remote discovery and credential boundaries, real 2D/3D robot
protocols, scoped/hosted-alias imports and TypeScript declarations. CI runs those
checks on Linux, macOS and Windows with Node 20 and 24. Tarball hashes and
qualification receipts are retained as workflow artifacts.

## Core source and licences

`sdk/core` includes the Rust source, generated ABI, pinned dependency source and
committed WebAssembly module. `protocol/v2` contains the protocol fixtures,
vectors and API specification used by the core tests. Build from the committed
generated source; full API regeneration belongs to the canonical game source.

With Rust 1.96.0, its `wasm32-unknown-unknown` target and Binaryen 120 installed:

```sh
sdk/core/build-wasm.sh rebuilt-core.wasm
```

The build uses vendored dependencies offline. The SDK npm artifact includes the
complete relevant crate notices and Rust standard-library notices. The MIT
project licence accompanies all three packages.

`release-source.json` identifies the canonical source commit and hashes every
exported input. Package versions change explicitly; released bytes are never
overwritten. Maintainers reconcile SDK contributions with the canonical game
source before exporting the next release. Issues and proposed changes are
welcome here.

See [RELEASING.md](RELEASING.md) for direct OIDC publication after all six qualification
jobs and the one-time npm-side setup.
