#!/usr/bin/env node
'use strict';

const path=require('node:path');
const {createRuntime}=require('./runtime.cjs');
const {createProject,migrate}=require('./project.cjs');
const catalogue=require('./catalogue.json');
const {version}=require('./package.json');

function parse(args) {
  let project=process.cwd();const positional=[];
  for(let i=0;i<args.length;i++){
    if(args[i]==='--project'){
      if(!args[i+1]||args[i+1].startsWith('--'))throw new Error('--project needs a directory.');
      project=path.resolve(args[++i]);
    }else if(args[i].startsWith('--project=')){
      if(!args[i].slice(10))throw new Error('--project needs a directory.');
      project=path.resolve(args[i].slice(10));
    }else positional.push(args[i]);
  }
  return {project,action:positional.shift()||'help',args:positional};
}
async function main(argv=process.argv.slice(2)) {
  const {project,action,args}=parse(argv);
  if(action==='--version'||action==='version'){console.log(version);return;}
  if(['help','--help','-h'].includes(action)){
    console.log('Clunkyard local robot CLI\n\nclunkyard [--project DIRECTORY] doctor|setup|build|match|replay|clean|migrate\nclunkyard new LANGUAGE DIRECTORY\nclunkyard list\n\nmatch [seed] [2d|3d] [ruleset]; replay [file] [port]');return;
  }
  if(action==='list'){
    for(const [language,row] of Object.entries(catalogue.languages))console.log(`${language.padEnd(22)} ${row.portable||['javascript','typescript'].includes(language)?'Node route':'Linux x86-64'}  https://github.com/clunkyard/starter-${language}`);
    return;
  }
  if(action==='new'){
    if(args.length!==2)throw new Error('Use clunkyard new LANGUAGE DIRECTORY.');
    const destination=await createProject(args[0],path.resolve(project,args[1]));
    console.log(`Created ${destination}. Run npx --yes @clunkyard/cli@2 doctor, build and match there.`);return;
  }
  if(action==='migrate'){
    migrate(project);console.log('Updated generated tooling. Robot source files are unchanged.');return;
  }
  await createRuntime(project,__filename).main(action,args);
}

if(require.main===module)main().catch(error=>{console.error(error.message);process.exitCode=1});
module.exports={main,parse};
