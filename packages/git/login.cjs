#!/usr/bin/env node
'use strict';

const { origin, saveCredential, request } = require('./client.cjs');

async function secret() {
  if (!process.stdin.isTTY) {
    let input = ''; for await (const data of process.stdin) { input += data; if (input.length > 2048) throw Error('Access key is too long.'); }
    return input.trim();
  }
  process.stderr.write('Agent access key (hidden): ');
  process.stdin.setRawMode(true); process.stdin.resume(); process.stdin.setEncoding('utf8');
  return new Promise((resolve, reject) => {
    let value = '';
    const finish = error => { process.stdin.removeListener('data', receive); process.stdin.setRawMode(false); process.stdin.pause(); process.stderr.write('\n'); error ? reject(error) : resolve(value); };
    function receive(data) { for (const character of data) { if (character === '\u0003') return finish(Error('Sign-in cancelled.')); if (character === '\r' || character === '\n') return finish(); if (character === '\u007f' || character === '\b') value = value.slice(0, -1); else value += character; if (value.length > 1024) return finish(Error('Access key is too long.')); } }
    process.stdin.on('data', receive);
  });
}
async function main() {
  const args = process.argv.slice(2);
  if (args.shift() !== 'login') throw Error('Usage: clunkyard-git login [--origin https://clunkyard.com] [--token-stdin]');
  let site = 'https://clunkyard.com';
  while (args.length) { const flag = args.shift(); if (flag === '--origin') site = args.shift(); else if (flag !== '--token-stdin') throw Error('Unknown sign-in option.'); }
  site = origin(site); const key = await secret();
  const me = await request(site, '/api/me', { key });
  if (!me.account?.handle) throw Error('This access key does not identify a player.');
  const file = saveCredential(site, key);
  process.stdout.write(`Signed in as @${me.account.handle}. Credentials saved at ${file}.\n`);
}
main().catch(error => { console.error(error.message); process.exitCode = 1; });
