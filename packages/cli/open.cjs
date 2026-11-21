'use strict';
// npx @clunkyard/cli open ROBOT [DIRECTORY] [--in code|cursor|claude|codex]
// Everything a player needs to work on a saved robot with local tools, in one
// command: sign in through the browser when this computer has no key, clone
// the robot over HTTPS (https://<origin>/git/<robot>.git) with that key,
// write the editor and assistant files that point at its reference and local
// commands, and open the chosen tool. Each step is skipped when already done,
// so the same command reopens an existing clone.

const fs=require('node:fs'),path=require('node:path');
const {spawn,spawnSync}=require('node:child_process');
const {DEFAULT_ORIGIN,siteOrigin,savedKey,saveKey,gitCredentialPath}=require('./credentials.cjs');
const {browserKey}=require('./sign-in.cjs');

const UUID=/[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}/i;
const TOOLS={code:{name:'VS Code',bin:'code',editor:true},cursor:{name:'Cursor',bin:'cursor',editor:true},claude:{name:'Claude Code',bin:'claude'},codex:{name:'Codex',bin:'codex'}};
const USAGE=`Usage: npx @clunkyard/cli open ROBOT [DIRECTORY] [--in ${Object.keys(TOOLS).join('|')}] [--origin URL]

ROBOT is the robot's ID or its workbench address. Signs in through your
browser the first time, clones the robot into
DIRECTORY (its project name by default), adds editor and assistant settings
and opens the tool you choose.`;

// The robot and server from an ID, a workbench or robot page address, or a
// clone address.
function parseTarget(value,originFlag){
  const id=UUID.exec(String(value||''))?.[0]?.toLowerCase();
  if(!id)throw new Error(USAGE);
  let origin=originFlag;
  const address=String(value).replace(/^clunkyard::/,'');
  if(!origin&&/^https?:\/\//.test(address))origin=new URL(address).origin;
  return{id,origin:siteOrigin(origin||DEFAULT_ORIGIN)};
}
function parse(args){
  const positional=[];let tool=null,origin=null;
  for(let i=0;i<args.length;i++){
    const arg=args[i],value=()=>{const next=arg.includes('=')?arg.slice(arg.indexOf('=')+1):args[++i];if(!next||next.startsWith('--'))throw new Error(`${arg.split('=')[0]} needs a value.\n\n${USAGE}`);return next;};
    if(arg==='--in'||arg.startsWith('--in='))tool=value();
    else if(arg==='--origin'||arg.startsWith('--origin='))origin=value();
    else if(arg.startsWith('--'))throw new Error(USAGE);
    else positional.push(arg);
  }
  if(!positional.length||positional.length>2)throw new Error(USAGE);
  if(tool&&tool!=='none'&&!TOOLS[tool])throw new Error(`--in takes ${Object.keys(TOOLS).join(', ')} or none.`);
  return{...parseTarget(positional[0],origin),directory:positional[1]||null,tool};
}
function onPath(name,env=process.env){
  const extensions=process.platform==='win32'?(env.PATHEXT||'.EXE;.CMD;.BAT').split(';'):[''];
  for(const directory of String(env.PATH||'').split(path.delimiter).filter(Boolean))for(const extension of extensions){const file=path.join(directory,name+extension);try{if(fs.statSync(file).isFile())return file;}catch{}}
  return null;
}
// Windows installs npm, code and cursor as .cmd shims, which need a shell.
const shell=process.platform==='win32';
const quote=value=>shell?`"${value}"`:value;
function run(command,args,options={}){
  const result=spawnSync(command,shell?args.map(quote):args,{stdio:'inherit',shell,...options});
  if(result.error)throw result.error;
  return result.status;
}
function say(text){process.stderr.write(text+'\n');}

async function api(origin,key,route){
  const response=await fetch(origin+route,{redirect:'error',headers:{Authorization:`Bearer ${key}`,Accept:'application/json'},signal:AbortSignal.timeout(30000)});
  const value=await response.json().catch(()=>({}));
  if(!response.ok)throw Object.assign(new Error(response.status===404?'That robot is not one of yours on this account.':value.error||`Clunkyard answered ${response.status}.`),{status:response.status});
  return value;
}

// The saved key, or a new one from a browser sign-in, which is saved.
async function ensureKey(origin,{again=false}={}){
  let key=again?null:savedKey(origin);if(key)return key;
  say(again?'The saved key no longer works (revoked or expired); signing in again.':'Signing in to Clunkyard through your browser.');
  key=await browserKey(origin);saveKey(origin,key);
  return key;
}
function remoteOf(directory){const result=spawnSync('git',['-C',directory,'remote','get-url','origin'],{encoding:'utf8'});return result.status===0?result.stdout.trim():null;}

// The reference page for a language, as the MCP docs tool resolves it.
function referencePage(language){
  const slug=String(language||'').toLowerCase();
  if(slug==='typescript')return'javascript';
  if(/^(vane|cadence|glyph)-wasm$/.test(slug))return slug.replace(/-wasm$/,'');
  if(/-wasm$|^wat$/.test(slug))return'wasm';
  return slug;
}
// Local settings for editors and assistants. None is committed: each is
// listed in .git/info/exclude, so a push sends only the robot.
function projectFiles({origin,bot}){
  const hostedBuild=`${origin===DEFAULT_ORIGIN?'':`CLUNKYARD_ORIGIN=${origin} `}npx @clunkyard/cli hosted-build ${bot.id}`,mcp=`${origin}/mcp`,page=referencePage(bot.language),workbench=`${origin}/workbench?bot=${bot.id}`;
  const agents=`# ${bot.name}

A Clunkyard robot in ${bot.language} (robot ID ${bot.id}). This folder is its
private Git clone.

## Read first

- The ${bot.language} reference, with every robot API function's exact name and signature: ${origin}/sdk/${page}.md
- The rules sheet (what a robot senses, moves and spends): ${origin}/sdk/rules.md
- Everything else: ${origin}/llms.txt

## The loop

1. Edit the files here.
2. Build: \`npx @clunkyard/cli build\`
3. Play a local match on the same engine as hosted matches: \`npx @clunkyard/cli match 42 2d classic\`; \`npx @clunkyard/cli opponents\` lists house robots for \`--opponent NAME\`.
4. Commit and \`git push origin main\`. A push is a Clunkyard commit: it saves a revision and starts its hosted build.
5. Wait for the hosted build and its diagnostics: \`${hostedBuild}\` (exit 0 passed, 1 compiler errors, 2 otherwise).

If the push is refused because the robot changed elsewhere (the browser, or a
commit through MCP), \`git pull --no-rebase origin main\`, build and test
again, and push. Only \`main\` is accepted; no force pushes, tags, symlinks or
submodules. The workbench: ${workbench}

## MCP

The Clunkyard MCP server (${mcp}) is set up for this folder. Its tools:
\`docs\`, \`robots\`, \`matches\`, \`standings\`, \`create_robot\`, \`build_robot\`,
\`update_robot\`, \`stage_robot\`, \`practice_match\`, \`tournament_entry\` and
\`send_feedback\`. Here, use it for hosted practice (\`practice_match\` with
\`opponents: ["entry"]\`), what happened in a match (\`matches\` with
\`view: "events"\`) and standings. Do not use \`build_robot\` edits while
working in this clone: they change the robot's draft on the server, which
this clone does not see. Edit here and push instead. Ask before entering a
tournament.
`;
  return{
    'AGENTS.md':agents,
    'CLAUDE.md':'@AGENTS.md\n',
    '.mcp.json':JSON.stringify({mcpServers:{clunkyard:{type:'http',url:mcp}}},null,2)+'\n',
    '.cursor/mcp.json':JSON.stringify({mcpServers:{clunkyard:{url:mcp}}},null,2)+'\n',
    '.codex/config.toml':`[mcp_servers.clunkyard]\nurl = "${mcp}"\n`,
    '.vscode/tasks.json':JSON.stringify({version:'2.0.0',tasks:[
      {label:'Clunkyard: build',type:'shell',command:'npx @clunkyard/cli build',group:{kind:'build',isDefault:true},problemMatcher:[]},
      {label:'Clunkyard: local match',type:'shell',command:'npx @clunkyard/cli match 42 2d classic',group:'test',problemMatcher:[]},
      {label:'Clunkyard: push and wait for the hosted build',type:'shell',command:`git push origin main && ${hostedBuild}`,problemMatcher:[]},
    ]},null,2)+'\n',
  };
}
function writeProjectFiles(directory,context){
  const written=[];
  for(const [name,content] of Object.entries(projectFiles(context))){
    const file=path.join(directory,name);
    if(fs.existsSync(file))continue; // The robot's own file, or one written before.
    fs.mkdirSync(path.dirname(file),{recursive:true});fs.writeFileSync(file,content);written.push(name);
  }
  const exclude=path.join(directory,'.git','info','exclude');
  const existing=fs.existsSync(exclude)?fs.readFileSync(exclude,'utf8'):'';
  const lines=new Set(existing.split('\n')),add=[...written.map(name=>`/${name}`),'/.clunkyard/','/replays/'].filter(line=>!lines.has(line));
  if(add.length){fs.mkdirSync(path.dirname(exclude),{recursive:true});fs.appendFileSync(exclude,`${existing&&!existing.endsWith('\n')?'\n':''}# Local settings, build cache and replays from npx @clunkyard/cli\n${add.join('\n')}\n`);}
  return written;
}

function launch(tool,directory){
  const spec=TOOLS[tool];
  if(!onPath(spec.bin)){say(`${spec.name} is not on your PATH. Open ${directory} in it yourself.`);return 0;}
  if(spec.editor){
    const child=spawn(spec.bin,[quote(directory)],{stdio:'ignore',detached:true,shell});child.on('error',()=>{});child.unref();
    say(`Opened ${directory} in ${spec.name}.`);return 0;
  }
  say(`Starting ${spec.name} in ${directory}. The first time, approve the clunkyard MCP server and sign in when it asks.`);
  return run(spec.bin,[],{cwd:directory});
}

// Git reads the key from the CLI's credential store for this server only;
// the player's own Git configuration is untouched.
function credentialConfig(origin){return[`credential.${origin}.helper`,`store --file=${gitCredentialPath()}`];}
// How far the clone is behind the server's main, after a fetch.
function behind(directory){
  if(run('git',['-C',directory,'fetch','-q','origin'],{stdio:'ignore'})!==0)return null;
  const result=spawnSync('git',['-C',directory,'rev-list','--count','HEAD..origin/main'],{encoding:'utf8'});return result.status===0?Number(result.stdout.trim()):null;
}

async function open(args){
  const target=parse(args);
  if(!onPath('git'))throw new Error('Install Git first: https://git-scm.com/downloads');
  const project=async key=>(await api(target.origin,key,`/api/bots/${target.id}/project`)).project?.bot;
  let bot,key=await ensureKey(target.origin);
  try{bot=await project(key);}
  catch(error){if(error.status!==401)throw error;key=await ensureKey(target.origin,{again:true});bot=await project(key);}
  // Git's credential store gets the key that works, wherever it came from.
  saveKey(target.origin,key);
  if(!bot?.slug)throw new Error('Clunkyard did not return this robot\'s project.');
  const directory=path.resolve(target.directory||bot.slug),remote=`${target.origin}/git/${target.id}.git`,[setting,helper]=credentialConfig(target.origin);
  if(fs.existsSync(directory)){
    const current=remoteOf(directory);
    // A clone made with the retired Private Git helper keeps its history: the
    // commits are the same, so only its address changes.
    if(current===`clunkyard::${target.origin}/bots/${target.id}`){run('git',['-C',directory,'remote','set-url','origin',remote]);say(`Moved ${directory} to ${remote}.`);}
    else if(current!==remote)throw new Error(`${directory} already exists and is not this robot's clone. Pass another DIRECTORY.`);
    run('git',['-C',directory,'config',setting,helper]);
    say(`Using the existing clone in ${directory}.`);
    const count=behind(directory);if(count)say(`Your clone is ${count} commit${count===1?'':'s'} behind origin/main: git pull.`);
  }else{
    if(run('git',['-c',`${setting}=${helper}`,'clone',remote,directory])!==0)throw new Error('git clone failed; see its message above.');
    run('git',['-C',directory,'config',setting,helper]);
  }
  const written=writeProjectFiles(directory,{origin:target.origin,bot});
  if(written.length)say(`Added ${written.join(', ')} (kept out of Git).`);
  if(!target.tool||target.tool==='none'){
    say(`\nNext:\n  cd ${path.relative(process.cwd(),directory)||'.'}\n  npx @clunkyard/cli build\n  npx @clunkyard/cli match\n  git push origin main\nAdd --in ${Object.keys(TOOLS).join('|')} to open it in a tool.`);
    return 0;
  }
  return launch(target.tool,directory);
}

module.exports={open,parse,projectFiles,writeProjectFiles,referencePage,ensureKey};
