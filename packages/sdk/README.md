# Clunkyard robot SDK for Node.js

Write a Clunkyard robot in JavaScript or TypeScript. This package contains the
robot API, TypeScript declarations and the shared WebAssembly core. Install
Node 20 or later to run it.

## Use the same import as the arena

Clunkyard's hosted JavaScript and TypeScript starters import `clunkyard`.
Install this package under that name to keep the same source locally and online:

```sh
npm install clunkyard@npm:@clunkyard/sdk
```

```javascript
const cy = require('clunkyard');

class Scout extends cy.Robot {
  onTurn() {
    const target = this.sensors.nearest;
    this.drive.set({ throttle: 0.5, turn: 0.2 });
    this.scanner.set({ heading: target ? target.bearing : this.me.heading,
      arc: target ? 12 : 40 });
    if (target) {
      const power = 0.5;
      const aim = this.gun.lead(target, power);
      this.gun.aim({ heading: aim ? aim.heading : target.bearing,
        pitch: aim ? aim.pitch : target.elevation });
      if (this.gun.canFire(power, 10)) this.gun.fire({ power });
    }
  }
}

cy.run(new Scout());
```

The same alias works with `import * as cy from 'clunkyard'` in TypeScript.
Keep `package.json` and your lock file in the local project. The arena supplies
the matching binding when it builds your bot; do not upload `node_modules`.

For a standalone Node application, install `@clunkyard/sdk` directly and use
`require('@clunkyard/sdk')`. Use the alias above for hosted arena bot source.

## Run and test a robot

`cy.run(robot)` reads observations from standard input and writes the robot's
replies. Use the [language starter](https://github.com/clunkyard/starter-javascript)
and its `make match` and `make replay` commands to play local matches.

`cy.play(robot, lines)` accepts recorded protocol lines and returns reply lines
in memory, which is useful for focused tests. It uses the same core parser,
tracking, geometry and control serialization as `cy.run`.

Facts are properties of `robot.me`, `robot.sensors`, `robot.battery` and the other
subsystems. Controls take objects; later calls replace only the fields supplied.
An absent optional result is `null`. Throw `new cy.Forfeit(code, message)` to
forfeit deliberately. Other callback exceptions are logged to standard error;
the current turn still sends its accumulated controls.

Send diagnostics to standard error and keep top-level output away from the
protocol stream. The language starter provides the arena's normal launcher.

[Complete API](https://clunkyard.com/sdk/javascript) ·
[C API and adapters](https://clunkyard.com/sdk) ·
[Local development](https://clunkyard.com/sdk/player-kit) ·
[Private Git](https://clunkyard.com/sdk/private-git)

The [core source](https://github.com/clunkyard/sdk/tree/main/sdk/core) and
bundled dependency and toolchain notices are available for review.
