#!/usr/bin/env node
'use strict';

// Publish the once-packed artifacts after every supported CI consumer passes.
// npm exchanges this main-branch job's OIDC identity; no registry token is used.
const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto');
const {execFileSync}=require('node:child_process');
const {isDeepStrictEqual}=require('node:util');
const names=['@clunkyard/git','@clunkyard/sdk','@clunkyard/cli'];
const matrix=[['ubuntu-24.04','linux'],['macos-latest','darwin'],['windows-latest','win32']].flatMap(([label,platform])=>[20,24].map(node=>({directory:`qualification-${label}-node${node}`,platform,node})));
const json=file=>JSON.parse(fs.readFileSync(file,'utf8'));
const digest=(data,algorithm='sha256')=>crypto.createHash(algorithm).update(data).digest(algorithm==='sha256'?'hex':'base64');
function ordinary(file){if(!fs.lstatSync(file).isFile()||fs.lstatSync(file).isSymbolicLink())throw Error('Release inputs must be ordinary files: '+file);return fs.readFileSync(file);}
function assertTrustedWorkflow(env){
  if(env.GITHUB_ACTIONS!=='true'||env.GITHUB_REPOSITORY!=='clunkyard/sdk'||env.GITHUB_REF!=='refs/heads/main'||env.GITHUB_EVENT_NAME!=='workflow_dispatch'||env.GITHUB_WORKFLOW_REF!=='clunkyard/sdk/.github/workflows/npm-release.yml@refs/heads/main'||env.NPM_RELEASE_ENVIRONMENT!=='npm-release'||!env.ACTIONS_ID_TOKEN_REQUEST_URL||!env.ACTIONS_ID_TOKEN_REQUEST_TOKEN)throw Error('Use the main-branch npm-release GitHub Actions job with OIDC.');
  if(env.NPM_TOKEN||env.NODE_AUTH_TOKEN)throw Error('The release job uses OIDC, not an npm authentication token.');
}
function checkArchive(root,value){
  const data=ordinary(path.join(root,'dist',value.filename));
  if(data.length!==value.bytes||digest(data)!==value.sha256||'sha512-'+digest(data,'sha512')!==value.integrity)throw Error('Package changed after qualification: '+value.name);
}
function reviewedRelease(root,selection){
  const source=json(path.join(root,'release-source.json')),manifest=json(path.join(root,'dist/packages.json'));
  if(source.sourceDirty!==false||!(/^[a-f0-9]{40}$/.test(source.sourceCommit))||source.sourceCommit!==manifest.sourceCommit||!Array.isArray(source.files))throw Error('The package manifest does not match a clean source export.');
  for(const file of source.files){
    if(typeof file.path!=='string'||path.isAbsolute(file.path)||file.path.includes('\\')||!path.resolve(root,file.path).startsWith(path.resolve(root)+path.sep))throw Error('Invalid source input path.');
    const data=ordinary(path.join(root,file.path));if(data.length!==file.bytes||digest(data)!==file.sha256)throw Error('Export content changed: '+file.path);
  }
  if(!Array.isArray(manifest.packages)||manifest.packages.length!==names.length||!isDeepStrictEqual([...manifest.packages.map(p=>p.name)].sort(),[...names].sort()))throw Error('The release must contain exactly the known player packages.');
  const sourcePaths=new Set(source.files.map(file=>file.path));
  for(const value of manifest.packages){
    const key=value.name.split('/')[1];
    if(!sourcePaths.has(`packages/${key}/package.json`))throw Error('Package metadata is absent from the source export.');
    if(!/^\d+\.\d+\.\d+$/.test(value.version)||value.filename!==`clunkyard-${key}-${value.version}.tgz`)throw Error('Unexpected package identity.');
    const metadata=json(path.join(root,'packages',key,'package.json'));
    if(metadata.name!==value.name||metadata.version!==value.version||metadata.private||metadata.scripts||Object.keys(metadata.dependencies||{}).length||metadata.publishConfig?.access!=='public')throw Error('Package metadata differs from the reviewed release: '+value.name);
    checkArchive(root,value);
  }
  const expected=manifest.packages.map(({name,version,sha256})=>({name,version,sha256})),directory=path.join(root,'dist/qualifications');
  if(!isDeepStrictEqual(fs.readdirSync(directory).sort(),matrix.map(row=>row.directory).sort()))throw Error('Exactly six platform qualification receipts are required.');
  for(const row of matrix){
    const receipt=json(path.join(directory,row.directory,'qualification.json'));
    if(receipt.passed!==true||receipt.sourceCommit!==source.sourceCommit||receipt.platform!==row.platform||!new RegExp(`^v${row.node}\\.`).test(receipt.node)||!isDeepStrictEqual(receipt.packages,expected))throw Error('Qualification receipt does not match the packed artifacts: '+row.directory);
  }
  const selected=selection===undefined||selection===''?names:selection.split(',');
  if(!selected.length||new Set(selected).size!==selected.length||selected.some(name=>!names.includes(name)))throw Error('Select known package names without empty or duplicate entries.');
  return{manifest,selected:manifest.packages.filter(value=>selected.includes(value.name))};
}
async function registryVersion(value,fetchImpl,timeoutMs=30000){
  const signal=AbortSignal.timeout(Math.max(1,Math.ceil(timeoutMs)));
  let response,published;
  try{
    response=await fetchImpl(`https://registry.npmjs.org/${encodeURIComponent(value.name)}/${value.version}`,{cache:'no-store',redirect:'error',signal});
    if(response.ok)published=await response.json();
  }catch(error){
    if(error instanceof SyntaxError)throw error;
    const code=error.cause?.code||error.code;
    if((signal.aborted&&signal.reason?.name==='TimeoutError')||error.name==='TimeoutError'||['EAI_AGAIN','ENOTFOUND','ECONNRESET','ECONNREFUSED','ETIMEDOUT','ENETUNREACH','UND_ERR_CONNECT_TIMEOUT','UND_ERR_SOCKET'].includes(code))error.registryRetryable=true;
    throw error;
  }
  if(response.status===404)return false;
  if(!response.ok){const error=Error('Cannot check npm version: HTTP '+response.status);error.registryRetryable=[429,500,502,503,504].includes(response.status);throw error;}
  if(published?.name!==value.name||published.version!==value.version||published.dist?.integrity!==value.integrity)throw Error(`Published bytes differ for ${value.name}@${value.version}; choose a new version.`);
  return true;
}
async function waitForPublicVersion(value,fetchImpl,{timeoutMs=300000,intervalMs=5000,requestTimeoutMs=30000,pause=ms=>new Promise(resolve=>setTimeout(resolve,ms)),now=Date.now}={}){
  const deadline=now()+timeoutMs;
  const pending=()=>{const error=Error(`npm accepted ${value.name}@${value.version}, but public registry visibility is still pending; do not repeat publication.`);error.code='PUBLICATION_PENDING';return error;};
  for(;;){
    let remaining=deadline-now();if(remaining<=0)throw pending();
    try{if(await registryVersion(value,fetchImpl,Math.min(requestTimeoutMs,remaining)))return;}
    catch(error){if(!error.registryRetryable)throw error;}
    remaining=deadline-now();if(remaining<=0)throw pending();
    await pause(Math.min(intervalMs,remaining));
  }
}
async function publishReviewed({root,selection,apply=false,env=process.env,fetchImpl=fetch,run=execFileSync,visibility}){
  assertTrustedWorkflow(env);if(apply&&!env.npm_execpath)throw Error('Use npm exec so the release npm CLI is explicit.');const {manifest,selected}=reviewedRelease(root,selection),results=[];
  const receipt={v:1,sourceCommit:manifest.sourceCommit,checkedAt:new Date().toISOString(),run:`https://github.com/${env.GITHUB_REPOSITORY}/actions/runs/${env.GITHUB_RUN_ID}`,apply,complete:false,results};
  const save=()=>fs.writeFileSync(path.join(root,'dist/publication.json'),JSON.stringify(receipt,null,2)+'\n');
  // Preflight the whole selection before the first immutable registry write.
  const published=await Promise.all(selected.map(value=>registryVersion(value,fetchImpl)));
  save();
  for(let i=0;i<selected.length;i++){
    const value=selected[i];
    if(published[i])results.push({...value,status:'already-published'});
    else if(!apply)results.push({...value,status:'ready'});
    else{
      checkArchive(root,value);
      const result={...value,status:'publishing'};results.push(result);save();
      try{
        run(process.execPath,[env.npm_execpath,'publish',path.join(root,'dist',value.filename),'--access','public','--tag','latest','--provenance','--ignore-scripts','--registry=https://registry.npmjs.org/'],{cwd:root,env,stdio:'inherit'});
        result.status='submitted';result.submittedAt=new Date().toISOString();save();
        // npm may accept the immutable write before its public metadata appears.
        // Retry only reads; never retry npm publish inside this operation.
        await waitForPublicVersion(value,fetchImpl,visibility);
        result.status='published';result.confirmedAt=new Date().toISOString();
      }catch(error){result.status=error.code==='PUBLICATION_PENDING'?'pending':'unconfirmed';result.error={code:error.code||'PUBLICATION_UNCONFIRMED',message:error.message.slice(0,512)};save();throw error;}
    }
    save();
  }
  receipt.complete=true;save();return receipt;
}
if(require.main===module){const index=process.argv.indexOf('--package'),selection=index<0?process.env.NPM_RELEASE_PACKAGES:process.argv[index+1];if(index>=0&&!selection)throw Error('--package requires known package names.');publishReviewed({root:path.resolve(__dirname,'..'),selection,apply:process.argv.includes('--apply')}).then(receipt=>{for(const row of receipt.results)console.log(`${row.status}: ${row.name}@${row.version} (${row.sha256})`);}).catch(error=>{console.error(error.message);process.exitCode=1});}
module.exports={matrix,assertTrustedWorkflow,reviewedRelease,publishReviewed,waitForPublicVersion};
