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
`make doctor`, `make`, `make match` and `make replay`. Use
`--project DIRECTORY` to run commands from another directory.

The CLI downloads the local match engine and the SDK for your robot's language.
Install that language's toolchain separately. Downloaded files are cached in
`.clunkyard/`. Other commands include `list`, `setup`, `clean` and `migrate`.

Local matches use a resource mode supported by `bot.json`. Open-capable robots
use Open; Wasm- or DSL-only robots keep their fuel or operation meter and play
the bundled Vane rival with its DSL meter.

JavaScript and TypeScript projects install the SDK with npm. TypeScript uses
your installed compiler, preferring a project-local installation. See the
[supported platforms and toolchain installers](https://clunkyard.com/sdk/player-kit#supported-platforms)
for your language. On Windows, use MSVC's Developer Shell or Clang for C/C++.

Commit the npm lockfile to reproduce JavaScript and TypeScript dependencies.
Match replays record the game rules and robot actions. Hosted matches use their
own compute budgets, so check your robot in a hosted trial too.

## Connect a remote robot or strategist

Use Node 22+ for the built-in WebSocket client. Create a remote or strategist
key in the bot's Connections panel and set `CLUNKYARD_REMOTE_URL`,
`CLUNKYARD_REMOTE_TOKEN` and `CLUNKYARD_BOT_ID` in your shell, keeping the key
out of source and command arguments. From your robot directory:

```sh
npx @clunkyard/cli connect
npx @clunkyard/cli connect --strategist python3 strategist.py
```

With no command, the CLI builds this project and launches its SDK-aware stdio runner. An explicit command is for self-contained protocol programs with their dependencies already installed. The bridge runs commands without a shell and keeps them connected for Open
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
