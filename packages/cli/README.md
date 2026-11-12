# Clunkyard local CLI

Build a robot, play a real local match and inspect its replay with Node 20+,
npm and your existing language toolchain:

```sh
npx --yes @clunkyard/cli@2 new javascript my-robot
cd my-robot
npx --yes @clunkyard/cli@2 doctor
npx --yes @clunkyard/cli@2 build
npx --yes @clunkyard/cli@2 match 42 2d classic
npx --yes @clunkyard/cli@2 replay
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
support Linux x86-64/glibc 2.39+, selected languages on macOS arm64/x86-64,
and C/C++/Python/Rust/C# on Windows x86-64. See the
[supported platform table and official toolchain installers](https://clunkyard.com/sdk/player-kit#supported-platforms).
Use MSVC's Developer Shell or a working Clang toolchain for Windows C/C++.
Python, Rust and .NET binding installation needs no Unix shell or GNU Make.
JavaScript, TypeScript and the Vane/Cadence/Glyph Node routes use WebAssembly.

`@2` receives compatible tooling updates. For an exact tool version, run
`npx --yes @clunkyard/cli@2.1.1` or set
`CLUNKYARD='npx --yes @clunkyard/cli@2.1.1'` with Make. Commit the npm lockfile
for reproducible JavaScript/TypeScript dependencies. Match replays record the
game rules and robot actions; local timings do not qualify hosted budgets.

For an older generated kit checkout, commit your work and run
`npx --yes @clunkyard/cli@2 migrate`. It removes only byte-recognized generated
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
