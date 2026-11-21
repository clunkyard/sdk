# Clunkyard local CLI

Build a robot, play a real local match and inspect its replay with Node 24,
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
Install that language's toolchain separately, or use Docker: `build --docker`
and `match --docker` run in the language's public toolchain image,
`ghcr.io/clunkyard/toolchain/<language>`, and the CLI picks Docker by itself
when a tool is missing and Docker is running (`--no-docker` keeps a local
build). Downloaded files are cached in
`.clunkyard/`. Other commands include `list`, `setup`, `clean` and `migrate`.

Local matches use a resource mode supported by `bot.json`. Open-capable robots
use Open; Wasm- or DSL-only robots keep their fuel or operation meter and play
the bundled Vane rival with its DSL meter.

## Open a saved robot in your tools

```sh
npx @clunkyard/cli open ROBOT_ID --in code   # or cursor, claude, codex
```

Signs in through your browser the first time, clones the robot with Git
over HTTPS and adds local `AGENTS.md`, MCP and
VS Code task settings that Git ignores. The workbench's **Open in** shows the
command with the robot's ID. See
[Open in your tools](https://clunkyard.com/sdk/open-in).

## Play the house robots

```sh
npx @clunkyard/cli opponents
npx @clunkyard/cli match 42 2d classic --opponent skyhook
npx @clunkyard/cli match 7 2d classic --opponent orbit,loom
```

`opponents` lists the house robots and whether this machine can build and run
each. Their published source comes from the hosted projects on clunkyard.com
(the list and sources are cached in `.clunkyard/house`, so a fetched robot plays
offline; `opponents --refresh` updates them) and builds through the same
per-language path as your robot, so you need that language's toolchain. DSL
robots need only Node. `match` takes up to three `--opponent` names (repeat the
flag or use commas) and defaults to the bundled practice rival. Each house
robot plays only the modes and rulesets its `bot.json` declares. `match` ends
with who won and how ("Harrier won on score at the time limit", with the
scores) and the replay's path from the project; add `--json` for one JSON
document on stdout instead, with progress on stderr. `doctor` reports the CLI,
engine and SDK API versions: the SDK API version is the robot-facing API, so
it moves less often than the CLI. Hosted house
robots keep being tuned; the cached copy is the version of its last fetch.

JavaScript and TypeScript projects install the SDK with npm. TypeScript uses
your installed compiler, preferring a project-local installation. See the
[supported platforms and toolchain installers](https://clunkyard.com/sdk/player-kit#supported-platforms)
for your language. On Windows, use MSVC's Developer Shell or Clang for C/C++.

Commit the npm lockfile to reproduce JavaScript and TypeScript dependencies.
Match replays record the game rules and robot actions. Hosted matches use their
own compute budgets, so check your robot in a hosted trial too.

## Connect a remote robot or strategist

Create a remote or strategist
key in the bot's Connections panel and set `CLUNKYARD_REMOTE_TOKEN` and
`CLUNKYARD_BOT_ID` in your shell, keeping the key out of source and command
arguments. `CLUNKYARD_REMOTE_URL` defaults to clunkyard.com. From your robot directory:

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

Robot builds and local matches execute programs as your user. Use trusted
source and tools. For online play, use
`npx @clunkyard/cli open` (Git) or Clunkyard's
GitHub integration, then run hosted trials before entering a tournament.

[SDK and adapters](https://clunkyard.com/sdk) ·
[Local trials](https://clunkyard.com/sdk/local-trials) ·
[Private Git](https://clunkyard.com/sdk/private-git)
