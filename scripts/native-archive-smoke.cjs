#!/usr/bin/env node
'use strict';

// A hash-reviewed public bundle provides the exact release/CLI bytes. This job
// installs no game credentials, runs no service and cannot publish a release.
const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto');
const {execFileSync}=require('node:child_process');
const hash=bytes=>crypto.createHash('sha256').update(bytes).digest('hex');
function inputs(environment){
  const tag=String(environment.NATIVE_RELEASE_TAG||''),name=String(environment.NATIVE_BUNDLE_NAME||''),sha256=String(environment.NATIVE_BUNDLE_SHA256||'').toLowerCase();
  if(!/^[A-Za-z0-9][A-Za-z0-9._-]{0,127}$/.test(tag)||!/^[A-Za-z0-9][A-Za-z0-9._-]{0,127}\.tar\.gz$/.test(name)||!/^[a-f0-9]{64}$/.test(sha256))throw Error('Supply a simple release tag, .tar.gz asset name and exact SHA-256.');
  return {tag,name,sha256};
}
function unpack(file,expected,destination,extract){
  if(fs.statSync(file).size>64*1024*1024)throw Error('Qualification bundle exceeds 64 MiB.');
  const bytes=fs.readFileSync(file);if(hash(bytes)!==expected)throw Error('Qualification bundle SHA-256 differs from the reviewed bytes.');
  if(fs.existsSync(destination))throw Error('Choose a new bundle directory.');
  fs.mkdirSync(destination);extract(bytes,destination,256*1024*1024);
  const allowed=name=>/^release\/assets\/(?:catalogue\.json|SHA256SUMS|[A-Za-z0-9][A-Za-z0-9._-]*\.tar\.gz)$/.test(name)||/^tools\/player-cli\/(?:cli\.cjs|runtime\.cjs|project\.cjs|archive\.cjs|catalogue\.json|package\.json|README\.md|LICENSE)$/.test(name)||/^scripts\/player-kit\/(?:validate|verify|archive)\.cjs$/.test(name);
  function scan(directory,prefix=''){for(const entry of fs.readdirSync(directory,{withFileTypes:true})){const name=prefix+entry.name,file=path.join(directory,entry.name);if(entry.isDirectory())scan(file,name+'/');else if(!entry.isFile()||!allowed(name))throw Error('Unexpected qualification bundle file: '+name);}}
  scan(destination);
  for(const name of ['release/assets/catalogue.json','release/assets/SHA256SUMS','tools/player-cli/cli.cjs','tools/player-cli/runtime.cjs','tools/player-cli/project.cjs','tools/player-cli/archive.cjs','tools/player-cli/catalogue.json','tools/player-cli/package.json','tools/player-cli/LICENSE','scripts/player-kit/validate.cjs','scripts/player-kit/verify.cjs','scripts/player-kit/archive.cjs'])if(!fs.statSync(path.join(destination,name)).isFile())throw Error('Missing qualification input: '+name);
  return {sha256:hash(bytes),bytes:bytes.length};
}
function collect(results,artifacts){
  if(!fs.existsSync(results))return;
  for(const entry of fs.readdirSync(results,{withFileTypes:true})){
    const file=path.join(results,entry.name);
    if(entry.isFile()&&(entry.name==='results.json'||entry.name.endsWith('.log')))fs.copyFileSync(file,path.join(artifacts,entry.name));
    if(entry.isDirectory()){
      const replay=path.join(file,'replays/trial-42.json');if(fs.existsSync(replay))fs.copyFileSync(replay,path.join(artifacts,entry.name+'-42.json'));
    }
  }
}
function main(){
  const index=process.argv.indexOf('--out'),output=index>=0?process.argv[index+1]:null;if(!output)throw Error('Use --out NEW_DIRECTORY.');
  if(process.platform!=='darwin'||process.arch!=='x64')throw Error('Exact Intel qualification must run on darwin-x64.');
  const expected=inputs(process.env),root=path.resolve(output),artifacts=path.join(root,'artifacts');if(fs.existsSync(root))throw Error('Choose a new qualification output directory.');fs.mkdirSync(artifacts,{recursive:true});
  const receipt={v:1,checkedAt:new Date().toISOString(),platform:'darwin-x64',repository:'clunkyard/sdk',release:expected.tag,bundle:expected.name,expectedSHA256:expected.sha256,complete:false};
  const save=()=>fs.writeFileSync(path.join(artifacts,'bundle-qualification.json'),JSON.stringify(receipt,null,2)+'\n');save();
  const results=path.join(root,'results');
  try{
    const release=JSON.parse(execFileSync('gh',['release','view',expected.tag,'--repo','clunkyard/sdk','--json','isPrerelease,isDraft,assets'],{encoding:'utf8'}));
    const asset=release.assets.find(asset=>asset.name===expected.name);
    if(!release.isPrerelease||release.isDraft||!asset||asset.size>64*1024*1024)throw Error('Use a reviewed public prerelease bundle in clunkyard/sdk.');
    const downloads=path.join(root,'download');fs.mkdirSync(downloads);execFileSync('gh',['release','download',expected.tag,'--repo','clunkyard/sdk','--pattern',expected.name,'--dir',downloads],{stdio:'inherit'});
    const {extract}=require(path.resolve(__dirname,'../packages/cli/archive.cjs')),bundle=path.join(root,'bundle');
    Object.assign(receipt,unpack(path.join(downloads,expected.name),expected.sha256,bundle,extract));save();
    const environment={...process.env};for(const key of Object.keys(environment))if(/^CLUNKYARD_/i.test(key)||/^(?:GH_TOKEN|GITHUB_TOKEN)$/i.test(key))delete environment[key];
    const log=fs.openSync(path.join(artifacts,'validation.log'),'w');try{execFileSync(process.execPath,[path.join(bundle,'scripts/player-kit/validate.cjs'),'--release',path.join(bundle,'release'),'--out',results,'--native','--cli',path.join(bundle,'tools/player-cli')],{cwd:bundle,env:environment,stdio:['ignore',log,log],timeout:12*60*1000});}finally{fs.closeSync(log);collect(results,artifacts);}
    const {verifyRelease}=require(path.join(bundle,'scripts/player-kit/verify.cjs')),{catalogue}=verifyRelease(path.join(bundle,'release'),path.join(results,'results.json'));
    Object.assign(receipt,{complete:true,source:catalogue.source,engineSHA256:catalogue.engine.sha256,languages:catalogue.languages.map(row=>row.language)});save();console.log(JSON.stringify(receipt,null,2));
  }catch(error){receipt.error=error.message;save();throw error;}
}
if(require.main===module)try{main()}catch(error){console.error(error.message);process.exitCode=1}
module.exports={inputs,unpack};
