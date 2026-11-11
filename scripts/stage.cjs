#!/usr/bin/env node
'use strict';

// This command stages reviewed bytes. It never approves or directly publishes.
const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto'),{execFileSync}=require('node:child_process');
const root=path.resolve(__dirname,'..'),dist=path.join(root,'dist'),manifest=JSON.parse(fs.readFileSync(path.join(dist,'packages.json'))),qualification=JSON.parse(fs.readFileSync(path.join(dist,'qualification.json')));
if(process.env.GITHUB_ACTIONS!=='true'||process.env.GITHUB_REPOSITORY!=='clunkyard/sdk'||process.env.GITHUB_REF!=='refs/heads/main'||!process.env.ACTIONS_ID_TOKEN_REQUEST_URL)throw Error('Stage from the reviewed public main workflow using npm trusted publishing.');
if(!qualification.passed||qualification.sourceCommit!==manifest.sourceCommit||JSON.stringify(qualification.packages)!==JSON.stringify(manifest.packages.map(({name,version,sha256})=>({name,version,sha256}))))throw Error('Qualification receipt does not match the packed artifacts.');
const selectedIndex=process.argv.indexOf('--package'),selection=selectedIndex<0?process.env.NPM_STAGE_PACKAGES:process.argv[selectedIndex+1],selected=selection?selection.split(','):null;
if((selectedIndex>=0&&!selected)||(selected&&selected.some(name=>!manifest.packages.some(value=>value.name===name))))throw Error('Select known package names with --package NAME[,NAME].');
for(const value of manifest.packages){
  if(!['@clunkyard/git','@clunkyard/sdk','@clunkyard/cli'].includes(value.name)||!/^\d+\.\d+\.\d+$/.test(value.version))throw Error('Unexpected package identity.');
  if(path.basename(value.filename)!==value.filename)throw Error('Unexpected package archive path.');
  const file=path.join(dist,value.filename),bytes=fs.readFileSync(file);
  if(bytes.length!==value.bytes||crypto.createHash('sha256').update(bytes).digest('hex')!==value.sha256)throw Error('Package changed after qualification.');
}
async function main(){
  for(const value of manifest.packages.filter(value=>!selected||selected.includes(value.name))){
    const response=await fetch(`https://registry.npmjs.org/${encodeURIComponent(value.name)}/${value.version}`,{redirect:'error',signal:AbortSignal.timeout(30000)});
    if(response.ok){
      const published=await response.json();
      if(published.dist?.integrity!==value.integrity)throw Error(`Published bytes differ for ${value.name}@${value.version}; choose a new version.`);
      console.log(`Already published with reviewed integrity: ${value.name}@${value.version}`);
      continue;
    }
    if(response.status!==404)throw Error(`Cannot check npm version: HTTP ${response.status}`);
    const args=[process.env.npm_execpath,'stage','publish',path.join(dist,value.filename),'--access','public','--tag','latest','--provenance','--ignore-scripts','--registry=https://registry.npmjs.org/'];
    if(!process.argv.includes('--apply'))console.log(`Ready to stage ${value.name}@${value.version} (${value.sha256})`);
    else execFileSync(process.execPath,args,{cwd:root,env:process.env,stdio:'inherit'});
  }
}
main().catch(error=>{console.error(error.message);process.exitCode=1});
