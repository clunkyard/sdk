#!/usr/bin/env node
'use strict';

// Exercise the installed CLI and SDK, including an unresponsive robot. The
// WebSocket server is a development dependency, never part of the CLI package.
const fs=require('node:fs'),path=require('node:path'),assert=require('node:assert/strict');
const {spawn}=require('node:child_process');
const {WebSocketServer}=require('ws');
const [cli,project,fixtureDirectory]=process.argv.slice(2);
const messages=fs.readFileSync(path.join(fixtureDirectory,'2d-classic.ndjson'),'utf8').trim().split('\n').slice(0,2).map(JSON.parse);
const delay=ms=>new Promise(resolve=>setTimeout(resolve,ms));
function alive(pid){try{process.kill(pid,0);return true;}catch(error){if(error.code==='ESRCH')return false;throw error;}}
async function until(predicate,description,timeout=5000){const end=Date.now()+timeout;while(Date.now()<end){if(predicate())return;await delay(20);}throw Error(description);}
async function trial(mode,source){
  const server=new WebSocketServer({port:0,host:'127.0.0.1'});await new Promise(resolve=>server.once('listening',resolve));
  const pidFile=path.join(project,'.clunkyard',`connect-${mode}.pid`);fs.rmSync(pidFile,{force:true});fs.writeFileSync(path.join(project,'bot.js'),source);
  const child=spawn(process.execPath,[cli,'--project',project,'connect'],{env:{...process.env,CLUNKYARD_REMOTE_URL:`ws://127.0.0.1:${server.address().port}`,CLUNKYARD_REMOTE_TOKEN:'fixture-connect-key',CLUNKYARD_BOT_ID:'fixture-bot',clunkyard_remote_token:'fixture-connect-key',ClUnKyArD_BoT_Id:'fixture-bot',CLUNKYARD_BOT_ENV_ALLOW:'clunkyard_remote_token,ClUnKyArD_BoT_Id'},stdio:['ignore','pipe','pipe']});
  let output='',socket,robotPID;for(const stream of [child.stdout,child.stderr])stream.on('data',value=>{output+=value});const exited=new Promise(resolve=>child.once('exit',resolve));
  try{
    await new Promise((resolve,reject)=>{
      const timer=setTimeout(()=>reject(Error('SDK-aware project connection timed out: '+output)),45000),fail=error=>{clearTimeout(timer);reject(error)};
      child.once('error',fail);child.once('exit',code=>fail(Error(`connection exited ${code}: ${output}`)));
      server.once('connection',connected=>{socket=connected;socket.on('message',bytes=>{try{
        const value=JSON.parse(bytes);
        if(value.type==='auth'){assert.equal(value.token,'fixture-connect-key');socket.send(JSON.stringify({v:1,type:'authenticated',botId:'fixture-bot'}));socket.send(JSON.stringify({v:1,type:'request',sessionId:'s',turnId:'hello',deadlineMs:15000,payload:messages[0]}));}
        else if(value.type==='response'&&value.turnId==='hello'){assert.equal(value.payload.type,'ready');assert.equal(value.payload.sdk.version,'2.0.0');socket.send(JSON.stringify({v:1,type:'request',sessionId:'s',turnId:'state',deadlineMs:15000,payload:messages[1]}));if(mode!=='normal'){clearTimeout(timer);resolve();}}
        else if(value.type==='response'&&value.turnId==='state'){assert.equal(value.payload.type,'act');clearTimeout(timer);resolve();}
      }catch(error){fail(error)}});});
    });
    if(mode==='normal')socket.send(JSON.stringify({v:1,type:'close-session',sessionId:'s'}));
    else{
      await until(()=>fs.existsSync(pidFile),'busy robot did not start');robotPID=Number(fs.readFileSync(pidFile,'utf8'));assert.ok(alive(robotPID));
      if(mode==='close-session')socket.send(JSON.stringify({v:1,type:'close-session',sessionId:'s'}));else child.kill('SIGTERM');
      await until(()=>!alive(robotPID),`busy robot ${robotPID} survived ${mode}`);
      if(mode==='close-session'){assert.equal(child.exitCode,null,'closing one match must leave bridge available');assert.equal(child.signalCode,null,'closing one match must not signal the bridge');}
    }
    assert.doesNotMatch(output,/fixture-connect-key/);
    return {mode,sdkReady:true,...(mode==='normal'?{sdkAct:true,credentialIsolation:true}:{busyRobotReaped:true})};
  }finally{
    if(child.exitCode===null&&child.signalCode===null)child.kill('SIGTERM');
    let shutdownTimer;
    try{await Promise.race([exited,new Promise((_,reject)=>{shutdownTimer=setTimeout(()=>{child.kill('SIGKILL');reject(Error('bridge shutdown timed out: '+output));},5000)})]);}
    finally{clearTimeout(shutdownTimer);if(robotPID&&alive(robotPID))process.kill(robotPID,'SIGKILL');for(const client of server.clients)client.terminate();await new Promise(resolve=>server.close(resolve));fs.rmSync(pidFile,{force:true});}
  }
}
async function main(){
  const bot=path.join(project,'bot.js'),original=fs.readFileSync(bot,'utf8'),results=[];
  try{
    results.push(await trial('normal',"if(Object.keys(process.env).some(k=>k.toUpperCase().startsWith('CLUNKYARD_REMOTE_')||k.toUpperCase()==='CLUNKYARD_BOT_ID'))throw Error('hub credential leaked into robot');\n"+original));
    for(const mode of ['close-session',...(process.platform==='win32'?[]:['SIGTERM'])]){
      const source=`const fs=require('node:fs'),cy=require('clunkyard');process.on('SIGTERM',()=>{});process.on('SIGINT',()=>{});class Busy extends cy.Robot{onTurn(){fs.writeFileSync(${JSON.stringify(path.join(project,'.clunkyard',`connect-${mode}.pid`))},String(process.pid));for(;;){}}}cy.run(new Busy());\n`;
      results.push(await trial(mode,source));
    }
    console.log(JSON.stringify({projectConnect:true,actualInstalledCLI:true,platform:process.platform,results}));
  }finally{fs.writeFileSync(bot,original);}
}
main().catch(error=>{console.error(error);process.exitCode=1});
