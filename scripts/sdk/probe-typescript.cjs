'use strict';

const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { createRequire } = require('node:module');
const [project, fixtures] = process.argv.slice(2);
const resolve = createRequire(path.join(project, 'package.json'));
const cy = resolve('clunkyard');
const { Probe } = require(path.join(project, 'typescript-output/typescript-probe.js'));
const lines = fs.readFileSync(path.join(fixtures, '2d-classic.ndjson'), 'utf8').trim().split('\n');
const replies = cy.play(new Probe(), lines).map(JSON.parse);
assert.equal(replies[0].type, 'ready');
assert.equal(replies.length, lines.length);
assert.ok(replies.slice(1).every(reply => reply.type === 'act'
  && reply.status.text === 'TYPE_CHECKED' && reply.drive.throttle === 0.4));
console.log('TypeScript declarations and compiled robot passed');
