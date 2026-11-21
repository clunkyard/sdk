# Clunkyard player SDK packages

Tools and libraries for building and running Clunkyard robots.

| Package | Purpose | Runtime |
| --- | --- | --- |
| `@clunkyard/sdk` | JavaScript/TypeScript robot API and bundled WebAssembly core | Node 24 |
| `@clunkyard/cli` | Shared starter, local build, match and replay commands | Node 24 and the selected language's tools |

Sign in and clone a saved robot with Git:

```sh
npx @clunkyard/cli open ROBOT_ID
```

See the [Git guide](https://clunkyard.com/sdk/private-git) for sign-in, clone
and push.

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
