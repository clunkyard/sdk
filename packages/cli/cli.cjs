#!/usr/bin/env node
'use strict';

const fs=require('node:fs'),path=require('node:path');
const {spawnSync}=require('node:child_process');
const {pathToFileURL}=require('node:url');
const {createRuntime}=require('./runtime.cjs');
const {createProject,migrate}=require('./project.cjs');
const catalogue=require('./catalogue.json');
const {version}=require('./package.json');

function parse(args) {
  let project=process.cwd(),sharedCache=null,docker;const positional=[];
  for(let i=0;i<args.length;i++){
    if(positional[0]!=='connect'&&['--docker','--no-docker'].includes(args[i]))docker=args[i]==='--docker';
    else if(args[i]==='--project'){
      if(!args[i+1]||args[i+1].startsWith('--'))throw new Error('--project needs a directory.');
      project=path.resolve(args[++i]);
    }else if(args[i].startsWith('--project=')){
      if(!args[i].slice(10))throw new Error('--project needs a directory.');
      project=path.resolve(args[i].slice(10));
    }else if(args[i]==='--shared-cache'&&args[i+1]){
      // Internal: a house opponent's runner shares the player's engine and SDK downloads.
      sharedCache=path.resolve(args[++i]);
    }else{positional.push(args[i]);if(positional[0]==='connect'){positional.push(...args.slice(i+1));break;}}
  }
  return {project,...(sharedCache?{sharedCache}:{}),...(docker===undefined?{}:{docker}),action:positional.shift()||'help',args:positional};
}

// Docker mode: build and match in the language's public toolchain image,
// which carries the hosted builder's toolchain and this CLI. --docker asks
// for it; without --no-docker it is also chosen when this computer lacks
// part of the robot's toolchain and Docker is running.
const DOCKER_ACTIONS={build:'build',match:'match',doctor:'check'};
const dockerImage=language=>`ghcr.io/clunkyard/toolchain/${language}`;
function dockerRunning(){return spawnSync('docker',['version','--format','{{.Server.Version}}'],{stdio:'ignore',timeout:10000}).status===0;}
function useDocker(project,action,docker){
  if(!DOCKER_ACTIONS[action]||docker===false)return null;
  const runtime=createRuntime(project,__filename),{language}=runtime.manifest();
  if(docker)return {language};
  const missing=runtime.diagnose().problems[0];
  if(!missing||action==='doctor'||!dockerRunning())return null;
  console.log(`This computer lacks part of the ${language} toolchain (${missing}), so the robot builds in ${dockerImage(language)} with Docker. Use --no-docker to build here.`);
  return {language};
}
// linux/amd64 is the images' supported platform; an Arm computer runs it
// under emulation.
function runInDocker(project,language,action,args){
  const interactive=process.stdin.isTTY?['-it']:[];
  const result=spawnSync('docker',['run','--rm','--platform','linux/amd64',...interactive,'-v',`${project}:/bot`,dockerImage(language),DOCKER_ACTIONS[action],...args],{stdio:'inherit'});
  if(result.error)throw new Error(`docker: ${result.error.message}. Install Docker, or use --no-docker with the language's toolchain installed.`);
  if(result.status!==0)throw new Error(`The ${language} toolchain image exited with status ${result.status}.`);
}
async function main(argv=process.argv.slice(2)) {
  const {project,sharedCache,docker,action,args}=parse(argv);
  if(action==='--version'||action==='version'){console.log(version);return;}
  if(['help','--help','-h'].includes(action)){
    console.log(`Clunkyard local robot CLI ${version}

Usage: npx @clunkyard/cli [--project DIRECTORY] COMMAND

  new LANGUAGE DIRECTORY   create a starter project (list shows the languages)
  doctor                   check this computer's tools for the robot
  build                    build the robot
  match [SEED] [2d|3d] [RULESET] [--opponent NAME]... [--refresh] [--json]
                           play a local match (default 42, 2d, classic; 3d uses zero-g)
                           add --drones on to let robots launch drones,
                           --objective hill|pickups for an objective
  opponents [--refresh]    list the house robots you can play locally
  drone list | add NAME    list the kit drones, or copy one into drones/
                           and name it in bot.json
  replay [FILE] [PORT]     watch a replay in your browser
  setup | clean | migrate  download, remove or update the shared tooling
  --docker | --no-docker   build, match and doctor in the language's toolchain
                           image; without --no-docker, chosen when a tool is missing
  connect [COMMAND [ARGS...]] | connect --strategist COMMAND [ARGS...]
                           connect this robot or a strategist to clunkyard.com
  hosted-build ROBOT_ID [--json]
                           wait for the hosted build of the robot's latest push
                           and print its diagnostics
  open ROBOT [DIRECTORY] [--in code|cursor|claude|codex]
                           sign in, clone a saved robot and open it in your tools
  login [--origin URL] [--token-stdin]
                           sign in through your browser, or with a key on
                           standard input, and save the key
  list                     list languages and their starter repositories
  --version`);return;
  }
  if(action==='list'){
    for(const [language,row] of Object.entries(catalogue.languages))console.log(`${language.padEnd(22)} ${platformLabel(language,row)}  https://github.com/clunkyard/starter-${language}`);
    return;
  }
  if(action==='connect'){
    if(args[0]==='--help'){console.log('Usage: npx @clunkyard/cli connect [COMMAND [ARGS...]]\n       npx @clunkyard/cli connect --strategist COMMAND [ARGS...]\n\nSet CLUNKYARD_REMOTE_TOKEN and CLUNKYARD_BOT_ID from Connections. CLUNKYARD_REMOTE_URL defaults to wss://clunkyard.com/api/remote-bots. Robot commands run without a shell in this directory.');return;}
    if(args[0]==='--strategist'&&args.length===1)throw new Error('Pass the strategist command after --strategist.');
    if(!args.length){
      if(!process.env.CLUNKYARD_REMOTE_TOKEN||!process.env.CLUNKYARD_BOT_ID)throw new Error('Set CLUNKYARD_REMOTE_TOKEN and CLUNKYARD_BOT_ID from Connections, then run connect from your robot project.');
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
  if(action==='hosted-build'){process.exitCode=await require('./hosted-build.cjs').hostedBuild(args);return;}
  if(action==='open'){process.exitCode=await require('./open.cjs').open(args);return;}
  if(action==='login'){
    const {siteOrigin,DEFAULT_ORIGIN}=require('./credentials.cjs'),index=args.indexOf('--origin'),origin=siteOrigin(index>=0?args[index+1]:args.find(arg=>arg.startsWith('--origin='))?.slice(9)||DEFAULT_ORIGIN);
    if(args.includes('--token-stdin')){const chunks=[];for await(const chunk of process.stdin)chunks.push(chunk);const key=Buffer.concat(chunks).toString('utf8').trim();if(!/^[\x21-\x7e]{16,256}$/.test(key))throw new Error('Enter one agent key from your Account page.');require('./credentials.cjs').saveKey(origin,key);}
    else await require('./open.cjs').ensureKey(origin,{again:true});
    console.log(`Signed in to ${origin}. Git clones from npx @clunkyard/cli open use this key.`);return;
  }
  if(action==='new'){
    if(args.length!==2)throw new Error('Use npx @clunkyard/cli new LANGUAGE DIRECTORY, for example new python my-robot.');
    const destination=await createProject(args[0],path.resolve(project,args[1]));
    console.log(`Created ${destination}. Next:\n  cd ${path.relative(process.cwd(),destination)}\n  npx @clunkyard/cli doctor\n  npx @clunkyard/cli build\n  npx @clunkyard/cli match`);return;
  }
  if(action==='migrate'){
    migrate(project);console.log('Updated generated tooling. Robot source files are unchanged.');return;
  }
  const contained=!sharedCache&&useDocker(project,action,docker);
  if(contained)return runInDocker(project,contained.language,action,args);
  await createRuntime(project,__filename,sharedCache?{sharedCache}:{}).main(action,args);
}

function platformLabel(language,row){
  if(row.portable||['javascript','typescript'].includes(language))return 'Linux / macOS / Windows (Node)';
  const names={'linux-x64':'Linux x86-64','linux-arm64':'Linux AArch64','darwin-arm64':'macOS Apple Silicon','darwin-x64':'macOS Intel','win32-x64':'Windows x86-64'};
  return [...new Set(['linux-x64',...Object.keys(row.sdkPlatforms||{})])].map(key=>names[key]||key).join(' / ');
}

if(require.main===module)main().catch(error=>{console.error(error.message);process.exitCode=1});
module.exports={main,parse,platformLabel};
