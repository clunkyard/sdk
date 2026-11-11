#!/usr/bin/env node
'use strict';

// Runs only in the reviewed public source export. No game service, credentials
// or private application source is needed. Installed toolchains remain external.
const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto'),os=require('node:os');
const {execFileSync}=require('node:child_process');
const {coreNotices,writeNotices}=require(fs.existsSync(path.join(__dirname,'core-notices.cjs'))?'./core-notices.cjs':'../core-notices.cjs');
const hash=data=>crypto.createHash('sha256').update(data).digest('hex');
const shellQuote=value=>"'"+String(value).replaceAll("'","'\\''")+"'";
function verifyExport(root){
  const file=path.join(root,'release-source.json'),bytes=fs.readFileSync(file),manifest=JSON.parse(bytes);
  if(manifest.sourceDirty!==false||!Array.isArray(manifest.files)||!manifest.files.length||!/^[a-f0-9]{40}$/.test(manifest.sourceCommit))throw Error('Native qualification needs a clean canonical source export.');
  for(const entry of manifest.files){
    if(!entry.path||entry.path.includes('\\')||entry.path.startsWith('/')||entry.path.split('/').some(part=>part==='..'||part==='.'||!part))throw Error('Unsafe export manifest path.');
    const data=fs.readFileSync(path.join(root,entry.path));if(data.length!==entry.bytes||hash(data)!==entry.sha256)throw Error('Export content changed: '+entry.path);
  }
  return {sourceCommit:manifest.sourceCommit,manifestSHA256:hash(bytes)};
}
function main(){
  const option=name=>{const i=process.argv.indexOf(name);return i<0?null:process.argv[i+1]};
  const root=path.resolve(__dirname,'..'),expected=option('--expected-platform'),output=option('--out');
  if(!expected||!output)throw Error('Use --expected-platform OS-ARCH --out NEW_DIRECTORY.');
  const platform=process.platform+'-'+process.arch;if(platform!==expected)throw Error(`Expected ${expected}, got ${platform}.`);
  const provenance=verifyExport(root),directory=path.resolve(output),artifacts=path.join(directory,'artifacts');
  if(fs.existsSync(directory))throw Error('Choose a new qualification output directory.');
  fs.mkdirSync(artifacts,{recursive:true});
  const receipt={v:1,checkedAt:new Date().toISOString(),platform,host:{os:os.release(),node:process.version},...provenance,publicCommit:execFileSync('git',['rev-parse','HEAD'],{cwd:root,encoding:'utf8'}).trim(),results:[]};
  const save=()=>fs.writeFileSync(path.join(artifacts,'native-qualification.json'),JSON.stringify(receipt,null,2)+'\n');save();
  const environment={...process.env};for(const key of Object.keys(environment))if(/^(?:CLUNKYARD_|GIT_CONFIG|GIT_DIR|GIT_WORK_TREE|NPM_CONFIG_(?:USERCONFIG|CACHE))/i.test(key))delete environment[key];
  const config=path.join(directory,'empty-config');fs.writeFileSync(config,'');Object.assign(environment,{GIT_CONFIG_GLOBAL:config,GIT_CONFIG_NOSYSTEM:'1',GIT_TERMINAL_PROMPT:'0',NPM_CONFIG_USERCONFIG:config,NPM_CONFIG_CACHE:path.join(directory,'npm-cache')});
  function run(command,args,{cwd=root,log}={}){
    const options={cwd,env:environment,encoding:'utf8',timeout:300000};let fd;
    if(log){fd=fs.openSync(log,'a');options.stdio=['ignore',fd,fd];}
    try{return execFileSync(command,args,options)?.trim();}finally{if(fd!==undefined)fs.closeSync(fd);}
  }
  try{
    const packages=JSON.parse(fs.readFileSync(path.join(root,'dist/packages.json'))),candidate=packages.packages.find(row=>row.name==='@clunkyard/cli');
    if(!candidate||packages.sourceCommit!==provenance.sourceCommit)throw Error('Pack the candidate CLI from this exact export first.');
    const archive=path.join(root,'dist',candidate.filename);if(hash(fs.readFileSync(archive))!==candidate.sha256)throw Error('Candidate CLI archive changed.');
    const prefix=path.join(directory,'installed CLI');run(process.execPath,[process.env.npm_execpath||findNpm(),'install','--prefix',prefix,'--ignore-scripts','--no-audit','--no-fund','--registry=https://registry.npmjs.org',archive],{log:path.join(artifacts,'install.log')});
    const cli=path.join(prefix,'node_modules/@clunkyard/cli/cli.cjs');receipt.cli={version:candidate.version,sha256:candidate.sha256};
    receipt.tools={cc:run(process.env.CC||'cc',['--version']).split('\n')[0],cpp:run(process.env.CXX||'c++',['--version']).split('\n')[0],python:run('python3',['--version']),rust:run('rustc',['--version'])};
    const sdk=path.join(directory,'native SDK'),sdkLog=path.join(artifacts,'sdk-build.log');
    run(process.execPath,[path.join(root,'scripts/player-kit/build-native.cjs'),'--out',sdk,'--build-dir',path.join(directory,'core-build'),'--bindings','c,cpp,python,rust'],{log:sdkLog});
    const metadataFile=path.join(sdk,'platform.json'),metadata=JSON.parse(fs.readFileSync(metadataFile));
    if(metadata.platform!==platform||metadata.sourceCommit!==receipt.publicCommit||metadata.sourceDirty!==false)throw Error('SDK helper produced another source or platform.');
    // The full export manifest was verified before compiling. Preserve actual
    // public checkout identity while binding its bytes to canonical inputs.
    Object.assign(metadata,{sourceCommit:provenance.sourceCommit,publicCommit:receipt.publicCommit,exportManifestSHA256:provenance.manifestSHA256});
    fs.writeFileSync(metadataFile,JSON.stringify(metadata,null,2)+'\n');
    writeNotices(sdk,coreNotices(root,{title:'Third-party notices for the Clunkyard native SDK',description:'The native core is built from the public sdk/core source.',target:'native',rustVersion:metadata.rustVersion}));
    const {archive:packArchive}=require(path.join(prefix,'node_modules/@clunkyard/cli/archive.cjs'));
    const sdkArchive=packArchive(sdk),sdkName='native-sdk-'+platform+'.tar.gz';fs.writeFileSync(path.join(artifacts,sdkName),sdkArchive);receipt.sdk={file:sdkName,sha256:hash(sdkArchive),bytes:sdkArchive.length,metadata};
    environment.CLUNKYARD_SDK=sdk;
    const invocation=shellQuote(process.execPath)+' '+shellQuote(cli);
    for(const language of ['c','cpp','python','rust']){
      const project=path.join(directory,'fresh robot projects',language),log=path.join(artifacts,language+'.log');fs.mkdirSync(path.dirname(project),{recursive:true});
      run('git',['-c','credential.helper=','clone','--depth','1','https://github.com/clunkyard/starter-'+language+'.git',project],{log});
      const sourceCommit=run('git',['rev-parse','HEAD'],{cwd:project});
      run('make',['CLUNKYARD='+invocation,'doctor'],{cwd:project,log});
      for(const [seed,mode,ruleset] of [[42,'2d','classic'],[77,'3d','zero-g']]){
        run(process.execPath,[cli,'--project',project,'match',String(seed),mode,ruleset],{cwd:project,log});
        const replay=JSON.parse(fs.readFileSync(path.join(project,'replays','trial-'+seed+'.json'))),bots=replay.stats.bots;
        if(replay.failures?.length||bots.some(bot=>bot.reliability.failures)||!bots[0].combat.shots)throw Error(`The ${language} starter failed or did not fire.`);
        fs.copyFileSync(path.join(project,'replays','trial-'+seed+'.json'),path.join(artifacts,language+'-'+seed+'.json'));
        receipt.results.push({language,sourceCommit,seed,mode,ruleset,passed:true,shots:bots[0].combat.shots,hits:bots[0].combat.hits,failures:0});save();
      }
      if(run('git',['status','--porcelain'],{cwd:project}))throw Error('Qualification changed player source files.');
    }
    receipt.complete=true;save();console.log(JSON.stringify({platform,sourceCommit:receipt.sourceCommit,publicCommit:receipt.publicCommit,matches:receipt.results.length,artifacts},null,2));
  }catch(error){receipt.complete=false;receipt.error=String(error.message);save();throw error;}
}
function findNpm(){
  const candidates=[path.join(path.dirname(process.execPath),'node_modules/npm/bin/npm-cli.js'),path.resolve(path.dirname(process.execPath),'../lib/node_modules/npm/bin/npm-cli.js'),path.resolve(path.dirname(process.execPath),'../share/nodejs/npm/bin/npm-cli.js')];
  const file=candidates.find(file=>fs.existsSync(file));if(!file)throw Error('Use Node with npm installed.');return file;
}
if(require.main===module)try{main()}catch(error){console.error(error.message);process.exitCode=1}
module.exports={verifyExport};
