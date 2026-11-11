'use strict';

const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');

function credentialPath() {
  const root = process.platform === 'win32' ? process.env.APPDATA || path.join(os.homedir(), 'AppData', 'Roaming') : process.env.XDG_CONFIG_HOME || path.join(os.homedir(), '.config');
  if (!path.isAbsolute(root)) throw Error('The Clunkyard configuration directory must be absolute.');
  return path.join(root, 'clunkyard', 'credentials.json');
}
function origin(value) {
  const url = new URL(value);
  if (url.username || url.password || url.search || url.hash || url.pathname !== '/' || url.protocol !== 'https:' && !(url.protocol === 'http:' && ['localhost', '127.0.0.1', '[::1]'].includes(url.hostname))) throw Error('Use an HTTPS Clunkyard origin, without a path or credentials.');
  return url.origin;
}
function readCredentials() {
  const file = credentialPath();
  let stat; try { stat = fs.lstatSync(file); } catch (error) { if (error.code === 'ENOENT') return {}; throw Error('Cannot read Clunkyard credentials.'); }
  if (!stat.isFile() || stat.isSymbolicLink() || stat.size > 65536 || process.platform !== 'win32' && (stat.mode & 0o077 || stat.uid !== process.getuid())) throw Error('Clunkyard credentials must be a private, owner-only file.');
  const value = JSON.parse(fs.readFileSync(file, 'utf8'));
  if (!value || typeof value !== 'object' || Array.isArray(value)) throw Error('Invalid Clunkyard credentials.');
  return value;
}
function saveCredential(site, key) {
  site = origin(site);
  if (typeof key !== 'string' || !key || /\s/.test(key) || key.length > 1024) throw Error('Invalid Clunkyard access key.');
  const file = credentialPath(), directory = path.dirname(file), credentials = readCredentials();
  fs.mkdirSync(directory, { recursive:true, mode:0o700 });
  const stat = fs.lstatSync(directory);
  if (!stat.isDirectory() || stat.isSymbolicLink() || process.platform !== 'win32' && (stat.mode & 0o077 || stat.uid !== process.getuid())) throw Error('Clunkyard credentials need a private configuration directory.');
  credentials[site] = key;
  const temporary = path.join(directory, `.credentials-${require('node:crypto').randomUUID()}`);
  try { fs.writeFileSync(temporary, JSON.stringify(credentials) + '\n', { mode:0o600, flag:'wx' }); fs.renameSync(temporary, file); }
  finally { fs.rmSync(temporary, { force:true }); }
  return file;
}
function keyFor(site) {
  // An environment key is scoped too: a hostile Git remote must not receive
  // the player's production credential merely because it is in the shell.
  if (process.env.CLUNKYARD_AGENT_KEY && site === origin(process.env.CLUNKYARD_ORIGIN || 'https://clunkyard.com')) return process.env.CLUNKYARD_AGENT_KEY;
  const key = readCredentials()[site];
  if (!key) throw Error(`Sign in first with clunkyard-git login --origin ${site}`);
  return key;
}
async function readJson(response, limit = 40 * 1024 * 1024) {
  const reader = response.body?.getReader(), chunks = []; let size = 0;
  if (!reader) throw Error('Clunkyard returned an empty response.');
  for (;;) { const { done, value } = await reader.read(); if (done) break; size += value.length; if (size > limit) { await reader.cancel(); throw Error('Clunkyard response exceeds the client limit.'); } chunks.push(Buffer.from(value)); }
  let result; try { result = JSON.parse(Buffer.concat(chunks).toString('utf8')); } catch { throw Error('Clunkyard returned an invalid response.'); }
  if (!response.ok) throw Object.assign(Error(typeof result.error === 'string' ? result.error.slice(0, 240) : `Clunkyard request failed (${response.status}).`), { status:response.status });
  return result;
}
async function request(site, route, { method = 'GET', body, operationId, key = keyFor(site) } = {}) {
  const response = await fetch(site + route, { method, redirect:'error', signal:AbortSignal.timeout(45000), headers:{ Authorization:`Bearer ${key}`, Accept:'application/json', ...(body === undefined ? {} : { 'Content-Type':'application/json' }), ...(operationId ? { 'X-Clunkyard-Request':operationId } : {}) }, ...(body === undefined ? {} : { body:JSON.stringify(body) }) });
  return readJson(response);
}

module.exports = { credentialPath, origin, readCredentials, saveCredential, keyFor, request };
