'use strict';
// Browser sign-in for `npx @clunkyard/cli login` and `open`: the browser
// grants this computer an agent key.

const http = require('node:http');
const os = require('node:os');
const crypto = require('node:crypto');
const { spawn } = require('node:child_process');

function openBrowser(url) {
  const [command, args] = process.platform === 'darwin' ? ['open', [url]] : process.platform === 'win32' ? ['rundll32', ['url.dll,FileProtocolHandler', url]] : ['xdg-open', [url]];
  try { const child = spawn(command, args, { stdio:'ignore', detached:true }); child.on('error', () => {}); child.unref(); } catch {}
}
// RFC 8252 loopback sign-in with PKCE: the browser brings a one-time code
// back to this process, which alone holds the verifier that redeems it.
async function browserKey(site, { timeoutMs = 5 * 60 * 1000, open = openBrowser, log = text => process.stderr.write(text + '\n') } = {}) {
  const verifier = crypto.randomBytes(32).toString('base64url'), state = crypto.randomBytes(16).toString('base64url');
  const challenge = crypto.createHash('sha256').update(verifier).digest('base64url');
  let finish; const done = new Promise((resolve, reject) => { finish = { resolve, reject }; });
  const server = http.createServer((req, res) => {
    const url = new URL(req.url, 'http://127.0.0.1');
    if (url.pathname !== '/callback') { res.writeHead(404).end(); return; }
    const ok = url.searchParams.get('state') === state && url.searchParams.get('code');
    res.writeHead(ok ? 200 : 400, { 'Content-Type':'text/html; charset=utf-8', 'Cache-Control':'no-store' });
    res.end(`<!doctype html><meta charset="utf-8"><title>Clunkyard</title><p>${ok ? 'Signed in. You can close this tab and return to your terminal.' : 'Sign-in was cancelled or did not match this command. Return to your terminal.'}</p>`);
    if (url.searchParams.get('state') !== state) return; // Not this sign-in; keep waiting.
    if (!ok) finish.reject(Error('Sign-in was cancelled in the browser.'));
    else finish.resolve(url.searchParams.get('code'));
  });
  await new Promise((resolve, reject) => { server.once('error', reject); server.listen(0, '127.0.0.1', resolve); });
  const redirect = `http://127.0.0.1:${server.address().port}/callback`;
  const target = new URL('/git/sign-in', site);
  for (const [name, value] of Object.entries({ redirect_uri:redirect, code_challenge:challenge, code_challenge_method:'S256', state, device:os.hostname() })) target.searchParams.set(name, value);
  log(`Opening your browser to sign in. If it does not open, visit:\n  ${target.href}\nOn a computer without a browser, press Ctrl-C, run npx @clunkyard/cli login --token-stdin and enter an agent key from your Account page.`);
  open(target.href);
  const timer = setTimeout(() => finish.reject(Error('Sign-in timed out. Run the command again.')), timeoutMs);
  try {
    const code = await done;
    const response = await fetch(new URL('/git/sign-in/token', site), { method:'POST', redirect:'error', signal:AbortSignal.timeout(45000), headers:{ 'Content-Type':'application/json', Accept:'application/json' }, body:JSON.stringify({ code, code_verifier:verifier, redirect_uri:redirect }) });
    const value = await response.json().catch(() => ({}));
    if (!response.ok || typeof value.key !== 'string') throw Error(typeof value.error === 'string' ? value.error.slice(0, 240) : response.status === 404 ? 'This server does not offer browser sign-in. Run clunkyard-git login --paste.' : `Sign-in failed (${response.status}).`);
    return value.key;
  } finally { clearTimeout(timer); server.close(); }
}

module.exports = { browserKey };
