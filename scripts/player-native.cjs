#!/usr/bin/env node
'use strict';

// Runs on the actual target runner. A whole-bundle hash and the ordinary
// export manifest bind every compiler input to the canonical clean snapshot.
const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto'),{execFileSync,spawnSync}=require('node:child_process');
const hash=bytes=>crypto.createHash('sha256').update(bytes).digest('hex');
function selectJava(env,{platform=process.platform,exists=fs.existsSync,probe=spawnSync}={}){
  const paths=platform==='win32'?path.win32:path.posix;
  const entries=Object.entries(env).filter(([key])=>/^JAVA_HOME(?:_\d+_(?:X64|ARM64))?$/.test(key));
  for(const [,home]of entries.reverse()){
    const compiler=home&&paths.join(home,'bin',platform==='win32'?'javac.exe':'javac');
    if(!compiler||!exists(compiler))continue;
    const result=probe(compiler,['-version'],{encoding:'utf8',stdio:'pipe',timeout:10000});
    const text=String(result.stdout||'')+String(result.stderr||'');
    if(result.error||result.status!==0||Number(/javac\s+(\d+)/.exec(text)?.[1])<22||!/javac\s+\d+/.test(text))continue;
    const selected={...env,JAVA_HOME:home},javaBin=paths.join(home,'bin');
    if(platform==='win32'){
      // A plain object can contain Path and PATH. Node passes only one of
      // these case-insensitive names on Windows; merge before canonicalising
      // so selecting Java cannot discard Cargo or the other toolchains.
      const keys=Object.keys(selected).filter(key=>key.toUpperCase()==='PATH');
      const values=[javaBin,...keys.flatMap(key=>String(selected[key]||'').split(';'))];
      for(const key of keys)delete selected[key];
      const seen=new Set();selected.PATH=values.filter(value=>{const key=value.toLowerCase();if(seen.has(key))return false;seen.add(key);return true;}).join(';');
    }else selected.PATH=javaBin+(env.PATH?':'+env.PATH:'');
    return selected;
  }
  return env;
}
function main(argv=process.argv.slice(2),environment=process.env){
  const option=name=>{const i=argv.indexOf(name);return i<0?null:argv[i+1]},input=option('--bundle'),out=option('--out');if(!input||!out||!/^[a-f0-9]{64}$/.test(environment.PLAYER_BUNDLE_SHA256||''))throw Error('Use a digest-bound --bundle and new --out directory.');
  const platform=process.platform+'-'+process.arch;if(platform!==environment.PLAYER_PLATFORM)throw Error('Native runner platform differs from the planned cohort.');
  const root=path.resolve(out),source=path.join(root,'source'),output=path.join(root,'output');if(fs.existsSync(root))throw Error('Native output directory already exists.');fs.mkdirSync(source,{recursive:true});fs.mkdirSync(output);
  const data=fs.readFileSync(input);if(data.length>96*1024*1024||hash(data)!==environment.PLAYER_BUNDLE_SHA256)throw Error('Native qualification bundle digest differs.');
  const {extract}=require(fs.existsSync(path.join(__dirname,'../packages/cli/archive.cjs'))?'../packages/cli/archive.cjs':'../../tools/player-cli/archive.cjs');extract(data,source,256*1024*1024);
  const provenance=require(path.join(source,'scripts/npm-release/native-smoke.cjs')).verifyExport(source),config=JSON.parse(fs.readFileSync(path.join(source,'player-assets.json'))),languages=config.platforms[platform];if(!languages?.length)throw Error('No reviewed languages for this platform.');
  let env={...environment,TMPDIR:root};for(const key of Object.keys(env))if(/^(?:CLUNKYARD_|GH_TOKEN$|GITHUB_TOKEN$|NODE_AUTH_TOKEN$|NPM_TOKEN$)/i.test(key))delete env[key];env=selectJava(env);
  const run=(file,args)=>execFileSync(process.execPath,[path.join(source,file),...args],{cwd:source,env,stdio:'inherit',timeout:1800000});
  const installed=path.join(root,'installed-sdk'),release=path.join(output,'release'),results=path.join(output,'validation');
  run('scripts/player-kit/build-native.cjs',['--source',source,'--out',installed,'--build-dir',path.join(root,'core-build'),'--bindings',languages.join(','),'--verified-export']);
  env.CLUNKYARD_RELEASE_DIRECTORY=path.join(source,'release/assets');
  run('scripts/player-kit/build-assets.cjs',['--source',source,'--out',release,'--version',config.version,'--platform',platform,'--languages',languages.join(','),'--installed-sdk',installed,'--reuse-engine','--verified-export']);
  const {readRelease}=require(path.join(source,'scripts/player-kit/verify.cjs')),{mergeCatalogue}=require(path.join(source,'scripts/player-kit/catalogue-from-assets.cjs')),catalogueFile=path.join(source,'tools/player-cli/catalogue.json'),previous=JSON.parse(fs.readFileSync(catalogueFile));
  fs.writeFileSync(catalogueFile,JSON.stringify(mergeCatalogue(previous,readRelease(release),previous.npmSDK.integrity),null,2)+'\n');
  run('scripts/player-kit/validate.cjs',['--release',release,'--out',results,'--native','--cli',path.join(source,'tools/player-cli')]);
  fs.writeFileSync(path.join(output,'native-source.json'),JSON.stringify({...provenance,platform,languages,bundleSHA256:hash(data),passed:true},null,2)+'\n');
}
if(require.main===module)try{main()}catch(error){console.error(error.stack);process.exitCode=1;}
module.exports={main,selectJava};
