#!/usr/bin/env node
'use strict';

// Qualify the actual npm tarball in a fresh user-owned prefix and project.
const assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path');
const {spawnSync}=require('node:child_process');

const tarball=process.argv[2];
if(!tarball||!process.env.npm_execpath||!process.env.TMPDIR)throw new Error('Use npm exec -- node smoke-cli.cjs PACKAGE.tgz with an absolute TMPDIR.');
const root=fs.mkdtempSync(path.join(process.env.TMPDIR,'npm-cli-consumer-'));
const prefix=path.join(root,'npm prefix'),project=path.join(root,'robot project');
const config=path.join(root,'empty.npmrc');fs.writeFileSync(config,'');
const env={...process.env,NPM_CONFIG_USERCONFIG:config,NPM_CONFIG_CACHE:path.join(root,'npm-cache'),NPM_CONFIG_REGISTRY:'https://registry.npmjs.org'};
delete env.NODE_PATH;
function run(command,args,options={}) {
  const result=spawnSync(command,args,{cwd:root,env,encoding:'utf8',timeout:240000,maxBuffer:2*1024*1024,...options});
  if(result.error)throw result.error;
  assert.equal(result.status,0,result.stderr||result.stdout);
  return result.stdout.trim();
}
try {
  run(process.execPath,[process.env.npm_execpath,'install','--global','--prefix',prefix,'--ignore-scripts','--no-audit','--no-fund',path.resolve(tarball)]);
  const directory=path.join(prefix,...(process.platform==='win32'?[]:['lib']),'node_modules','@clunkyard','cli');
  const entry=path.join(directory,'cli.cjs');
  const bin=path.join(prefix,...(process.platform==='win32'?[]:['bin']),'clunkyard'+(process.platform==='win32'?'.cmd':''));
  assert.ok(fs.existsSync(bin),'npm did not install the CLI command shim.');
  if(process.platform==='win32')assert.equal(run(process.env.ComSpec||'cmd.exe',['/d','/s','/c',`""${bin}" --version"`],{windowsVerbatimArguments:true}),'2.1.0');
  else assert.equal(run(bin,['--version']),'2.1.0');
  assert.equal(run(process.execPath,[entry,'--version']),'2.1.0');
  run(process.execPath,[entry,'new','javascript',project]);
  for(const file of ['clunkyard.cjs','clunkyard-archive.cjs','clunkyard.lock.json','clunkyard-template.json'])assert.equal(fs.existsSync(path.join(project,file)),false,'Copied tooling: '+file);
  run(process.execPath,[entry,'doctor','--project',project]);
  run(process.execPath,[entry,'--project',project,'match','42','2d','classic']);
  const sdk=require(path.join(project,'node_modules','clunkyard','package.json'));
  assert.equal(sdk.name,'@clunkyard/sdk');assert.equal(sdk.version,'2.0.0');
  const replay=JSON.parse(fs.readFileSync(path.join(project,'replays','trial-42.json')));
  assert.deepEqual(replay.failures||[],[]);
  assert.ok(replay.stats.bots[0].combat.shots>0,'Starter never fired.');
  const launch=JSON.parse(fs.readFileSync(path.join(project,'.clunkyard','local-bot.json'))).command;
  assert.equal(launch[1],entry);assert.deepEqual(launch.slice(2),['--project',project,'stdio']);
  // A repeated build reuses the verified engine and installed npm SDK.
  // Both asset reads and an unexpected npm reinstall fail in this check.
  const offline=path.join(root,'offline.cjs');fs.writeFileSync(offline,"global.fetch=()=>{throw Error('Unexpected asset download')};const cp=require('node:child_process'),spawn=cp.spawnSync;cp.spawnSync=(cmd,args,opts)=>{if(args.includes('ci'))throw Error('Unexpected npm reinstall');return spawn(cmd,args,opts)}\n");
  const emptyAssets=path.join(root,'empty-assets');fs.mkdirSync(emptyAssets);
  run(process.execPath,['--require',offline,entry,'build','--project',project],{env:{...env,CLUNKYARD_RELEASE_DIRECTORY:emptyAssets}});
  const typescript=path.join(root,'nested typescript project');
  run(process.execPath,[entry,'new','typescript',typescript]);
  fs.mkdirSync(path.join(typescript,'src'));fs.renameSync(path.join(typescript,'bot.ts'),path.join(typescript,'src','bot.ts'));
  const manifestFile=path.join(typescript,'bot.json'),manifest=JSON.parse(fs.readFileSync(manifestFile));manifest.entry='src/bot.ts';fs.writeFileSync(manifestFile,JSON.stringify(manifest));
  const packageFile=path.join(typescript,'package.json'),metadata=JSON.parse(fs.readFileSync(packageFile));metadata.type='module';fs.writeFileSync(packageFile,JSON.stringify(metadata));
  // The player supplies their own compiler. Install this test's prerequisite
  // explicitly and verify that the CLI prefers a project-local TypeScript.
  run(process.execPath,[process.env.npm_execpath,'install','--save-dev','--save-exact','typescript@6.0.2','--ignore-scripts','--no-audit','--no-fund'],{cwd:typescript});
  run(process.execPath,[entry,'doctor','--project',typescript]);
  run(process.execPath,[entry,'match','77','3d','zero-g','--project',typescript]);
  const tsReplay=JSON.parse(fs.readFileSync(path.join(typescript,'replays','trial-77.json')));
  assert.deepEqual(tsReplay.failures||[],[]);assert.ok(tsReplay.stats.bots[0].combat.shots>0,'TypeScript starter never fired.');
  console.log(JSON.stringify({installedCommand:true,projectWithSpaces:true,explicitProject:true,realMatch:true,npmSDK:true,cacheReuse:true,nestedTypeScript:true,typeModuleProject:true}));
} finally {fs.rmSync(root,{recursive:true,force:true});}
