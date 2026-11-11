'use strict';

const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto');
const {spawnSync}=require('node:child_process');
const {extract,readAsset}=require('./archive.cjs');
const catalogue=require('./catalogue.json');
const {readManifest}=require('./project.cjs');

function selectLocalResourceMode(bot,supported) {
  if(supported.includes('open'))return 'open';
  const metered=supported.find(mode=>mode==='dsl'||mode==='wasm');
  if(metered)return metered;
  throw new Error(`${bot.name} supports resource modes [${supported.join(', ')}]. Local trials need open, dsl or wasm; use a hosted trial for trusted budgets.`);
}

function createRuntime(project,entry=path.join(__dirname,'cli.cjs')) {
const root=path.resolve(project),initial=readManifest(root),lock=catalogue.languages[initial.language];
const cache=path.join(root,'.clunkyard');
const engine=path.join(cache,'engine-'+lock.engine.sha256.slice(0,16));
const sdk=process.env.CLUNKYARD_SDK||path.join(cache,'sdk-'+lock.sdk.sha256.slice(0,16));
const npmSDK=['javascript','typescript'].includes(initial.language)?catalogue.npmSDK:null;
const manifest=()=>readManifest(root);
const sha=data=>crypto.createHash('sha256').update(data).digest('hex');
const skip=new Set(['.git','.clunkyard','build','replays','node_modules','__pycache__','obj','bin']);

function run(command, args, options={}) {
  const result=spawnSync(command,args,{cwd:root,env:environment(),stdio:'inherit',...options});
  if(result.error)throw new Error(`${command}: ${result.error.message}. See README.md for this language's toolchain.`);
  if(result.status!==0)throw new Error(`${command} exited with status ${result.status}`);
  return result;
}
function environment() {
  const paths=[path.join(sdk,'lib'),process.env.LD_LIBRARY_PATH].filter(Boolean).join(path.delimiter);
  return {...process.env,CLUNKYARD_SDK:sdk,CLUNKYARD_PROJECT:root,CLUNKYARD_LIB:path.join(sdk,'lib','libclunkyard.so.2'),LD_LIBRARY_PATH:paths,
    TMPDIR:path.join(cache,'tmp'),NODE_PATH:npmSDK?path.join(root,'node_modules'):path.join(sdk,'bindings','javascript'),
    DOTNET_CLI_TELEMETRY_OPTOUT:'1',DOTNET_NOLOGO:'1'};
}
function safeCache() {
  if(fs.existsSync(cache)&&(!fs.lstatSync(cache).isDirectory()||fs.lstatSync(cache).isSymbolicLink()))throw new Error('.clunkyard must be an ordinary directory');
  fs.mkdirSync(cache,{recursive:true});fs.mkdirSync(path.join(cache,'tmp'),{recursive:true});
}
async function download(asset, destination) {
  if(fs.existsSync(destination)) {
    if(fs.readFileSync(path.join(destination,'.release'),'utf8')===asset.sha256)return;
    throw new Error(`${destination} belongs to another release. Remove it explicitly before changing SDK versions.`);
  }
  const bytes=await readAsset(asset);
  const temporary=fs.mkdtempSync(path.join(cache,'install-'));
  try{extract(bytes,temporary,asset.expandedBytes);fs.writeFileSync(path.join(temporary,'.release'),asset.sha256);fs.renameSync(temporary,destination);}finally{fs.rmSync(temporary,{recursive:true,force:true});}
}
async function setup() {
  safeCache();
  if(!npmSDK&&!lock.portable&&(process.platform!=='linux'||process.arch!=='x64'))throw new Error('This native SDK release supports Linux x86-64 (Ubuntu 24.04+/WSL2). Use JavaScript/Vane/Cadence/Glyph for the portable Node route; other native platforms are not yet qualified.');
  await download(lock.engine,engine);
  if(npmSDK)installNpmSDK();
  else if(!process.env.CLUNKYARD_SDK)await download(lock.sdk,sdk);
  else if(fs.readFileSync(path.join(sdk,'VERSION'),'utf8').trim()!==lock.sdkVersion)throw new Error('CLUNKYARD_SDK points to another SDK version');
}
function installNpmSDK() {
  const packageFile=path.join(root,'package.json'),lockFile=path.join(root,'package-lock.json');
  if(!fs.existsSync(packageFile)||!fs.existsSync(lockFile))throw new Error('JavaScript/TypeScript projects need the npm SDK dependency and package-lock.json. Run clunkyard migrate for an older generated starter, or follow the SDK npm guide.');
  const metadata=JSON.parse(fs.readFileSync(packageFile)),packageLock=JSON.parse(fs.readFileSync(lockFile));
  if(metadata.dependencies?.clunkyard!==`npm:${npmSDK.name}@${npmSDK.version}`||packageLock.packages?.['node_modules/clunkyard']?.integrity!==npmSDK.integrity)throw new Error('Pin clunkyard to the reviewed npm SDK alias and integrity in package.json/package-lock.json.');
  const installedFile=path.join(root,'node_modules','clunkyard','package.json');
  const identity=sha(fs.readFileSync(lockFile)),stamp=path.join(cache,'npm-ready');
  if(fs.existsSync(stamp)&&fs.readFileSync(stamp,'utf8')===identity&&fs.existsSync(installedFile)){
    const installed=JSON.parse(fs.readFileSync(installedFile));
    if(installed.name===npmSDK.name&&installed.version===npmSDK.version)return;
  }
  const candidates=[process.env.npm_execpath,path.join(path.dirname(process.execPath),'node_modules/npm/bin/npm-cli.js'),path.resolve(path.dirname(process.execPath),'../lib/node_modules/npm/bin/npm-cli.js'),path.resolve(path.dirname(process.execPath),'../share/nodejs/npm/bin/npm-cli.js')];
  const cli=candidates.find(file=>file&&fs.existsSync(file)&&/\.[cm]?js$/.test(file));
  if(!cli)throw new Error('Install Node with npm, then run npm ci in this starter.');
  run(process.execPath,[cli,'ci','--ignore-scripts','--no-audit','--no-fund','--registry=https://registry.npmjs.org']);
  const installed=JSON.parse(fs.readFileSync(installedFile));
  if(installed.name!==npmSDK.name||installed.version!==npmSDK.version)throw new Error('Installed npm SDK differs from the project dependency.');
  fs.writeFileSync(stamp,identity);
}
function worker(project=root) {
  Object.assign(process.env,environment());
  process.env.CLUNKYARD_PROJECT=project;
  const file=path.join(engine,'sandbox','worker.js');delete require.cache[require.resolve(file)];return require(file);
}
function rewrite(value) {return value.replaceAll('/opt/arena/libexec/wasm-host',path.join(sdk,'bin','wasm-host')).replaceAll('/opt/arena/',`${engine}/`).replaceAll('/opt/wasmtime/',`${sdk}/`);}
function sources(from,to) {
  fs.mkdirSync(to,{recursive:true});
  for(const entry of fs.readdirSync(from,{withFileTypes:true})){
    if(skip.has(entry.name))continue;
    const input=path.join(from,entry.name),output=path.join(to,entry.name);
    if(entry.isDirectory())sources(input,output);else if(entry.isFile())fs.copyFileSync(input,output);
  }
}
function rebuildBinding() {
  // Compiler-specific artefacts are rebuilt by the player's own compiler.
  // This downloads no compiler and keeps native modules compatible locally.
  for(const binding of npmSDK?[]:(lock.rebuildBindings||[])){
    const directory=path.join(sdk,'binding-src',binding);
    run('make',['-C',directory,'install',`SDK=${sdk}`,`ROOT=${engine}`,`BUILD=${path.join(cache,'tmp','binding-'+binding)}`]);
  }
}
function typescriptCompiler() {
  // Prefer the project's compiler, then the player's installed toolchain.
  // Execute its JS entry point so spaces and Windows cmd shims are harmless.
  const directories=[path.join(root,'node_modules','.bin'),...(process.env.PATH||'').split(path.delimiter)];
  for(const directory of directories){
    for(const file of [path.join(directory,'node_modules/typescript/bin/tsc'),path.resolve(directory,'../typescript/bin/tsc'),path.resolve(directory,'../lib/node_modules/typescript/bin/tsc')]){
      if(fs.existsSync(file))return [process.execPath,file];
    }
    const executable=path.join(directory,'tsc');
    if(fs.existsSync(executable)){
      const file=fs.realpathSync(executable);
      if(fs.readFileSync(file,'utf8').startsWith('#!/usr/bin/env node'))return [process.execPath,file];
    }
  }
  throw new Error('Install TypeScript 6+ (tsc on PATH), then run clunkyard doctor.');
}
async function build() {
  await setup();const m=manifest(),out=path.join(cache,'build');fs.mkdirSync(out,{recursive:true});
  const bindingStamp=path.join(cache,'bindings-ready'),bindingIdentity=npmSDK?.integrity||lock.sdk.sha256;
  if(!fs.existsSync(bindingStamp)||fs.readFileSync(bindingStamp,'utf8')!==bindingIdentity){rebuildBinding();fs.writeFileSync(bindingStamp,bindingIdentity);}
  const copied=path.join(cache,'source');fs.rmSync(copied,{recursive:true,force:true});sources(root,copied);
  const action=npmSDK&&m.language==='typescript'?[...typescriptCompiler(),'--ignoreConfig','--ignoreDeprecations','6.0','--target','ES2022','--module','commonjs','--rootDir',copied,'--outDir',out,path.join(copied,m.entry)]:worker(copied).command(m.language,path.join(copied,m.entry),out,m);
  if(action){const steps=Array.isArray(action[0])?action:[action];for(const step of steps){const args=step.map(rewrite);if(m.language==='c')args[0]=process.env.CC||'cc';if(m.language==='cpp')args[0]=process.env.CXX||'c++';run(args[0],args.slice(1),{cwd:copied});}}
  const runtimeSource=path.join(out,'source');fs.rmSync(runtimeSource,{recursive:true,force:true});sources(copied,runtimeSource);
  if(m.language==='typescript')fs.writeFileSync(path.join(out,'package.json'),'{"type":"commonjs"}\n');
  fs.writeFileSync(path.join(cache,'local-bot.json'),JSON.stringify({...m,command:[process.execPath,entry,'--project',root,'stdio']},null,2)+'\n');
  console.log(`Built ${m.name}. Run make match or npx --yes @clunkyard/cli@2 match.`);
}
function stdio() {
  const m=manifest(),out=path.join(cache,'build'),entry=path.join(out,'source',m.entry);
  let command=worker().runCommand(m.language,path.join(root,m.entry),out,m);
  if(['javascript','typescript'].includes(m.language))command=[process.execPath,m.language==='typescript'?path.join(out,m.entry.replace(/\.ts$/,'.js')):entry];
  if(m.language==='julia')command=['julia','--startup-file=no',entry];
  if(m.language==='clojure')command=command.map(value=>value.replace('/usr/share/java/clojure.jar',clojureClasspath()));
  if(m.language==='scala'&&process.env.SCALA_LIBRARY_JAR)command=command.map(value=>value.replace('/usr/share/scala-2.11/lib/scala-library.jar',process.env.SCALA_LIBRARY_JAR));
  else if(m.language==='scala'&&!fs.existsSync('/usr/share/scala-2.11/lib/scala-library.jar'))command=['scala','-J--enable-native-access=ALL-UNNAMED','-cp',`${path.join(out,'classes')}:${sdk}/bindings/java/clunkyard.jar`,m.mainClass||path.basename(m.entry,'.scala')];
  if(m.language==='php')command=['php','-d','ffi.enable=true','-d',`include_path=${sdk}/bindings/php`,entry];
  const julia=m.language==='julia'?{JULIA_LOAD_PATH:`${sdk}/bindings/julia${path.delimiter}@stdlib`,JULIA_DEPOT_PATH:path.join(cache,'julia-depot'),OPENBLAS_NUM_THREADS:'1'}:{};
  const values=command.map(rewrite);run(values[0],values.slice(1),{env:{...environment(),...julia}});
}
async function match(args) {
  const seed=args[0]||'42',mode=args[1]||'2d',ruleset=args[2]||(mode==='3d'?'zero-g':'classic');
  if(!/^\d{1,10}$/.test(seed)||Number(seed)>0xffffffff)throw new Error('Match seed must be an integer from 0 to 4294967295.');
  await build();
  fs.mkdirSync(path.join(root,'replays'),{recursive:true});
  const replay=path.join(root,'replays',`trial-${seed}.json`);
  const {supportedResourceModes}=require(path.join(engine,'src','resources.js'));
  const m=manifest(),resourceMode=selectLocalResourceMode(m,supportedResourceModes(m));
  if(resourceMode==='open'){
    run(process.execPath,[path.join(engine,'src','cli.js'),'match','--bot','@'+path.join(cache,'local-bot.json'),'--bot','@'+path.join(engine,'bots','practice','bot.json'),'--mode',mode,'--ruleset',ruleset,'--resource-mode',resourceMode,'--seed',seed,'--ticks','360','--turn-ms','2000','--startup-ms','60000','--replay',replay]);
  }else{
    // The pinned engine already supports mixed Reflex entries: the player's
    // Wasm/DSL robot and the bundled Vane rival each keep their own meter.
    const {resolveBot,validateSelection}=require(path.join(engine,'src','roster.js'));
    const {runMatch,saveReplay}=require(path.join(engine,'src','match.js'));
    const player=resolveBot('@'+path.join(cache,'local-bot.json'),[],engine,mode,ruleset);
    const rival=resolveBot('@'+path.join(engine,'bots','practice','bot.json'),[],engine,mode,ruleset);
    player.resourcePolicy={mode:resourceMode};rival.resourcePolicy={mode:'dsl'};
    const bots=[player,rival];validateSelection(bots,mode,ruleset);
    const result=await runMatch(bots,{mode,ruleset,seed:Number(seed),maxTicks:360,turnMs:2000,startupMs:60000,resourcePolicy:{v:3,class:'reflex',policy:'fair-compute-v1'}});
    saveReplay(result.replay,replay);
    console.log(`Result: ${result.result?.winners?.join(', ')||'draw'} (${result.result?.reason||'unknown'})`);
    console.log(`Replay: ${replay}`);
  }
  const failures=JSON.parse(fs.readFileSync(replay,'utf8')).failures||[];
  if(failures.length)throw new Error(`Match recorded ${failures.map(value=>`${value.botId}: ${value.code}`).join(', ')}. Inspect ${replay}.`);
}
function clojureClasspath(){
  if(process.env.CLOJURE_CLASSPATH)return process.env.CLOJURE_CLASSPATH;
  if(process.env.CLOJURE_JAR){if(!fs.existsSync(process.env.CLOJURE_JAR))throw new Error('CLOJURE_JAR does not exist');return process.env.CLOJURE_JAR;}
  if(fs.existsSync('/usr/share/java/clojure.jar'))return '/usr/share/java/clojure.jar';
  const result=spawnSync('clojure',['-Spath'],{encoding:'utf8',timeout:10000});
  if(result.status===0&&result.stdout.trim())return result.stdout.trim();
  throw new Error('Install Clojure or set CLOJURE_CLASSPATH / CLOJURE_JAR to its runtime. See README.md.');
}
function toolAvailable(tool){
  if(npmSDK&&tool==='tsc'){try{typescriptCompiler();return true}catch{return false}}
  const extensions=process.platform==='win32'?['',...(process.env.PATHEXT||'.EXE;.CMD;.BAT').split(';')]:[''];
  for(const directory of (process.env.PATH||'').split(path.delimiter))for(const extension of extensions){
    const file=path.join(directory||root,tool+extension);
    try{if(fs.statSync(file).isFile()){fs.accessSync(file,process.platform==='win32'?fs.constants.F_OK:fs.constants.X_OK);return true}}catch{}
  }
  return false;
}
function doctor(){
  console.log(`Clunkyard SDK ${lock.sdkVersion}; ${lock.language}; ${process.platform}/${process.arch}; Node ${process.versions.node}`);
  let missing=0;
  for(const tool of lock.tools.filter(tool=>tool!=='make'||(!npmSDK&&lock.rebuildBindings?.length))){
    const ok=toolAvailable(tool);
    console.log(`${ok?'found':'missing'} ${tool}`);if(!ok)missing++;
  }
  for(const check of lock.requirements?.checks||[]){
    let command=[check.command];
    if(npmSDK&&check.command==='tsc'){try{command=typescriptCompiler()}catch{}}
    const result=spawnSync(command[0],[...command.slice(1),...check.args],{encoding:'utf8',env:environment(),timeout:10000});
    const output=String(result.stdout||'')+String(result.stderr||'');
    const version=check.versionPattern?Number(output.match(new RegExp(check.versionPattern))?.[1]):null;
    const ok=!result.error&&result.status===0&&(!check.minimum||version>=check.minimum);
    console.log(`${ok?'ok':'missing'} ${check.label}${ok?'':': '+check.hint}`);if(!ok)missing++;
  }
  if(lock.language==='clojure'){try{console.log('ok Clojure runtime: '+clojureClasspath())}catch(error){console.log('missing '+error.message);missing++}}
  if(lock.language==='scala'&&process.env.SCALA_LIBRARY_JAR&&!fs.existsSync(process.env.SCALA_LIBRARY_JAR)){console.log('missing SCALA_LIBRARY_JAR file');missing++}
  if(lock.language==='scala'&&!process.env.SCALA_LIBRARY_JAR&&!fs.existsSync('/usr/share/scala-2.11/lib/scala-library.jar')&&spawnSync('scala',['-version'],{timeout:10000}).error){console.log('missing Scala runtime; set SCALA_LIBRARY_JAR or install the scala launcher.');missing++}
  const glibc=process.report?.getReport().header.glibcVersionRuntime;
  if(!npmSDK&&!lock.portable&&(process.platform!=='linux'||process.arch!=='x64'||!glibc||Number(glibc.split('.')[0])<2||(Number(glibc.split('.')[0])===2&&Number(glibc.split('.')[1])<39))){console.log('unsupported native platform: use Linux x86-64 with glibc 2.39+ (Ubuntu 24.04+ or WSL2).');missing++}
  for(const note of lock.requirements?.notes||[])console.log(note);
  console.log(npmSDK||lock.portable?'Portable Node route.':'Native assets: Linux x86-64, glibc 2.39+; Ubuntu 24.04+/WSL2.');
  console.log('No toolchains are downloaded automatically. See README.md for official installers.');
  if(missing)process.exitCode=1;
}
async function main(action='build',args=[]){
  if(action==='doctor')return doctor();
  safeCache();
  if(action==='setup')return setup();
  if(action==='build')return build();
  if(action==='stdio')return stdio();
  if(action==='match')return match(args);
  if(action==='replay'){await setup();return run(process.execPath,[path.join(engine,'src','cli.js'),'replay',path.resolve(root,args[0]||'replays/trial-42.json'),'--port',args[1]||'8787']);}
  if(action==='clean'){fs.rmSync(path.join(cache,'build'),{recursive:true,force:true});fs.rmSync(path.join(cache,'bindings-ready'),{force:true});return;}
  throw new Error('Use doctor, setup, build, match [seed] [2d|3d] [ruleset], replay [file] [port], or clean.');
}

return {main};
}

module.exports={createRuntime,selectLocalResourceMode};
