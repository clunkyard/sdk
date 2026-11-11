#!/usr/bin/env node
'use strict';

const fs=require('node:fs'),path=require('node:path');
if(!process.argv[2])throw new Error('Pass the SDK installation directory.');
const destination=path.join(path.resolve(process.argv[2]),'bindings','python','clunkyard');
fs.mkdirSync(destination,{recursive:true});
for(const file of fs.readdirSync(path.join(__dirname,'clunkyard'))){
  if(file.endsWith('.py'))fs.copyFileSync(path.join(__dirname,'clunkyard',file),path.join(destination,file));
}
