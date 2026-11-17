//! Wall awareness beyond the vectors: me.wall_contact reads the state's
//! self.wall, and nav.wall_margin is geometry from the position, the radius
//! and the arena size, with the walls' unit normals pointing into the arena.

mod harness;

use clunkyard::generated::ffi::*;
use clunkyard::generated::*;
use clunkyard::json::{self, Value};
use harness::*;

const RADIUS: f64 = 18.0;

fn number(value: f64) -> Value {
    Value::Number(value)
}

fn vector(x: f64, y: f64, z: f64) -> Value {
    Value::Object(vec![("x".into(), number(x)), ("y".into(), number(y)), ("z".into(), number(z))])
}

/// A robot started on an input's hello and fed that input's first state with
/// its position (and altitude, and wall) replaced.
fn robot_at(input: &str, position: (f64, f64, f64), altitude: Option<f64>, wall: Option<(f64, f64, f64)>) -> Core {
    let lines: Vec<String> = read(&format!("protocol/v2/vectors/api/inputs/{input}.ndjson")).trim().split('\n').map(str::to_string).collect();
    let robot = Core::open("clunkyard-test", "2.0.0");
    robot.feed(&lines[0]);
    assert_eq!(robot.next(), Event::Hello as i32);
    let mut state = json::parse(&lines[1]).unwrap();
    let mut me = state.get("self").unwrap().clone();
    me.insert("position", vector(position.0, position.1, position.2));
    if let Some(altitude) = altitude {
        me.insert("altitude", number(altitude));
    }
    if let Some(wall) = wall {
        me.insert("wall", vector(wall.0, wall.1, wall.2));
    }
    state.insert("self", me);
    robot.feed(&state.to_json());
    assert_eq!(robot.next(), Event::State as i32);
    robot
}

fn contact(robot: &Core) -> Vec3 {
    let mut out = Vec3::default();
    assert_eq!(cy_me_wall_contact(robot.robot, &mut out), CY_OK);
    out
}

fn margin(robot: &Core) -> WallMarginV1 {
    let mut out = WallMarginV1::default();
    assert_eq!(cy_nav_wall_margin_v1(robot.robot, &mut out), CY_OK);
    out
}

fn unit(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3 { x, y, z }
}

#[test]
fn a_state_without_a_wall_reads_as_no_contact() {
    let robot = robot_at("walls-2d-classic", (500.0, 350.0, 0.0), None, None);
    assert_eq!(contact(&robot), unit(0.0, 0.0, 0.0));
}

#[test]
fn a_corner_contact_sums_the_walls_normals() {
    let robot = robot_at("walls-2d-classic", (982.0, 682.0, 0.0), None, Some((-1.0, -1.0, 0.0)));
    assert_eq!(contact(&robot), unit(-1.0, -1.0, 0.0));
}

#[test]
fn the_margin_is_the_gap_to_the_nearer_wall_on_each_axis() {
    let robot = robot_at("walls-2d-classic", (100.0, 650.0, 0.0), None, None);
    let m = margin(&robot);
    assert_eq!((m.x, m.y), (100.0 - RADIUS, 700.0 - RADIUS - 650.0));
    assert_eq!(m.nearest, m.y);
    assert_eq!(m.normal, unit(0.0, -1.0, 0.0), "the far wall's normal points back toward the arena");
    assert_eq!(m.z, FAR, "2D has no z walls");
}

#[test]
fn touching_a_wall_leaves_no_gap_and_a_tie_takes_the_first_axis() {
    let robot = robot_at("walls-2d-classic", (RADIUS, 350.0, 0.0), None, None);
    let m = margin(&robot);
    assert_eq!((m.x, m.nearest, m.normal), (0.0, 0.0, unit(1.0, 0.0, 0.0)));
    let corner = margin(&robot_at("walls-2d-classic", (50.0, 50.0, 0.0), None, None));
    assert_eq!((corner.nearest, corner.normal), (32.0, unit(1.0, 0.0, 0.0)), "equal gaps pick the first axis");
}

#[test]
fn a_robot_cannot_be_closer_than_nothing_to_a_wall() {
    // A rounded position a hair inside the radius still reads as touching.
    let robot = robot_at("walls-2d-classic", (RADIUS - 0.01, 350.0, 0.0), None, None);
    assert_eq!(margin(&robot).x, 0.0);
}

#[test]
fn in_3d_without_gravity_the_floor_and_ceiling_are_walls() {
    let low = margin(&robot_at("walls-3d-zero-g", (500.0, 350.0, 60.0), None, None));
    assert_eq!((low.x, low.y, low.z, low.nearest), (482.0, 332.0, 60.0 - RADIUS, 60.0 - RADIUS));
    assert_eq!(low.normal, unit(0.0, 0.0, 1.0));
    let high = margin(&robot_at("walls-3d-zero-g", (500.0, 350.0, 570.0), None, None));
    assert_eq!((high.z, high.normal), (600.0 - RADIUS - 570.0, unit(0.0, 0.0, -1.0)));
}

#[test]
fn under_gravity_the_floor_is_the_ground_at_the_robots_altitude() {
    let robot = margin(&robot_at("walls-3d-gravity-flat", (500.0, 350.0, 300.0), Some(7.5), None));
    assert_eq!((robot.z, robot.nearest, robot.normal), (7.5, 7.5, unit(0.0, 0.0, 1.0)));
}
