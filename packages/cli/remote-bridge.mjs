#!/usr/bin/env node

import {spawn} from 'node:child_process';
import readline from 'node:readline';

let WebSocket=globalThis.WebSocket;
if(!WebSocket)try{({WebSocket}=await import('ws'));}catch{console.error('remote-bridge requires Node 22+ or the ws package (npm install ws@8)');process.exit(2);}

// With --strategist the bridge runs a strategist for the bot's DSL reflex in
// Open matches instead of the bot itself (docs/protocol.md, "Strategists").
const argv=process.argv.slice(2),strategist=argv[0]==='--strategist',[command,...args]=strategist?argv.slice(1):argv,url=process.env.CLUNKYARD_REMOTE_URL,token=process.env.CLUNKYARD_REMOTE_TOKEN,botId=process.env.CLUNKYARD_BOT_ID;
if(!url||!token||!botId||!command){console.error('set CLUNKYARD_REMOTE_URL, CLUNKYARD_REMOTE_TOKEN and CLUNKYARD_BOT_ID; usage: remote-bridge [--strategist] COMMAND [ARGS...]');process.exit(2);}

const sessions=new Map(),responseCache=new Map(),allowedEnv=new Set(String(process.env.CLUNKYARD_BOT_ENV_ALLOW||'').split(',').map(value=>value.trim()).filter(Boolean));let socket=null,stopping=false,retry=250;
function botEnvironment(){const result={PATH:process.env.PATH||'/usr/local/bin:/usr/bin:/bin',HOME:process.env.HOME||process.cwd(),TMPDIR:process.env.TMPDIR||'/tmp',LANG:process.env.LANG||'C.UTF-8',LC_ALL:process.env.LC_ALL||''};for(const name of allowedEnv)if(!name.startsWith('CLUNKYARD_REMOTE_')&&name!=='CLUNKYARD_BOT_ID'&&process.env[name]!==undefined)result[name]=process.env[name];return result;}
class Session{
  constructor(id){this.id=id;this.waiting=[];this.chain=Promise.resolve();this.child=spawn(command,args,{shell:false,stdio:['pipe','pipe','inherit'],env:botEnvironment()});this.lines=readline.createInterface({input:this.child.stdout,crlfDelay:Infinity});this.lines.on('line',line=>{const next=this.waiting.shift();if(next)next.resolve(line);else this.fail(new Error('bot wrote outside an Arena request'));});this.child.on('exit',(code,signal)=>this.fail(new Error(`bot exited (${signal||`code ${code}`})`)));}
  request(payload,deadlineMs){const work=this.chain.catch(()=>{}).then(()=>new Promise((resolve,reject)=>{const item={resolve:value=>{clearTimeout(item.timer);resolve(value);},reject:error=>{clearTimeout(item.timer);reject(error);}};item.timer=setTimeout(()=>{const index=this.waiting.indexOf(item);if(index>=0)this.waiting.splice(index,1);reject(new Error(`local bot exceeded ${deadlineMs} ms bridge deadline`));},Math.max(1,Number(deadlineMs)||1000));this.waiting.push(item);this.child.stdin.write(`${JSON.stringify(payload)}\n`,error=>{if(error)item.reject(error);});}));this.chain=work.catch(()=>{});return work;}
  fail(error){for(const item of this.waiting.splice(0))item.reject(error);}
  close(){this.lines.close();this.child.kill('SIGTERM');}
}
// A strategist session: the strategist reads the hub's strategist-hello,
// observe and policy-result messages on stdin, one JSON object a line, and
// writes policy lines to stdout whenever it likes. Nothing waits for it.
class StrategistSession{
  constructor(id){this.id=id;this.child=spawn(command,args,{shell:false,stdio:['pipe','pipe','inherit'],env:botEnvironment()});this.child.stdin.on('error',()=>{});this.lines=readline.createInterface({input:this.child.stdout,crlfDelay:Infinity});this.lines.on('line',line=>{let value;try{value=JSON.parse(line);}catch{console.error('strategist wrote a line that is not JSON');return;}if(value?.type==='decision'){send({...value,v:1,type:'decision',sessionId:this.id});return;}if(value?.type!=='policy')return;send({v:1,type:'policy',sessionId:this.id,seq:value.seq,basedOnTick:value.basedOnTick,...(Number.isSafeInteger(value.decisionCall)?{decisionCall:value.decisionCall}:{}),...(value.params!==undefined?{params:value.params}:{}),...(value.source!==undefined?{source:value.source}:{})});});this.child.on('exit',(code,signal)=>{if(sessions.get(this.id)===this)console.error(`strategist exited (${signal||`code ${code}`})`);});}
  deliver(message){if(this.child.stdin.writable)this.child.stdin.write(`${JSON.stringify(message)}\n`);}
  close(){this.lines.close();this.child.kill('SIGTERM');}
}
function handleStrategist(message){
  if(!message.sessionId)return;
  let session=sessions.get(message.sessionId);
  if(message.type==='close-session'){if(session){sessions.delete(message.sessionId);session.close();}return;}
  if(!['strategist-hello','observe','policy-result'].includes(message.type))return;
  if(!session){if(message.type!=='strategist-hello')return;session=new StrategistSession(message.sessionId);sessions.set(message.sessionId,session);}
  session.deliver(message);
}
function connect(){if(stopping)return;socket=new WebSocket(url);socket.addEventListener('open',()=>socket.send(JSON.stringify({v:1,type:'auth',token,botId,...(strategist?{role:'strategist'}:{})})));socket.addEventListener('message',event=>handleMessage(event).catch(error=>console.error(error.message)));socket.addEventListener('close',event=>{if(stopping)return;if(event.code===4403||event.code===4401){console.error(`access rejected (${event.code})`);return shutdown(1);}console.error(`connection lost (${event.code}); reconnecting`);setTimeout(connect,retry);retry=Math.min(5_000,retry*2);});socket.addEventListener('error',()=>{});}
async function handleMessage(event){let message;try{message=JSON.parse(String(event.data));}catch{return;}if(message.type==='authenticated'){retry=250;console.error(`connected as ${message.botId}${strategist?' (strategist)':''}`);return;}if(strategist)return handleStrategist(message);if(message.type==='close-session'){const session=sessions.get(message.sessionId);if(session){session.close();sessions.delete(message.sessionId);}return;}if(message.type!=='request'||!message.sessionId||!message.turnId)return;const cached=responseCache.get(message.turnId);if(cached)return send(cached);let session=sessions.get(message.sessionId);if(!session){if(message.payload?.type!=='hello')return send(failure(message,'SESSION_MISSING','match session is not active'));session=new Session(message.sessionId);sessions.set(message.sessionId,session);}let response;try{const line=await session.request(message.payload,message.deadlineMs);response={v:1,type:'response',turnId:message.turnId,payload:JSON.parse(line)};}catch(error){response=failure(message,'BRIDGE_ERROR',error.message);}responseCache.set(message.turnId,response);while(responseCache.size>2048)responseCache.delete(responseCache.keys().next().value);send(response);}
function failure(message,code,text){return{v:1,type:'response',turnId:message.turnId,payload:{v:1,type:'failure',tick:message.payload?.tick,code,message:String(text).slice(0,240)}};}
function send(value){if(socket?.readyState===WebSocket.OPEN)socket.send(JSON.stringify(value));}
function shutdown(code){if(stopping)return;stopping=true;process.exitCode=code;try{socket?.close();}catch{}for(const session of sessions.values())session.close();setTimeout(()=>process.exit(code),20);}
process.once('SIGINT',()=>shutdown(0));process.once('SIGTERM',()=>shutdown(0));connect();
