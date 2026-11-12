# Clunkyard local CLI

Build a robot, play a real local match and inspect its replay with Node 20+,
npm and your existing language toolchain:

```sh
npx @clunkyard/cli new javascript my-robot
cd my-robot
npx @clunkyard/cli doctor
npx @clunkyard/cli build
npx @clunkyard/cli match 42 2d classic
npx @clunkyard/cli replay
```

Or clone any [language starter](https://github.com/clunkyard/game-kit) and use
`make doctor`, `make`, `make match` and `make replay`. A project contains robot
source, `bot.json` and normal language build metadata. The shared CLI owns the
tooling; it does not copy a bootstrapper, archive reader or custom lock file
into every repository. `--project DIRECTORY` works from another directory.

The package contains the reviewed SDK 2.0.0/protocol 2 catalogue, checksums and
build recipes from immutable game-kit releases, including native desktop
SDKs in `v2.0.0-kit.3`. It downloads only the
engine and selected language SDK. It never downloads compilers or the
all-language builder. SDK/engine caches under ignored `.clunkyard/` are keyed
by their content hashes and remain reusable across CLI updates. Other
commands are `list`, `setup`, `clean` and `migrate`.

Local matches use a resource mode supported by `bot.json`. Open-capable robots
use Open; Wasm- or DSL-only robots keep their fuel or operation meter and play
the bundled Vane rival with its DSL meter.

JavaScript/TypeScript projects use the ordinary npm alias
`clunkyard@npm:@clunkyard/sdk@2.0.0`. `package-lock.json` pins the reviewed
integrity. The first build runs `npm ci --ignore-scripts`; the package has no
install hooks or runtime dependencies. TypeScript uses your installed
TypeScript 6+ compiler, preferring a project-local installation. Native SDKs
support Linux x86-64/glibc 2.39+, C/C++ on Linux AArch64, selected languages on macOS arm64/x86-64,
and C/C++/Python/Rust/C# on Windows x86-64. See the
[supported platform table and official toolchain installers](https://clunkyard.com/sdk/player-kit#supported-platforms).
Use MSVC's Developer Shell or a working Clang toolchain for Windows C/C++.
Python, Rust and .NET binding installation needs no Unix shell or GNU Make.
JavaScript, TypeScript and the Vane/Cadence/Glyph Node routes use WebAssembly.

Use `npx @clunkyard/cli` for current tooling. Record `--version` with your
source commit when comparing trials. Commit the npm lockfile for reproducible
JavaScript/TypeScript dependencies. Match replays record the game rules and
robot actions; local timings do not qualify hosted budgets.

## Connect a remote robot or strategist

Use Node 22+ for the built-in WebSocket client. Create a remote or strategist
key in the bot's Connections panel and set `CLUNKYARD_REMOTE_URL`,
`CLUNKYARD_REMOTE_TOKEN` and `CLUNKYARD_BOT_ID` in your shell, keeping the key
out of source and command arguments. From your robot directory:

```sh
npx @clunkyard/cli connect python3 bot.py
npx @clunkyard/cli connect --strategist python3 strategist.py
```

The bridge runs the command without a shell and keeps it connected for Open
matches. A strategist supplies policy for the bot's hosted reflex; it does not
replace the per-turn robot. Child processes do not receive connection keys.
Use `CLUNKYARD_BOT_ENV_ALLOW` only for additional environment variable names
your code needs. Keep the CLI running until the match ends; Ctrl-C disconnects
and stops its robot processes.

For an older generated kit checkout, commit your work and run
`npx @clunkyard/cli migrate`. It removes only byte-recognized generated
helper files, preserves robot source and custom README text, and refuses to
overwrite customized build scripts or dependency manifests. Historical kit
archives remain available with their original tooling.

Robot builds and local matches execute programs as your user. Use trusted
source and tools. For online play, use
[`@clunkyard/git`](https://www.npmjs.com/package/@clunkyard/git) or Clunkyard's
GitHub integration, then run hosted trials before entering a tournament.

[SDK and adapters](https://clunkyard.com/sdk) ·
[Local trials](https://clunkyard.com/sdk/local-trials) ·
[Private Git](https://clunkyard.com/sdk/private-git)
