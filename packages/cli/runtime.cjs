'use strict';

const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto');
const {spawn,spawnSync}=require('node:child_process');
const {extract,readAsset}=require('./archive.cjs');
const catalogue=require('./catalogue.json');
const {readManifest}=require('./project.cjs');

function selectPlatformSdk(lock,platform=process.platform,arch=process.arch) {
  if(lock.portable)return lock.sdk;
  const key=`${platform}-${arch}`,asset=lock.sdkPlatforms?.[key];
  if(asset)return asset;
  if(key==='linux-x64')return lock.sdk;
  throw new Error(`No native ${lock.language} SDK is released for ${key}. Use a hosted build or trial, or install a matching SDK with CLUNKYARD_SDK. See the language guide for toolchain requirements.`);
}

// Execute the worker's environment-prefixed recipes without requiring Unix env
// on the player's machine. Arguments stay separate, including paths with spaces.
function recipeInvocation(command,args,{cwd,env}) {
  if(command!=='env')return {command,args,cwd,env};
  args=[...args];env={...env};
  if(args[0]==='--chdir'){args.shift();cwd=args.shift();}
  while(args.length&&/^[A-Za-z_][A-Za-z0-9_]*=/.test(args[0])){
    const value=args.shift(),index=value.indexOf('=');env[value.slice(0,index)]=value.slice(index+1);
  }
  command=args.shift();
  if(!command)throw new Error('The local recipe has no executable.');
  return {command,args,cwd,env};
}

function bindingCompilerIdentity(lock,environment,probe=spawnSync,platform=process.platform){
  return lock.tools.filter(tool=>!['node','make'].includes(tool)&&!(platform==='win32'&&lock.language==='rust'&&tool==='cc')).map(tool=>{
    const variable={rustc:'RUSTC',cc:'CC','c++':'CXX',swiftc:'SWIFTC'}[tool];
    const command=variable&&environment[variable]||tool;
    const result=probe(command,[tool==='javac'?'-version':'--version'],{env:environment,encoding:'utf8',timeout:10000,maxBuffer:64*1024});
    return {command,status:result.status,version:(String(result.stdout||'')+String(result.stderr||'')).slice(0,4096),error:result.error?.code||null};
  });
}

function selectLocalResourceMode(bot,supported) {
  if(supported.includes('open'))return 'open';
  const metered=supported.find(mode=>mode==='dsl'||mode==='wasm');
  if(metered)return metered;
  throw new Error(`${bot.name} supports resource modes [${supported.join(', ')}]. Local trials need open, dsl or wasm; use a hosted trial for trusted budgets.`);
}

function createRuntime(project,entry=path.join(__dirname,'cli.cjs'),options={}) {
const root=path.resolve(project),initial=readManifest(root),lock=catalogue.languages[initial.language];
const cache=path.join(root,'.clunkyard');
// A house opponent keeps its own build state under its staged directory but
// shares the player's engine and SDK downloads (options.sharedCache).
const store=options.sharedCache?path.resolve(options.sharedCache):cache;
const engine=path.join(store,'engine-'+lock.engine.sha256.slice(0,16));
const npmSDK=['javascript','typescript'].includes(initial.language)?catalogue.npmSDK:null;
// CLUNKYARD_SDK is the player's robot's SDK; a house opponent in another language never uses it.
const suppliedSdk=process.env.CLUNKYARD_SDK&&!options.sharedCache?path.resolve(process.env.CLUNKYARD_SDK):null;
let sdkAsset,platformError,suppliedMetadata;
try{sdkAsset=npmSDK?lock.sdk:selectPlatformSdk(lock);}catch(error){platformError=error;}
if(suppliedSdk&&fs.existsSync(path.join(suppliedSdk,'platform.json')))suppliedMetadata=JSON.parse(fs.readFileSync(path.join(suppliedSdk,'platform.json'),'utf8'));
const sdk=suppliedSdk||path.join(store,'sdk-'+(sdkAsset?.sha256.slice(0,16)||'unavailable'));
const library=suppliedMetadata?.library||sdkAsset?.library||({darwin:'libclunkyard.2.dylib',win32:'clunkyard.dll'}[process.platform]||'libclunkyard.so.2');
if(path.basename(library)!==library||library.includes('\\'))throw new Error('SDK library must be a file name.');
const manifest=()=>readManifest(root);
const sha=data=>crypto.createHash('sha256').update(data).digest('hex');
const skip=new Set(['.git','.clunkyard','build','replays','node_modules','__pycache__','obj','bin']);

// A language with a WebAssembly route builds to it, as the hosted builder
// does (the engine's src/build-targets.js), unless bot.json says "target":
// "native". That needs this platform's SDK to carry the command libraries
// and the host; elsewhere the robot builds natively, as before.
function wasmRoute(){return suppliedSdk?fs.existsSync(path.join(sdk,'wasm32-wasip1')):Boolean(sdkAsset?.wasmTarget);}
function buildTarget(m){
  const file=path.join(engine,'src','build-targets.js');
  const target=fs.existsSync(file)?require(file).buildTarget(m.language,m):m.target==='native'?m.language:sdkAsset?.wasmTarget||m.language;
  return target===m.language||wasmRoute()?target:m.language;
}
function hostedTarget(m){const file=path.join(engine,'src','build-targets.js');return fs.existsSync(file)?require(file).buildTarget(m.language,m):sdkAsset?.wasmTarget&&m.target!=='native'?sdkAsset.wasmTarget:m.language;}
// The hosted builder's WASI toolchain is Clang 22 with the wasi-sdk 33
// sysroot in /opt/wasi-sysroot and Hew in /opt/hew. A player's comes from
// WASI_SDK_PATH (a wasi-sdk installation), WASI_SYSROOT and HEW_ROOT, or the
// compilers on PATH.
let wasiCache;
function wasiTools(){
  if(wasiCache)return wasiCache;
  const home=process.env.WASI_SDK_PATH,tool=name=>home?path.join(home,'bin',name):toolAvailable(name+'-22')?name+'-22':name;
  return wasiCache={executables:{'clang-22':tool('clang'),'clang++-22':tool('clang++'),'wasm-ld-22':tool('wasm-ld')},
    sysroot:process.env.WASI_SYSROOT||(home?path.join(home,'share','wasi-sysroot'):'/opt/wasi-sysroot'),hew:hewRoot()};
}
function hewRoot(){
  if(process.env.HEW_ROOT)return path.resolve(process.env.HEW_ROOT);
  for(const directory of (process.env.PATH||'').split(path.delimiter)){
    try{const prefix=path.dirname(path.dirname(fs.realpathSync(path.join(directory||root,'hew'))));if(fs.existsSync(path.join(prefix,'include','hew.h')))return prefix;}catch{}
  }
  return '/opt/hew';
}

function run(command, args, options={}) {
  const invocation=recipeInvocation(command,args,{cwd:options.cwd||root,env:options.env||environment()});
  command=invocation.command;
  if(process.platform==='win32'&&command==='python3')command=process.env.PYTHON||'python';
  const result=spawnSync(command,invocation.args,{stdio:'inherit',...options,cwd:invocation.cwd,env:invocation.env});
  if(result.error)throw new Error(`${command}: ${result.error.message}. See README.md for this language's toolchain.`);
  if(result.status!==0)throw new Error(`${command} exited with status ${result.status}`);
  return result;
}
function stream(command,args,options={}) {
  const invocation=recipeInvocation(command,args,{cwd:options.cwd||root,env:options.env||environment()});
  command=invocation.command;
  if(process.platform==='win32'&&command==='python3')command=process.env.PYTHON||'python';
  return new Promise((resolve,reject)=>{
    let child,termTimer,killTimer,failure,stopping=false,settled=false;
    const cleanup=()=>{clearTimeout(termTimer);clearTimeout(killTimer);process.removeListener('SIGINT',interrupt);process.removeListener('SIGTERM',terminate);process.stdin.removeListener('end',eof);if(child){process.stdin.unpipe(child.stdin);child.stdin?.destroy();}process.stdin.pause();};
    const finish=error=>{if(settled)return;settled=true;cleanup();if(error)reject(error);else resolve();};
    const stop=signal=>{if(stopping||settled)return;stopping=true;clearTimeout(termTimer);process.stdin.unpipe(child.stdin);child.stdin.end();child.kill(signal);killTimer=setTimeout(()=>child.kill('SIGKILL'),750);};
    const interrupt=()=>stop('SIGINT'),terminate=()=>stop('SIGTERM');
    const eof=()=>{if(!stopping&&!settled)termTimer=setTimeout(terminate,250);};
    process.once('SIGINT',interrupt);process.once('SIGTERM',terminate);process.stdin.once('end',eof);
    child=spawn(command,invocation.args,{cwd:invocation.cwd,env:invocation.env,stdio:['pipe','inherit','inherit'],shell:false});
    child.stdin.on('error',error=>{if(!['EPIPE','ERR_STREAM_DESTROYED'].includes(error.code)){failure=error;stop('SIGTERM');}});
    child.once('error',error=>finish(new Error(`${command}: ${error.message}. See README.md for this language's toolchain.`)));
    child.once('exit',(code,signal)=>finish(failure||(!stopping&&code!==0?new Error(`${command} exited with ${signal||'status '+code}`):null)));
    process.stdin.pipe(child.stdin);
  });
}
function environment() {
  const paths=[path.join(sdk,'lib'),process.env.LD_LIBRARY_PATH].filter(Boolean).join(path.delimiter);
  return {...process.env,CLUNKYARD_SDK:sdk,CLUNKYARD_PROJECT:root,CLUNKYARD_WASM_INCLUDE:path.join(engine,'sdk','wasm','abi4'),CLUNKYARD_LIB:path.join(sdk,'lib',library),LD_LIBRARY_PATH:paths,
    ...(process.platform==='darwin'?{DYLD_LIBRARY_PATH:[path.join(sdk,'lib'),process.env.DYLD_LIBRARY_PATH].filter(Boolean).join(path.delimiter)}:{}),
    ...(process.platform==='win32'?{PATH:[path.join(sdk,'lib'),process.env.PATH].filter(Boolean).join(path.delimiter),CC:cCompiler(false),CXX:cCompiler(true)}:{}),
    // Hew's binding build and WebAssembly link find Hew's headers and libraries here.
    ...(lock.language==='hew'&&!process.env.HEW_ROOT?{HEW_ROOT:wasiTools().hew}:{}),
    TMPDIR:path.join(cache,'tmp'),NODE_PATH:npmSDK?path.join(root,'node_modules'):path.join(sdk,'bindings','javascript'),
    DOTNET_CLI_TELEMETRY_OPTOUT:'1',DOTNET_NOLOGO:'1'};
}
function cCompiler(cpp){return process.env[cpp?'CXX':'CC']||(process.platform==='win32'?(toolAvailable(cpp?'clang++':'clang')?(cpp?'clang++':'clang'):'cl'):(cpp?'c++':'cc'));}
function safeCache() {
  if(fs.existsSync(cache)&&(!fs.lstatSync(cache).isDirectory()||fs.lstatSync(cache).isSymbolicLink()))throw new Error('.clunkyard must be an ordinary directory');
  fs.mkdirSync(cache,{recursive:true});fs.mkdirSync(path.join(cache,'tmp'),{recursive:true});
  if(store!==cache)fs.mkdirSync(store,{recursive:true});
}
async function download(asset, destination) {
  if(fs.existsSync(destination)) {
    if(fs.readFileSync(path.join(destination,'.release'),'utf8')===asset.sha256)return;
    throw new Error(`${destination} belongs to another release. Remove it explicitly before changing SDK versions.`);
  }
  const bytes=await readAsset(asset);
  const temporary=fs.mkdtempSync(path.join(store,'install-'));
  try{extract(bytes,temporary,asset.expandedBytes);fs.writeFileSync(path.join(temporary,'.release'),asset.sha256);fs.renameSync(temporary,destination);}finally{fs.rmSync(temporary,{recursive:true,force:true});}
}
async function setup() {
  safeCache();
  checkNativePlatform();
  await download(lock.engine,engine);
  if(npmSDK)installNpmSDK();
  else if(!suppliedSdk)await download(sdkAsset,sdk);
  else if(fs.readFileSync(path.join(sdk,'VERSION'),'utf8').trim()!==lock.sdkVersion)throw new Error('CLUNKYARD_SDK points to another SDK version');
}
function checkNativePlatform(){
  if(npmSDK||lock.portable)return;
  if(suppliedSdk){
    if(suppliedMetadata&&suppliedMetadata.platform!==`${process.platform}-${process.arch}`)throw new Error(`CLUNKYARD_SDK targets ${suppliedMetadata.platform}, not ${process.platform}-${process.arch}.`);
    if(!suppliedMetadata&&process.platform!=='linux')throw new Error('A custom native SDK on this platform needs platform.json from the native SDK builder.');
  }else if(platformError)throw platformError;
  const glibc=process.report?.getReport().header.glibcVersionRuntime;
  if(process.platform==='linux'&&(!glibc||Number(glibc.split('.')[0])<2||(Number(glibc.split('.')[0])===2&&Number(glibc.split('.')[1])<39)))throw new Error('This native Linux SDK needs glibc 2.39+ (Ubuntu 24.04+ or WSL2). Use a hosted build or trial on other systems.');
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
function rewrite(value) {
  value=value.replaceAll('/opt/arena/libexec/wasm-host',path.join(sdk,'bin','wasm-host')).replaceAll('/opt/arena/',`${engine}/`).replaceAll('/opt/wasmtime/',`${sdk}/`);
  if(!/^(?:clang(?:\+\+)?|wasm-ld)-22$|\/opt\/(?:wasi-sysroot|hew\/)/.test(value))return value;
  const tools=wasiTools();
  return tools.executables[value]||value.replaceAll('/opt/wasi-sysroot',tools.sysroot).replaceAll('/opt/hew/',`${tools.hew}/`);
}
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
    const installer=path.join(directory,'install.cjs');
    if(fs.existsSync(installer))run(process.execPath,[installer,sdk]);
    else run('make',['-C',directory,'install',`SDK=${sdk}`,`ROOT=${engine}`,`BUILD=${path.join(cache,'tmp','binding-'+binding)}`]);
  }
  // The Rust binding for WebAssembly commands, as scripts/build-wasm-command.sh
  // builds it: an rlib links only with the rustc that built it. Without the
  // target installed, the robot's own build reports it (and doctor checks it).
  const wasm=path.join(sdk,'wasm32-wasip1'),rustc=process.env.RUSTC||'rustc';
  if(lock.rebuildBindings?.includes('rust')&&fs.existsSync(path.join(wasm,'libclunkyard.a'))){
    const library=spawnSync(rustc,['--print','target-libdir','--target','wasm32-wasip1'],{encoding:'utf8'});
    if(library.status===0&&fs.existsSync(String(library.stdout).trim()))run(rustc,['--edition=2021','-O','--target','wasm32-wasip1','--crate-type','rlib','--crate-name','clunkyard','-L',`native=${wasm}`,path.join(sdk,'binding-src','rust','clunkyard.rs'),'-o',path.join(wasm,'libclunkyard.rlib')]);
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
async function build({opponent=false}={}) {
  await setup();const m=manifest(),out=path.join(cache,'build');fs.mkdirSync(out,{recursive:true});
  const bindingStamp=path.join(cache,'bindings-ready'),bindingIdentity=npmSDK?.integrity||sha(Buffer.from(JSON.stringify({asset:suppliedSdk?{sdk,metadata:suppliedMetadata||{},version:lock.sdkVersion}:sdkAsset.sha256,compilers:lock.rebuildBindings?.length?bindingCompilerIdentity(lock,environment()):[]})));
  if(!fs.existsSync(bindingStamp)||fs.readFileSync(bindingStamp,'utf8')!==bindingIdentity){rebuildBinding();fs.writeFileSync(bindingStamp,bindingIdentity);}
  const copied=path.join(cache,'source');fs.rmSync(copied,{recursive:true,force:true});sources(root,copied);
  const target=buildTarget(m);
  if(target!==hostedTarget(m))console.log(`Hosted builds compile ${m.name} to WebAssembly; this platform's SDK has no WebAssembly host, so it builds natively here.`);
  const action=npmSDK&&m.language==='typescript'?[...typescriptCompiler(),'--ignoreConfig','--ignoreDeprecations','6.0','--target','ES2022','--module','commonjs','--rootDir',copied,'--outDir',out,path.join(copied,m.entry)]:worker(copied).command(target,path.join(copied,m.entry),out,m);
  if(action){const steps=Array.isArray(action[0])?action:[action];for(const step of steps){const args=step.map(rewrite);if(target==='c')args[0]=cCompiler(false);if(target==='cpp')args[0]=cCompiler(true);run(args[0],args.slice(1),{cwd:copied});}}
  const runtimeSource=path.join(out,'source');fs.rmSync(runtimeSource,{recursive:true,force:true});sources(copied,runtimeSource);
  if(m.language==='typescript')fs.writeFileSync(path.join(out,'package.json'),'{"type":"commonjs"}\n');
  fs.writeFileSync(path.join(cache,'local-bot.json'),JSON.stringify({...m,command:[process.execPath,entry,'--project',root,...(options.sharedCache?['--shared-cache',store]:[]),'stdio']},null,2)+'\n');
  console.log(`Built ${m.name}${target===m.language?'':` for WebAssembly (${target})`}.${opponent?'':' Run make match or npx @clunkyard/cli match.'}`);
}
function stdio() {
  const m=manifest(),out=path.join(cache,'build'),entry=path.join(out,'source',m.entry);
  let command=worker().runCommand(buildTarget(m),path.join(root,m.entry),out,m);
  if(['javascript','typescript'].includes(m.language))command=[process.execPath,m.language==='typescript'?path.join(out,m.entry.replace(/\.ts$/,'.js')):entry];
  if(m.language==='julia')command=['julia','--startup-file=no',entry];
  if(m.language==='clojure')command=command.map(value=>value.replace('/usr/share/java/clojure.jar',clojureClasspath()));
  if(m.language==='scala'&&process.env.SCALA_LIBRARY_JAR)command=command.map(value=>value.replace('/usr/share/scala-2.11/lib/scala-library.jar',process.env.SCALA_LIBRARY_JAR));
  else if(m.language==='scala'&&!fs.existsSync('/usr/share/scala-2.11/lib/scala-library.jar'))command=['scala','-J--enable-native-access=ALL-UNNAMED','-cp',`${path.join(out,'classes')}:${sdk}/bindings/java/clunkyard.jar`,m.mainClass||path.basename(m.entry,'.scala')];
  if(m.language==='php')command=['php','-d','ffi.enable=true','-d',`include_path=${sdk}/bindings/php`,entry];
  const julia=m.language==='julia'?{JULIA_LOAD_PATH:`${sdk}/bindings/julia${path.delimiter}@stdlib`,JULIA_DEPOT_PATH:path.join(cache,'julia-depot'),OPENBLAS_NUM_THREADS:'1'}:{};
  const values=command.map(rewrite);return stream(values[0],values.slice(1),{env:{...environment(),...julia}});
}
// A WebAssembly robot plays on the season's fuel, as hosted: a build of a
// language with a fuel tier (wasm:<language>) has its Standard budget, in an
// Open match beside the DSL rival; any other WebAssembly robot has Reflex's.
function wasmPolicy(m){
  const ratesFile=path.join(engine,'src','rates.js');if(!fs.existsSync(ratesFile))return null;
  const run=worker().runCommand(buildTarget(m),path.join(root,m.entry),path.join(cache,'build'),m);
  if(path.basename(String(run[0]))!=='wasm-host')return null;
  const rates=require(ratesFile),{table}=rates.loadRates();
  let runtime=null;try{runtime=rates.runtimeKey(run,m.language);}catch{}
  const budget=runtime?.startsWith('wasm:')&&rates.wasmEntryBudget(table,runtime);
  if(budget){const {rate,...limits}=budget;return {matchClass:'open',entry:{mode:'wasm',limits,rate},label:`Season ${table.season} Standard, ${runtime} fuel tier ${rate.tier}`};}
  return {matchClass:'reflex',entry:{mode:'wasm',limits:{wasmFuelPerTurn:table.reflex.wasmFuelPerTurn,wasmFuelPerMatch:table.reflex.wasmFuelPerMatch}},label:`Season ${table.season} Reflex`};
}
// Builds this robot and describes how it takes part in a local match.
async function prepare(buildOptions) {
  await build(buildOptions);
  const {supportedResourceModes}=require(path.join(engine,'src','resources.js'));
  const m=manifest(),fuel=wasmPolicy(m),supported=supportedResourceModes(m);
  return {file:path.join(cache,'local-bot.json'),manifest:m,fuel,resourceMode:fuel?'wasm':selectLocalResourceMode(m,supported),dsl:supported.includes('dsl')};
}

// House opponents. The house robots' hosted projects are the authority: their
// published source is public, and the CLI reads the list and each robot's
// active revision from the service, then builds it by this CLI's own
// per-language path. The list and sources are cached in .clunkyard/house, so
// a robot that has been fetched once also plays offline.
const HOUSE_ORIGIN=()=>String(process.env.CLUNKYARD_ORIGIN||'https://clunkyard.com').replace(/\/+$/,'');
const HOUSE_ID=/^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/;
const HOUSE_KEY=/^[a-z0-9][a-z0-9-]{0,47}$/;
const keyFor=name=>String(name).toLowerCase().replace(/[^a-z0-9]+/g,'-').replace(/^-|-$/g,'').slice(0,48);
const houseDirectory=()=>path.join(cache,'house');
async function fetchLimited(url,limit){
  const response=await fetch(url,{signal:AbortSignal.timeout(30000),headers:{accept:'application/json, application/gzip'}});
  if(!response.ok)throw new Error(`${url} answered ${response.status}`);
  const bytes=Buffer.from(await response.arrayBuffer());
  if(bytes.length>limit)throw new Error(`${url} sent more than ${limit} bytes`);
  return bytes;
}
// The cached list, unless refresh asks for the service's. A service that
// cannot be reached leaves the cache in use; with none, there is nothing to play.
async function houseRoster({refresh=false}={}) {
  const file=path.join(houseDirectory(),'index.json');
  let cached=null;try{cached=JSON.parse(fs.readFileSync(file,'utf8'));}catch{}
  if(cached&&!refresh&&cached.origin===HOUSE_ORIGIN())return {...cached,fresh:false};
  try{
    const value=JSON.parse((await fetchLimited(`${HOUSE_ORIGIN()}/api/arena/house-robots`,1024*1024)).toString('utf8'));
    const robots=(Array.isArray(value.robots)?value.robots:[]).filter(row=>HOUSE_ID.test(String(row.id))&&typeof row.name==='string'&&typeof row.language==='string'&&HOUSE_KEY.test(keyFor(row.name))&&keyFor(row.name)!=='practice').map(row=>({id:row.id,key:keyFor(row.name),name:row.name,language:row.language,stable:row.stable||null,manifestHash:typeof row.manifestHash==='string'?row.manifestHash:null,supportedModes:Array.isArray(row.supportedModes)?row.supportedModes:[],supportedRulesets:Array.isArray(row.supportedRulesets)?row.supportedRulesets:['classic'],strategist:Boolean(row.strategist)}));
    const result={origin:HOUSE_ORIGIN(),fetchedAt:new Date().toISOString(),robots};
    fs.mkdirSync(houseDirectory(),{recursive:true});fs.writeFileSync(file,JSON.stringify(result,null,2)+'\n');
    return {...result,fresh:true};
  }catch(error){
    if(cached&&cached.origin===HOUSE_ORIGIN())return {...cached,fresh:false,offline:error.message};
    throw new Error(`The house robots come from ${HOUSE_ORIGIN()} and it cannot be reached (${error.message}), with nothing cached in .clunkyard/house. Connect once to fetch them; they then play offline.`);
  }
}
// Unpacks a published-source bundle: gzip JSON of files with sizes and digests.
function unpackHouseSource(bytes,to){
  const bundle=JSON.parse(require('node:zlib').gunzipSync(bytes,{maxOutputLength:48*1024*1024}).toString('utf8'));
  if(bundle?.v!==1||!Array.isArray(bundle.files)||!bundle.files.length||bundle.files.length>512)throw new Error('The published source is not a version 1 bundle.');
  let total=0;const files=bundle.files.map(file=>{
    const name=String(file?.path);
    if(!name||name.includes('\\')||name.includes('\0')||path.posix.isAbsolute(name)||name.split('/').some(part=>!part||part==='.'||part==='..'||part==='.git'||part==='.clunkyard'))throw new Error(`The published source holds an unsafe path: ${JSON.stringify(name)}`);
    const data=Buffer.from(String(file.data),'base64');total+=data.length;
    if(total>16*1024*1024||data.length!==file.size||sha(data)!==file.sha256)throw new Error(`The published source file ${name} does not match its digest.`);
    return {name,data,executable:Number(file.mode)&0o111};
  });
  for(const name of fs.existsSync(to)?fs.readdirSync(to):[])if(name!=='.clunkyard')fs.rmSync(path.join(to,name),{recursive:true,force:true});
  fs.mkdirSync(to,{recursive:true});
  for(const {name,data,executable} of files){const target=path.join(to,...name.split('/'));fs.mkdirSync(path.dirname(target),{recursive:true});fs.writeFileSync(target,data,{mode:executable?0o755:0o644});}
}
// The robot's staged project: its published source as of the last fetch.
async function stageHouseRobot(row) {
  const to=path.join(houseDirectory(),row.key),stamp=path.join(to,'.revision');
  const current=()=>{try{return fs.readFileSync(stamp,'utf8')===(row.manifestHash||row.id)&&fs.existsSync(path.join(to,'bot.json'));}catch{return false;}};
  if(!current()){
    try{
      unpackHouseSource(await fetchLimited(`${HOUSE_ORIGIN()}/api/arena/v2/bots/${row.id}/source`,16*1024*1024),to);
      fs.writeFileSync(stamp,row.manifestHash||row.id);
    }catch(error){
      if(!fs.existsSync(path.join(to,'bot.json')))throw new Error(`Could not fetch ${row.name}'s source from ${HOUSE_ORIGIN()}: ${error.message}. Connect once to fetch it; it then plays offline.`);
      console.log(`Using the cached ${row.name}; its source could not be refreshed (${error.message}).`);
    }
  }
  return createRuntime(to,entry,{sharedCache:store});
}
function chooseOpponents(names,roster) {
  const chosen=[],practice={key:'practice',name:'Practice rival',practice:true};
  for(const name of names){
    const wanted=name.toLowerCase(),row=wanted==='practice'||wanted==='practice rival'?practice:roster?.robots.find(row=>row.key===wanted||row.name.toLowerCase()===wanted);
    if(!row)throw new Error(`No local opponent named ${name}. Run clunkyard opponents to list them.`);
    if(chosen.includes(row))throw new Error(`${row.name} is listed twice; each opponent seats once.`);
    chosen.push(row);
  }
  return chosen;
}
function matchOptions(args) {
  const names=[],rest=[];let refresh=false;
  for(let i=0;i<args.length;i++){
    const value=args[i];
    if(value==='--opponent'){
      if(!args[i+1]||args[i+1].startsWith('--'))throw new Error('--opponent needs a robot name. Run clunkyard opponents to list them.');
      names.push(...args[++i].split(','));
    }else if(value.startsWith('--opponent='))names.push(...value.slice(11).split(','));
    else if(value==='--refresh')refresh=true;
    else if(value.startsWith('--'))throw new Error(`Unknown option ${value}. Use match [seed] [2d|3d] [ruleset] [--opponent NAME]... [--refresh]`);
    else rest.push(value);
  }
  return {names:names.map(name=>name.trim()).filter(Boolean),rest,refresh};
}
async function opponents(args=[]) {
  const unknown=args.filter(value=>value!=='--refresh');
  if(unknown.length)throw new Error('Use clunkyard opponents [--refresh].');
  safeCache();
  const roster=await houseRoster({refresh:args.includes('--refresh')});
  console.log(`Local opponents for match --opponent NAME. The house robots are the published source of their hosted projects at ${roster.origin}, as of ${roster.fetchedAt}${roster.fresh?'':' (cached; opponents --refresh updates it)'}.`);
  if(roster.offline)console.log(`${roster.origin} could not be reached (${roster.offline}); showing the cache.`);
  console.log('');
  console.log(`${'practice'.padEnd(10)} ${'Practice rival'.padEnd(15)} ready       vane; 2d and 3d; all rulesets (default)`);
  for(const row of roster.robots){
    let detail=`${row.language}; ${row.supportedModes.join(' and ')||'no modes'}; ${row.supportedRulesets.length>=5?'all rulesets':row.supportedRulesets.join(', ')}${row.strategist?'; plays its reflex without its hosted strategist':''}`,problems=[];
    try{problems=(await stageHouseRobot(row)).diagnose().problems;}catch(error){problems=[error.message];}
    console.log(`${row.key.padEnd(10)} ${row.name.padEnd(15)} ${problems.length?'needs setup':'ready      '} ${detail}`);
    for(const problem of problems)console.log(`           ${problem}`);
  }
  if(!roster.robots.length)console.log('The service lists no house robots right now.');
}
async function match(args) {
  const {names,rest,refresh}=matchOptions(args),seed=rest[0]||'42',mode=rest[1]||'2d',ruleset=rest[2]||(mode==='3d'?'zero-g':'classic');
  if(!/^\d{1,10}$/.test(seed)||Number(seed)>0xffffffff)throw new Error('Match seed must be an integer from 0 to 4294967295.');
  // Name and seat the opponents before the slower builds. A house robot only
  // plays what its own bot.json declares.
  const chosen=[];
  if(names.length){
    if(names.length>3)throw new Error('A local match seats four robots: yours and up to three opponents.');
    const roster=names.some(name=>!['practice','practice rival'].includes(name.toLowerCase()))?await houseRoster({refresh}):null;
    for(const row of chooseOpponents(names,roster)){
      if(row.practice){chosen.push({row});continue;}
      const runtime=await stageHouseRobot(row),m=runtime.manifest();
      const rulesets=Array.isArray(m.supportedRulesets)?m.supportedRulesets:['classic'];
      if(!(m.supportedModes||[]).includes(mode))throw new Error(`${m.name} does not play ${mode} (supports: ${(m.supportedModes||[]).join(', ')||'none'}). Run clunkyard opponents to see what each plays.`);
      if(!rulesets.includes(ruleset))throw new Error(`${m.name} does not play ${ruleset} (supports: ${rulesets.join(', ')}). Run clunkyard opponents to see what each plays.`);
      chosen.push({row,runtime});
    }
  }
  const baseEnvironment={...process.env};
  const self=await prepare();
  fs.mkdirSync(path.join(root,'replays'),{recursive:true});
  const replay=path.join(root,'replays',`trial-${seed}.json`);
  const rivals=[];
  for(const {row,runtime} of chosen.length?chosen:[{row:{practice:true}}]){
    if(row.practice)rivals.push({file:path.join(engine,'bots','practice','bot.json'),manifest:null,fuel:null,resourceMode:'open',dsl:true});
    else{console.log(`Preparing opponent ${row.name} (${row.language}).`);rivals.push(await runtime.prepare({opponent:true}));}
  }
  // Each robot's own builder set the process environment for its SDK; the
  // robots launched below work out theirs again from their own projects.
  if(chosen.length){for(const key of Object.keys(process.env))if(!Object.hasOwn(baseEnvironment,key))delete process.env[key];Object.assign(process.env,baseEnvironment);console.log(`Playing ${self.manifest.name} against ${chosen.map(({row})=>row.name).join(', ')}.`);}
  const entries=[self,...rivals],fuel=self.fuel;
  if(entries.every(value=>value.resourceMode==='open')){
    run(process.execPath,[path.join(engine,'src','cli.js'),'match',...entries.flatMap(value=>['--bot','@'+value.file]),'--mode',mode,'--ruleset',ruleset,'--resource-mode','open','--seed',seed,'--ticks','360','--turn-ms','2000','--startup-ms','60000','--replay',replay]);
  }else{
    // The pinned engine already supports mixed entries: each robot keeps its
    // own meter (the player's Wasm or DSL fuel, a DSL rival's operations, an
    // Open robot's none). An Open robot makes the match class Open.
    const {resolveBot,validateSelection}=require(path.join(engine,'src','roster.js'));
    const {runMatch,saveReplay}=require(path.join(engine,'src','match.js'));
    const policies=entries.map(value=>value.fuel?.entry||{mode:value.resourceMode==='open'&&value.dsl?'dsl':value.resourceMode});
    const bots=entries.map((value,index)=>{const bot=resolveBot('@'+value.file,[],engine,mode,ruleset);bot.resourcePolicy=policies[index];return bot;});
    validateSelection(bots,mode,ruleset);
    const matchClass=entries.some(value=>value.fuel?.matchClass==='open')||policies.some(policy=>policy.mode==='open')?'open':'reflex';
    const result=await runMatch(bots,{mode,ruleset,seed:Number(seed),maxTicks:360,turnMs:2000,startupMs:60000,resourcePolicy:{v:3,class:matchClass,policy:'fair-compute-v1'}});
    saveReplay(result.replay,replay);
    console.log(`Result: ${result.result?.winners?.join(', ')||'draw'} (${result.result?.reason||'unknown'})`);
    if(fuel){
      const turns=result.replay.frames.flatMap(frame=>frame.telemetry?.decisions||[]).filter(decision=>decision.botId===bots[0].id).map(decision=>decision.resources?.usage?.wasmFuel).filter(Number.isFinite).sort((a,b)=>a-b);
      const limit=fuel.entry.limits.wasmFuelPerTurn,format=value=>Math.round(value).toLocaleString('en-CA');
      if(turns.length)console.log(`Fuel: ${format(turns[Math.floor(turns.length/2)])} a turn at the median, ${format(turns.at(-1))} at most, of ${format(limit)} (${fuel.label}).`);
    }
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
// The WebAssembly toolchain a target's build uses, as rewrite() resolves it.
function wasmChecks(target){
  if(!['c-wasi','cpp-wasm','hew-wasm','rust-wasi','zig-wasi'].includes(target))return [];
  const tools=wasiTools(),checks=[],file=(label,name,hint)=>checks.push({label,ok:fs.existsSync(name),hint});
  const clang=tools.executables[target==='cpp-wasm'?'clang++-22':'clang-22'];
  if(['c-wasi','cpp-wasm','hew-wasm'].includes(target)){
    file('WASI sysroot',path.join(tools.sysroot,'lib','wasm32-wasip1','libc.a'),'Install wasi-sdk 33 and set WASI_SDK_PATH, or set WASI_SYSROOT to its share/wasi-sysroot.');
  }
  if(['c-wasi','cpp-wasm'].includes(target)){
    // Clang links the wasm32-wasip1 compiler-rt builtins, which wasi-sdk carries.
    const result=spawnSync(clang,['--target=wasm32-wasip1','-print-libgcc-file-name'],{encoding:'utf8',timeout:10000});
    checks.push({label:`${clang} for wasm32-wasip1`,ok:result.status===0&&fs.existsSync(String(result.stdout).trim()),hint:'Use the clang from wasi-sdk 33 (WASI_SDK_PATH), or install its libclang_rt builtins for wasm32-wasip1 into your Clang 22.'});
  }
  if(target==='hew-wasm'){
    checks.push({label:'hew',ok:toolAvailable('hew'),hint:'Install Hew 0.10.0 or later.'});
    file('Hew wasm32-wasip1 runtime',path.join(tools.hew,'lib','wasm32-wasip1','libhew_runtime.a'),'Install a Hew distribution with its wasm32-wasip1 libraries, or set HEW_ROOT to it.');
    checks.push({label:tools.executables['wasm-ld-22'],ok:path.isAbsolute(tools.executables['wasm-ld-22'])?fs.existsSync(tools.executables['wasm-ld-22']):toolAvailable(tools.executables['wasm-ld-22']),hint:'Install wasm-ld (lld), or wasi-sdk 33 with WASI_SDK_PATH.'});
  }
  if(target==='rust-wasi'){
    const result=spawnSync(process.env.RUSTC||'rustc',['--print','target-libdir','--target','wasm32-wasip1'],{encoding:'utf8',timeout:10000});
    checks.push({label:'Rust wasm32-wasip1 target',ok:result.status===0&&fs.existsSync(String(result.stdout).trim()),hint:'Run rustup target add wasm32-wasip1.'});
  }
  return checks;
}
function diagnose(){
  const lines=[],problems=[],log=text=>lines.push(text),fail=text=>{problems.push(text);lines.push(text);};
  log(`Clunkyard SDK ${lock.sdkVersion}; ${lock.language}; ${process.platform}/${process.arch}; Node ${process.versions.node}`);
  let missing=0,m=null;try{m=manifest();}catch{}
  const target=m?buildTarget(m):lock.language,wasm=m&&target!==m.language;
  if(m&&target!==hostedTarget(m))log(`Hosted builds compile this robot to WebAssembly (${hostedTarget(m)}); this platform's SDK has no WebAssembly host, so local builds are native.`);
  else if(wasm)log(`Builds to WebAssembly (${target}) and runs metered by fuel, as hosted. Set "target": "native" in bot.json for a native build.`);
  else if(m?.target==='native'&&hostedTarget({...m,target:undefined})!==m.language)log('bot.json asks for a native build: it plays Open, and Standard refuses it.');
  // A WebAssembly build needs no native C or C++ compiler for these languages.
  const native=tool=>!(wasm&&['c','cpp','rust'].includes(lock.language)&&['cc','c++'].includes(tool));
  for(const tool of lock.tools.filter(native).filter(tool=>!(process.platform==='win32'&&lock.language==='rust'&&tool==='cc')).filter(tool=>tool!=='make'||(!npmSDK&&lock.rebuildBindings?.some(binding=>!sdkAsset?.portableBindingInstallers?.includes(binding)&&!fs.existsSync(path.join(sdk,'binding-src',binding,'install.cjs')))))){
    const command=process.platform==='win32'?({python3:process.env.PYTHON||'python',cc:cCompiler(false),'c++':cCompiler(true)}[tool]||tool):tool;
    const ok=toolAvailable(command);
    (ok?log:fail)(`${ok?'found':'missing'} ${command}`);if(!ok)missing++;
  }
  for(const check of lock.requirements?.checks||[]){
    let command=[process.platform==='win32'&&check.command==='python3'?(process.env.PYTHON||'python'):check.command];
    if(npmSDK&&check.command==='tsc'){try{command=typescriptCompiler()}catch{}}
    const result=spawnSync(command[0],[...command.slice(1),...check.args],{encoding:'utf8',env:environment(),timeout:10000});
    const output=String(result.stdout||'')+String(result.stderr||'');
    const version=check.versionPattern?Number(output.match(new RegExp(check.versionPattern))?.[1]):null;
    const ok=!result.error&&result.status===0&&(!check.minimum||version>=check.minimum)&&(!check.outputPattern||new RegExp(check.outputPattern).test(output));
    (ok?log:fail)(`${ok?'ok':'missing'} ${check.label}${ok?'':': '+check.hint}`);if(!ok)missing++;
  }
  for(const check of wasmChecks(target)){(check.ok?log:fail)(`${check.ok?'ok':'missing'} ${check.label}${check.ok?'':': '+check.hint}`);if(!check.ok)missing++;}
  if(lock.language==='clojure'){try{log('ok Clojure runtime: '+clojureClasspath())}catch(error){fail('missing '+error.message);missing++}}
  if(lock.language==='scala'&&process.env.SCALA_LIBRARY_JAR&&!fs.existsSync(process.env.SCALA_LIBRARY_JAR)){fail('missing SCALA_LIBRARY_JAR file');missing++}
  if(lock.language==='scala'&&!process.env.SCALA_LIBRARY_JAR&&!fs.existsSync('/usr/share/scala-2.11/lib/scala-library.jar')&&spawnSync('scala',['-version'],{timeout:10000}).error){fail('missing Scala runtime; set SCALA_LIBRARY_JAR or install the scala launcher.');missing++}
  try{checkNativePlatform()}catch(error){fail(error.message);missing++}
  for(const note of lock.requirements?.notes||[])log(note);
  log(npmSDK||lock.portable?'Portable Node route.':`Native SDK: ${process.platform}-${process.arch}${suppliedSdk?' (user-provided)':sdkAsset?.minimumOS?'; minimum OS '+sdkAsset.minimumOS:''}.`);
  log('No toolchains are downloaded automatically. See README.md for official installers.');
  return {lines,problems,missing};
}
function doctor(){
  const {lines,missing}=diagnose();
  for(const line of lines)console.log(line);
  if(missing)process.exitCode=1;
}
async function main(action='build',args=[]){
  if(action==='doctor')return doctor();
  safeCache();
  if(action==='setup')return setup();
  if(action==='build')return build();
  if(action==='stdio')return stdio();
  if(action==='match')return match(args);
  if(action==='opponents')return opponents(args);
  if(action==='replay'){await setup();return run(process.execPath,[path.join(engine,'src','cli.js'),'replay',path.resolve(root,args[0]||'replays/trial-42.json'),'--port',args[1]||'8787']);}
  if(action==='clean'){fs.rmSync(path.join(cache,'build'),{recursive:true,force:true});fs.rmSync(path.join(cache,'bindings-ready'),{force:true});return;}
  throw new Error('Use doctor, setup, build, match [seed] [2d|3d] [ruleset] [--opponent NAME], opponents, replay [file] [port], or clean.');
}

return {main,manifest,diagnose,prepare};
}

module.exports={createRuntime,selectLocalResourceMode,selectPlatformSdk,recipeInvocation,bindingCompilerIdentity};
