#!/usr/bin/env node
'use strict';

// Exercise the same package-scoped exchange as npm, without publishing bytes.
const fs = require('node:fs');
const path = require('node:path');

async function main() {
  const env = process.env;
  require('./publish.cjs').assertTrustedWorkflow(env);
  const requestURL = new URL(env.ACTIONS_ID_TOKEN_REQUEST_URL);
  if (requestURL.protocol !== 'https:') throw Error('GitHub OIDC requires HTTPS.');
  requestURL.searchParams.set('audience', 'npm:registry.npmjs.org');
  const identity = await fetch(requestURL, {
    headers: { Authorization: `Bearer ${env.ACTIONS_ID_TOKEN_REQUEST_TOKEN}` },
    redirect: 'error', signal: AbortSignal.timeout(30000),
  });
  if (!identity.ok) throw Error(`GitHub OIDC request failed: HTTP ${identity.status}`);
  const { value: idToken } = await identity.json();
  if (typeof idToken !== 'string' || !idToken) throw Error('GitHub returned no OIDC token.');

  const packages = [];
  for (const name of ['@clunkyard/git', '@clunkyard/sdk', '@clunkyard/cli']) {
    const response = await fetch(`https://registry.npmjs.org/-/npm/v1/oidc/token/exchange/package/${name.replace('/', '%2f')}`, {
      method: 'POST', headers: { Authorization: `Bearer ${idToken}` },
      redirect: 'error', signal: AbortSignal.timeout(30000),
    });
    if (!response.ok) throw Error(`${name}: npm OIDC exchange failed: HTTP ${response.status}`);
    const body = await response.json();
    if (typeof body.token !== 'string' || !body.token) throw Error(`${name}: npm returned no package token.`);
    // Neither credential is logged, persisted or used to mutate the registry.
    packages.push({ name, authenticated: true });
    console.log(`${name}: npm accepted the GitHub OIDC identity.`);
  }
  const receipt = {
    v: 1, repository: env.GITHUB_REPOSITORY, commit: env.GITHUB_SHA,
    run: `https://github.com/${env.GITHUB_REPOSITORY}/actions/runs/${env.GITHUB_RUN_ID}`,
    checkedAt: new Date().toISOString(), packages, staged: false, published: false,
  };
  const output = path.resolve('dist/oidc-verification.json');
  fs.mkdirSync(path.dirname(output), { recursive: true });
  fs.writeFileSync(output, JSON.stringify(receipt, null, 2) + '\n');
}

main().catch(error => { console.error(error.message); process.exitCode = 1; });
