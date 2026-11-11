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
