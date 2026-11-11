'use strict';
// Node's run loop. The generated API marshals calls to the core; the shared
// wasm32 loader owns session parsing, act serialization and fault recovery.
const {attach,subsystem,core,CoreError,CoreFault}=require('./api');

class Forfeit extends Error {
  constructor(code,message=''){super(message);this.code=code;}
}
class Robot { onStart(){} onTurn(){} }
const angles=subsystem(core.robot({binding:'clunkyard-javascript-angles'}),'angles');
function turn(robot,event){
  try{event==='hello'?robot.onStart():robot.onTurn();}
  catch(error){
    if(error instanceof Forfeit){robot.session.forfeit(error.code,error.message);return;}
    console.error(error.stack||error);
  }
  robot.session.send();
}
function next(robot){
  try{return robot.session.next();}
  catch(error){
    console.error(error.stack||error);
    if(error instanceof CoreFault||(error instanceof CoreError&&error.status===-6)){
      try{robot.session.send();}catch(sendError){console.error(sendError.stack||sendError);return 'end';}
    }
    return null;
  }
}
function play(robot,lines,options={}){
  const handle=attach(robot,options),out=[];
  try{
    for(const line of lines){
      robot.session.feed(line);const event=next(robot);if(event==='end')break;
      if(event)turn(robot,event);let reply;while((reply=robot.session.reply())!==null)out.push(reply);
    }
    return out;
  }finally{handle.close();}
}
async function run(robot,options={}){
  const handle=attach(robot,options),stdoutWrite=process.stdout.write.bind(process.stdout);
  const write=process.env.CLUNKYARD_PROTOCOL_FD==='3'
    ? line=>{require('node:fs').writeSync(3,line);return true;}:stdoutWrite;
  // Robot diagnostics use stderr. Only core replies use the captured writer.
  const stdout=process.stdout.write;process.stdout.write=process.stderr.write.bind(process.stderr);
  const lines=require('node:readline').createInterface({input:process.stdin,crlfDelay:Infinity});
  try{
    for await(const line of lines){
      robot.session.feed(line);const event=next(robot);
      if(event==='end')break;
      if(event)turn(robot,event);
      let reply;while((reply=robot.session.reply())!==null)if(!write(reply+'\n'))await new Promise(resolve=>process.stdout.once('drain',resolve));
    }
  }finally{lines.close();process.stdout.write=stdout;handle.close();}
}
module.exports={Robot,Forfeit,angles,run,play};
