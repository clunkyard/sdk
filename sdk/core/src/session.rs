//! The session: hello, ready, state, act and forfeit, over the robot's port.

use crate::act::empty_act;
use crate::generated::*;
use crate::json::{self, Value};
use crate::message::{Hello, State};
use crate::port::Frame;
use crate::runtime::Robot;

/// A line to standard error, never to the protocol. wasm32 has none.
pub fn log_line(line: &str) {
    #[cfg(not(target_family = "wasm"))]
    eprintln!("{line}");
    #[cfg(target_family = "wasm")]
    let _ = line;
}

fn write(r: &mut Robot, line: &str) -> Result<(), Error> {
    r.port.write_frame(line.as_bytes()).map_err(|error| Error::internal(format!("cannot write the reply: {}", error.0)))
}

/// The ready reply, naming the binding.
pub fn ready_line(name: &str, version: &str) -> String {
    let sdk = Value::Object(vec![("name".into(), Value::String(name.into())), ("version".into(), Value::String(version.into()))]);
    Value::Object(vec![("v".into(), Value::Number(2.0)), ("type".into(), Value::String("ready".into())), ("sdk".into(), sdk)]).to_json()
}

/// A forfeit: the code cut to 64 code points and the message to 300, the
/// schema's limits, and the message left out when empty.
pub fn forfeit_line(tick: f64, code: &str, message: &str) -> Result<String, Error> {
    if code.is_empty() {
        return Err(Error::argument("a forfeit needs a code"));
    }
    let mut line = Value::Object(vec![
        ("v".into(), Value::Number(2.0)),
        ("type".into(), Value::String("forfeit".into())),
        ("tick".into(), Value::Number(tick)),
        ("code".into(), Value::String(code.chars().take(64).collect())),
    ]);
    if !message.is_empty() {
        line.insert("message", Value::String(message.chars().take(300).collect()));
    }
    Ok(line.to_json())
}

pub fn event(r: &Robot) -> Result<Event, Error> {
    r.event.ok_or_else(|| Error::phase("nothing read yet"))
}

pub fn next(r: &mut Robot) -> Result<Event, Error> {
    if r.forfeited {
        r.event = Some(Event::End);
        return Ok(Event::End);
    }
    let mut buf = Vec::new();
    loop {
        match r.port.read_frame(&mut buf).map_err(|error| Error::internal(format!("cannot read: {}", error.0)))? {
            Frame::End => {
                r.event = Some(Event::End);
                return Ok(Event::End);
            }
            Frame::Data => {}
        }
        let line = String::from_utf8_lossy(&buf).into_owned();
        if line.trim().is_empty() {
            continue;
        }
        let message = json::parse(&line).map_err(|error| Error::argument(format!("not a JSON line: {error}")))?;
        match message.get("type").and_then(Value::as_str) {
            Some("hello") => {
                let hello = Hello::decode(message)?;
                r.begin(hello, line);
                r.event = Some(Event::Hello);
                r.answered = false;
                return Ok(Event::Hello);
            }
            Some("state") => {
                if r.hello.is_none() {
                    return Err(Error::phase("a state before the hello"));
                }
                // Should decoding fault, send still answers this tick, with
                // an empty act; update clears it once the state is in.
                r.fault_tick = Some(message.get("tick").and_then(Value::as_f64).unwrap_or(0.0));
                r.event = Some(Event::State);
                r.answered = false;
                #[cfg(feature = "test-panic")]
                if message.get("cyTestPanic").is_some() {
                    panic!("cyTestPanic");
                }
                r.update(State::decode(message), line);
                return Ok(Event::State);
            }
            // Messages the core does not know are not for it.
            _ => continue,
        }
    }
}

pub fn send(r: &mut Robot) -> Result<(), Error> {
    if r.answered {
        return Err(Error::phase("nothing to answer"));
    }
    let line = match r.event {
        Some(Event::Hello) => ready_line(&r.binding, &r.binding_version),
        Some(Event::State) => match r.fault_tick {
            Some(tick) => empty_act(tick).to_json(),
            None => {
                let (hello, state) = (r.hello()?, r.current()?);
                if r.reset || r.poisoned() { empty_act(state.tick).to_json() } else { r.act.line(state.tick, hello) }
            }
        },
        _ => return Err(Error::phase("nothing to answer")),
    };
    write(r, &line)?;
    r.answered = true;
    r.act = crate::act::Act::default();
    Ok(())
}

pub fn forfeit(r: &mut Robot, code: &str, message: &str) -> Result<(), Error> {
    r.hello()?;
    let tick = r.state.as_ref().map(|state| state.tick).or(r.fault_tick).unwrap_or(0.0);
    let line = forfeit_line(tick, code, message)?;
    write(r, &line)?;
    r.answered = true;
    r.forfeited = true;
    Ok(())
}

pub fn log(_r: &mut Robot, line: &str) -> Result<(), Error> {
    log_line(line);
    Ok(())
}

pub fn feed(r: &mut Robot, line: &str) -> Result<(), Error> {
    let port = r.port.memory().ok_or_else(|| Error::argument("feed needs a memory port"))?;
    port.input.push_back(line.as_bytes().to_vec());
    Ok(())
}

/// The memory port's next written line, left in place until take_reply,
/// so a buffer that is too short (CY_SHORT) loses nothing.
pub fn reply(r: &Robot) -> Result<Option<String>, Error> {
    let port = r.port.memory_ref().ok_or_else(|| Error::argument("reply needs a memory port"))?;
    Ok(port.output.front().cloned())
}

/// Takes the line reply gave, once the caller has it.
pub fn take_reply(r: &mut Robot) {
    if let Some(port) = r.port.memory() {
        port.output.pop_front();
    }
}
