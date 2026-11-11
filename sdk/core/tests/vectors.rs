//! The core against every vector in protocol/v2/vectors, as
//! test/protocol-v2-schema.test.js checks the reference against them.
//!
//! - api/*.json through the C exports: each stream's hello, then each state
//!   with its slots and cases; the angle and session cases; the controls'
//!   ops and the act they write; the DSL words record after each state.
//! - formulas.json, lead.json and tracker.json through the internals no
//!   export reaches.
//!
//! Each result's exact bits go into a transcript, in two parts: abi, which
//! test/core-wasm.test.js writes the same way through the committed wasm32
//! build, and core. Their digests are in tests/vectors.digest, so native,
//! `cargo test --target wasm32-wasip1` and the committed module must agree
//! bit for bit. CY_BLESS=1 rewrites the digests; CY_TRANSCRIPT=DIR writes
//! the transcripts there, for a diff.

mod harness;

use clunkyard::formula::Formula;
use clunkyard::generated::ffi::*;
use clunkyard::generated::*;
use clunkyard::json::{self, Value};
use clunkyard::lead::{lead, Shot};
use clunkyard::message::State;
use clunkyard::tracker::{Settings, Tracker};
use harness::*;

struct Run {
    api: Api,
    lines: Vec<String>,
    failures: Vec<String>,
    skipped: Vec<String>,
}

impl Run {
    fn fail(&mut self, message: String) {
        self.failures.push(message);
    }

    fn status(&self, status: i32) -> String {
        self.api.status_name(status)
    }

    /// One case: its outcome against what it expects, as checkCase does.
    fn check(&mut self, item: &Value, outcome: &Outcome, tolerance: f64, at: &str) {
        let Outcome::Done(status, value) = outcome else {
            let Outcome::Unrepresentable(why) = outcome else { unreachable!() };
            if item.get("error").is_none() {
                self.fail(format!("{at}: the C ABI cannot carry it ({why}), yet it expects a result"));
            }
            let call = item.get("call").or_else(|| item.get("set")).or_else(|| item.get("get")).and_then(Value::as_str).unwrap_or("");
            self.skipped.push(format!("{call} {why}"));
            return;
        };
        if let Some(error) = item.get("error").and_then(Value::as_str) {
            if *status != self.api.status(error) {
                self.fail(format!("{at}: expected {error}, got {}", self.status(*status)));
            }
        } else if item.get("none").is_some() {
            if *status != CY_NONE {
                self.fail(format!("{at}: expected none, got {}", self.status(*status)));
            }
        } else if item.get("ok").is_some() {
            if *status != CY_OK {
                self.fail(format!("{at}: expected ok, got {}", self.status(*status)));
            }
        } else {
            let expected = item.get("value").unwrap();
            match value {
                Some(value) if *status == CY_OK && close(value, expected, tolerance) => {}
                _ => self.fail(format!("{at}: expected {}, got {} {}", expected.to_json(), self.status(*status), value.as_ref().map_or(String::new(), Value::to_json))),
            }
        }
    }

    fn case(&mut self, robot: &Core, item: &Value, tolerance: f64, at: String) {
        let name = text(item, "call");
        let outcome = call(&self.api, robot.robot, &name, list(item, "args"));
        self.check(item, &outcome, tolerance, &at);
        self.lines.push(format!("{at} {}", outcome_text(&outcome)));
    }

    /// The slots robot_id gives, against the robot ids the vectors list.
    fn slots(&mut self, robot: &Core, slots: &[Value], at: &str) {
        let robots = match call(&self.api, robot.robot, "match.robots", &[]) {
            Outcome::Done(CY_OK, Some(Value::Number(n))) => n as usize,
            _ => panic!("{at}: match.robots"),
        };
        let mut ids = Vec::new();
        for slot in 1..=robots {
            match call(&self.api, robot.robot, "match.robot_id", &[Value::Number(slot as f64)]) {
                Outcome::Done(CY_OK, Some(Value::String(id))) => ids.push(id),
                _ => panic!("{at}: robot_id {slot}"),
            }
        }
        let expected: Vec<String> = (0..robots).map(|i| slots.get(i).and_then(Value::as_str).unwrap_or("").to_string()).collect();
        if ids != expected {
            self.fail(format!("{at}: slots {ids:?}, expected {expected:?}"));
        }
        self.lines.push(format!("{at} slots {}", canon(&Value::Array(ids.into_iter().map(Value::String).collect()))));
    }
}

fn tolerance(file: &Value, call: &str) -> f64 {
    file.at("tolerances").and_then(|t| t.get(call)).or_else(|| file.get("tolerance")).and_then(number).unwrap()
}

fn tick_of(line: &str) -> f64 {
    json::parse(line).unwrap().get("tick").and_then(Value::as_f64).unwrap()
}

/// A fixture's first state with its tick set.
fn state_at(input: &str, tick: f64) -> String {
    let lines = read(&format!("protocol/v2/{input}"));
    let mut state = json::parse(lines.lines().nth(1).unwrap()).unwrap();
    state.insert("tick", Value::Number(tick));
    state.to_json()
}

fn hello_of(input: &str) -> String {
    read(&format!("protocol/v2/{input}")).lines().next().unwrap().to_string()
}

fn begin(robot: &Core, input: &str) {
    robot.feed(&hello_of(input));
    assert_eq!(robot.next(), Event::Hello as i32);
    assert_eq!(robot.send(), CY_OK);
    robot.reply();
}

/// Replays one stream as the core would: the hello, then each state,
/// answering each.
fn stream(run: &mut Run, file: &Value, name: &str, index: usize, stream: &Value) {
    let input = text(stream, "input");
    let source = read(&format!("protocol/v2/{input}"));
    let lines: Vec<&str> = source.trim().split('\n').collect();
    let robot = Core::open("clunkyard-test", "2.0.0");
    robot.feed(lines[0]);
    assert_eq!(robot.next(), Event::Hello as i32, "{input}");
    if let Some(configure) = stream.get("configure").and_then(Value::as_array) {
        assert!(matches!(call(&run.api, robot.robot, "tracker.configure", configure), Outcome::Done(CY_OK, _)), "{input} configure");
    }
    for (i, item) in list(stream, "hello").iter().enumerate() {
        let at = format!("{name} {index} hello {i} {}", text(item, "call"));
        run.case(&robot, item, tolerance(file, &text(item, "call")), at);
    }
    assert_eq!(robot.send(), CY_OK);
    robot.reply();
    let states = list(stream, "states");
    for line in &lines[1..] {
        robot.feed(line);
        assert_eq!(robot.next(), Event::State as i32, "{input}");
        let tick = tick_of(line);
        if let Some(expected) = states.iter().find(|state| state.get("tick").and_then(Value::as_f64) == Some(tick)) {
            if name == "dsl" {
                words(run, file, &robot, expected, &format!("{name} {index} {tick}"));
            } else {
                run.slots(&robot, list(expected, "slots"), &format!("{name} {index} {tick}"));
                for (i, item) in list(expected, "cases").iter().enumerate() {
                    let at = format!("{name} {index} {tick} {i} {}", text(item, "call"));
                    run.case(&robot, item, tolerance(file, &text(item, "call")), at);
                }
            }
        }
        assert_eq!(robot.send(), CY_OK, "{input} {tick} send");
        robot.reply();
    }
}

/// The DSL words record after a state, against the words the vectors list.
fn words(run: &mut Run, file: &Value, robot: &Core, expected: &Value, at: &str) {
    let fields: Vec<Value> = list(&run.api.json, "words").iter().filter(|word| word.get("alias").is_none()).cloned().collect();
    let mut record = DslWordsV1::default();
    assert_eq!(cy_dsl_fill_v1(robot.robot, &mut record), CY_OK, "{at} words");
    // SAFETY: DslWordsV1 is one f64 a word, in api.json's order.
    let values: &[f64] = unsafe { std::slice::from_raw_parts(&record as *const DslWordsV1 as *const f64, fields.len()) };
    assert_eq!(std::mem::size_of::<DslWordsV1>(), fields.len() * 8);
    let words = expected.get("words").unwrap();
    for (word, value) in fields.iter().zip(values) {
        let name = text(word, "name");
        let actual = match word.get("type").and_then(Value::as_str).unwrap() {
            "boolean" => Value::Bool(*value != 0.0),
            "text" => {
                let wire = run.api.enum_wire(&text(word, "enum"), *value as i32);
                // The interpreter's word is "targets" for direct controls
                // (src/dsl/common.js); src/dsl/words.js gives "direct".
                if name == "control-model" && wire.as_str() == Some("direct") { Value::String("targets".into()) } else { wire }
            }
            _ => Value::Number(*value),
        };
        let want = words.get(&name).or_else(|| words.get(list(&run.api.json, "words").iter().find(|w| w.get("alias").and_then(Value::as_str) == Some(&name)).map_or("", |w| w.get("name").and_then(Value::as_str).unwrap())));
        match want {
            Some(want) if close(&actual, want, tolerance(file, &name)) => {}
            want => run.fail(format!("{at} {name}: expected {}, got {}", want.map_or("nothing".into(), Value::to_json), actual.to_json())),
        }
    }
    for (key, _) in words.as_object() {
        if !fields.iter().any(|w| text(w, "name") == *key) && !list(&run.api.json, "words").iter().any(|w| text(w, "name") == *key) {
            run.fail(format!("{at}: the vectors have a word {key} the record does not"));
        }
    }
    run.lines.push(format!("{at} words {}", values.iter().map(|v| canon(&Value::Number(*v))).collect::<Vec<_>>().join(",")));
}

trait Members {
    fn as_object(&self) -> Vec<(String, Value)>;
}

impl Members for Value {
    fn as_object(&self) -> Vec<(String, Value)> {
        match self {
            Value::Object(members) => members.clone(),
            _ => Vec::new(),
        }
    }
}

fn angles(run: &mut Run, file: &Value) {
    for (i, item) in list(file, "cases").iter().enumerate() {
        let name = text(item, "call");
        let outcome = call(&run.api, std::ptr::null_mut(), &name, list(item, "args"));
        let at = format!("angles {i} {name}");
        run.check(item, &outcome, tolerance(file, &name), &at);
        run.lines.push(format!("{at} {}", outcome_text(&outcome)));
    }
}

fn session(run: &mut Run, file: &Value) {
    let input = "fixtures/2d-classic.ndjson";
    for (i, item) in list(file, "cases").iter().enumerate() {
        let name = text(item, "call");
        let args = list(item, "args");
        let at = format!("session {i} {name}");
        let arg = |n: usize| args[n].as_str().unwrap().to_string();
        let (status, reply) = match name.as_str() {
            "session.ready" => {
                let robot = Core::open(&arg(0), &arg(1));
                robot.feed(&hello_of(input));
                robot.next();
                (robot.send(), robot.reply())
            }
            "session.act" | "session.forfeit" => {
                let robot = Core::open("clunkyard-test", "2.0.0");
                begin(&robot, input);
                robot.feed(&state_at(input, number(&args[0]).unwrap()));
                assert_eq!(robot.next(), Event::State as i32);
                let status = if name == "session.act" {
                    robot.send()
                } else {
                    let (code, message) = (arg(1), arg(2));
                    cy_session_forfeit(robot.robot, code.as_ptr(), code.len() as u32, message.as_ptr(), message.len() as u32)
                };
                (status, robot.reply())
            }
            other => panic!("session case {other}"),
        };
        let expected = item.get("value").map(Value::to_json);
        match (item.get("error").and_then(Value::as_str), &expected) {
            (Some(error), _) if status != run.api.status(error) => run.fail(format!("{at}: expected {error}, got {}", run.status(status))),
            (None, Some(expected)) if reply.as_ref() != Some(expected) => run.fail(format!("{at}: expected {expected}, got {reply:?}")),
            _ => {}
        }
        run.lines.push(format!("{at} {status} {}", canon(&reply.map_or(Value::Null, Value::String))));
    }
}

/// A control field's setter or getter symbol: drive.throttle is
/// cy_drive_throttle and cy_drive_get_throttle.
fn field_symbol(field: &str, getter: bool) -> String {
    let (subsystem, key) = field.split_once('.').unwrap();
    if getter { format!("cy_{subsystem}_get_{key}") } else { format!("cy_{subsystem}_{key}") }
}

fn controls(run: &mut Run, file: &Value) {
    for (i, item) in list(file, "cases").iter().enumerate() {
        let input = text(item, "input");
        let robot = Core::open("clunkyard-test", "2.0.0");
        begin(&robot, &input);
        robot.feed(&state_at(&input, item.get("tick").and_then(Value::as_f64).unwrap()));
        assert_eq!(robot.next(), Event::State as i32);
        for (j, op) in list(item, "ops").iter().enumerate() {
            let at = format!("controls {i} {j}");
            let mut args = Args::default();
            let mut out = Out::new(4096, 4096);
            let field = op.get("set").or_else(|| op.get("get")).and_then(Value::as_str);
            let symbol = field.map(|field| field_symbol(field, op.get("get").is_some()));
            // A control the API does not have has no export to call, and a
            // text field takes only text.
            let unrepresentable = match (&symbol, op.get("arg")) {
                (Some(symbol), _) if !run.api.has_symbol(symbol) => Some(format!("no export {symbol}")),
                (_, Some(arg)) if (field == Some("status.text")) != arg.is_string() => Some(format!("{} is not {}", arg.to_json(), if arg.is_string() { "a number" } else { "text" })),
                _ => None,
            };
            if let Some(why) = unrepresentable {
                run.check(op, &Outcome::Unrepresentable(why), 0.0, &at);
                run.lines.push(format!("{at} skip"));
                continue;
            }
            let (status, value) = if let Some(field) = op.get("set").and_then(Value::as_str) {
                let arg = op.get("arg").unwrap();
                match arg.as_str() {
                    Some(text) => args.text(text),
                    None => args.number(number(arg).unwrap()),
                }
                (dispatch(&field_symbol(field, false), robot.robot, &args, &mut out).unwrap(), None)
            } else if let Some(field) = op.get("get").and_then(Value::as_str) {
                let status = dispatch(&field_symbol(field, true), robot.robot, &args, &mut out).unwrap();
                let value = (status == CY_OK).then(|| {
                    if field == "status.text" {
                        Value::String(String::from_utf8(out.bytes()[..out.count as usize].to_vec()).unwrap())
                    } else {
                        Value::Number(f64::from_le_bytes(out.bytes()[..8].try_into().unwrap()))
                    }
                });
                (status, value)
            } else if op.get("fire").is_some() {
                (cy_gun_fire(robot.robot), None)
            } else {
                let json = text(op, "extra");
                (cy_raw_extra(robot.robot, json.as_ptr(), json.len() as u32), None)
            };
            let outcome = Outcome::Done(status, value);
            if op.get("get").is_some() || op.get("error").is_some() {
                run.check(op, &outcome, 0.0, &at);
            } else if status != CY_OK {
                run.fail(format!("{at}: {} gave {}", op.to_json(), run.status(status)));
            }
            run.lines.push(format!("{at} {}", outcome_text(&outcome)));
        }
        assert_eq!(robot.send(), CY_OK);
        let act = robot.reply().unwrap();
        let expected = item.get("act").unwrap().to_json();
        if act != expected {
            run.fail(format!("controls {i} {}: wrote {act}, expected {expected}", text(item, "name")));
        }
        run.lines.push(format!("controls {i} act {}", canon(&Value::String(act))));
    }
}

/// The skips the C ABI forces, and nothing else: a slot that is not an
/// integer, tick counts that are not whole, which an int32_t cannot hold,
/// and controls the API does not have. Each must expect an error.
const UNREPRESENTABLE: [&str; 6] = [
    "drive.speed no export cy_drive_speed",
    "gun.heading no export cy_gun_get_heading",
    "gun.lead gun.lead: contact.slot is not a number",
    "status.text 7 is not text",
    "tracker.configure tracker.configure: 2.5 is not an i32",
    "tracker.configure tracker.configure: {\"f64\":\"Infinity\"} is not an i32",
];

static DIGESTS: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn compare_digest(part: &str, lines: &[String]) {
    let _one_at_a_time = DIGESTS.lock();
    let digest = digest(lines);
    let path = harness::root().join("sdk/core/tests/vectors.digest");
    if let Ok(dir) = std::env::var("CY_TRANSCRIPT") {
        std::fs::write(std::path::Path::new(&dir).join(format!("{part}.txt")), lines.join("\n") + "\n").unwrap();
    }
    let recorded = std::fs::read_to_string(&path).unwrap_or_default();
    let mut entries: Vec<(String, String)> = recorded.lines().filter_map(|line| line.split_once(' ')).map(|(a, b)| (a.to_string(), b.to_string())).collect();
    if std::env::var("CY_BLESS").is_ok() {
        entries.retain(|(name, _)| name != part);
        entries.push((part.to_string(), digest));
        entries.sort();
        std::fs::write(&path, entries.iter().map(|(name, digest)| format!("{name} {digest}\n")).collect::<String>()).unwrap();
        return;
    }
    let want = entries.iter().find(|(name, _)| name == part).map(|(_, digest)| digest.clone());
    assert_eq!(want.as_deref(), Some(digest.as_str()), "the {part} transcript ({} lines) differs from tests/vectors.digest; CY_TRANSCRIPT=DIR writes it", lines.len());
}

#[test]
fn every_api_vector_through_the_exports() {
    let mut run = Run { api: Api::load(), lines: Vec::new(), failures: Vec::new(), skipped: Vec::new() };
    let dir = harness::root().join("protocol/v2/vectors/api");
    let mut names: Vec<String> = std::fs::read_dir(&dir).unwrap().filter_map(|e| e.ok()?.file_name().into_string().ok()).filter(|n| n.ends_with(".json")).collect();
    names.sort();
    for file_name in names {
        let name = file_name.trim_end_matches(".json").to_string();
        let file = load(&format!("protocol/v2/vectors/api/{file_name}"));
        match name.as_str() {
            "angles" => angles(&mut run, &file),
            "session" => session(&mut run, &file),
            "controls" => controls(&mut run, &file),
            _ => {
                for (index, item) in list(&file, "streams").iter().enumerate() {
                    stream(&mut run, &file, &name, index, item);
                }
            }
        }
    }
    let mut kinds: Vec<&str> = run.skipped.iter().map(String::as_str).collect();
    kinds.sort();
    kinds.dedup();
    assert_eq!(kinds, UNREPRESENTABLE, "the cases the C ABI cannot carry");
    let shown: Vec<&String> = run.failures.iter().take(40).collect();
    assert!(run.failures.is_empty(), "{} of {} results differ from the vectors:\n{}", run.failures.len(), run.lines.len(), shown.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n"));
    compare_digest("abi", &run.lines);
}

fn vec3(value: &Value) -> Vec3 {
    let n = |key: &str| value.get(key).and_then(number).unwrap_or(0.0);
    Vec3 { x: n("x"), y: n("y"), z: n("z") }
}

#[test]
fn formulas_lead_and_tracker_through_the_internals() {
    let mut lines = Vec::new();
    let mut failures = Vec::new();
    let near = |a: f64, e: f64, tolerance: f64| (a - e).abs() <= tolerance * if e.abs() > 1.0 { e.abs() } else { 1.0 };

    let formulas = load("protocol/v2/vectors/formulas.json");
    let tolerance = number(formulas.get("tolerance").unwrap()).unwrap();
    for group in list(&formulas, "groups") {
        let formula = Formula::from_json(group.get("formula").unwrap()).unwrap();
        for item in list(group, "cases") {
            let (x, y) = (number(item.get("x").unwrap()).unwrap(), number(item.get("y").unwrap()).unwrap());
            let got = formula.evaluate(x).unwrap();
            if !near(got, y, tolerance) {
                failures.push(format!("{}({x}) = {got}, not {y}", text(group, "name")));
            }
            lines.push(format!("formula {} {}", text(group, "name"), canon(&Value::Number(got))));
        }
    }

    let leads = load("protocol/v2/vectors/lead.json");
    let tolerance = number(leads.get("tolerance").unwrap()).unwrap();
    for item in list(&leads, "cases") {
        let shot = Shot {
            shooter_velocity: item.get("shooterVelocity").map_or(Vec3::default(), vec3),
            gravity: item.get("gravity").map_or(Vec3::default(), vec3),
            muzzle: item.get("muzzle").and_then(number).unwrap_or(0.0),
            max_time: item.get("maxTime").and_then(number).unwrap_or(5.0),
        };
        let got = lead(vec3(item.get("shooter").unwrap()), vec3(item.get("target").unwrap()), vec3(item.get("targetVelocity").unwrap()), number(item.get("shotSpeed").unwrap()).unwrap(), &shot);
        let name = text(item, "name");
        match (got, item.get("expected").unwrap()) {
            (None, Value::Null) => {}
            (Some(aim), expected @ Value::Object(_)) => {
                for (key, value) in [("heading", aim.heading), ("pitch", aim.pitch), ("time", aim.time)] {
                    if !near(value, expected.get(key).and_then(number).unwrap(), tolerance) {
                        failures.push(format!("lead {name} {key} = {value}, not {}", expected.get(key).unwrap().to_json()));
                    }
                }
            }
            (got, expected) => failures.push(format!("lead {name}: {got:?}, not {}", expected.to_json())),
        }
        lines.push(format!("lead {name} {}", got.map_or("n".into(), |aim| [aim.heading, aim.pitch, aim.time].map(|v| canon(&Value::Number(v))).join(","))));
    }

    let tracking = load("protocol/v2/vectors/tracker.json");
    let tolerance = number(tracking.get("tolerance").unwrap()).unwrap();
    let setting = |key: &str| number(tracking.get(key).unwrap()).unwrap();
    for run in list(&tracking, "runs") {
        let mut tracker = Tracker::new(Settings { smoothing: setting("smoothing"), forget_ticks: setting("forgetTicks"), cue_ticks: setting("cueTicks") });
        let tick_rate = number(run.get("tickRate").unwrap()).unwrap();
        let name = text(run, "name");
        for step in list(run, "steps") {
            let state = State::decode(step.get("state").unwrap().clone());
            tracker.update(state.tick, state.me.position, &state.contacts, tick_rate);
            for (id, expected) in step.get("tracks").unwrap().as_object() {
                let Some(track) = tracker.tracks.get(&id) else {
                    failures.push(format!("tracker {name} {id}: no track"));
                    continue;
                };
                if Some(track.tick) != expected.get("lastSeen").and_then(number) {
                    failures.push(format!("tracker {name} {id}: last seen {}", track.tick));
                }
                for (what, got) in [("position", track.position), ("velocity", track.velocity)] {
                    let want = vec3(expected.get(what).unwrap());
                    if !(near(got.x, want.x, tolerance) && near(got.y, want.y, tolerance) && near(got.z, want.z, tolerance)) {
                        failures.push(format!("tracker {name} {id} {what}: {got:?}, not {want:?}"));
                    }
                    lines.push(format!("tracker {name} {} {id} {what} {}", state.tick, [got.x, got.y, got.z].map(|v| canon(&Value::Number(v))).join(",")));
                }
            }
            let cue = tracker.cue();
            match (cue, step.get("cue").filter(|c| !matches!(c, Value::Null))) {
                (None, None) => {}
                (Some(cue), Some(want)) => {
                    let position = vec3(want.get("position").unwrap());
                    let source = Source::from_wire(&text(want, "source")).map_or(0, |s| s as i32);
                    if !(near(cue.position.x, position.x, tolerance) && near(cue.position.y, position.y, tolerance) && near(cue.position.z, position.z, tolerance))
                        || Some(cue.tick) != want.get("tick").and_then(number)
                        || cue.source != source
                        || !near(cue.bearing_error, want.get("bearingError").and_then(number).unwrap(), tolerance)
                    {
                        failures.push(format!("tracker {name} {} cue: {cue:?}, not {}", state.tick, want.to_json()));
                    }
                }
                (cue, want) => failures.push(format!("tracker {name} {} cue: {cue:?}, not {want:?}", state.tick)),
            }
            lines.push(format!("tracker {name} {} cue {}", state.tick, cue.map_or("n".into(), |c| [c.position.x, c.position.y, c.position.z, c.tick, c.bearing_error].map(|v| canon(&Value::Number(v))).join(","))));
        }
    }

    assert!(failures.is_empty(), "{} differ:\n{}", failures.len(), failures.iter().take(40).cloned().collect::<Vec<_>>().join("\n"));
    compare_digest("core", &lines);
}
