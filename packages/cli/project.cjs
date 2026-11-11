'use strict';

const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto');
const {extract,readAsset}=require('./archive.cjs');
const catalogue=require('./catalogue.json');
const json=value=>JSON.stringify(value,null,2)+'\n';
const hash=data=>crypto.createHash('sha256').update(data).digest('hex');
const normalized=data=>data.toString('utf8').replace(/\r\n/g,'\n');
const known=(data,expected)=>Boolean(expected)&&(hash(data)===expected||hash(normalized(data))===expected);
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
    build:'npx --yes @clunkyard/cli@2 build',doctor:'npx --yes @clunkyard/cli@2 doctor',
    match:'npx --yes @clunkyard/cli@2 match',replay:'npx --yes @clunkyard/cli@2 replay',
  },engines:{node:'>=20'},dependencies:{clunkyard:`npm:${sdk.name}@${sdk.version}`}};
  const lock={name:metadata.name,version:metadata.version,lockfileVersion:3,requires:true,packages:{
    '':{name:metadata.name,version:metadata.version,dependencies:metadata.dependencies,engines:metadata.engines},
    'node_modules/clunkyard':{name:sdk.name,version:sdk.version,resolved:'https://registry.npmjs.org/@clunkyard/sdk/-/sdk-2.0.0.tgz',integrity:sdk.integrity,license:'MIT',engines:{node:'>=20'}},
  }};
  return {'package.json':json(metadata),'package-lock.json':json(lock)};
}

function makefile() {
  return 'CLUNKYARD ?= npx --yes @clunkyard/cli@2\n\n.PHONY: all setup doctor match replay clean\nall:\n\t$(CLUNKYARD) build\nsetup:\n\t$(CLUNKYARD) setup\ndoctor:\n\t$(CLUNKYARD) doctor\nmatch:\n\t$(CLUNKYARD) match\nreplay:\n\t$(CLUNKYARD) replay\nclean:\n\t$(CLUNKYARD) clean\n';
}

function readme(language,entry) {
  const row=catalogue.languages[language],portable=nodeLanguage(language)||row.portable;
  const tools=row.tools.filter(tool=>!['node','make'].includes(tool));
  const guide=language==='typescript'?'javascript':language==='wat'||language.endsWith('-wasm')?'wasm':language;
  return `# A Clunkyard robot in ${language}

Edit \`${entry}\`, build, play a real local match and inspect its replay. Install Node 20+ with npm${tools.length?' and your existing language tools: '+tools.map(tool=>'\`'+tool+'\`').join(', '):''}.
${portable?'This language uses the portable Node/WebAssembly route.':'Native SDK assets require Linux x86-64 and glibc 2.39+ (Ubuntu 24.04+ or WSL2 Ubuntu 24.04+).'}

\`\`\`sh
git clone https://github.com/clunkyard/starter-${language}.git
cd starter-${language}
npx --yes @clunkyard/cli@2 doctor
npx --yes @clunkyard/cli@2 build
npx --yes @clunkyard/cli@2 match
npx --yes @clunkyard/cli@2 replay
\`\`\`

If you use Make, the equivalent commands are \`make doctor\`, \`make\`, \`make match\` and \`make replay\`. The shared CLI downloads the checksum-pinned local engine and only this language's SDK${nodeLanguage(language)?' through npm':''}. Downloads, build output and temporary files stay in ignored \`.clunkyard/\`${nodeLanguage(language)?'; the npm SDK is installed in ignored \`node_modules/\`':''}. No account, Docker image, administrator access or all-language builder is needed.

${nodeLanguage(language)?`The ordinary npm dependency is \`clunkyard@npm:@clunkyard/sdk@${catalogue.sdkVersion}\`, pinned with its integrity in \`package-lock.json\`. Existing \`require('clunkyard')\` and \`import ... from 'clunkyard'\` source works locally and online. The first build runs \`npm ci --ignore-scripts\`; you can also run \`npm ci\` yourself. Keep both package files in Git. ${language==='typescript'?'Use your installed TypeScript 6+ compiler; the CLI downloads no compiler.':''}\n\n`:''}${(row.requirements?.notes||[]).join(' ')}

The [language guide](https://clunkyard.com/sdk/${guide}) lists the adapter, official language reference and toolchain prerequisites. \`doctor\` checks your existing installation without downloading a compiler. Compiler-sensitive native bindings rebuild with your own toolchain.

## Repeat a trial

\`\`\`sh
npx --yes @clunkyard/cli@2 match 42 2d classic
npx --yes @clunkyard/cli@2 match 77 3d zero-g
\`\`\`

Keep a baseline commit. Repeat several seeds and rulesets; compare contacts, battery, aim, shots, hits and eliminations in the replay. Local trials run programs as your user. Use hosted trials to measure arena budgets before a tournament.

The CLI packages SDK ${catalogue.sdkVersion}/protocol 2 asset identities from the immutable [${catalogue.release} release](https://github.com/clunkyard/game-kit/releases/tag/${catalogue.release}). \`@2\` receives compatible tooling fixes without copying scripts into your robot repository. To reproduce an exact tool version, use \`npx --yes @clunkyard/cli@2.0.1\` or set \`CLUNKYARD='npx --yes @clunkyard/cli@2.0.1'\` with Make. SDK/engine caches are identified by their content hashes and remain reusable across CLI updates.

## Play online

Create or copy a bot at [clunkyard.com](https://clunkyard.com). For Clunkyard's private Git remote, install \`npm install --global @clunkyard/git@2.0.1\`, run \`clunkyard-git login\` and follow the [private Git guide](https://clunkyard.com/sdk/private-git) to clone your bot. Copy tracked robot files into that private clone while keeping its own \`.git\` directory; do not import unrelated starter history or commit generated caches. An accepted push saves a revision and requests its hosted build.

You can also create your own GitHub repository from this template, connect it in My bots and select the repository/ref for build-on-push. [WebDAV](https://clunkyard.com/manual#local-development) is available for desktop editors. Save browser changes before syncing; resolve any source conflict before publishing.

[API and adapters](https://clunkyard.com/sdk) · [Local trials](https://clunkyard.com/sdk/local-trials) · [Game manual](https://clunkyard.com/manual)
`;
}

function migrate(directory) {
  const root=path.resolve(directory),manifest=readManifest(root),row=catalogue.languages[manifest.language];
  const files={'Makefile':makefile(),...(nodeLanguage(manifest.language)?nodeMetadata(manifest.language):{})};
  const retired=['clunkyard.cjs','clunkyard-archive.cjs','clunkyard.lock.json','clunkyard-template.json'];
  const remove=[...retired,...(!nodeLanguage(manifest.language)?['package.json']:[])];
  // Preflight all replacements before changing anything. A player's source,
  // customized tools and dependency manifests are never overwritten blindly.
  for(const name of [...new Set([...remove,...Object.keys(files),'README.md','.gitignore','Dockerfile'])]){
    let stat;try{stat=fs.lstatSync(path.join(root,name))}catch(error){if(error.code==='ENOENT')continue;throw error}
    if(!stat.isFile())throw new Error(`Preserving nonordinary ${name}; migration requires ordinary files.`);
  }
  for(const name of [...remove,...Object.keys(files)]) {
    const file=path.join(root,name);
    if(!fs.existsSync(file))continue;
    const data=fs.readFileSync(file),old=row.boilerplate[name];
    if(known(data,old)||(files[name]&&normalized(data)===files[name]))continue;
    throw new Error(`Preserving customized ${name}; migrate its CLI command manually before retrying.`);
  }
  const docker=path.join(root,'Dockerfile');
  if(fs.existsSync(docker)){
    const content=fs.readFileSync(docker);
    if(!known(content,row.boilerplate.Dockerfile)&&/clunkyard(?:-archive)?\.cjs|clunkyard\.lock\.json/.test(content.toString('utf8')))throw new Error('Preserving customized Dockerfile; replace its retired helper command before migrating.');
  }
  const guide=path.join(root,'README.md'),oldGuide=fs.existsSync(guide)?fs.readFileSync(guide,'utf8'):null;
  const generated=readme(manifest.language,manifest.entry);
  if(oldGuide===null||known(oldGuide,row.boilerplate['README.md']))fs.writeFileSync(guide,generated);
  else if(!oldGuide.includes('## Clunkyard CLI'))fs.appendFileSync(guide,'\n## Clunkyard CLI\n\nShared tooling is available through `npx --yes @clunkyard/cli@2 doctor`, `build`, `match` and `replay`. See [the local development guide](https://clunkyard.com/sdk/player-kit) for npm installation and private Git. SDK/engine asset identities are pinned in the CLI package.\n');
  const ignore=path.join(root,'.gitignore'),ignored=fs.existsSync(ignore)?fs.readFileSync(ignore,'utf8'):'';
  const additions=['.clunkyard/','replays/','build/','node_modules/'].filter(line=>!ignored.split(/\r?\n/).includes(line));
  if(additions.length)fs.writeFileSync(ignore,ignored+(ignored&&!ignored.endsWith('\n')?'\n':'')+additions.join('\n')+'\n');
  for(const [name,data] of Object.entries(files))fs.writeFileSync(path.join(root,name),data);
  if(fs.existsSync(docker)&&known(fs.readFileSync(docker),row.boilerplate.Dockerfile)){
    let content=fs.readFileSync(docker,'utf8');
    content=content.replace('COPY --from=node /usr/local/bin/node /usr/local/bin/node','COPY --from=node /usr/local/bin/node /usr/local/bin/node\nCOPY --from=node /usr/local/lib/node_modules/npm /usr/local/lib/node_modules/npm\nRUN ln -s ../lib/node_modules/npm/bin/npm-cli.js /usr/local/bin/npm && ln -s ../lib/node_modules/npm/bin/npx-cli.js /usr/local/bin/npx');
    content=content.replace('ENTRYPOINT ["node", "clunkyard.cjs"]','ENTRYPOINT ["npx", "--yes", "@clunkyard/cli@2"]');
    fs.writeFileSync(docker,content);
  }
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
  try{extract(bytes,root,row.starter.expandedBytes);migrate(root);}catch(error){fs.rmSync(root,{recursive:true,force:true});throw error;}
  return root;
}

module.exports={readManifest,nodeMetadata,makefile,readme,migrate,createProject};
