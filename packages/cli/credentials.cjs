'use strict';
// The agent keys the CLI saves when it signs in, one per Clunkyard server,
// and the same keys in Git's credential store format for clones of robots
// (`credential.<origin>.helper store --file=...`). Both files are the
// owner's alone.

const fs=require('node:fs'),os=require('node:os'),path=require('node:path');
const DEFAULT_ORIGIN='https://clunkyard.com';

function credentialPath(env=process.env){
  const root=process.platform==='win32'?env.APPDATA||path.join(os.homedir(),'AppData','Roaming'):env.XDG_CONFIG_HOME||path.join(os.homedir(),'.config');
  return path.join(root,'clunkyard','credentials.json');
}
function siteOrigin(value){
  const url=new URL(value);
  if(url.username||url.password||url.protocol!=='https:'&&!(url.protocol==='http:'&&['localhost','127.0.0.1','[::1]'].includes(url.hostname)))throw new Error('Use an HTTPS Clunkyard address.');
  return url.origin;
}
// The saved key for site, or null. An environment key counts only for its own origin.
function savedKey(site,env=process.env){
  if(env.CLUNKYARD_AGENT_KEY&&site===siteOrigin(env.CLUNKYARD_ORIGIN||DEFAULT_ORIGIN))return env.CLUNKYARD_AGENT_KEY;
  const file=credentialPath(env);let stat;
  try{stat=fs.lstatSync(file);}catch(error){if(error.code==='ENOENT')return null;throw error;}
  if(!stat.isFile()||process.platform!=='win32'&&(stat.mode&0o077||stat.uid!==process.getuid()))throw new Error(`Clunkyard credentials at ${file} must be a private, owner-only file.`);
  const value=JSON.parse(fs.readFileSync(file,'utf8'));
  return typeof value?.[site]==='string'?value[site]:null;
}

function gitCredentialPath(env=process.env){return path.join(path.dirname(credentialPath(env)),'git-credentials');}
// Saves key for site in both files, replacing what was saved for it.
function saveKey(site,key,env=process.env){
  const file=credentialPath(env),git=gitCredentialPath(env);fs.mkdirSync(path.dirname(file),{recursive:true,mode:0o700});
  let saved={};try{saved=JSON.parse(fs.readFileSync(file,'utf8'));}catch(error){if(error.code!=='ENOENT')throw error;}
  saved[site]=key;fs.writeFileSync(file,JSON.stringify(saved,null,2)+'\n',{mode:0o600});fs.chmodSync(file,0o600);
  const url=new URL(site),line=`${url.protocol}//clunkyard:${encodeURIComponent(key)}@${url.host}`;
  let lines=[];try{lines=fs.readFileSync(git,'utf8').split('\n').filter(Boolean);}catch(error){if(error.code!=='ENOENT')throw error;}
  lines=lines.filter(item=>{try{const old=new URL(item);return old.host!==url.host||old.protocol!==url.protocol;}catch{return false;}});
  fs.writeFileSync(git,[...lines,line].join('\n')+'\n',{mode:0o600});fs.chmodSync(git,0o600);
}

module.exports={DEFAULT_ORIGIN,credentialPath,gitCredentialPath,siteOrigin,savedKey,saveKey};
