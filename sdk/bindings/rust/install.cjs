#!/usr/bin/env node
'use strict';

// The SDK and Make use the same recipe; players need rustc, not a Unix shell.
const fs=require('node:fs'),path=require('node:path');
const {execFileSync}=require('node:child_process');
const sdk=process.argv[2];
if(!sdk)throw new Error('Pass the SDK installation directory.');
const destination=path.join(sdk,'bindings','rust');
fs.mkdirSync(destination,{recursive:true});
execFileSync(process.argv[3]||process.env.RUSTC||'rustc',[
  '--edition=2021','--crate-name','clunkyard','--crate-type','rlib','-O',
  '-L',`native=${path.join(sdk,'lib')}`,path.join(__dirname,'clunkyard.rs'),
  '-o',path.join(destination,'libclunkyard.rlib'),
],{stdio:'inherit'});
