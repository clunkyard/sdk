# Release the player packages

The three public packages are `@clunkyard/git`, `@clunkyard/sdk` and
`@clunkyard/cli`. The application root stays private. Package versions are
explicit release identities; the robot API remains 2.0.0, protocol 2 and Wasm
ABI 4 unless a separately reviewed API change says otherwise.

## Review exact bytes

Review source changes, licences, package contents and installed-package tests
before changing `packages/*/package.json`. Dispatch **Review npm packages** on
`main` with `publish=false` and `verify_oidc=false` for review. CI packs once on
Ubuntu/Node 24 and tests those same tarballs on Linux, macOS and Windows with
Node 20 and 24. All six jobs must pass.

Download `npm-reviewed`: `packages.json` records each archive's SHA-256, npm
integrity and canonical source commit. Each `qualification-*` artifact records
the source and package digests actually tested. Do not rebuild different bytes
after reviewing an archive.

## Configure trusted publishing once

The npm package owner configures this exact identity for each package:

```sh
npm trust github @clunkyard/git --repo clunkyard/sdk --file npm-release.yml --environment npm-release --allow-publish
npm trust github @clunkyard/sdk --repo clunkyard/sdk --file npm-release.yml --environment npm-release --allow-publish
npm trust github @clunkyard/cli --repo clunkyard/sdk --file npm-release.yml --environment npm-release --allow-publish
```

Creating these relationships requires the owner's npm authentication and 2FA.
The GitHub `npm-release` environment must restrict deployment to `main` and
must not require a recurring manual environment approval. There is no npm
publishing token or `NPM_TOKEN` secret. The publish job receives `id-token:
write`; npm 11.21.2 exchanges that job's GitHub identity automatically.

Read back the exact repository, workflow, environment and direct-publish
permission before releasing. npm requires a new trusted publisher's first
successful publication within two days. An authentication-only token exchange
or a run that skips an unchanged package does not satisfy that requirement.
Each newly configured package must actually publish a reviewed new version.

To check authentication without publication, dispatch on `main` with
`verify_oidc=true` and `publish=false`. The job discards temporary credentials
and retains a credential-free verification receipt.

## Publish a reviewed release

Dispatch **Review npm packages** on `main` with `publish=true`. The optional
`packages` input selects known names, such as `@clunkyard/cli`; leave it empty
to release all three reviewed packages.

The workflow packs once, qualifies all six platform/Node combinations, verifies
OIDC, then downloads the same tarballs and all six qualification receipts into
the `npm-release` job. `scripts/publish.cjs` checks every receipt's platform,
Node major, source commit and package digests; checks every exported source
input; and recomputes archive SHA-256 and SHA-512 integrity before running
`npm publish ARCHIVE --provenance --ignore-scripts`. Pushes and pull requests
only review and qualify: they cannot publish. No stage approval or per-release
npm 2FA is part of this normal path.

Already-published versions are skipped only when registry integrity matches the
reviewed archive. All selected packages are preflighted before any immutable
registry write. A conflicting published version fails the run; choose a new
version instead of replacing bytes. `npm-publication` records the outcome.
Verify the public registry archive and a fresh anonymous installation before
updating player instructions or claiming release delivery.

## Existing staged candidates and new names

The earlier stage-only workflow produced pending CLI 2.2.0 and 2.2.1 candidates.
They remain unpublished and are outside the normal release path. Staged and
published versions share npm's unique version index: direct publication cannot
reuse a version that is still staged. Never silently approve, reject or replace
one of these candidates. The first direct OIDC cutover therefore uses CLI
2.2.2, Git 2.0.2 and SDK 2.0.1. These packaging/provenance bootstrap patches keep
all runtime bytes except package metadata unchanged, including the SDK's API
and core version, and do not change existing starter dependency locks.

New package names require owner bootstrap before a trusted publisher can be
configured. Review their exact archive and use npm's supported owner flow;
keep that exceptional setup separate from routine direct releases.

Official references:

- [Trusted publishing](https://docs.npmjs.com/trusted-publishers/)
- [Trust configuration](https://docs.npmjs.com/cli/v11/commands/npm-trust/)
- [Staged version uniqueness](https://docs.npmjs.com/cli/v11/commands/npm-stage/)

## Native SDK qualification

The public export also includes the native core build helper and the C, C++,
Python and Rust adapter sources. These are first-party SDK surfaces; no arena
application or deployment configuration is exported for this workflow.

Dispatch **Native starter qualification** to build the core with Rust 1.96.0
and run fresh anonymous starter clones through an installed candidate CLI.
The Intel job explicitly uses `macos-15-intel`; `macos-15` is a separate Apple
Silicon job. Ubuntu 24.04 supplies a Linux regression. Each runs real 2D and
3D trials, checks firing and zero robot failures, and retains source commits,
compiler metadata, logs, replays and the installed SDK archive. Runner labels
are documented in [GitHub's hosted-runner reference](https://docs.github.com/en/actions/reference/runners/github-hosted-runners).

The initial qualification uses `CLUNKYARD_SDK` to test newly built native
libraries before their immutable archives exist. It does not prove registry
installation or catalogue download selection. After platform asset hashes are
added, repeat the public fresh-install flow without that override before
claiming release delivery.

Before compiling, the smoke runner checks every file against the clean export's
`release-source.json`. Its SDK artifact then records the canonical source
commit, actual public checkout commit and export-manifest SHA-256. This explicit
mapping lets the private release assembler bind SDK bytes to their canonical
inputs without silently relabelling a different source tree. Public package
contents and existing npm SDK notice bytes are unaffected by the extra CI files.

For an exact prepublication Intel archive gate, prepare one reviewed bundle with
`release/assets/`, the candidate `tools/player-cli/` and
`scripts/player-kit/{validate,verify,archive}.cjs`. Upload its `.tar.gz` once to a
public prerelease in `clunkyard/sdk`; retain its SHA-256 and never replace it.
Dispatch **Exact native archive qualification** with that tag, exact asset name
and hash. The read-only Intel job verifies the hash before safe extraction, then
runs the existing native validator against those exact SDK and engine archives.
It strips custom SDK overrides and GitHub tokens from validation subprocesses,
and uploads the matching results, logs and replays. The workflow publishes
nothing. A passing receipt closes archive selection/extraction separately from
the earlier installed-SDK bootstrap and later anonymous registry installation.

After publication, dispatch **Public native npm qualification** for the exact
registry version. Intel macOS and Windows x64 runners install the public package
with empty npm configuration and fresh caches. Both clone C/C++/Python/Rust;
Windows also checks C#. They run real 2D/3D matches with no SDK or local-archive
override. The Windows job uses the installed Visual Studio Developer Shell and
normal .NET/Rust SDKs; the smoke runner invokes Node directly, requiring no Make.
It checks registry integrity, installed bytes, normal source preservation and
robot results.
The same public `scripts/native-registry-smoke.cjs` accepts an explicit platform
and selected language list for other native hosts; it never installs toolchains.
