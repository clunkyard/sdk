'use strict';

// Run after installing the packed SDK both directly and under the hosted
// `clunkyard` alias in a clean project. Fixtures come from protocol/v2/fixtures.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { spawnSync } = require('node:child_process');

const [projectArgument, fixturesArgument, compiler = 'tsc', expectedVersion] = process.argv.slice(2);
if (!projectArgument || !fixturesArgument) throw Error('Usage: node smoke.cjs PROJECT FIXTURE_DIRECTORY [TSC]');
const project = path.resolve(projectArgument), fixtures = path.resolve(fixturesArgument);
const temporaryRoot = process.env.TMPDIR;
if (!temporaryRoot || !path.isAbsolute(temporaryRoot)) throw Error('Set TMPDIR to an absolute project temporary directory.');
const scratch = fs.mkdtempSync(path.join(temporaryRoot, 'npm-sdk-smoke-'));
function run(command, args, { input, stderrMatch } = {}) {
  const env = { ...process.env }; delete env.NODE_PATH;
  const result = spawnSync(command, args, { cwd: scratch, env, input, encoding: 'utf8', maxBuffer: 1 << 20 });
  if (result.error) throw result.error;
  assert.equal(result.status, 0, `${command}: ${result.stderr || result.stdout}`);
  if (stderrMatch) assert.match(result.stderr, stderrMatch);
  return result.stdout.trim();
}
function compile(args) {
  // An explicit TypeScript entry point runs through Node on every platform;
  // Windows npm's tsc.cmd shim cannot be spawned directly without a shell.
  return path.isAbsolute(compiler) && fs.existsSync(compiler)
    ? run(process.execPath, [compiler, ...args]) : run(compiler, args);
}
try {
  for (const specifier of ['@clunkyard/sdk', 'clunkyard']) {
    console.log(run(process.execPath, [path.join(__dirname, 'probe.cjs'), project, fixtures, specifier, expectedVersion || require(path.join(project,'node_modules','@clunkyard','sdk','package.json')).version]));
    const input = fs.readFileSync(path.join(fixtures, '2d-classic.ndjson'), 'utf8');
    const replies = run(process.execPath, [path.join(__dirname, 'probe-stdio.cjs'), project, specifier], {
      input, stderrMatch: /SDK diagnostic belongs on standard error/,
    }).split('\n').map(JSON.parse);
    assert.equal(replies.length, input.trim().split('\n').length);
    assert.equal(replies[0].type, 'ready');
    assert.ok(replies.slice(1).every(reply => reply.type === 'act' && reply.drive.throttle === 0.4));
  }
  const source = path.join(project, 'typescript-probe.ts');
  if (fs.existsSync(source)) throw Error('The smoke project already contains typescript-probe.ts.');
  try {
    fs.copyFileSync(path.join(__dirname, 'probe.ts'), source);
    compile(['--strict', '--target', 'ES2022', '--module', 'NodeNext', '--moduleResolution', 'NodeNext', '--outDir', path.join(project, 'typescript-output'), source]);
    const scoped = path.join(project, 'typescript-scoped.ts');
    if (fs.existsSync(scoped)) throw Error('The smoke project already contains typescript-scoped.ts.');
    try {
      fs.writeFileSync(scoped, fs.readFileSync(source, 'utf8').replace("from 'clunkyard'", "from '@clunkyard/sdk'"));
      compile(['--strict', '--target', 'ES2022', '--module', 'NodeNext', '--moduleResolution', 'NodeNext', '--noEmit', scoped]);
    } finally { fs.rmSync(scoped, { force: true }); }
    console.log(run(process.execPath, [path.join(__dirname, 'probe-typescript.cjs'), project, fixtures]));
  } finally { fs.rmSync(source, { force: true }); }
} finally { fs.rmSync(scratch, { recursive: true, force: true }); }
