'use strict';

// One dependency inventory serves npm, engine and native SDK distributions.
// Compiler-provided libraries are accounted for by the matching Rust inventory.
const fs=require('node:fs'),path=require('node:path');
const dependencies=['libm','serde_core','serde_json','itoa','memchr','zmij'];
function coreNotices(source,{title='Third-party notices for the Clunkyard JavaScript/TypeScript SDK',description='The bundled WebAssembly core is built from the public sdk/core source.',rustVersion='1.96.0',rustInventory=null,target='WebAssembly'}={}){
  const version=String(rustVersion).match(/(?:^rustc\s+)?(\d+\.\d+\.\d+)(?:\s|$)/)?.[1];
  if(!version)throw new Error('Record the Rust compiler version before packaging native libraries.');
  const lock=fs.readFileSync(path.join(source,'sdk/core/Cargo.lock'),'utf8');
  const versions=new Map([...lock.matchAll(/\[\[package\]\]\s+name = "([^"]+)"\s+version = "([^"]+)"/g)].map(m=>[m[1],m[2]]));
  const files=new Map();let notices=title+'\n\n'+description+'\n';
  for(const name of dependencies){
    const directory=path.join(source,'sdk/core/vendor',name),names=fs.readdirSync(directory).filter(file=>/^(?:LICENSE|COPYING|UNLICENSE)/.test(file)).sort();
    if(!versions.has(name)||!names.length)throw new Error(`Missing pinned dependency notice: ${name}`);
    notices+=`\n${name} ${versions.get(name)}\n`;
    for(const file of names){const relative=`licenses/${name}/${file}`;files.set(relative,fs.readFileSync(path.join(directory,file)));notices+=`  ${relative}\n`;}
  }
  const inventoryName=`rust-stdlib-${version}.html`,inventory=rustInventory||path.join(source,'sdk/core/notices',inventoryName);
  if(!fs.existsSync(inventory))throw new Error(`Missing Rust ${version} copyright inventory for the native core.`);
  files.set('licenses/'+inventoryName,fs.readFileSync(inventory));
  for(const file of ['rust-MIT.txt','rust-Apache-2.0.txt'])files.set('licenses/'+file,fs.readFileSync(path.join(source,'sdk/core/notices',file)));
  notices+=`\nRust standard library ${version}\n  licenses/${inventoryName} contains the toolchain library copyright\n  and licence inventory, including platform alternatives that need not be\n  linked into this ${target} target. MIT and Apache texts accompany it.\n`;
  files.set('THIRD_PARTY_NOTICES.txt',Buffer.from(notices));return files;
}
function writeNotices(destination,files){for(const [name,bytes] of files){const file=path.join(destination,name);fs.mkdirSync(path.dirname(file),{recursive:true});fs.writeFileSync(file,bytes);}}
module.exports={coreNotices,writeNotices};
