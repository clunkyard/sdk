# Release the player packages

Both packages are public and scoped to `@clunkyard`. The application root remains
private. `@clunkyard/git` starts at 2.0.1 to distinguish its npm-ready metadata
and licence from the earlier 2.0.0 game-kit helper. `@clunkyard/sdk` starts at
2.0.0, matching the robot API; this does not change protocol 2 or Wasm ABI 4.

## Review exact bytes

Versions and public APIs are explicit in `packages/*/package.json`. Review the
source diff, licences, packlist and real installed-package qualification before
changing a version. Run the **Review npm packages** workflow with `stage` left
off first. All six operating-system/Node jobs must pass. Download the
`npm-reviewed` artifact: `packages.json` records each tarball's SHA-256, npm
integrity, file list and canonical source commit. `qualification.json` binds
the tested packages to those hashes.

Never publish from the private application checkout or rebuild a different
tarball after review. The public workflow stages the same Ubuntu/Node 24
tarballs that its installed-package checks exercised.

## Bootstrap new package names

The npm account must own or have write access to the Clunkyard organization.
Use a current npm CLI with staged publishing support (the workflow pins
11.21.2). For package names that do not exist yet, stage the reviewed artifacts
from an authenticated maintainer terminal:

```sh
npm stage publish ./clunkyard-git-2.0.1.tgz --access public --tag latest --ignore-scripts
npm stage publish ./clunkyard-sdk-2.0.0.tgz --access public --tag latest --ignore-scripts
```

Staging reserves the version but does not make it publicly installable. It
requires no 2FA approval. Review its details and bytes before approving. Do not
replace a pending staged version blindly; inspect the existing stage first.

## Configure trusted publishing

Once the package exists, configure the **exact** public workflow identity:

```sh
npm trust github @clunkyard/git --repo clunkyard/sdk --file npm-release.yml --environment npm-staging --allow-stage-publish
npm trust github @clunkyard/sdk --repo clunkyard/sdk --file npm-release.yml --environment npm-staging --allow-stage-publish
npm trust github @clunkyard/cli --repo clunkyard/sdk --file npm-release.yml --environment npm-staging --allow-stage-publish
```

These settings authorize staging only. There is no registry token in GitHub
and no automatic direct publish. npm requires account 2FA to configure this
relationship; complete its authenticated browser challenge when prompted.
Trusted publisher settings must be exercised within npm's current validity
window. Inspect existing relationships before replacing any configuration.

Dispatch **Review npm packages** on `main` with `verify_oidc` enabled and `stage`
disabled to check the relationship without changing registry contents. The
`verify-oidc` job exchanges GitHub's identity for a temporary package token for
each package, discards the tokens and saves a credential-free verification
receipt. This proves authentication; it does not count as a successful publish
for npm's initial trusted-publisher validity window.

For later versions, dispatch **Review npm packages** on `main` with `stage`
enabled. Qualification runs first; the stage job uses GitHub OIDC and npm
provenance to stage exactly those reviewed tarballs. Other branches and pull
requests cannot run the stage job.

Use the optional `packages` input to select one package, such as
`@clunkyard/cli`, or a comma-separated list. Already-published versions are
skipped only when their registry integrity matches the reviewed tarball.
Different bytes require a new version; the workflow fails rather than
restaging them.

## Approve a release

```sh
npm stage list @clunkyard/git --json
npm stage view STAGE_ID --json
npm stage download STAGE_ID
# Compare the downloaded SHA-256 with the reviewed packages.json.
npm stage approve STAGE_ID
```

Approval is the public immutable release and requires npm's interactive 2FA.
After approval, verify `npm view PACKAGE@VERSION dist --json` and perform a clean
registry install. Only then switch the website and starter instructions to npm.
Fixes after publication use a new version; keep prior tarballs and receipts.

Official references:

- https://docs.npmjs.com/cli/v11/commands/npm-stage/
- https://docs.npmjs.com/cli/v11/commands/npm-trust/
- https://docs.npmjs.com/trusted-publishers/

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
