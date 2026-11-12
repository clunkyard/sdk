'use strict';

const assert = require('node:assert/strict');
const fs = require('node:fs');
const { createRequire } = require('node:module');
const path = require('node:path');

const [project, fixtures, specifier, expectedVersion] = process.argv.slice(2);
const resolve = createRequire(path.join(project, 'package.json'));
const cy = resolve(specifier);
assert.equal(resolve(`${specifier}/package.json`).name, '@clunkyard/sdk');
assert.equal(resolve(`${specifier}/package.json`).version, expectedVersion);
assert.equal(cy.angles.normalize(-450), 270);

for (const [file, mode, control] of [
  ['2d-classic.ndjson', '2d', 'direct'],
  ['3d-zero-g.ndjson', '3d', 'forces'],
]) {
  const lines = fs.readFileSync(path.join(fixtures, file), 'utf8').trim().split('\n');
  const observations = lines.map(JSON.parse);
  const starts = [], ticks = [];
  class Probe extends cy.Robot {
    onStart() { starts.push({ mode: this.match.mode, control: this.match.control }); }
    onTurn() {
      ticks.push(this.clock.tick);
      if (control === 'direct') {
        this.drive.set({ throttle: 0.4 });
        this.drive.set({ turn: 0.25 });
      } else {
        this.drive.forces({ thrust: 0.3, lift: -0.2 });
        this.drive.forces({ yawTorque: 0.1 });
      }
      this.scanner.set({ heading: this.me.heading, arc: 40 });
      this.status.set({ text: 'Élan', value: this.clock.tick });
    }
  }
  const replies = cy.play(new Probe(), lines).map(JSON.parse);
  assert.deepEqual(starts, [{ mode, control }]);
  assert.deepEqual(ticks, observations.filter(value => value.type === 'state').map(value => value.tick));
  assert.equal(replies[0].type, 'ready');
  assert.equal(replies[0].v, 2);
  const acts = replies.slice(1);
  assert.equal(acts.length, observations.filter(value => value.type === 'state').length);
  for (const [index, act] of acts.entries()) {
    assert.equal(act.v, 2);
    assert.equal(act.type, 'act');
    assert.equal(act.tick, observations[index + 1].tick);
    if (control === 'direct') assert.deepEqual(act.drive, { throttle: 0.4, turn: 0.25 });
    else assert.deepEqual(act.forces, { thrust: 0.3, lift: -0.2, yawTorque: 0.1 });
    assert.equal(act.status.text, 'Élan');
    assert.equal(act.status.value, act.tick);
  }
  class Finished extends cy.Robot {
    onTurn() { throw new cy.Forfeit('TRIAL_DONE', 'Trial complete.'); }
  }
  const forfeits = cy.play(new Finished(), lines).map(JSON.parse);
  assert.ok(forfeits.some(reply => reply.type === 'forfeit'
    && reply.code === 'TRIAL_DONE' && reply.message === 'Trial complete.'));
}
console.log(JSON.stringify({ specifier, protocol: 2, fixtures: 2 }));
