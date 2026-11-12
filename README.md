# Clunkyard player SDK packages

Tools and libraries for building and running Clunkyard robots.

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

For a hosted starter, install the SDK under its `clunkyard` import name:

```sh
npm install clunkyard@npm:@clunkyard/sdk
```

This preserves `require('clunkyard')` and TypeScript imports when the source goes
back to the arena. Direct consumers can install and import `@clunkyard/sdk`.
Use a [language starter](https://github.com/clunkyard/game-kit) for local matches,
replays and optional language containers.

The shared CLI creates a clean starter and supplies its local build and match
commands:

```sh
npx @clunkyard/cli new python my-robot
cd my-robot
npx @clunkyard/cli doctor
npx @clunkyard/cli match
```

The CLI downloads the local match engine and your language's SDK. Install your
language tools separately.

Contributions are welcome. See [RELEASING.md](RELEASING.md) for build and
release instructions, and [LICENSE](LICENSE) for the project licence.
