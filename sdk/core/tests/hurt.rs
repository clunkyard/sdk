//! Damage by cause beyond the vectors: me.hurt and the hurt-* DSL words read
//! each state's self.hurt afresh, so a loss is reported for the one state
//! that follows it and never carried into the next.

mod harness;

use clunkyard::generated::ffi::*;
use clunkyard::generated::*;
use clunkyard::json::{self, Value};
use harness::*;

fn hurt(robot: &Core) -> HurtV1 {
    let mut out = HurtV1::default();
    assert_eq!(cy_me_hurt_v1(robot.robot, &mut out), CY_OK);
    out
}

fn words(robot: &Core) -> DslWordsV1 {
    let mut out = DslWordsV1::default();
    assert_eq!(cy_dsl_fill_v1(robot.robot, &mut out), CY_OK);
    out
}

fn told(state: &Value) -> HurtV1 {
    let Some(hurt) = state.get("self").and_then(|me| me.get("hurt")) else { return HurtV1::default() };
    let cause = |name| hurt.get(name).and_then(Value::as_f64).unwrap_or(0.0);
    HurtV1 { shot: cause("shot"), burst: cause("burst"), collision: cause("collision"), boundary: cause("boundary") }
}

#[test]
fn each_state_reports_only_the_loss_of_the_tick_before_it() {
    let (mut hurt_states, mut clear_after_hurt) = (0, 0);
    for input in ["hurt-2d-inertial-2d", "hurt-3d-zero-g"] {
        let lines: Vec<String> = read(&format!("protocol/v2/vectors/api/inputs/{input}.ndjson")).trim().split('\n').map(str::to_string).collect();
        let robot = Core::open("clunkyard-test", "2.0.0");
        robot.feed(&lines[0]);
        assert_eq!(robot.next(), Event::Hello as i32);
        let mut before = HurtV1::default();
        for line in &lines[1..] {
            robot.feed(line);
            assert_eq!(robot.next(), Event::State as i32);
            let want = told(&json::parse(line).unwrap());
            let (got, w) = (hurt(&robot), words(&robot));
            assert_eq!(got, want, "{input}: me.hurt is this state's self.hurt, or zero");
            assert_eq!((w.hurt_shot, w.hurt_burst, w.hurt_collision, w.hurt_boundary), (want.shot, want.burst, want.collision, want.boundary), "{input}: the words follow me.hurt");
            if want != HurtV1::default() {
                hurt_states += 1;
            } else if before != HurtV1::default() {
                clear_after_hurt += 1;
            }
            before = want;
        }
    }
    assert!(hurt_states > 0 && clear_after_hurt > 0, "the inputs have hurt states and clear states after them");
}

#[test]
fn a_cause_the_state_leaves_out_reads_as_zero() {
    let lines: Vec<String> = read("protocol/v2/vectors/api/inputs/hurt-2d-inertial-2d.ndjson").trim().split('\n').map(str::to_string).collect();
    let robot = Core::open("clunkyard-test", "2.0.0");
    robot.feed(&lines[0]);
    assert_eq!(robot.next(), Event::Hello as i32);
    let mut state = json::parse(&lines[1]).unwrap();
    let mut me = state.get("self").unwrap().clone();
    me.insert("hurt", Value::Object(vec![("boundary".into(), Value::Number(0.5))]));
    state.insert("self", me);
    robot.feed(&state.to_json());
    assert_eq!(robot.next(), Event::State as i32);
    assert_eq!(hurt(&robot), HurtV1 { boundary: 0.5, ..HurtV1::default() });
}
