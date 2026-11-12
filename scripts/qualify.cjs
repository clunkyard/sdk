#!/usr/bin/env node
'use strict';
const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto'),{spawnSync}=require('node:child_process');
const root=path.resolve(__dirname,'..'),manifest=JSON.parse(fs.readFileSync(path.join(root,'dist/packages.json'))),hash=data=>crypto.createHash('sha256').update(data).digest('hex');
for(const value of manifest.packages){
  const file=path.join(root,'dist',value.filename),data=fs.readFileSync(file);
  if(data.length!==value.bytes||hash(data)!==value.sha256)throw Error('Packed content changed before qualification.');
  let args;
  let project;
  if(value.name==='@clunkyard/git')args=[path.join(root,'scripts/smoke-git.cjs'),file];
  else if(value.name==='@clunkyard/cli')args=[path.join(root,'scripts/smoke-cli.cjs'),file];
  else{
    if(!process.env.TMPDIR||!path.isAbsolute(process.env.TMPDIR))throw Error('Set TMPDIR to an absolute temporary directory.');
    project=fs.mkdtempSync(path.join(process.env.TMPDIR,'npm-sdk-consumer-'));
    fs.writeFileSync(path.join(project,'package.json'),JSON.stringify({name:'sdk-consumer',private:true,dependencies:{'@clunkyard/sdk':'file:'+file.replace(/\\/g,'/'),clunkyard:'file:'+file.replace(/\\/g,'/')}}));
    const install=spawnSync(process.execPath,[process.env.npm_execpath,'install','--ignore-scripts','--no-audit','--no-fund','--package-lock=false','--offline'],{cwd:project,env:process.env,stdio:'inherit'});
    if(install.status!==0)process.exit(install.status||1);
    args=[path.join(root,'scripts/sdk/smoke.cjs'),project,path.join(root,'protocol/v2/fixtures'),require.resolve('typescript/bin/tsc'),value.version];
  }
  const child=spawnSync(process.execPath,args,{cwd:root,env:process.env,stdio:'inherit'});
  if(project)fs.rmSync(project,{recursive:true,force:true});
  if(child.status!==0)process.exit(child.status||1);
}
fs.writeFileSync(path.join(root,'dist/qualification.json'),JSON.stringify({node:process.version,platform:process.platform,arch:process.arch,sourceCommit:manifest.sourceCommit,packages:manifest.packages.map(({name,version,sha256})=>({name,version,sha256})),passed:true},null,2)+'\n');
