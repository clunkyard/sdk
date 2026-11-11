#!/usr/bin/env node
'use strict';
const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto'),{execFileSync}=require('node:child_process');
const root=path.resolve(__dirname,'..'),output=path.join(root,'dist');
const hash=data=>crypto.createHash('sha256').update(data).digest('hex');
const source=JSON.parse(fs.readFileSync(path.join(root,'release-source.json')));
if(source.sourceDirty)throw Error('An uncommitted preview export cannot be released.');
for(const file of source.files){const data=fs.readFileSync(path.join(root,file.path));if(data.length!==file.bytes||hash(data)!==file.sha256)throw Error('Export content changed: '+file.path);}
fs.mkdirSync(output,{recursive:true});
const npm=process.env.npm_execpath;if(!npm)throw Error('Run npm run pack so the active npm CLI is explicit.');
const packages=[];
for(const name of source.packages){
  const directory=path.join(root,'packages',name),metadata=JSON.parse(fs.readFileSync(path.join(directory,'package.json')));
  if(metadata.private||metadata.name!=='@clunkyard/'+name||Object.keys(metadata.dependencies||{}).length||metadata.scripts||metadata.publishConfig?.access!=='public')throw Error('Unexpected package metadata.');
  const result=JSON.parse(execFileSync(process.execPath,[npm,'pack','--ignore-scripts','--json','--pack-destination',output],{cwd:directory,encoding:'utf8'}))[0];
  const names=result.files.map(file=>file.path);
  const reviewed=new Set(source.files.filter(file=>file.path.startsWith(`packages/${name}/`)).map(file=>file.path.slice(`packages/${name}/`.length)));
  for(const file of names)if(!reviewed.has(file))throw Error('Unreviewed packed file: '+file);
  const allowed=new Set(['package.json',...metadata.files.filter(file=>!file.endsWith('/'))]);
  for(const file of names)if(!allowed.has(file)&&!(metadata.files.includes('licenses/')&&file.startsWith('licenses/')))throw Error('Unexpected packed file: '+file);
  for(const file of metadata.files.filter(file=>!file.endsWith('/')))if(!names.includes(file))throw Error('Missing packed file: '+file);
  if(!names.includes('LICENSE')||names.some(file=>/(?:^|\/)(?:\.npmrc|\.env|credentials\.json|node_modules)(?:$|\/)/.test(file)))throw Error('Invalid licence or private package content.');
  const data=fs.readFileSync(path.join(output,result.filename));
  packages.push({name:metadata.name,version:metadata.version,filename:result.filename,bytes:data.length,sha256:hash(data),integrity:result.integrity,files:names});
}
fs.writeFileSync(path.join(output,'packages.json'),JSON.stringify({v:1,sourceCommit:source.sourceCommit,packages},null,2)+'\n');
console.log(JSON.stringify(packages.map(({name,version,filename,bytes,sha256})=>({name,version,filename,bytes,sha256})),null,2));
