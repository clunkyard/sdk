#!/usr/bin/env node
'use strict';

// This command stages reviewed bytes. It never approves or directly publishes.
const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto'),{execFileSync}=require('node:child_process');
const root=path.resolve(__dirname,'..'),dist=path.join(root,'dist'),manifest=JSON.parse(fs.readFileSync(path.join(dist,'packages.json'))),qualification=JSON.parse(fs.readFileSync(path.join(dist,'qualification.json')));
if(process.env.GITHUB_ACTIONS!=='true'||process.env.GITHUB_REPOSITORY!=='clunkyard/sdk'||process.env.GITHUB_REF!=='refs/heads/main'||!process.env.ACTIONS_ID_TOKEN_REQUEST_URL)throw Error('Stage from the reviewed public main workflow using npm trusted publishing.');
if(!qualification.passed||qualification.sourceCommit!==manifest.sourceCommit||JSON.stringify(qualification.packages)!==JSON.stringify(manifest.packages.map(({name,version,sha256})=>({name,version,sha256}))))throw Error('Qualification receipt does not match the packed artifacts.');
for(const value of manifest.packages){
  if(!['@clunkyard/git','@clunkyard/sdk'].includes(value.name)||!/^\d+\.\d+\.\d+$/.test(value.version))throw Error('Unexpected package identity.');
  if(path.basename(value.filename)!==value.filename)throw Error('Unexpected package archive path.');
  const file=path.join(dist,value.filename),bytes=fs.readFileSync(file);
  if(bytes.length!==value.bytes||crypto.createHash('sha256').update(bytes).digest('hex')!==value.sha256)throw Error('Package changed after qualification.');
  const args=[process.env.npm_execpath,'stage','publish',file,'--access','public','--tag','latest','--provenance','--ignore-scripts','--registry=https://registry.npmjs.org/'];
  if(!process.argv.includes('--apply'))console.log(`Ready to stage ${value.name}@${value.version} (${value.sha256})`);
  else execFileSync(process.execPath,args,{cwd:root,env:process.env,stdio:'inherit'});
}
