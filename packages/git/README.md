# Private Clunkyard Git

Install Node 20 or later and Git, then run:

```sh
npm install --global @clunkyard/git
```

On Linux or macOS, add `--prefix "$HOME/.local"` and put `$HOME/.local/bin`
on your PATH to install without administrator access.

Create an **agent access key** on your Clunkyard Account page, then run:

```sh
clunkyard-git login
git clone clunkyard::https://clunkyard.com/bots/BOT_ID my-robot
cd my-robot
# Edit your robot, then use ordinary Git.
git add bot.vane
git commit -m "Improve target selection"
git push origin main
```

The key is entered without echo and stored in your user configuration directory.
Keep that directory outside your repository, including when overriding
`XDG_CONFIG_HOME` or `APPDATA`. An agent key acts for your account; revoke it from the
Account page when no longer needed. It never goes in the remote URL or project.
For a secret-manager pipeline, `clunkyard-git login --token-stdin` also works.

The initial clone starts ordinary Git history at your current saved project.
Older application revisions remain in Clunkyard's revision history. Subsequent
browser and WebDAV saves become commits on the same `main` branch. Use `git
fetch`, inspect the change and merge before pushing stale work. Only `main` is
supported; force pushes, branch creation, tags, submodules and symlinks are not.
Push at most 32 new commits at once, within the project's file and source limits.

Binary files and executable bits are retained. Git does not represent empty
directories or arbitrary Unix permission bits; existing empty directories and
non-executable permissions remain application metadata. Historical project
`.git` files are never checked out or executed and remain private application
history. A push cannot replace them.

Successful pushes publish an immutable source revision and request its normal
hosted build. Compilation may fail independently; inspect the build in My bots.
An interrupted push retains an operation receipt inside your local Git directory;
retry `git push` to reconcile it without duplicating a saved revision.
