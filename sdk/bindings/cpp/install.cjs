#!/usr/bin/env node
'use strict';

const fs=require('node:fs'),path=require('node:path');
if(!process.argv[2])throw new Error('Pass the SDK installation directory.');
const destination=path.join(path.resolve(process.argv[2]),'bindings','cpp');
fs.mkdirSync(destination,{recursive:true});
fs.copyFileSync(path.join(__dirname,'clunkyard.hpp'),path.join(destination,'clunkyard.hpp'));
