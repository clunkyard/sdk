#!/usr/bin/env node
'use strict';

const fs=require('node:fs'),path=require('node:path');
const {pathToFileURL}=require('node:url');
const {createRuntime}=require('./runtime.cjs');
const {createProject,migrate}=require('./project.cjs');
const catalogue=require('./catalogue.json');
const {version}=require('./package.json');

function parse(args) {
  let project=process.cwd();const positional=[];
  for(let i=0;i<args.length;i++){
    if(args[i]==='--project'){
      if(!args[i+1]||args[i+1].startsWith('--'))throw new Error('--project needs a directory.');
      project=path.resolve(args[++i]);
    }else if(args[i].startsWith('--project=')){
      if(!args[i].slice(10))throw new Error('--project needs a directory.');
      project=path.resolve(args[i].slice(10));
    }else{positional.push(args[i]);if(positional[0]==='connect'){positional.push(...args.slice(i+1));break;}}
  }
  return {project,action:positional.shift()||'help',args:positional};
}
async function main(argv=process.argv.slice(2)) {
  const {project,action,args}=parse(argv);
  if(action==='--version'||action==='version'){console.log(version);return;}
  if(['help','--help','-h'].includes(action)){
    console.log('Clunkyard local robot CLI\n\nclunkyard [--project DIRECTORY] doctor|setup|build|match|replay|clean|migrate\nclunkyard new LANGUAGE DIRECTORY\nclunkyard connect [COMMAND [ARGS...]]\nclunkyard connect --strategist COMMAND [ARGS...]\nclunkyard list\n\nmatch [seed] [2d|3d] [ruleset]; replay [file] [port]');return;
  }
  if(action==='list'){
    for(const [language,row] of Object.entries(catalogue.languages))console.log(`${language.padEnd(22)} ${platformLabel(language,row)}  https://github.com/clunkyard/starter-${language}`);
    return;
  }
  if(action==='connect'){
    if(args[0]==='--help'){console.log('clunkyard connect [COMMAND [ARGS...]]\nclunkyard connect --strategist COMMAND [ARGS...]\n\nUse Node 22+ and set CLUNKYARD_REMOTE_URL, CLUNKYARD_REMOTE_TOKEN and CLUNKYARD_BOT_ID from Connections. Robot commands run without a shell in this directory.');return;}
    if(typeof globalThis.WebSocket!=='function')throw new Error('Remote connections need Node 22+ with its built-in WebSocket client. Other CLI commands support Node 20+.');
    if(args[0]==='--strategist'&&args.length===1)throw new Error('Pass the strategist command after --strategist.');
    if(!args.length){
      if(!process.env.CLUNKYARD_REMOTE_URL||!process.env.CLUNKYARD_REMOTE_TOKEN||!process.env.CLUNKYARD_BOT_ID)throw new Error('Set CLUNKYARD_REMOTE_URL, CLUNKYARD_REMOTE_TOKEN and CLUNKYARD_BOT_ID from Connections, then run connect from your robot project.');
      // Build subprocesses must not receive hub credentials either. Restore
      // the transport environment only after the SDK-aware build completes.
      const transport=Object.fromEntries(Object.entries(process.env).filter(([key])=>key.toUpperCase().startsWith('CLUNKYARD_REMOTE_')||key.toUpperCase()==='CLUNKYARD_BOT_ID'));
      for(const key of Object.keys(transport))delete process.env[key];
      try{await createRuntime(project,__filename).main('build');}finally{Object.assign(process.env,transport);}
      args.push(process.execPath,__filename,'--project',project,'stdio');
      if(process.env.CLUNKYARD_SDK)process.env.CLUNKYARD_BOT_ENV_ALLOW=[process.env.CLUNKYARD_BOT_ENV_ALLOW,'CLUNKYARD_SDK'].filter(Boolean).join(',');
    }
    const bundled=path.join(__dirname,'remote-bridge.mjs'),bridge=fs.existsSync(bundled)?bundled:path.join(__dirname,'../../sdk/remote-bridge.mjs');
    process.chdir(project);process.argv=[process.execPath,bridge,...args];await import(pathToFileURL(bridge).href);return;
  }
  if(action==='new'){
    if(args.length!==2)throw new Error('Use clunkyard new LANGUAGE DIRECTORY.');
    const destination=await createProject(args[0],path.resolve(project,args[1]));
    console.log(`Created ${destination}. Run npx @clunkyard/cli doctor, build and match there.`);return;
  }
  if(action==='migrate'){
    migrate(project);console.log('Updated generated tooling. Robot source files are unchanged.');return;
  }
  await createRuntime(project,__filename).main(action,args);
}

function platformLabel(language,row){
  if(row.portable||['javascript','typescript'].includes(language))return 'Linux / macOS / Windows (Node)';
  const names={'linux-x64':'Linux x86-64','linux-arm64':'Linux AArch64','darwin-arm64':'macOS Apple Silicon','darwin-x64':'macOS Intel','win32-x64':'Windows x86-64'};
  return [...new Set(['linux-x64',...Object.keys(row.sdkPlatforms||{})])].map(key=>names[key]||key).join(' / ');
}

if(require.main===module)main().catch(error=>{console.error(error.message);process.exitCode=1});
module.exports={main,parse,platformLabel};
