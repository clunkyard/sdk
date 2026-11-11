#!/usr/bin/env node
'use strict';

// Public-registry acceptance uses the player's normal compiler/interpreter.
// No SDK override, local release directory, registry key or service is used.
const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto');
const {execFileSync}=require('node:child_process');
const hash=bytes=>crypto.createHash('sha256').update(bytes).digest('hex');
function parameters(argv,environment){
  const option=name=>{const i=argv.indexOf(name);return i<0?null:argv[i+1]},version=String(environment.NATIVE_CLI_VERSION||'');
  const platform=option('--expected-platform'),output=option('--out'),languages=(option('--languages')||'').split(',');
  if(!/^\d+\.\d+\.\d+$/.test(version)||!['linux-x64','darwin-x64','darwin-arm64','win32-x64'].includes(platform)||!output||!languages.length||languages.some(language=>!['c','cpp','python','rust','javascript','csharp'].includes(language))||new Set(languages).size!==languages.length)throw Error('Use an exact public CLI version, supported platform, language list and new output directory.');
  return {version,platform,output,languages};
}
function anonymousEnvironment(base,directory){
  const environment={...base};for(const key of Object.keys(environment))if(/^CLUNKYARD_/i.test(key)||/^(?:GH_TOKEN|GITHUB_TOKEN|NODE_AUTH_TOKEN|NPM_TOKEN|NPM_CONFIG_.*|GIT_CONFIG.*|GIT_DIR|GIT_WORK_TREE)$/i.test(key))delete environment[key];
  const config=path.join(directory,'empty-config'),globalConfig=path.join(directory,'empty-global-config');fs.writeFileSync(config,'');fs.writeFileSync(globalConfig,'');
  return {...environment,NPM_CONFIG_USERCONFIG:config,NPM_CONFIG_GLOBALCONFIG:globalConfig,NPM_CONFIG_CACHE:path.join(directory,'npm-cache'),GIT_CONFIG_GLOBAL:config,GIT_CONFIG_NOSYSTEM:'1',GIT_TERMINAL_PROMPT:'0'};
}
function npmCli(){
  return [process.env.npm_execpath,path.join(path.dirname(process.execPath),'node_modules/npm/bin/npm-cli.js'),path.resolve(path.dirname(process.execPath),'../lib/node_modules/npm/bin/npm-cli.js'),path.resolve(path.dirname(process.execPath),'../share/nodejs/npm/bin/npm-cli.js')].find(file=>file&&/\.[cm]?js$/.test(file)&&fs.existsSync(file));
}
function main(){
  const input=parameters(process.argv.slice(2),process.env),platform=process.platform+'-'+process.arch;if(platform!==input.platform)throw Error(`Expected ${input.platform}, got ${platform}.`);
  const directory=path.resolve(input.output);if(fs.existsSync(directory))throw Error('Choose a new output directory.');const artifacts=path.join(directory,'artifacts');fs.mkdirSync(artifacts,{recursive:true});
  const environment=anonymousEnvironment(process.env,directory),receipt={v:1,checkedAt:new Date().toISOString(),platform,node:process.version,package:'@clunkyard/cli',version:input.version,anonymous:true,freshCache:true,sdkOverride:false,localReleaseOverride:false,results:[],complete:false};
  const save=()=>fs.writeFileSync(path.join(artifacts,'registry-qualification.json'),JSON.stringify(receipt,null,2)+'\n');save();
  function run(command,args,{cwd=directory,log}={}){let fd;const options={cwd,env:environment,encoding:'utf8',timeout:300000};if(log){fd=fs.openSync(log,'a');options.stdio=['ignore',fd,fd];}try{return execFileSync(command,args,options)?.trim()}finally{if(fd!==undefined)fs.closeSync(fd)}}
  try{
    const npm=npmCli();if(!npm)throw Error('Use Node with npm installed.');
    const prefix=path.join(directory,'installed CLI'),spec='@clunkyard/cli@'+input.version;
    receipt.registry=JSON.parse(run(process.execPath,[npm,'view',spec,'dist','--json','--registry=https://registry.npmjs.org']));
    const downloads=path.join(directory,'download');fs.mkdirSync(downloads);
    const packed=JSON.parse(run(process.execPath,[npm,'pack',spec,'--ignore-scripts','--json','--pack-destination',downloads,'--registry=https://registry.npmjs.org']))[0],tarball=path.join(downloads,packed.filename),bytes=fs.readFileSync(tarball);
    const actualIntegrity='sha512-'+crypto.createHash('sha512').update(bytes).digest('base64');if(actualIntegrity!==receipt.registry.integrity)throw Error('Registry download differs from its integrity.');
    receipt.download={bytes:bytes.length,sha256:hash(bytes),integrity:actualIntegrity};save();
    run(process.execPath,[npm,'install','--prefix',prefix,'--ignore-scripts','--no-audit','--no-fund','--registry=https://registry.npmjs.org',spec],{log:path.join(artifacts,'install.log')});
    const installed=path.join(prefix,'node_modules/@clunkyard/cli'),cli=path.join(installed,'cli.cjs'),metadata=JSON.parse(fs.readFileSync(path.join(installed,'package.json'))),catalogue=JSON.parse(fs.readFileSync(path.join(installed,'catalogue.json')));
    if(metadata.version!==input.version||metadata.name!=='@clunkyard/cli')throw Error('Installed another package version.');
    const {extract}=require(path.join(installed,'archive.cjs')),unpacked=path.join(directory,'reviewed-registry-bytes');fs.mkdirSync(unpacked);extract(bytes,unpacked);
    for(const file of ['cli.cjs','runtime.cjs','project.cjs','archive.cjs','package.json','catalogue.json','README.md','LICENSE'])if(!fs.readFileSync(path.join(installed,file)).equals(fs.readFileSync(path.join(unpacked,'package',file))))throw Error('Installed package differs from the registry archive: '+file);
    receipt.cliCatalogueSHA=hash(fs.readFileSync(path.join(installed,'catalogue.json')));receipt.engineSHA=catalogue.engine.sha256;
    for(const language of input.languages){
      const project=path.join(directory,'fresh robot projects',language),log=path.join(artifacts,language+'.log');fs.mkdirSync(path.dirname(project),{recursive:true});
      run('git',['-c','credential.helper=','clone','--depth','1','https://github.com/clunkyard/starter-'+language+'.git',project],{log});
      const sourceCommit=run('git',['rev-parse','HEAD'],{cwd:project}),manifest=fs.readFileSync(path.join(project,'bot.json'));
      run(process.execPath,[cli,'--project',project,'doctor'],{cwd:project,log});
      for(const [seed,mode,ruleset]of [[42,'2d','classic'],[77,'3d','zero-g']]){
        run(process.execPath,[cli,'--project',project,'match',String(seed),mode,ruleset],{cwd:project,log});
        const file=path.join(project,'replays','trial-'+seed+'.json'),replay=JSON.parse(fs.readFileSync(file)),bots=replay.stats.bots;
        if(replay.failures?.length||bots.some(bot=>bot.reliability.failures)||!bots[0].combat.shots)throw Error('Starter failed or never fired: '+language);
        fs.copyFileSync(file,path.join(artifacts,language+'-'+seed+'.json'));const row=catalogue.languages[language],asset=row.sdkPlatforms?.[platform]||row.sdk;
        receipt.results.push({language,sourceCommit,seed,mode,ruleset,shots:bots[0].combat.shots,hits:bots[0].combat.hits,failures:0,sdkSHA:asset.sha256,passed:true});save();
      }
      if(!manifest.equals(fs.readFileSync(path.join(project,'bot.json')))||run('git',['status','--porcelain'],{cwd:project}))throw Error('The public workflow changed player source: '+language);
    }
    receipt.sourcePreserved=true;receipt.complete=true;save();console.log(JSON.stringify({platform,version:input.version,matches:receipt.results.length,sha256:receipt.download.sha256,artifacts},null,2));
  }catch(error){receipt.error=error.message;save();throw error}
}
if(require.main===module)try{main()}catch(error){console.error(error.message);process.exitCode=1}
module.exports={parameters,anonymousEnvironment};
