//! The session's rules beyond the vectors: version refusal, phases,
//! CY_SHORT, and the panic reset and poisoning (design section 1.4).
//! wasm32 cannot unwind, so the panic tests are native only; there the
//! loader re-instantiates (test/core-wasm.test.js).

mod harness;

use clunkyard::generated::ffi::*;
use clunkyard::generated::*;
use clunkyard::json::{self, Value};
use harness::*;

const INPUT: &str = "protocol/v2/vectors/api/inputs/mechanics-2d-classic.ndjson";

fn lines() -> Vec<String> {
    read(INPUT).trim().split('\n').map(str::to_string).collect()
}

fn started() -> (Core, Vec<String>) {
    let robot = Core::open("clunkyard-test", "2.0.0");
    let lines = lines();
    robot.feed(&lines[0]);
    assert_eq!(robot.next(), Event::Hello as i32);
    assert_eq!(robot.send(), CY_OK);
    assert!(robot.reply().unwrap().contains("\"ready\""));
    (robot, lines)
}

fn turn(robot: &Core, line: &str) {
    robot.feed(line);
    assert_eq!(robot.next(), Event::State as i32);
}

// Used by the panic tests, which are native only.
#[cfg_attr(target_family = "wasm", allow(dead_code))]
fn act(robot: &Core) -> Value {
    assert_eq!(robot.send(), CY_OK);
    json::parse(&robot.reply().unwrap()).unwrap()
}

fn tick(line: &str) -> f64 {
    json::parse(line).unwrap().get("tick").and_then(Value::as_f64).unwrap()
}

#[test]
fn open_refuses_another_major_or_a_newer_minor() {
    let mut robot = std::ptr::null_mut();
    let name = "clunkyard-test";
    for (major, minor, status) in [(API_MAJOR + 1, 0, CY_E_VERSION), (API_MAJOR - 1, 0, CY_E_VERSION), (API_MAJOR, API_MINOR + 1, CY_E_VERSION), (API_MAJOR, API_MINOR, CY_OK)] {
        assert_eq!(cy_open_memory(major, minor, name.as_ptr(), name.len() as u32, name.as_ptr(), 0, &mut robot), status, "{major}.{minor}");
    }
    cy_close(robot);
    let (mut a, mut b, mut c) = (0, 0, 0);
    assert_eq!(cy_version(&mut a, &mut b, &mut c), CY_OK);
    assert_eq!((a, b, c), (API_MAJOR, API_MINOR, API_PATCH));
    assert_eq!(cy_session_send(std::ptr::null_mut()), CY_E_ARGUMENT, "a null handle");
}

#[test]
fn a_protocol_1_hello_is_refused() {
    let robot = Core::open("clunkyard-test", "2.0.0");
    robot.feed(read("protocol/v2/fixtures/protocol-1-hello.json").trim());
    assert_eq!(robot.next(), CY_E_VERSION);
    assert!(robot.last_error().contains("protocol 2"), "{}", robot.last_error());
}

#[test]
fn each_call_keeps_to_its_phase() {
    let robot = Core::open("clunkyard-test", "2.0.0");
    assert_eq!(robot.send(), CY_E_PHASE, "nothing to answer");
    let mut tick = 0;
    assert_eq!(cy_clock_tick(robot.robot, &mut tick), CY_E_PHASE, "no hello");
    assert_eq!(robot.next(), Event::End as i32, "an empty memory port ends");
    let lines = lines();
    robot.feed(&lines[1]);
    assert_eq!(robot.next(), CY_E_PHASE, "a state before the hello");
    robot.feed(&lines[0]);
    assert_eq!(robot.next(), Event::Hello as i32);
    assert_eq!(cy_clock_tick(robot.robot, &mut tick), CY_E_PHASE, "no state");
    assert_eq!(robot.send(), CY_OK);
    assert_eq!(robot.send(), CY_E_PHASE, "the hello is answered");
    robot.feed("{\"v\":2,\"type\":\"news\"}");
    robot.feed(&lines[1]);
    assert_eq!(robot.next(), Event::State as i32, "a message the core does not know is skipped");
    let mut event = 0;
    assert_eq!(cy_session_event(robot.robot, &mut event), CY_OK);
    assert_eq!(event, Event::State as i32);
}

#[test]
fn a_short_buffer_reports_the_length_and_loses_nothing() {
    let (robot, lines) = started();
    turn(&robot, &lines[1]);
    let mut buf = [0u8; 2];
    let mut length = 0;
    assert_eq!(cy_robot_id(robot.robot, 1, buf.as_mut_ptr(), 2, &mut length), CY_SHORT);
    assert_eq!(length, "scout".len() as u32);
    assert_eq!(robot.send(), CY_OK);
    assert_eq!(cy_session_reply(robot.robot, buf.as_mut_ptr(), 2, &mut length), CY_SHORT);
    let line = robot.reply().expect("the reply is still there after CY_SHORT");
    assert_eq!(line.len() as u32, length);
    assert_eq!(robot.reply(), None);
}

#[test]
fn a_forfeit_ends_the_session_with_later_states_queued() {
    let (robot, lines) = started();
    for line in &lines[1..4] {
        robot.feed(line);
    }
    assert_eq!(robot.next(), Event::State as i32);
    assert_eq!(cy_session_forfeit(robot.robot, b"".as_ptr(), 0, b"".as_ptr(), 0), CY_E_ARGUMENT);
    assert_eq!(robot.send(), CY_OK, "an invalid forfeit leaves the tick playable");
    assert!(robot.reply().unwrap().contains("\"act\""));
    assert_eq!(robot.next(), Event::State as i32, "an invalid forfeit does not end the session");
    let (code, message) = ("LOST_TRACK", "");
    assert_eq!(cy_session_forfeit(robot.robot, code.as_ptr(), code.len() as u32, message.as_ptr(), 0), CY_OK);
    assert_eq!(robot.reply().unwrap(), format!("{{\"v\":2,\"type\":\"forfeit\",\"tick\":{},\"code\":\"LOST_TRACK\"}}", tick(&lines[2])));
    assert_eq!(robot.next(), Event::End as i32, "the queued state cannot restart a forfeited match");
    assert_eq!(robot.next(), Event::End as i32, "a forfeited session stays ended");
    assert_eq!(robot.send(), CY_E_PHASE, "the tick is answered");
    assert_eq!(robot.reply(), None, "no act follows the forfeit");
}

#[cfg_attr(target_family = "wasm", allow(dead_code))]
fn throttle(robot: &Core, value: f64) {
    assert_eq!(cy_drive_throttle(robot.robot, value), CY_OK);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn a_panic_answers_with_an_empty_act_and_resets_on_the_next_state() {
    let (robot, lines) = started();
    // A few states, so the Tracker has a moving track for robot 2.
    for line in &lines[1..6] {
        turn(&robot, line);
        throttle(&robot, 0.5);
        assert_eq!(act(&robot).at("drive.throttle").and_then(Value::as_f64), Some(0.5));
    }
    let mut seen = 0;
    assert_eq!(cy_tracker_last_seen(robot.robot, 2, &mut seen), CY_OK);
    let mut velocity = Vec3::default();
    assert_eq!(cy_tracker_velocity(robot.robot, 2, &mut velocity), CY_OK);
    assert!(velocity != Vec3::default(), "the track moves before the fault");

    turn(&robot, &lines[6]);
    throttle(&robot, 0.5);
    assert_eq!(cy_test_panic(robot.robot), CY_E_INTERNAL);
    assert!(robot.last_error().contains("fault"), "{}", robot.last_error());
    let empty = act(&robot);
    assert_eq!(empty.to_json(), format!("{{\"v\":2,\"type\":\"act\",\"tick\":{}}}", tick(&lines[6])), "the faulted tick's act is empty");

    // The next state rebuilds the session: the Tracker starts empty, so the
    // track restarts at rest, and play goes on.
    turn(&robot, &lines[7]);
    assert_eq!(cy_tracker_velocity(robot.robot, 2, &mut velocity), CY_OK);
    assert_eq!(velocity, Vec3::default(), "a fresh track is at rest");
    let mut id = [0u8; 16];
    let mut length = 0;
    assert_eq!(cy_robot_id(robot.robot, 2, id.as_mut_ptr(), 16, &mut length), CY_OK, "slots survive the reset");
    throttle(&robot, 0.25);
    assert_eq!(act(&robot).at("drive.throttle").and_then(Value::as_f64), Some(0.25));
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn three_panics_poison_the_handle() {
    let (robot, lines) = started();
    for (i, line) in lines[1..].iter().enumerate().take(6) {
        turn(&robot, line);
        throttle(&robot, 0.5);
        if i < 3 {
            assert_eq!(cy_test_panic(robot.robot), CY_E_INTERNAL);
        }
        let written = act(&robot);
        assert_eq!(written.get("drive"), None, "state {i}: an empty act");
        assert_eq!(written.get("tick").and_then(Value::as_f64), Some(tick(line)));
    }
    // A new match (a new hello) starts clean.
    robot.feed(&lines[0]);
    assert_eq!(robot.next(), Event::Hello as i32);
    assert_eq!(robot.send(), CY_OK);
    robot.reply();
    turn(&robot, &lines[1]);
    throttle(&robot, 0.5);
    assert_eq!(act(&robot).at("drive.throttle").and_then(Value::as_f64), Some(0.5));
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn a_panic_while_reading_a_state_still_answers_its_tick() {
    let (robot, lines) = started();
    turn(&robot, &lines[1]);
    assert_eq!(act(&robot).get("type").and_then(Value::as_str), Some("act"));
    let mut state = json::parse(&lines[2]).unwrap();
    state.insert("cyTestPanic", Value::Bool(true));
    robot.feed(&state.to_json());
    assert_eq!(robot.next(), CY_E_INTERNAL);
    assert_eq!(act(&robot).to_json(), format!("{{\"v\":2,\"type\":\"act\",\"tick\":{}}}", tick(&lines[2])));
    turn(&robot, &lines[3]);
    throttle(&robot, 0.5);
    assert_eq!(act(&robot).at("drive.throttle").and_then(Value::as_f64), Some(0.5), "the next state plays");
}
