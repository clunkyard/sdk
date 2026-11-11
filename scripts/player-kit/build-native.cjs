#!/usr/bin/env node
'use strict';

// Release input preparation only. Player projects download the resulting SDK;
// they do not need Cargo unless Rust is their selected language.
const fs=require('node:fs'),path=require('node:path'),os=require('node:os');
const {execFileSync}=require('node:child_process');
const option=name=>{const i=process.argv.indexOf(name);return i<0?null:process.argv[i+1]};
const source=path.resolve(option('--source')||path.join(__dirname,'../..'));
const sourceDirty=Boolean(execFileSync('git',['status','--porcelain'],{cwd:source,encoding:'utf8'}).trim());
if(sourceDirty&&!process.argv.includes('--allow-dirty'))throw new Error('Commit native SDK source first, or use --allow-dirty for a nonpublishable preview.');
const output=option('--out');
if(!output)throw new Error('Use --out NEW_SDK_DIRECTORY [--build-dir DIRECTORY] [--bindings c,python,rust]');
const sdk=path.resolve(output),platform=process.platform,arch=process.arch;
const files={linux:{shared:'libclunkyard.so',installed:'libclunkyard.so.2',static:'libclunkyard.a'},darwin:{shared:'libclunkyard.dylib',installed:'libclunkyard.2.dylib',static:'libclunkyard.a'},win32:{shared:'clunkyard.dll',installed:'clunkyard.dll',static:'clunkyard.lib',import:'clunkyard.dll.lib'}}[platform];
if(!files)throw new Error(`No native SDK packaging recipe for ${platform}.`);
if(fs.existsSync(sdk))throw new Error('Choose a new SDK directory; existing release inputs are never overwritten.');
const bindings=(option('--bindings')||'').split(',').filter(Boolean);
for(const binding of bindings)if(!/^[a-z][a-z0-9-]*$/.test(binding)||!fs.existsSync(path.join(source,'sdk/bindings',binding,'Makefile')))throw new Error(`Unknown binding: ${binding}`);
const build=path.resolve(option('--build-dir')||fs.mkdtempSync(path.join(os.tmpdir(),'clunkyard-native-core-')));
const env={...process.env,...(platform==='darwin'?{MACOSX_DEPLOYMENT_TARGET:process.env.MACOSX_DEPLOYMENT_TARGET||'11.0'}:{})};
const rustCompiler=execFileSync(env.RUSTC||'rustc',['--version'],{env,encoding:'utf8'}).trim();
const rustVersion=/^rustc (\S+)/.exec(rustCompiler)?.[1];
if(!rustVersion)throw new Error('The native SDK build needs an identifiable Rust compiler.');
// Cargo discovers the vendored-source configuration from its working directory.
execFileSync('cargo',['build','--manifest-path',path.join(source,'sdk/core/Cargo.toml'),'--release','--locked','--offline','--lib','--target-dir',build],{cwd:path.join(source,'sdk/core'),env,stdio:'inherit'});
fs.mkdirSync(path.join(sdk,'lib'),{recursive:true});fs.mkdirSync(path.join(sdk,'include'),{recursive:true});
fs.copyFileSync(path.join(source,'sdk/core/include/clunkyard.h'),path.join(sdk,'include/clunkyard.h'));
for(const [from,to] of [[files.shared,files.installed],[files.static,files.static]])fs.copyFileSync(path.join(build,'release',from),path.join(sdk,'lib',to));
if(files.import)fs.copyFileSync(path.join(build,'release',files.import),path.join(sdk,'lib',files.import));
// Keep the compiler's ordinary -lclunkyard link name without archive symlinks.
if(files.shared!==files.installed)fs.copyFileSync(path.join(build,'release',files.shared),path.join(sdk,'lib',files.shared));
fs.copyFileSync(path.join(source,'LICENSE'),path.join(sdk,'LICENSE'));
fs.writeFileSync(path.join(sdk,'VERSION'),'2.0.0\n');
const metadata={sdkVersion:'2.0.0',platform:`${platform}-${arch}`,arch,library:files.installed,staticLibrary:files.static,rustVersion,rustCompiler,sourceCommit:execFileSync('git',['rev-parse','HEAD'],{cwd:source,encoding:'utf8'}).trim(),sourceDirty,...(platform==='darwin'?{minimumOS:env.MACOSX_DEPLOYMENT_TARGET}:{})};
fs.writeFileSync(path.join(sdk,'platform.json'),JSON.stringify(metadata,null,2)+'\n');
for(const binding of bindings){
  const directory=path.join(source,'sdk/bindings',binding);
  fs.cpSync(directory,path.join(sdk,'binding-src',binding),{recursive:true,filter:file=>!['target','node_modules','__pycache__','bin','obj'].includes(path.basename(file))});
  const installer=path.join(directory,'install.cjs');
  if(fs.existsSync(installer))execFileSync(process.execPath,[installer,sdk],{env,stdio:'inherit'});
  else execFileSync('make',['-C',directory,'install',`SDK=${sdk}`,`ROOT=${source}`,`BUILD=${path.join(build,'binding-'+binding)}`],{env,stdio:'inherit'});
}
console.log(JSON.stringify({sdk,build,...metadata,bindings},null,2));
