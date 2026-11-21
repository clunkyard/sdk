'use strict';

const fs=require('node:fs'),path=require('node:path');
const {extract,readAsset}=require('./archive.cjs');
const catalogue=require('./catalogue.json');
const json=value=>JSON.stringify(value,null,2)+'\n';
const normalized=data=>data.toString('utf8').replace(/\r\n/g,'\n');
// A generated package.json or package-lock.json: the same file apart from
// which @clunkyard/sdk release it selected.
function generatedNode(data,expected) {
  const strip=text=>{const value=JSON.parse(text);
    for(const dependencies of [value.dependencies,value.packages?.['']?.dependencies])if(String(dependencies?.clunkyard||'').startsWith('npm:@clunkyard/sdk@'))delete dependencies.clunkyard;
    if(value.packages?.['node_modules/clunkyard']?.name==='@clunkyard/sdk')delete value.packages['node_modules/clunkyard'];
    return value;};
  try{return require('node:util').isDeepStrictEqual(strip(normalized(data)),strip(expected));}catch{return false;}
}
const nodeLanguage=language=>['javascript','typescript'].includes(language);

function readManifest(directory) {
  if(!fs.lstatSync(path.join(directory,'bot.json')).isFile())throw new Error('bot.json must be an ordinary file.');
  const manifest=JSON.parse(fs.readFileSync(path.join(directory,'bot.json'),'utf8'));
  if(!Object.hasOwn(catalogue.languages,manifest.language))throw new Error(`Unsupported language: ${manifest.language}`);
  const entry=manifest.entry;
  if(typeof entry!=='string'||!entry||entry.includes('\\')||entry.includes('\0')||path.isAbsolute(entry)||entry.split('/').some(part=>!part||part==='.'||part==='..'))throw new Error('bot.json entry must be a relative source path inside this project.');
  let component=directory;
  for(const part of entry.split('/')){component=path.join(component,part);if(fs.lstatSync(component).isSymbolicLink())throw new Error('bot.json entry cannot use symbolic links.');}
  const file=path.join(directory,entry),relative=path.relative(fs.realpathSync(directory),fs.realpathSync(file));
  if(relative==='..'||relative.startsWith('..'+path.sep)||path.isAbsolute(relative)||!fs.lstatSync(file).isFile())throw new Error('bot.json entry must be an ordinary file inside this project.');
  return manifest;
}

function nodeMetadata(language) {
  const sdk=catalogue.npmSDK;
  const metadata={name:`clunkyard-starter-${language}`,version:'2.0.0',private:true,scripts:{
    build:'npx --yes @clunkyard/cli build',doctor:'npx --yes @clunkyard/cli doctor',
    match:'npx --yes @clunkyard/cli match',replay:'npx --yes @clunkyard/cli replay',
  },engines:{node:'>=20'},dependencies:{clunkyard:`npm:${sdk.name}@^${sdk.version}`}};
  const lock={name:metadata.name,version:metadata.version,lockfileVersion:3,requires:true,packages:{
    '':{name:metadata.name,version:metadata.version,dependencies:metadata.dependencies,engines:metadata.engines},
    'node_modules/clunkyard':{name:sdk.name,version:sdk.version,resolved:`https://registry.npmjs.org/@clunkyard/sdk/-/sdk-${sdk.version}.tgz`,integrity:sdk.integrity,license:'MIT',engines:{node:'>=20'}},
  }};
  return {'package.json':json(metadata),'package-lock.json':json(lock)};
}

function makefile() {
  return 'CLUNKYARD ?= npx --yes @clunkyard/cli\n\n.PHONY: all setup doctor match replay clean\nall:\n\t$(CLUNKYARD) build\nsetup:\n\t$(CLUNKYARD) setup\ndoctor:\n\t$(CLUNKYARD) doctor\nmatch:\n\t$(CLUNKYARD) match\nreplay:\n\t$(CLUNKYARD) replay\nclean:\n\t$(CLUNKYARD) clean\n';
}

function readme(language,entry) {
  const row=catalogue.languages[language],nodeOnly=['javascript','vane','cadence','glyph'].includes(language);
  const guide=language==='typescript'?'javascript':language==='wat'||language.endsWith('-wasm')?'wasm':language;
  const notes=[...(nodeLanguage(language)?[`The project uses the npm SDK under the \`clunkyard\` import name, both locally and online. Keep \`package.json\` and \`package-lock.json\` in Git.`]:[]),...(row.requirements?.notes||[])];
  return `# A Clunkyard robot in ${language}

Edit \`${entry}\`, build, play a local match and inspect its replay. With Docker, nothing else is needed:

\`\`\`sh
docker run --rm -v "$PWD":/bot ghcr.io/clunkyard/toolchain/${language} build
docker run --rm -v "$PWD":/bot ghcr.io/clunkyard/toolchain/${language} run
\`\`\`

To build without Docker, install [Node 24 with npm](https://nodejs.org/en/download)${nodeOnly?'.':" and your language's compiler or interpreter."}
See the [supported platforms and toolchain installers](https://clunkyard.com/sdk/player-kit#supported-platforms).

\`\`\`sh
git clone https://github.com/clunkyard/starter-${language}.git
cd starter-${language}
npx @clunkyard/cli doctor
npx @clunkyard/cli build
npx @clunkyard/cli match
npx @clunkyard/cli replay
\`\`\`

If you use Make, the equivalent commands are \`make doctor\`, \`make\`, \`make match\` and \`make replay\`. The CLI downloads the local match engine and your language's SDK. Downloads and build output stay in \`.clunkyard/\`.

${notes.length?`${notes.join('\n\n')}\n\n`:''}\
The [language guide](https://clunkyard.com/sdk/${guide}) covers the robot API and your toolchain. Use \`doctor\` to check your installation.

## Repeat a trial

\`\`\`sh
npx @clunkyard/cli match 42 2d classic
npx @clunkyard/cli match 77 3d zero-g
\`\`\`

Keep a baseline commit. Repeat several seeds and rulesets; compare contacts, battery, aim, shots, hits and eliminations in the replay. Local trials run programs as your user. Use hosted trials to measure arena budgets before a tournament.

## Play online

Create or copy a bot at [clunkyard.com](https://clunkyard.com). To work on it with Git, run \`npx @clunkyard/cli open ROBOT_ID\`, which signs in and clones it, and follow the [Git guide](https://clunkyard.com/sdk/private-git). Copy your robot files into the clone, then push to save a revision and start a hosted build.

You can also create your own GitHub repository from this template, connect it in My bots and select the repository/ref for build-on-push. Save browser changes before syncing; resolve any source conflict before publishing.

For a remote Open robot, copy the connection environment settings from your bot's Connections panel and run \`npx @clunkyard/cli connect\` here. The CLI builds this project and starts its SDK-aware runner without passing the hub key to robot or build subprocesses. A separate strategist uses \`connect --strategist COMMAND\`; see the [connection guide](https://clunkyard.com/sdk/player-kit#connect-code-running-on-your-computer).

[API and adapters](https://clunkyard.com/sdk) · [Local trials](https://clunkyard.com/sdk/local-trials) · [Game manual](https://clunkyard.com/manual)
`;
}

function migrate(directory) {
  const root=path.resolve(directory),manifest=readManifest(root);
  const files={'Makefile':makefile(),...(nodeLanguage(manifest.language)?nodeMetadata(manifest.language):{})};
  const remove=!nodeLanguage(manifest.language)?['package.json']:[];
  // Preflight all replacements before changing anything. A player's source,
  // customized tools and dependency manifests are never overwritten blindly.
  for(const name of [...new Set([...remove,...Object.keys(files),'README.md','.gitignore'])]){
    let stat;try{stat=fs.lstatSync(path.join(root,name))}catch(error){if(error.code==='ENOENT')continue;throw error}
    if(!stat.isFile())throw new Error(`Preserving nonordinary ${name}; migration requires ordinary files.`);
  }
  for(const name of [...remove,...Object.keys(files)]) {
    const file=path.join(root,name);
    if(!fs.existsSync(file))continue;
    const data=fs.readFileSync(file);
    if(files[name]&&(normalized(data)===files[name]||(name.startsWith('package')&&generatedNode(data,files[name]))))continue;
    throw new Error(`Preserving customized ${name}; migrate its CLI command manually before retrying.`);
  }
  const guide=path.join(root,'README.md'),oldGuide=fs.existsSync(guide)?fs.readFileSync(guide,'utf8'):null;
  const generated=readme(manifest.language,manifest.entry);
  if(oldGuide===null||oldGuide===generated)fs.writeFileSync(guide,generated);
  else if(!oldGuide.includes('## Clunkyard CLI'))fs.appendFileSync(guide,'\n## Clunkyard CLI\n\nShared tooling is available through `npx @clunkyard/cli doctor`, `build`, `match` and `replay`. See [the local development guide](https://clunkyard.com/sdk/player-kit) for npm installation and private Git. SDK/engine asset identities are pinned in the CLI package.\n');
  const ignore=path.join(root,'.gitignore'),ignored=fs.existsSync(ignore)?fs.readFileSync(ignore,'utf8'):'';
  const additions=['.clunkyard/','replays/','build/','node_modules/'].filter(line=>!ignored.split(/\r?\n/).includes(line));
  if(additions.length)fs.writeFileSync(ignore,ignored+(ignored&&!ignored.endsWith('\n')?'\n':'')+additions.join('\n')+'\n');
  for(const [name,data] of Object.entries(files))fs.writeFileSync(path.join(root,name),data);
  for(const name of remove)if(fs.existsSync(path.join(root,name)))fs.unlinkSync(path.join(root,name));
  return manifest;
}

async function createProject(language,destination) {
  const row=Object.hasOwn(catalogue.languages,language)&&catalogue.languages[language];if(!row)throw new Error(`Unknown language: ${language}`);
  const root=path.resolve(destination);
  if(fs.existsSync(root))throw new Error('Choose a directory that does not exist yet.');
  const bytes=await readAsset(row.starter);
  fs.mkdirSync(path.dirname(root),{recursive:true});
  fs.mkdirSync(root);
  try{
    extract(bytes,root,row.starter.expandedBytes);
    const manifest=migrate(root);
    // This is a new project from a verified archive, so its guide is generated.
    // Existing projects keep migrate's protection for player-written notes.
    fs.writeFileSync(path.join(root,'README.md'),readme(manifest.language,manifest.entry));
  }catch(error){fs.rmSync(root,{recursive:true,force:true});throw error;}
  return root;
}

module.exports={readManifest,nodeMetadata,makefile,readme,migrate,createProject};
