'use strict';
// npx @clunkyard/cli hosted-build ROBOT_ID: waits for the hosted build of
// the robot's latest saved revision (the one a git push just made) and
// prints its result the way the MCP server gives it: passed or failed, each
// compiler diagnostic as file:line:column: severity: message, and the end
// of the log. --json prints the same as JSON. Exit status: 0 passed, 1 the
// compiler failed, 2 anything else (the platform stopped it, or the wait ran
// out). Uses the key that `open` or `login` saved, or CLUNKYARD_AGENT_KEY;
// CLUNKYARD_ORIGIN picks another server.

const {DEFAULT_ORIGIN,siteOrigin,savedKey}=require('./credentials.cjs');

// HEAD of the Git clone in this directory, or null outside one.
function gitHead(){const result=require('node:child_process').spawnSync('git',['rev-parse','HEAD'],{encoding:'utf8',stdio:['ignore','pipe','ignore']});return result.status===0?result.stdout.trim():null;}
const TERMINAL=new Set(['complete','failed','cancelled']);
const sleep=ms=>new Promise(resolve=>setTimeout(resolve,ms));

async function hostedBuild(args,{env=process.env,fetchImpl=globalThis.fetch,out=console.log,pollMs=3000,now=Date.now,localHead=gitHead}={}){
  const json=args.includes('--json'),robotId=args.find(arg=>!arg.startsWith('--')),timeoutArg=args.find(arg=>arg.startsWith('--timeout='));
  if(!robotId)throw new Error('Use npx @clunkyard/cli hosted-build ROBOT_ID [--json] [--timeout=SECONDS]. The robot ID is in its Connections panel.');
  const origin=siteOrigin(env.CLUNKYARD_ORIGIN||DEFAULT_ORIGIN),key=savedKey(origin,env);
  if(!key)throw new Error(`Sign in first: npx @clunkyard/cli open ${robotId} signs in and clones the robot, or run npx @clunkyard/cli login${origin===DEFAULT_ORIGIN?'':` --origin ${origin}`}.`);
  const deadline=now()+1000*Number(timeoutArg?.slice(10)||600);
  const get=async route=>{const response=await fetchImpl(`${origin}${route}`,{headers:{Authorization:`Bearer ${key}`}});const value=await response.json().catch(()=>({}));if(!response.ok)throw new Error(value.error||`${route} answered ${response.status}`);return value;};
  const robot=encodeURIComponent(robotId),revisionId=(await get(`/api/bots/${robot}/project`)).project?.bot?.activeRevisionId;
  if(!revisionId)throw new Error('The robot has no saved revision yet: push one first.');
  // A clone that is not at the server's main pushed nothing new, or is behind.
  const local=localHead(),branch=local?await get(`/api/bots/${robot}/git`).catch(()=>null):null;
  if(branch&&local!==branch.head)process.stderr.write("The server's main is not your HEAD: git pull, then push.\n");
  let job=null;
  for(;;){
    // The build a push queues may appear a moment after the push returns.
    job=job?(await get(`/api/jobs/${encodeURIComponent(job.id)}`)).job:((await get('/api/me')).jobs||[]).find(item=>item.kind==='build'&&String(item.bot_id)===String(robotId)&&String(item.revision_id)===String(revisionId))||null;
    if(job&&TERMINAL.has(job.state))break;
    if(now()>deadline){out(json?JSON.stringify({state:job?'building':'not-queued',robotId,revisionId,jobId:job?.id||null}):`Still ${job?job.state:'waiting for a build to be queued'} for revision ${revisionId}. Run the command again to keep waiting.`);return 2;}
    if(!json)process.stderr.write(`build ${job?job.state:'not queued yet'}…\n`);
    await sleep(pollMs);
  }
  const record=(await get(`/api/jobs/${encodeURIComponent(job.id)}`)).job,build=record.build||{},state=record.state==='complete'?'passed':record.result?.code==='BUILD_FAILED'?'failed':'interrupted';
  let tail='';if(build.logCharacters){const page=(await get(`/api/jobs/${encodeURIComponent(job.id)}/log?offset=${Math.max(0,build.logCharacters-3000)}`)).log;tail=String(page?.text||'').split('\n').slice(-40).join('\n');}
  const result={state,robotId,revisionId,jobId:record.id,failure:state==='passed'?null:record.result?.message?.split('\n')[0]||build.failure||null,errorCount:build.errorCount??0,warningCount:build.warningCount??0,diagnostics:build.diagnostics||[],logTail:tail};
  if(json)out(JSON.stringify(result));
  else{out(`Build ${state} for revision ${revisionId}${result.failure?`: ${result.failure}`:''}`);for(const item of result.diagnostics)out(`${item.file||'?'}:${item.line??'?'}:${item.column??'?'}: ${item.severity}: ${item.message}`);if(state!=='passed'&&tail)out(`\n${tail}`);}
  return state==='passed'?0:state==='failed'?1:2;
}

module.exports={hostedBuild};
