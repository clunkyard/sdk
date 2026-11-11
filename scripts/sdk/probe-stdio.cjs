'use strict';

const path = require('node:path');
const { createRequire } = require('node:module');
const [project, specifier] = process.argv.slice(2);
const cy = createRequire(path.join(project, 'package.json'))(specifier);
class Probe extends cy.Robot {
  onStart() { console.log('SDK diagnostic belongs on standard error'); }
  onTurn() { this.drive.set({ throttle: 0.4 }); }
}
cy.run(new Probe()).catch(error => { console.error(error); process.exitCode = 1; });
