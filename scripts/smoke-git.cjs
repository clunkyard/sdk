#!/usr/bin/env node
'use strict';

// Exercise the packed package through npm's installed command shims and Git's
// normal remote-helper discovery. This fixture never uses a registry key or
// the production service. Run via `npm exec -- node ... TGZ` to select npm.
const assert = require('node:assert/strict');
const crypto = require('node:crypto');
const fs = require('node:fs');
const http = require('node:http');
const os = require('node:os');
const path = require('node:path');
const { spawn } = require('node:child_process');

function npmCli() {
  const candidates = [process.env.npm_execpath,
    path.join(path.dirname(process.execPath), 'node_modules/npm/bin/npm-cli.js'),
    path.resolve(path.dirname(process.execPath), '../lib/node_modules/npm/bin/npm-cli.js'),
    path.resolve(path.dirname(process.execPath), '../share/nodejs/npm/bin/npm-cli.js')];
  const file = candidates.find(value => value && /\.[cm]?js$/.test(value) && fs.existsSync(value));
  if (!file) throw Error('Run through npm exec -- node scripts/npm-release/smoke-git.cjs PACKAGE.tgz.');
  return file;
}

function gitObject(type, data) {
  data = Buffer.from(data);
  return { oid:crypto.createHash('sha1').update(`${type} ${data.length}\0`).update(data).digest('hex'), type, data };
}
const wire = object => ({ oid:object.oid, type:object.type, data:object.data.toString('base64') });

async function main() {
  if (process.argv.length !== 3) throw Error('Usage: npm exec -- node scripts/npm-release/smoke-git.cjs PACKAGE.tgz');
  const archive = path.resolve(process.argv[2]);
  assert.ok(fs.statSync(archive).isFile(), 'Supply the packed npm tarball.');
  const directory = fs.mkdtempSync(path.join(process.env.TMPDIR || os.tmpdir(), 'npm-git-smoke-'));
  const prefix = path.join(directory, 'npm prefix'), config = path.join(directory, 'owner config');
  const otherConfig = path.join(directory, 'other config');
  const emptyConfig = path.join(directory, 'empty-config');
  fs.writeFileSync(emptyConfig, '');
  const environment = { ...process.env };
  const inheritedPath = process.env.PATH || process.env.Path || '';
  for (const key of Object.keys(environment)) {
    if (['path', 'npm_config_userconfig', 'npm_config_cache'].includes(key.toLowerCase()) || /^GIT_CONFIG(?:_|$)/.test(key) || /^(?:CLUNKYARD_AGENT_KEY|CLUNKYARD_ORIGIN|GIT_DIR|GIT_WORK_TREE|GIT_INDEX_FILE|GIT_OBJECT_DIRECTORY|GIT_ALTERNATE_OBJECT_DIRECTORIES)$/.test(key)) delete environment[key];
  }
  Object.assign(environment, {
    PATH:[process.platform === 'win32' ? prefix : path.join(prefix, 'bin'), inheritedPath].join(path.delimiter),
    APPDATA:config, XDG_CONFIG_HOME:config,
    NPM_CONFIG_USERCONFIG:emptyConfig, NPM_CONFIG_CACHE:path.join(directory, 'npm-cache'),
    GIT_CONFIG_GLOBAL:emptyConfig, GIT_CONFIG_NOSYSTEM:'1', GIT_TERMINAL_PROMPT:'0',
    GIT_AUTHOR_NAME:'Clunkyard package check', GIT_AUTHOR_EMAIL:'package-check@example.invalid',
    GIT_COMMITTER_NAME:'Clunkyard package check', GIT_COMMITTER_EMAIL:'package-check@example.invalid',
  });
  const ownerKey = 'fixture-owner-' + crypto.randomUUID(), otherKey = 'fixture-other-' + crypto.randomUUID();
  const secrets = [ownerKey, otherKey];
  const clean = value => secrets.reduce((text, secret) => text.split(secret).join('[redacted]'), String(value));
  async function command(executable, args, { cwd = directory, input = '', env = environment, fails = false, verbatim = false } = {}) {
    const result = await new Promise((resolve, reject) => {
      const child = spawn(executable, args, { cwd, env, windowsVerbatimArguments:verbatim, detached:process.platform !== 'win32', stdio:['pipe', 'pipe', 'pipe'] });
      let stdout = '', stderr = '', timedOut = false;
      const timer = setTimeout(() => {
        timedOut = true;
        // Git and Windows command shims may have a helper child. Close that
        // process tree too, so a failed discovery check cannot hang CI pipes.
        if (process.platform === 'win32') spawn('taskkill', ['/PID', String(child.pid), '/T', '/F'], { stdio:'ignore' }).on('error', () => child.kill());
        else try { process.kill(-child.pid, 'SIGTERM'); } catch (error) { if (error.code !== 'ESRCH') child.kill(); }
      }, 45000);
      child.stdout.on('data', data => { stdout += data; });
      child.stderr.on('data', data => { stderr += data; });
      child.on('error', error => { clearTimeout(timer); reject(error); });
      child.on('close', code => { clearTimeout(timer); resolve({ code, stdout, stderr, timedOut }); });
      child.stdin.on('error', error => { if (error.code !== 'EPIPE') reject(error); });
      child.stdin.end(input);
    });
    assert.ok(!secrets.some(secret => (result.stdout + result.stderr).includes(secret)), 'An installed command exposed a fixture access key.');
    assert.equal(result.timedOut, false, `${path.basename(executable)} timed out.`);
    if (fails) assert.notEqual(result.code, 0, `${path.basename(executable)} unexpectedly succeeded.`);
    else assert.equal(result.code, 0, clean(`${path.basename(executable)} failed: ${result.stderr}`));
    return result;
  }
  const git = (cwd, args, options) => command('git', ['-c', 'commit.gpgsign=false', ...args], { ...options, cwd });
  let origin, revoked = false, pushes = 0, fixtureFailure;
  const botId = crypto.randomUUID(), objects = new Map();
  const blob = gitObject('blob', '# A private starter robot.\n'); objects.set(blob.oid, blob);
  const tree = gitObject('tree', Buffer.concat([Buffer.from('100644 bot.vane\0'), Buffer.from(blob.oid, 'hex')])); objects.set(tree.oid, tree);
  const initial = gitObject('commit', `tree ${tree.oid}\nauthor Clunkyard <package-check@example.invalid> 1794355200 +0000\ncommitter Clunkyard <package-check@example.invalid> 1794355200 +0000\n\nStart fixture history\n`); objects.set(initial.oid, initial);
  let head = initial.oid, revisionId = crypto.randomUUID();
  const route = `/api/bots/${botId}/git`;
  const server = http.createServer(async (request, response) => {
    const send = (status, value) => { response.writeHead(status, { 'Content-Type':'application/json' }); response.end(JSON.stringify(value)); };
    try {
      const token = String(request.headers.authorization || '').replace(/^Bearer /, '');
      if (token !== ownerKey && token !== otherKey || token === ownerKey && revoked) return send(401, { error:'Sign in to continue.' });
      if (request.method === 'GET' && request.url === '/api/me') return send(200, { account:{ handle:token === ownerKey ? 'owner' : 'other' } });
      if (token !== ownerKey) return send(404, { error:'Project not found.' });
      if (request.method === 'GET' && request.url === route) return send(200, { v:1, branch:'refs/heads/main', head, revisionId, limits:{ maxCommits:32, maxFileBytes:1000000, maxPushBytes:24000000 } });
      let text = '';
      for await (const chunk of request) { text += chunk; assert.ok(text.length <= 2000000, 'Fixture request is too large.'); }
      const input = text ? JSON.parse(text) : {};
      if (request.method === 'POST' && request.url === route + '/objects') {
        assert.ok(Array.isArray(input.oids) && input.oids.length <= 128);
        if (input.oids.some(oid => !objects.has(oid))) return send(404, { error:'Git object not found.' });
        return send(200, { objects:input.oids.map(oid => wire(objects.get(oid))), remaining:[] });
      }
      if (request.method === 'POST' && request.url === route + '/push') {
        assert.equal(input.base, head);
        assert.match(String(request.headers['x-clunkyard-request']), /^[0-9a-f-]{36}$/);
        for (const value of input.objects) {
          assert.ok(['blob', 'tree', 'commit'].includes(value.type));
          const data = Buffer.from(value.data, 'base64'), object = gitObject(value.type, data);
          assert.equal(data.toString('base64'), value.data); assert.equal(object.oid, value.oid);
          objects.set(object.oid, object);
        }
        assert.equal(objects.get(input.head)?.type, 'commit');
        assert.ok(objects.get(input.head).data.toString().split('\n').includes(`parent ${head}`));
        head = input.head; revisionId = crypto.randomUUID(); pushes++;
        return send(200, { state:'committed', head, revisionId, buildQueued:false });
      }
      return send(404, { error:'Fixture route not found.' });
    } catch (error) { fixtureFailure = clean(error.message); send(500, { error:'Fixture contract failed.' }); }
  });
  try {
    await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
    origin = `http://127.0.0.1:${server.address().port}`;
    await command(process.execPath, [npmCli(), 'install', '--global', '--prefix', prefix, '--offline', '--ignore-scripts', '--no-audit', '--no-fund', archive]);
    const packageDirectory = path.join(prefix, ...(process.platform === 'win32' ? [] : ['lib']), 'node_modules/@clunkyard/git');
    const metadata = JSON.parse(fs.readFileSync(path.join(packageDirectory, 'package.json'), 'utf8'));
    assert.equal(metadata.name, '@clunkyard/git');
    assert.match(fs.readFileSync(path.join(packageDirectory, 'LICENSE'), 'utf8'), /MIT License/);
    async function login(key, env = environment) {
      // .cmd files need cmd.exe on Windows. Only a fixed command name and our
      // generated numeric localhost URL enter shell text; the key is stdin.
      const args = ['login', '--origin', origin, '--token-stdin'];
      if (process.platform === 'win32') return command(process.env.ComSpec || 'cmd.exe', ['/d', '/v:off', '/s', '/c', 'clunkyard-git ' + args.join(' ')], { input:key + '\n', env, verbatim:true });
      return command('clunkyard-git', args, { input:key + '\n', env });
    }
    await login(ownerKey);
    const credentials = path.join(config, 'clunkyard/credentials.json');
    if (process.platform !== 'win32') assert.equal(fs.statSync(credentials).mode & 0o077, 0);
    const remote = `clunkyard::${origin}/bots/${botId}`, first = path.join(directory, 'first clone');
    await git(directory, ['clone', remote, first]);
    assert.equal(fs.readFileSync(path.join(first, 'bot.vane'), 'utf8'), blob.data.toString());
    fs.appendFileSync(path.join(first, 'bot.vane'), '# Improve target selection.\n');
    const binary = Buffer.from([0, 255, 10, 128, 1]);
    fs.writeFileSync(path.join(first, 'proof.bin'), binary);
    fs.writeFileSync(path.join(first, 'trial.sh'), '#!/bin/sh\nexit 0\n');
    await git(first, ['add', 'bot.vane', 'proof.bin', 'trial.sh']);
    await git(first, ['update-index', '--chmod=+x', 'trial.sh']);
    await git(first, ['commit', '-m', 'Improve the fixture robot']);
    const local = (await git(first, ['rev-parse', 'HEAD'])).stdout.trim();
    await git(first, ['push', 'origin', 'main']);
    assert.equal(head, local); assert.equal(pushes, 1);
    const second = path.join(directory, 'second clone');
    await git(directory, ['clone', remote, second]);
    assert.equal((await git(second, ['rev-parse', 'HEAD'])).stdout.trim(), local);
    assert.deepEqual(fs.readFileSync(path.join(second, 'proof.bin')), binary);
    assert.match((await git(second, ['ls-tree', 'HEAD', 'trial.sh'])).stdout, /^100755 /);
    const otherEnvironment = { ...environment, APPDATA:otherConfig, XDG_CONFIG_HOME:otherConfig };
    await login(otherKey, otherEnvironment);
    await git(directory, ['clone', remote, path.join(directory, 'denied clone')], { env:otherEnvironment, fails:true });
    revoked = true;
    await git(first, ['fetch', 'origin'], { fails:true });
    assert.equal(fixtureFailure, undefined, fixtureFailure);
    console.log(JSON.stringify({ package:metadata.name, version:metadata.version, platform:process.platform,
      installedCommands:true, exactGitIdentity:true, binary:true, executable:true,
      otherAccountDenied:true, revokedCredentialsDenied:true, fixturePushes:pushes }));
  } finally {
    await new Promise(resolve => server.close(resolve));
    fs.rmSync(directory, { recursive:true, force:true, maxRetries:5, retryDelay:100 });
  }
}

main().catch(error => { console.error(error.message); process.exitCode = 1; });
