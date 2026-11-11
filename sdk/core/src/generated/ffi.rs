// Generated from protocol/v2/api.json by protocol/v2/codegen/rust.js; do not edit. Run `make api`.
//! Every C export: argument checks, catch_unwind, and a call to the hand-written core.

#![allow(clippy::not_unsafe_ptr_arg_deref, clippy::missing_safety_doc, clippy::too_many_arguments)]

use super::records::*;
use crate::act;
use crate::runtime::{self, PortKind, Robot};
use std::panic::{self, AssertUnwindSafe};

fn guard(robot: *mut Robot, symbol: &'static str, body: impl FnOnce(&mut Robot) -> Result<i32, Error>) -> i32 {
    let Some(r) = (unsafe { robot.as_mut() }) else { return CY_E_ARGUMENT };
    match panic::catch_unwind(AssertUnwindSafe(|| body(&mut *r))) {
        Ok(Ok(status)) => status,
        Ok(Err(error)) => {
            runtime::record_error(r, &error);
            error.status
        }
        Err(_) => {
            runtime::record_panic(r, symbol);
            CY_E_INTERNAL
        }
    }
}

// For the stateless subsystems (angles), which take no handle: an error
// has nowhere to keep its message, so only the status comes back.
fn guard_static(body: impl FnOnce() -> Result<i32, Error>) -> i32 {
    match panic::catch_unwind(AssertUnwindSafe(body)) {
        Ok(Ok(status)) => status,
        Ok(Err(error)) => error.status,
        Err(_) => CY_E_INTERNAL,
    }
}

fn finite(value: f64, name: &str) -> Result<f64, Error> {
    if value.is_finite() { Ok(value) } else { Err(Error::not_finite(format!("{name} is not finite"))) }
}

fn record_in<'a, T>(pointer: *const T, name: &str) -> Result<&'a T, Error> {
    unsafe { pointer.as_ref() }.ok_or_else(|| Error::argument(format!("{name} is null")))
}

fn text_in<'a>(pointer: *const u8, length: u32, name: &str) -> Result<&'a str, Error> {
    if length == 0 {
        return Ok("");
    }
    if pointer.is_null() {
        return Err(Error::argument(format!("{name} is null")));
    }
    let bytes = unsafe { std::slice::from_raw_parts(pointer, length as usize) };
    std::str::from_utf8(bytes).map_err(|_| Error::argument(format!("{name} is not UTF-8")))
}

fn enum_in<T>(value: i32, from: fn(i32) -> Option<T>, name: &str) -> Result<T, Error> {
    from(value).ok_or_else(|| Error::argument(format!("{name} is not a known value: {value}")))
}

fn enum_or_any<T>(value: i32, from: fn(i32) -> Option<T>, name: &str) -> Result<Option<T>, Error> {
    if value == 0 { Ok(None) } else { enum_in(value, from, name).map(Some) }
}

fn write<T>(out: *mut T, value: T) -> Result<i32, Error> {
    if out.is_null() {
        return Err(Error::argument("out is null"));
    }
    unsafe { out.write(value) };
    Ok(CY_OK)
}

fn write_list<T: Copy>(out: *mut T, capacity: u32, count: *mut u32, items: &[T]) -> Result<i32, Error> {
    if count.is_null() || (out.is_null() && capacity > 0) {
        return Err(Error::argument("out or count is null"));
    }
    let length = u32::try_from(items.len()).map_err(|_| Error::internal("list too long"))?;
    unsafe { count.write(length) };
    if length > capacity {
        return Ok(CY_SHORT);
    }
    unsafe { std::ptr::copy_nonoverlapping(items.as_ptr(), out, items.len()) };
    Ok(CY_OK)
}

fn write_text(buf: *mut u8, capacity: u32, length: *mut u32, text: &str) -> Result<i32, Error> {
    if length.is_null() || (buf.is_null() && capacity > 0) {
        return Err(Error::argument("buf or length is null"));
    }
    let size = u32::try_from(text.len()).map_err(|_| Error::internal("text too long"))?;
    unsafe { length.write(size) };
    if size > capacity {
        return Ok(CY_SHORT);
    }
    unsafe {
        std::ptr::copy_nonoverlapping(text.as_ptr(), buf, text.len());
        if size < capacity {
            buf.add(text.len()).write(0);
        }
    }
    Ok(CY_OK)
}

/// The core's API version.
#[unsafe(no_mangle)]
pub extern "C" fn cy_version(major: *mut i32, minor: *mut i32, patch: *mut i32) -> i32 {
    if major.is_null() || minor.is_null() || patch.is_null() {
        return CY_E_ARGUMENT;
    }
    unsafe {
        major.write(API_MAJOR);
        minor.write(API_MINOR);
        patch.write(API_PATCH);
    }
    CY_OK
}

fn open(port: PortKind, api_major: i32, api_minor: i32, binding: *const u8, binding_length: u32, version: *const u8, version_length: u32, out: *mut *mut Robot) -> i32 {
    if out.is_null() {
        return CY_E_ARGUMENT;
    }
    if api_major != API_MAJOR || api_minor > API_MINOR {
        return CY_E_VERSION;
    }
    guard_static(|| {
        let robot = runtime::open(port, text_in(binding, binding_length, "binding")?, text_in(version, version_length, "binding_version")?)?;
        unsafe { out.write(Box::into_raw(Box::new(robot))) };
        Ok(CY_OK)
    })
}

/// Opens a robot on standard input and output. It refuses with CY_E_VERSION
/// when the majors differ or the core's minor is older than the binding's.
/// The binding's name and version go into the ready reply's sdk.
#[unsafe(no_mangle)]
pub extern "C" fn cy_open(api_major: i32, api_minor: i32, binding: *const u8, binding_length: u32, version: *const u8, version_length: u32, out: *mut *mut Robot) -> i32 {
    open(PortKind::Stdio, api_major, api_minor, binding, binding_length, version, version_length, out)
}

/// Opens a robot on a memory port: session.feed hands it lines and
/// session.reply takes its own. For tests and event-loop runtimes.
#[unsafe(no_mangle)]
pub extern "C" fn cy_open_memory(api_major: i32, api_minor: i32, binding: *const u8, binding_length: u32, version: *const u8, version_length: u32, out: *mut *mut Robot) -> i32 {
    open(PortKind::Memory, api_major, api_minor, binding, binding_length, version, version_length, out)
}

/// Frees a robot. A null robot is fine.
#[unsafe(no_mangle)]
pub extern "C" fn cy_close(robot: *mut Robot) -> i32 {
    if robot.is_null() {
        return CY_OK;
    }
    guard_static(|| {
        drop(unsafe { Box::from_raw(robot) });
        Ok(CY_OK)
    })
}

/// The message of the robot's last error, kept on the handle.
#[unsafe(no_mangle)]
pub extern "C" fn cy_last_error(robot: *mut Robot, buf: *mut u8, capacity: u32, length: *mut u32) -> i32 {
    guard(robot, "cy_last_error", |r| write_text(buf, capacity, length, runtime::last_error(r)))
}

/// Panics on purpose, for the panic reset and poisoning tests.
#[cfg(feature = "test-panic")]
#[unsafe(no_mangle)]
pub extern "C" fn cy_test_panic(robot: *mut Robot) -> i32 {
    guard(robot, "cy_test_panic", |_| panic!("cy_test_panic"))
}

/// The match is 2D or 3D.
#[unsafe(no_mangle)]
pub extern "C" fn cy_match_mode(robot: *mut Robot, out: *mut i32) -> i32 {
    guard(robot, "cy_match_mode", |r| {
        let value = crate::r#match::mode(&*r)?;
        write(out, value as i32)
    })
}

/// The ruleset in play.
#[unsafe(no_mangle)]
pub extern "C" fn cy_match_ruleset(robot: *mut Robot, out: *mut i32) -> i32 {
    guard(robot, "cy_match_ruleset", |r| {
        let value = crate::r#match::ruleset(&*r)?;
        write(out, value as i32)
    })
}

/// Whether the drive sets speeds (direct) or forces.
#[unsafe(no_mangle)]
pub extern "C" fn cy_match_control(robot: *mut Robot, out: *mut i32) -> i32 {
    guard(robot, "cy_match_control", |r| {
        let value = crate::r#match::control(&*r)?;
        write(out, value as i32)
    })
}

/// Robots in the match, you included; slots run 1 to this.
#[unsafe(no_mangle)]
pub extern "C" fn cy_match_robots(robot: *mut Robot, out: *mut i32) -> i32 {
    guard(robot, "cy_match_robots", |r| {
        let value = crate::r#match::robots(&*r)?;
        write(out, value)
    })
}

/// The match is 3D.
#[unsafe(no_mangle)]
pub extern "C" fn cy_match_is_3d(robot: *mut Robot, out: *mut i32) -> i32 {
    guard(robot, "cy_match_is_3d", |r| {
        let value = crate::r#match::is_3d(&*r)?;
        write(out, i32::from(value))
    })
}

/// Ticks per second.
#[unsafe(no_mangle)]
pub extern "C" fn cy_match_tick_rate(robot: *mut Robot, out: *mut i32) -> i32 {
    guard(robot, "cy_match_tick_rate", |r| {
        let value = crate::r#match::tick_rate(&*r)?;
        write(out, value)
    })
}

/// The last tick the match can reach.
#[unsafe(no_mangle)]
pub extern "C" fn cy_match_max_ticks(robot: *mut Robot, out: *mut i32) -> i32 {
    guard(robot, "cy_match_max_ticks", |r| {
        let value = crate::r#match::max_ticks(&*r)?;
        write(out, value)
    })
}

/// The id the protocol gives the robot in a slot.
#[unsafe(no_mangle)]
pub extern "C" fn cy_robot_id(robot: *mut Robot, slot: i32, buf: *mut u8, capacity: u32, length: *mut u32) -> i32 {
    guard(robot, "cy_robot_id", |r| {
        let text = crate::r#match::robot_id(&*r, slot)?;
        write_text(buf, capacity, length, &text)
    })
}

/// This tick, from 0.
#[unsafe(no_mangle)]
pub extern "C" fn cy_clock_tick(robot: *mut Robot, out: *mut i32) -> i32 {
    guard(robot, "cy_clock_tick", |r| {
        let value = crate::clock::tick(&*r)?;
        write(out, value)
    })
}

/// The match time in seconds.
#[unsafe(no_mangle)]
pub extern "C" fn cy_clock_seconds(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_clock_seconds", |r| {
        let value = crate::clock::seconds(&*r)?;
        write(out, value)
    })
}

/// Ticks left: match.max_ticks less this tick.
#[unsafe(no_mangle)]
pub extern "C" fn cy_clock_remaining_ticks(robot: *mut Robot, out: *mut i32) -> i32 {
    guard(robot, "cy_clock_remaining_ticks", |r| {
        let value = crate::clock::remaining_ticks(&*r)?;
        write(out, value)
    })
}

/// The arena runs from 0 to this on each axis.
#[unsafe(no_mangle)]
pub extern "C" fn cy_arena_size(robot: *mut Robot, out: *mut Vec3) -> i32 {
    guard(robot, "cy_arena_size", |r| {
        let value = crate::arena::size(&*r)?;
        write(out, value)
    })
}

/// The acceleration of gravity, units/s².
#[unsafe(no_mangle)]
pub extern "C" fn cy_arena_gravity(robot: *mut Robot, out: *mut Vec3) -> i32 {
    guard(robot, "cy_arena_gravity", |r| {
        let value = crate::arena::gravity(&*r)?;
        write(out, value)
    })
}

/// The kind of ground: none, flat or hills.
#[unsafe(no_mangle)]
pub extern "C" fn cy_arena_terrain_kind(robot: *mut Robot, out: *mut i32) -> i32 {
    guard(robot, "cy_arena_terrain_kind", |r| {
        let value = crate::arena::terrain_kind(&*r)?;
        write(out, value as i32)
    })
}

/// The height of the ground at a point, interpolated as the arena does.
#[unsafe(no_mangle)]
pub extern "C" fn cy_arena_ground_height(robot: *mut Robot, x: f64, y: f64, out: *mut f64) -> i32 {
    guard(robot, "cy_arena_ground_height", |r| {
        let value = crate::arena::ground_height(&*r, finite(x, "x")?, finite(y, "y")?)?;
        write(out, value)
    })
}

/// Health left.
#[unsafe(no_mangle)]
pub extern "C" fn cy_me_health(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_me_health", |r| {
        let value = crate::me::health(&*r)?;
        write(out, value)
    })
}

/// Your centre.
#[unsafe(no_mangle)]
pub extern "C" fn cy_me_position(robot: *mut Robot, out: *mut Vec3) -> i32 {
    guard(robot, "cy_me_position", |r| {
        let value = crate::me::position(&*r)?;
        write(out, value)
    })
}

/// Your velocity, units/s.
#[unsafe(no_mangle)]
pub extern "C" fn cy_me_velocity(robot: *mut Robot, out: *mut Vec3) -> i32 {
    guard(robot, "cy_me_velocity", |r| {
        let value = crate::me::velocity(&*r)?;
        write(out, value)
    })
}

/// Your chassis heading.
#[unsafe(no_mangle)]
pub extern "C" fn cy_me_heading(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_me_heading", |r| {
        let value = crate::me::heading(&*r)?;
        write(out, value)
    })
}

/// Your chassis pitch; 0 in 2D.
#[unsafe(no_mangle)]
pub extern "C" fn cy_me_pitch(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_me_pitch", |r| {
        let value = crate::me::pitch(&*r)?;
        write(out, value)
    })
}

/// Your chassis roll; 0 in 2D and classic.
#[unsafe(no_mangle)]
pub extern "C" fn cy_me_roll(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_me_roll", |r| {
        let value = crate::me::roll(&*r)?;
        write(out, value)
    })
}

/// Your rotation rates, degrees/s.
#[unsafe(no_mangle)]
pub extern "C" fn cy_me_angular_velocity(robot: *mut Robot, out: *mut Rotation) -> i32 {
    guard(robot, "cy_me_angular_velocity", |r| {
        let value = crate::me::angular_velocity(&*r)?;
        write(out, value)
    })
}

/// Your chassis axes in world axes.
#[unsafe(no_mangle)]
pub extern "C" fn cy_me_attitude_v1(robot: *mut Robot, out: *mut AttitudeV1) -> i32 {
    guard(robot, "cy_me_attitude_v1", |r| {
        let value = crate::me::attitude(&*r)?;
        write(out, value)
    })
}

/// You rest on the ground, under gravity.
#[unsafe(no_mangle)]
pub extern "C" fn cy_me_grounded(robot: *mut Robot, out: *mut i32) -> i32 {
    guard(robot, "cy_me_grounded", |r| {
        let value = crate::me::grounded(&*r)?;
        write(out, i32::from(value))
    })
}

/// In 3D, the height of your underside above the ground.
#[unsafe(no_mangle)]
pub extern "C" fn cy_me_altitude(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_me_altitude", |r| {
        let value = crate::me::altitude(&*r)?;
        write(out, value)
    })
}

/// What the previous tick applied to you.
#[unsafe(no_mangle)]
pub extern "C" fn cy_me_applied_v1(robot: *mut Robot, out: *mut AppliedV1) -> i32 {
    guard(robot, "cy_me_applied_v1", |r| {
        let value = crate::me::applied(&*r)?;
        write(out, value)
    })
}

/// Charge now, before this tick's regeneration and driving.
#[unsafe(no_mangle)]
pub extern "C" fn cy_battery_charge(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_battery_charge", |r| {
        let value = crate::battery::charge(&*r)?;
        write(out, value)
    })
}

/// Charge when full.
#[unsafe(no_mangle)]
pub extern "C" fn cy_battery_capacity(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_battery_capacity", |r| {
        let value = crate::battery::capacity(&*r)?;
        write(out, value)
    })
}

/// Charge regained each second.
#[unsafe(no_mangle)]
pub extern "C" fn cy_battery_regen(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_battery_regen", |r| {
        let value = crate::battery::regen(&*r)?;
        write(out, value)
    })
}

/// The drive fraction regeneration pays for forever.
#[unsafe(no_mangle)]
pub extern "C" fn cy_battery_sustainable_drive(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_battery_sustainable_drive", |r| {
        let value = crate::battery::sustainable_drive(&*r)?;
        write(out, value)
    })
}

/// How compute is charged, when the match charges it.
#[unsafe(no_mangle)]
pub extern "C" fn cy_battery_compute_v1(robot: *mut Robot, out: *mut ComputeV1) -> i32 {
    guard(robot, "cy_battery_compute_v1", |r| {
        match crate::battery::compute(&*r)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// What holding a drive fraction costs each second, by hello.formulas.driveCost.
#[unsafe(no_mangle)]
pub extern "C" fn cy_battery_drive_cost(robot: *mut Robot, fraction: f64, out: *mut f64) -> i32 {
    guard(robot, "cy_battery_drive_cost", |r| {
        let value = crate::battery::drive_cost(&*r, finite(fraction, "fraction")?)?;
        write(out, value)
    })
}

/// The forces.lift that cancels gravity for a level robot; 0 without gravity.
#[unsafe(no_mangle)]
pub extern "C" fn cy_drive_hover_lift(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_drive_hover_lift", |r| {
        let value = crate::drive::hover_lift(&*r)?;
        write(out, value)
    })
}

/// Ask for direct controls this tick; the arena decides what they do in the ruleset. Field: drive.throttle.
#[unsafe(no_mangle)]
pub extern "C" fn cy_drive_throttle(robot: *mut Robot, value: f64) -> i32 {
    guard(robot, "cy_drive_throttle", |r| {
        act::set_number(r, Field::DriveThrottle, finite(value, "throttle")?)?;
        Ok(CY_OK)
    })
}

/// Ask for direct controls this tick; the arena decides what they do in the ruleset. Field: drive.turn.
#[unsafe(no_mangle)]
pub extern "C" fn cy_drive_turn(robot: *mut Robot, value: f64) -> i32 {
    guard(robot, "cy_drive_turn", |r| {
        act::set_number(r, Field::DriveTurn, finite(value, "turn")?)?;
        Ok(CY_OK)
    })
}

/// Ask for direct controls this tick; the arena decides what they do in the ruleset. Field: drive.pitchTurn.
#[unsafe(no_mangle)]
pub extern "C" fn cy_drive_pitch_turn(robot: *mut Robot, value: f64) -> i32 {
    guard(robot, "cy_drive_pitch_turn", |r| {
        act::set_number(r, Field::DrivePitchTurn, finite(value, "pitch_turn")?)?;
        Ok(CY_OK)
    })
}

/// Ask for forces and torques this tick, in the force rulesets. Field: forces.thrust.
#[unsafe(no_mangle)]
pub extern "C" fn cy_drive_thrust(robot: *mut Robot, value: f64) -> i32 {
    guard(robot, "cy_drive_thrust", |r| {
        act::set_number(r, Field::DriveThrust, finite(value, "thrust")?)?;
        Ok(CY_OK)
    })
}

/// Ask for forces and torques this tick, in the force rulesets. Field: forces.strafe.
#[unsafe(no_mangle)]
pub extern "C" fn cy_drive_strafe(robot: *mut Robot, value: f64) -> i32 {
    guard(robot, "cy_drive_strafe", |r| {
        act::set_number(r, Field::DriveStrafe, finite(value, "strafe")?)?;
        Ok(CY_OK)
    })
}

/// Ask for forces and torques this tick, in the force rulesets. Field: forces.lift.
#[unsafe(no_mangle)]
pub extern "C" fn cy_drive_lift(robot: *mut Robot, value: f64) -> i32 {
    guard(robot, "cy_drive_lift", |r| {
        act::set_number(r, Field::DriveLift, finite(value, "lift")?)?;
        Ok(CY_OK)
    })
}

/// Ask for forces and torques this tick, in the force rulesets. Field: forces.yawTorque.
#[unsafe(no_mangle)]
pub extern "C" fn cy_drive_yaw_torque(robot: *mut Robot, value: f64) -> i32 {
    guard(robot, "cy_drive_yaw_torque", |r| {
        act::set_number(r, Field::DriveYawTorque, finite(value, "yaw_torque")?)?;
        Ok(CY_OK)
    })
}

/// Ask for forces and torques this tick, in the force rulesets. Field: forces.pitchTorque.
#[unsafe(no_mangle)]
pub extern "C" fn cy_drive_pitch_torque(robot: *mut Robot, value: f64) -> i32 {
    guard(robot, "cy_drive_pitch_torque", |r| {
        act::set_number(r, Field::DrivePitchTorque, finite(value, "pitch_torque")?)?;
        Ok(CY_OK)
    })
}

/// Ask for forces and torques this tick, in the force rulesets. Field: forces.rollTorque.
#[unsafe(no_mangle)]
pub extern "C" fn cy_drive_roll_torque(robot: *mut Robot, value: f64) -> i32 {
    guard(robot, "cy_drive_roll_torque", |r| {
        act::set_number(r, Field::DriveRollTorque, finite(value, "roll_torque")?)?;
        Ok(CY_OK)
    })
}

/// Ask for forces and torques this tick, in the force rulesets. Field: forces.brake.
#[unsafe(no_mangle)]
pub extern "C" fn cy_drive_brake(robot: *mut Robot, value: f64) -> i32 {
    guard(robot, "cy_drive_brake", |r| {
        act::set_number(r, Field::DriveBrake, finite(value, "brake")?)?;
        Ok(CY_OK)
    })
}

/// What this tick's act holds for a drive field so far, as set. Field: drive.throttle.
#[unsafe(no_mangle)]
pub extern "C" fn cy_drive_get_throttle(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_drive_get_throttle", |r| {
        match act::get(r, Field::DriveThrottle)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// What this tick's act holds for a drive field so far, as set. Field: drive.turn.
#[unsafe(no_mangle)]
pub extern "C" fn cy_drive_get_turn(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_drive_get_turn", |r| {
        match act::get(r, Field::DriveTurn)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// What this tick's act holds for a drive field so far, as set. Field: drive.pitchTurn.
#[unsafe(no_mangle)]
pub extern "C" fn cy_drive_get_pitch_turn(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_drive_get_pitch_turn", |r| {
        match act::get(r, Field::DrivePitchTurn)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// What this tick's act holds for a drive field so far, as set. Field: forces.thrust.
#[unsafe(no_mangle)]
pub extern "C" fn cy_drive_get_thrust(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_drive_get_thrust", |r| {
        match act::get(r, Field::DriveThrust)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// What this tick's act holds for a drive field so far, as set. Field: forces.strafe.
#[unsafe(no_mangle)]
pub extern "C" fn cy_drive_get_strafe(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_drive_get_strafe", |r| {
        match act::get(r, Field::DriveStrafe)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// What this tick's act holds for a drive field so far, as set. Field: forces.lift.
#[unsafe(no_mangle)]
pub extern "C" fn cy_drive_get_lift(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_drive_get_lift", |r| {
        match act::get(r, Field::DriveLift)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// What this tick's act holds for a drive field so far, as set. Field: forces.yawTorque.
#[unsafe(no_mangle)]
pub extern "C" fn cy_drive_get_yaw_torque(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_drive_get_yaw_torque", |r| {
        match act::get(r, Field::DriveYawTorque)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// What this tick's act holds for a drive field so far, as set. Field: forces.pitchTorque.
#[unsafe(no_mangle)]
pub extern "C" fn cy_drive_get_pitch_torque(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_drive_get_pitch_torque", |r| {
        match act::get(r, Field::DrivePitchTorque)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// What this tick's act holds for a drive field so far, as set. Field: forces.rollTorque.
#[unsafe(no_mangle)]
pub extern "C" fn cy_drive_get_roll_torque(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_drive_get_roll_torque", |r| {
        match act::get(r, Field::DriveRollTorque)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// What this tick's act holds for a drive field so far, as set. Field: forces.brake.
#[unsafe(no_mangle)]
pub extern "C" fn cy_drive_get_brake(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_drive_get_brake", |r| {
        match act::get(r, Field::DriveBrake)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// Where the turret points now; a shot leaves along it.
#[unsafe(no_mangle)]
pub extern "C" fn cy_gun_aim_now(robot: *mut Robot, out: *mut Direction) -> i32 {
    guard(robot, "cy_gun_aim_now", |r| {
        let value = crate::gun::aim_now(&*r)?;
        write(out, value)
    })
}

/// Ticks until the gun can fire; 0 means it can fire this tick.
#[unsafe(no_mangle)]
pub extern "C" fn cy_gun_cooldown_ticks(robot: *mut Robot, out: *mut i32) -> i32 {
    guard(robot, "cy_gun_cooldown_ticks", |r| {
        let value = crate::gun::cooldown_ticks(&*r)?;
        write(out, value)
    })
}

/// The aim spread a shot would have at your speed now.
#[unsafe(no_mangle)]
pub extern "C" fn cy_gun_spread(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_gun_spread", |r| {
        let value = crate::gun::spread(&*r)?;
        write(out, value)
    })
}

/// The heaviest shot the battery pays for now; 0 when it cannot pay for the lightest.
#[unsafe(no_mangle)]
pub extern "C" fn cy_gun_max_power(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_gun_max_power", |r| {
        let value = crate::gun::max_power(&*r)?;
        write(out, value)
    })
}

/// The escalation multiplier on hit damage this tick.
#[unsafe(no_mangle)]
pub extern "C" fn cy_gun_damage_scale(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_gun_damage_scale", |r| {
        let value = crate::gun::damage_scale(&*r)?;
        write(out, value)
    })
}

/// The cooldown is 0 and the battery pays for a shot at this power and still holds the reserve.
#[unsafe(no_mangle)]
pub extern "C" fn cy_gun_can_fire(robot: *mut Robot, power: f64, reserve: f64, out: *mut i32) -> i32 {
    guard(robot, "cy_gun_can_fire", |r| {
        let value = crate::gun::can_fire(&*r, finite(power, "power")?, finite(reserve, "reserve")?)?;
        write(out, i32::from(value))
    })
}

/// How fast a shot at this power flies, units/s, by hello.formulas.shotSpeed.
#[unsafe(no_mangle)]
pub extern "C" fn cy_gun_shot_speed(robot: *mut Robot, power: f64, out: *mut f64) -> i32 {
    guard(robot, "cy_gun_shot_speed", |r| {
        let value = crate::gun::shot_speed(&*r, finite(power, "power")?)?;
        write(out, value)
    })
}

/// What a shot at this power costs the battery, by hello.formulas.shotCost.
#[unsafe(no_mangle)]
pub extern "C" fn cy_gun_shot_cost(robot: *mut Robot, power: f64, out: *mut f64) -> i32 {
    guard(robot, "cy_gun_shot_cost", |r| {
        let value = crate::gun::shot_cost(&*r, finite(power, "power")?)?;
        write(out, value)
    })
}

/// The reload after a shot at this power, ticks, by hello.formulas.reloadTicks.
#[unsafe(no_mangle)]
pub extern "C" fn cy_gun_reload_ticks(robot: *mut Robot, power: f64, out: *mut f64) -> i32 {
    guard(robot, "cy_gun_reload_ticks", |r| {
        let value = crate::gun::reload_ticks(&*r, finite(power, "power")?)?;
        write(out, value)
    })
}

/// The damage of a hit at this power before escalation, by hello.formulas.shotDamage.
#[unsafe(no_mangle)]
pub extern "C" fn cy_gun_shot_damage(robot: *mut Robot, power: f64, out: *mut f64) -> i32 {
    guard(robot, "cy_gun_shot_damage", |r| {
        let value = crate::gun::shot_damage(&*r, finite(power, "power")?)?;
        write(out, value)
    })
}

/// The power whose shot flies at this speed, within the legal powers, by hello.formulas.powerForSpeed.
#[unsafe(no_mangle)]
pub extern "C" fn cy_gun_power_for_speed(robot: *mut Robot, speed: f64, out: *mut f64) -> i32 {
    guard(robot, "cy_gun_power_for_speed", |r| {
        let value = crate::gun::power_for_speed(&*r, finite(speed, "speed")?)?;
        write(out, value)
    })
}

/// The power whose shot covers this range in this time, within the legal powers.
#[unsafe(no_mangle)]
pub extern "C" fn cy_gun_power_for_flight_time(robot: *mut Robot, range: f64, seconds: f64, out: *mut f64) -> i32 {
    guard(robot, "cy_gun_power_for_flight_time", |r| {
        let value = crate::gun::power_for_flight_time(&*r, finite(range, "range")?, finite(seconds, "seconds")?)?;
        write(out, value)
    })
}

/// The aim at which a shot fired now meets the contact if it holds its tracked velocity; absent when no shot reaches it.
#[unsafe(no_mangle)]
pub extern "C" fn cy_gun_lead_v1(robot: *mut Robot, contact: *const ContactV1, power: f64, out: *mut SolutionV1) -> i32 {
    guard(robot, "cy_gun_lead_v1", |r| {
        match crate::gun::lead(&*r, record_in(contact, "contact")?, finite(power, "power")?)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// The aim at which a shot fired now meets a point moving at this velocity; absent when no shot reaches it.
#[unsafe(no_mangle)]
pub extern "C" fn cy_gun_lead_point_v1(robot: *mut Robot, position: *const Vec3, velocity: *const Vec3, power: f64, out: *mut SolutionV1) -> i32 {
    guard(robot, "cy_gun_lead_point_v1", |r| {
        match crate::gun::lead_point(&*r, record_in(position, "position")?, record_in(velocity, "velocity")?, finite(power, "power")?)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// Ask the turret to turn toward a direction; it slews at gun.turnRate. Field: aim.heading.
#[unsafe(no_mangle)]
pub extern "C" fn cy_gun_aim_heading(robot: *mut Robot, value: f64) -> i32 {
    guard(robot, "cy_gun_aim_heading", |r| {
        act::set_number(r, Field::GunAimHeading, finite(value, "heading")?)?;
        Ok(CY_OK)
    })
}

/// Ask the turret to turn toward a direction; it slews at gun.turnRate. Field: aim.pitch.
#[unsafe(no_mangle)]
pub extern "C" fn cy_gun_aim_pitch(robot: *mut Robot, value: f64) -> i32 {
    guard(robot, "cy_gun_aim_pitch", |r| {
        act::set_number(r, Field::GunAimPitch, finite(value, "pitch")?)?;
        Ok(CY_OK)
    })
}

/// Ask to fire this tick; the arena decides whether the gun and battery allow it. Field: fire.
#[unsafe(no_mangle)]
pub extern "C" fn cy_gun_fire(robot: *mut Robot) -> i32 {
    guard(robot, "cy_gun_fire", |r| {
        act::set_flag(r, Field::GunFire)?;
        Ok(CY_OK)
    })
}

/// Ask to fire this tick; the arena decides whether the gun and battery allow it. Field: fire.power.
#[unsafe(no_mangle)]
pub extern "C" fn cy_gun_power(robot: *mut Robot, value: f64) -> i32 {
    guard(robot, "cy_gun_power", |r| {
        act::set_number(r, Field::GunPower, finite(value, "power")?)?;
        Ok(CY_OK)
    })
}

/// What this tick's act holds for a gun field so far, as set. Field: aim.heading.
#[unsafe(no_mangle)]
pub extern "C" fn cy_gun_get_aim_heading(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_gun_get_aim_heading", |r| {
        match act::get(r, Field::GunAimHeading)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// What this tick's act holds for a gun field so far, as set. Field: aim.pitch.
#[unsafe(no_mangle)]
pub extern "C" fn cy_gun_get_aim_pitch(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_gun_get_aim_pitch", |r| {
        match act::get(r, Field::GunAimPitch)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// What this tick's act holds for a gun field so far, as set. Field: fire.
#[unsafe(no_mangle)]
pub extern "C" fn cy_gun_get_fire(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_gun_get_fire", |r| {
        match act::get(r, Field::GunFire)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// What this tick's act holds for a gun field so far, as set. Field: fire.power.
#[unsafe(no_mangle)]
pub extern "C" fn cy_gun_get_power(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_gun_get_power", |r| {
        match act::get(r, Field::GunPower)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// The scan that produced this tick's contacts.
#[unsafe(no_mangle)]
pub extern "C" fn cy_scanner_last_v1(robot: *mut Robot, out: *mut ScanV1) -> i32 {
    guard(robot, "cy_scanner_last_v1", |r| {
        let value = crate::scanner::last(&*r)?;
        write(out, value)
    })
}

/// Robots this close are detected in any direction.
#[unsafe(no_mangle)]
pub extern "C" fn cy_scanner_proximity_range(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_scanner_proximity_range", |r| {
        let value = crate::scanner::proximity_range(&*r)?;
        write(out, value)
    })
}

/// How far a scan of this arc reaches, by hello.formulas.reach.
#[unsafe(no_mangle)]
pub extern "C" fn cy_scanner_reach(robot: *mut Robot, arc: f64, out: *mut f64) -> i32 {
    guard(robot, "cy_scanner_reach", |r| {
        let value = crate::scanner::reach(&*r, finite(arc, "arc")?)?;
        write(out, value)
    })
}

/// The widest arc that still reaches this far, by hello.formulas.arcForReach.
#[unsafe(no_mangle)]
pub extern "C" fn cy_scanner_arc_for_reach(robot: *mut Robot, range: f64, out: *mut f64) -> i32 {
    guard(robot, "cy_scanner_arc_for_reach", |r| {
        let value = crate::scanner::arc_for_reach(&*r, finite(range, "range")?)?;
        write(out, value)
    })
}

/// Ask where the scanner points for the next state. Field: scan.heading.
#[unsafe(no_mangle)]
pub extern "C" fn cy_scanner_heading(robot: *mut Robot, value: f64) -> i32 {
    guard(robot, "cy_scanner_heading", |r| {
        act::set_number(r, Field::ScannerHeading, finite(value, "heading")?)?;
        Ok(CY_OK)
    })
}

/// Ask where the scanner points for the next state. Field: scan.arc.
#[unsafe(no_mangle)]
pub extern "C" fn cy_scanner_arc(robot: *mut Robot, value: f64) -> i32 {
    guard(robot, "cy_scanner_arc", |r| {
        act::set_number(r, Field::ScannerArc, finite(value, "arc")?)?;
        Ok(CY_OK)
    })
}

/// Ask where the scanner points for the next state. Field: scan.pitch.
#[unsafe(no_mangle)]
pub extern "C" fn cy_scanner_pitch(robot: *mut Robot, value: f64) -> i32 {
    guard(robot, "cy_scanner_pitch", |r| {
        act::set_number(r, Field::ScannerPitch, finite(value, "pitch")?)?;
        Ok(CY_OK)
    })
}

/// What this tick's act holds for a scanner field so far, as set. Field: scan.heading.
#[unsafe(no_mangle)]
pub extern "C" fn cy_scanner_get_heading(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_scanner_get_heading", |r| {
        match act::get(r, Field::ScannerHeading)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// What this tick's act holds for a scanner field so far, as set. Field: scan.arc.
#[unsafe(no_mangle)]
pub extern "C" fn cy_scanner_get_arc(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_scanner_get_arc", |r| {
        match act::get(r, Field::ScannerArc)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// What this tick's act holds for a scanner field so far, as set. Field: scan.pitch.
#[unsafe(no_mangle)]
pub extern "C" fn cy_scanner_get_pitch(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_scanner_get_pitch", |r| {
        match act::get(r, Field::ScannerPitch)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// Every contact this tick: scan and proximity first, nearest first, then passive ones.
#[unsafe(no_mangle)]
pub extern "C" fn cy_sensors_contacts_v1(robot: *mut Robot, out: *mut ContactV1, capacity: u32, count: *mut u32) -> i32 {
    guard(robot, "cy_sensors_contacts_v1", |r| {
        let items = crate::sensors::contacts(&*r)?;
        write_list(out, capacity, count, &items)
    })
}

/// The nearest precise contact this tick. Passive contacts are never returned.
#[unsafe(no_mangle)]
pub extern "C" fn cy_sensors_nearest_v1(robot: *mut Robot, out: *mut ContactV1) -> i32 {
    guard(robot, "cy_sensors_nearest_v1", |r| {
        match crate::sensors::nearest(&*r)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// This tick's passive contacts, most accurate (smallest bearing error) first.
#[unsafe(no_mangle)]
pub extern "C" fn cy_sensors_cues_v1(robot: *mut Robot, out: *mut ContactV1, capacity: u32, count: *mut u32) -> i32 {
    guard(robot, "cy_sensors_cues_v1", |r| {
        let items = crate::sensors::cues(&*r)?;
        write_list(out, capacity, count, &items)
    })
}

/// The precise contact for a robot this tick, if it was detected.
#[unsafe(no_mangle)]
pub extern "C" fn cy_sensors_contact_v1(robot: *mut Robot, slot: i32, out: *mut ContactV1) -> i32 {
    guard(robot, "cy_sensors_contact_v1", |r| {
        match crate::sensors::contact(&*r, slot)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// Where a contact places its robot: your position plus its range along its bearing and elevation.
#[unsafe(no_mangle)]
pub extern "C" fn cy_sensors_position_of_v1(robot: *mut Robot, contact: *const ContactV1, out: *mut Vec3) -> i32 {
    guard(robot, "cy_sensors_position_of_v1", |r| {
        let value = crate::sensors::position_of(&*r, record_in(contact, "contact")?)?;
        write(out, value)
    })
}

/// The most accurate passive contact heard in the last cue_ticks (60 by default), or absent.
#[unsafe(no_mangle)]
pub extern "C" fn cy_tracker_cue_v1(robot: *mut Robot, out: *mut CueV1) -> i32 {
    guard(robot, "cy_tracker_cue_v1", |r| {
        match crate::tracker::cue(&*r)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// How fast the nearest contact's bearing turns, from two sightings.
#[unsafe(no_mangle)]
pub extern "C" fn cy_tracker_nearest_bearing_rate(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_tracker_nearest_bearing_rate", |r| {
        let value = crate::tracker::nearest_bearing_rate(&*r)?;
        write(out, value)
    })
}

/// The robot's tracked velocity; zero when it has no track.
#[unsafe(no_mangle)]
pub extern "C" fn cy_tracker_velocity(robot: *mut Robot, slot: i32, out: *mut Vec3) -> i32 {
    guard(robot, "cy_tracker_velocity", |r| {
        let value = crate::tracker::velocity(&*r, slot)?;
        write(out, value)
    })
}

/// Where the robot was last seen, if it was.
#[unsafe(no_mangle)]
pub extern "C" fn cy_tracker_position(robot: *mut Robot, slot: i32, out: *mut Vec3) -> i32 {
    guard(robot, "cy_tracker_position", |r| {
        match crate::tracker::position(&*r, slot)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// The tick the robot was last seen, if it was.
#[unsafe(no_mangle)]
pub extern "C" fn cy_tracker_last_seen(robot: *mut Robot, slot: i32, out: *mut i32) -> i32 {
    guard(robot, "cy_tracker_last_seen", |r| {
        match crate::tracker::last_seen(&*r, slot)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// Sets the Tracker's parameters, in the robot's start only; after the first state it refuses with E_PHASE.
#[unsafe(no_mangle)]
pub extern "C" fn cy_tracker_configure(robot: *mut Robot, smoothing: f64, forget_ticks: i32, cue_ticks: i32) -> i32 {
    guard(robot, "cy_tracker_configure", |r| {
        crate::tracker::configure(r, finite(smoothing, "smoothing")?, forget_ticks, cue_ticks)?;
        Ok(CY_OK)
    })
}

/// The sudden-death safe zone, once sudden death starts.
#[unsafe(no_mangle)]
pub extern "C" fn cy_nav_zone_v1(robot: *mut Robot, out: *mut ZoneV1) -> i32 {
    guard(robot, "cy_nav_zone_v1", |r| {
        match crate::nav::zone(&*r)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// How far inside the zone a point is, by its shape's distance; absent while there is none.
#[unsafe(no_mangle)]
pub extern "C" fn cy_nav_zone_margin(robot: *mut Robot, point: *const Vec3, out: *mut f64) -> i32 {
    guard(robot, "cy_nav_zone_margin", |r| {
        match crate::nav::zone_margin(&*r, record_in(point, "point")?)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// The point is inside the zone by at least the margin; absent while there is none.
#[unsafe(no_mangle)]
pub extern "C" fn cy_nav_zone_contains(robot: *mut Robot, point: *const Vec3, margin: f64, out: *mut i32) -> i32 {
    guard(robot, "cy_nav_zone_contains", |r| {
        match crate::nav::zone_contains(&*r, record_in(point, "point")?, finite(margin, "margin")?)? {
            Some(value) => write(out, i32::from(value)),
            None => Ok(CY_NONE),
        }
    })
}

/// How far a point is from the zone's edge, 0 inside; absent while there is none.
#[unsafe(no_mangle)]
pub extern "C" fn cy_nav_zone_distance_from(robot: *mut Robot, point: *const Vec3, out: *mut f64) -> i32 {
    guard(robot, "cy_nav_zone_distance_from", |r| {
        match crate::nav::zone_distance_from(&*r, record_in(point, "point")?)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// The storm, once it enters.
#[unsafe(no_mangle)]
pub extern "C" fn cy_nav_storm_v1(robot: *mut Robot, out: *mut StormV1) -> i32 {
    guard(robot, "cy_nav_storm_v1", |r| {
        match crate::nav::storm(&*r)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// How far inside the storm a point is, by its shape's distance; absent while there is none.
#[unsafe(no_mangle)]
pub extern "C" fn cy_nav_storm_margin(robot: *mut Robot, point: *const Vec3, out: *mut f64) -> i32 {
    guard(robot, "cy_nav_storm_margin", |r| {
        match crate::nav::storm_margin(&*r, record_in(point, "point")?)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// The point is inside the storm by at least the margin; absent while there is none.
#[unsafe(no_mangle)]
pub extern "C" fn cy_nav_storm_contains(robot: *mut Robot, point: *const Vec3, margin: f64, out: *mut i32) -> i32 {
    guard(robot, "cy_nav_storm_contains", |r| {
        match crate::nav::storm_contains(&*r, record_in(point, "point")?, finite(margin, "margin")?)? {
            Some(value) => write(out, i32::from(value)),
            None => Ok(CY_NONE),
        }
    })
}

/// How far a point is from the storm's edge, 0 inside; absent while there is none.
#[unsafe(no_mangle)]
pub extern "C" fn cy_nav_storm_distance_from(robot: *mut Robot, point: *const Vec3, out: *mut f64) -> i32 {
    guard(robot, "cy_nav_storm_distance_from", |r| {
        match crate::nav::storm_distance_from(&*r, record_in(point, "point")?)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// The hill, once it appears.
#[unsafe(no_mangle)]
pub extern "C" fn cy_nav_hill_v1(robot: *mut Robot, out: *mut HillV1) -> i32 {
    guard(robot, "cy_nav_hill_v1", |r| {
        match crate::nav::hill(&*r)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// How far inside the hill a point is, by its shape's distance; absent while there is none.
#[unsafe(no_mangle)]
pub extern "C" fn cy_nav_hill_margin(robot: *mut Robot, point: *const Vec3, out: *mut f64) -> i32 {
    guard(robot, "cy_nav_hill_margin", |r| {
        match crate::nav::hill_margin(&*r, record_in(point, "point")?)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// The point is inside the hill by at least the margin; absent while there is none.
#[unsafe(no_mangle)]
pub extern "C" fn cy_nav_hill_contains(robot: *mut Robot, point: *const Vec3, margin: f64, out: *mut i32) -> i32 {
    guard(robot, "cy_nav_hill_contains", |r| {
        match crate::nav::hill_contains(&*r, record_in(point, "point")?, finite(margin, "margin")?)? {
            Some(value) => write(out, i32::from(value)),
            None => Ok(CY_NONE),
        }
    })
}

/// How far a point is from the hill's edge, 0 inside; absent while there is none.
#[unsafe(no_mangle)]
pub extern "C" fn cy_nav_hill_distance_from(robot: *mut Robot, point: *const Vec3, out: *mut f64) -> i32 {
    guard(robot, "cy_nav_hill_distance_from", |r| {
        match crate::nav::hill_distance_from(&*r, record_in(point, "point")?)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// Every pickup on the field.
#[unsafe(no_mangle)]
pub extern "C" fn cy_nav_pickups_v1(robot: *mut Robot, out: *mut PickupV1, capacity: u32, count: *mut u32) -> i32 {
    guard(robot, "cy_nav_pickups_v1", |r| {
        let items = crate::nav::pickups(&*r)?;
        write_list(out, capacity, count, &items)
    })
}

/// The pickup of this kind nearest you by field distance, if there is one.
#[unsafe(no_mangle)]
pub extern "C" fn cy_nav_nearest_pickup_v1(robot: *mut Robot, kind: i32, out: *mut PickupV1) -> i32 {
    guard(robot, "cy_nav_nearest_pickup_v1", |r| {
        match crate::nav::nearest_pickup(&*r, enum_or_any(kind, PickupKind::from_i32, "kind")?)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// The distance between two points as the field measures it: in the plane under gravity, in space otherwise.
#[unsafe(no_mangle)]
pub extern "C" fn cy_nav_field_distance(robot: *mut Robot, a: *const Vec3, b: *const Vec3, out: *mut f64) -> i32 {
    guard(robot, "cy_nav_field_distance", |r| {
        let value = crate::nav::field_distance(&*r, record_in(a, "a")?, record_in(b, "b")?)?;
        write(out, value)
    })
}

/// Your score; absent when the match does not score.
#[unsafe(no_mangle)]
pub extern "C" fn cy_score_me(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_score_me", |r| {
        match crate::score::me(&*r)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// The best other robot's score; absent when the match does not score.
#[unsafe(no_mangle)]
pub extern "C" fn cy_score_best(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_score_best", |r| {
        match crate::score::best(&*r)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// Your armour; absent when the match has no objectives.
#[unsafe(no_mangle)]
pub extern "C" fn cy_score_armour(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_score_armour", |r| {
        match crate::score::armour(&*r)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// An angle brought into 0 to 360, exactly: degrees % 360, plus 360 when that is negative.
#[unsafe(no_mangle)]
pub extern "C" fn cy_angles_normalize(degrees: f64, out: *mut f64) -> i32 {
    guard_static(|| {
        let value = crate::angles::normalize(finite(degrees, "degrees")?)?;
        write(out, value)
    })
}

/// The signed turn from current to target: normalize(target - current + 540) - 180.
#[unsafe(no_mangle)]
pub extern "C" fn cy_angles_angle_diff(target: f64, current: f64, out: *mut f64) -> i32 {
    guard_static(|| {
        let value = crate::angles::angle_diff(finite(target, "target")?, finite(current, "current")?)?;
        write(out, value)
    })
}

/// The heading from one point to another.
#[unsafe(no_mangle)]
pub extern "C" fn cy_angles_bearing_to(from: *const Vec3, to: *const Vec3, out: *mut f64) -> i32 {
    guard_static(|| {
        let value = crate::angles::bearing_to(record_in(from, "from")?, record_in(to, "to")?)?;
        write(out, value)
    })
}

/// The elevation from one point to another.
#[unsafe(no_mangle)]
pub extern "C" fn cy_angles_elevation_to(from: *const Vec3, to: *const Vec3, out: *mut f64) -> i32 {
    guard_static(|| {
        let value = crate::angles::elevation_to(record_in(from, "from")?, record_in(to, "to")?)?;
        write(out, value)
    })
}

/// The unit vector along a heading and pitch.
#[unsafe(no_mangle)]
pub extern "C" fn cy_angles_direction(heading: f64, pitch: f64, out: *mut Vec3) -> i32 {
    guard_static(|| {
        let value = crate::angles::direction(finite(heading, "heading")?, finite(pitch, "pitch")?)?;
        write(out, value)
    })
}

/// The straight-line distance between two points.
#[unsafe(no_mangle)]
pub extern "C" fn cy_angles_distance(a: *const Vec3, b: *const Vec3, out: *mut f64) -> i32 {
    guard_static(|| {
        let value = crate::angles::distance(record_in(a, "a")?, record_in(b, "b")?)?;
        write(out, value)
    })
}

/// A value held within low and high.
#[unsafe(no_mangle)]
pub extern "C" fn cy_angles_clamp(value: f64, low: f64, high: f64, out: *mut f64) -> i32 {
    guard_static(|| {
        let value = crate::angles::clamp(finite(value, "value")?, finite(low, "low")?, finite(high, "high")?)?;
        write(out, value)
    })
}

/// Set the status line; leaving it out keeps the last one. Field: status.text.
#[unsafe(no_mangle)]
pub extern "C" fn cy_status_text(robot: *mut Robot, text: *const u8, text_length: u32) -> i32 {
    guard(robot, "cy_status_text", |r| {
        act::set_text(r, Field::StatusText, text_in(text, text_length, "text")?)?;
        Ok(CY_OK)
    })
}

/// Set the status line; leaving it out keeps the last one. Field: status.value.
#[unsafe(no_mangle)]
pub extern "C" fn cy_status_value(robot: *mut Robot, value: f64) -> i32 {
    guard(robot, "cy_status_value", |r| {
        act::set_number(r, Field::StatusValue, finite(value, "value")?)?;
        Ok(CY_OK)
    })
}

/// What this tick's act holds for the status line so far, as set. Field: status.text.
#[unsafe(no_mangle)]
pub extern "C" fn cy_status_get_text(robot: *mut Robot, buf: *mut u8, capacity: u32, length: *mut u32) -> i32 {
    guard(robot, "cy_status_get_text", |r| {
        match act::get_text(r, Field::StatusText)? {
            Some(text) => write_text(buf, capacity, length, &text),
            None => Ok(CY_NONE),
        }
    })
}

/// What this tick's act holds for the status line so far, as set. Field: status.value.
#[unsafe(no_mangle)]
pub extern "C" fn cy_status_get_value(robot: *mut Robot, out: *mut f64) -> i32 {
    guard(robot, "cy_status_get_value", |r| {
        match act::get(r, Field::StatusValue)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// The number at a path, if there is one.
#[unsafe(no_mangle)]
pub extern "C" fn cy_raw_number(robot: *mut Robot, path: *const u8, path_length: u32, out: *mut f64) -> i32 {
    guard(robot, "cy_raw_number", |r| {
        match crate::raw::number(&*r, text_in(path, path_length, "path")?)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// The string at a path, if there is one.
#[unsafe(no_mangle)]
pub extern "C" fn cy_raw_text(robot: *mut Robot, path: *const u8, path_length: u32, buf: *mut u8, capacity: u32, length: *mut u32) -> i32 {
    guard(robot, "cy_raw_text", |r| {
        match crate::raw::text(&*r, text_in(path, path_length, "path")?)? {
            Some(text) => write_text(buf, capacity, length, &text),
            None => Ok(CY_NONE),
        }
    })
}

/// Something is at the path.
#[unsafe(no_mangle)]
pub extern "C" fn cy_raw_has(robot: *mut Robot, path: *const u8, path_length: u32, out: *mut i32) -> i32 {
    guard(robot, "cy_raw_has", |r| {
        let value = crate::raw::has(&*r, text_in(path, path_length, "path")?)?;
        write(out, i32::from(value))
    })
}

/// The boolean at a path, if there is one.
#[unsafe(no_mangle)]
pub extern "C" fn cy_raw_flag(robot: *mut Robot, path: *const u8, path_length: u32, out: *mut i32) -> i32 {
    guard(robot, "cy_raw_flag", |r| {
        match crate::raw::flag(&*r, text_in(path, path_length, "path")?)? {
            Some(value) => write(out, i32::from(value)),
            None => Ok(CY_NONE),
        }
    })
}

/// The length of the array at a path, if there is one.
#[unsafe(no_mangle)]
pub extern "C" fn cy_raw_count(robot: *mut Robot, path: *const u8, path_length: u32, out: *mut i32) -> i32 {
    guard(robot, "cy_raw_count", |r| {
        match crate::raw::count(&*r, text_in(path, path_length, "path")?)? {
            Some(value) => write(out, value),
            None => Ok(CY_NONE),
        }
    })
}

/// The last line the session read: the hello, or the latest state.
#[unsafe(no_mangle)]
pub extern "C" fn cy_raw_line(robot: *mut Robot, buf: *mut u8, capacity: u32, length: *mut u32) -> i32 {
    guard(robot, "cy_raw_line", |r| {
        let text = crate::raw::line(&*r)?;
        write_text(buf, capacity, length, &text)
    })
}

/// Adds fields the API does not cover to this tick's act; a key the API writes (v, type, tick, drive, forces, aim, scan, fire, status) is refused with E_ARGUMENT.
#[unsafe(no_mangle)]
pub extern "C" fn cy_raw_extra(robot: *mut Robot, json: *const u8, json_length: u32) -> i32 {
    guard(robot, "cy_raw_extra", |r| {
        crate::raw::extra(r, text_in(json, json_length, "json")?)?;
        Ok(CY_OK)
    })
}

/// What the last next() read.
#[unsafe(no_mangle)]
pub extern "C" fn cy_session_event(robot: *mut Robot, out: *mut i32) -> i32 {
    guard(robot, "cy_session_event", |r| {
        let value = crate::session::event(&*r)?;
        write(out, value as i32)
    })
}

/// Reads the next message and decodes it.
#[unsafe(no_mangle)]
pub extern "C" fn cy_session_next(robot: *mut Robot, out: *mut i32) -> i32 {
    guard(robot, "cy_session_next", |r| {
        let value = crate::session::next(r)?;
        write(out, value as i32)
    })
}

/// Writes the ready reply after a hello, or this tick's act after a state, and clears the pending act.
#[unsafe(no_mangle)]
pub extern "C" fn cy_session_send(robot: *mut Robot) -> i32 {
    guard(robot, "cy_session_send", |r| {
        crate::session::send(r)?;
        Ok(CY_OK)
    })
}

/// Writes a forfeit in place of this tick's act.
#[unsafe(no_mangle)]
pub extern "C" fn cy_session_forfeit(robot: *mut Robot, code: *const u8, code_length: u32, message: *const u8, message_length: u32) -> i32 {
    guard(robot, "cy_session_forfeit", |r| {
        crate::session::forfeit(r, text_in(code, code_length, "code")?, text_in(message, message_length, "message")?)?;
        Ok(CY_OK)
    })
}

/// Writes a line to standard error, never to the protocol.
#[unsafe(no_mangle)]
pub extern "C" fn cy_session_log(robot: *mut Robot, line: *const u8, line_length: u32) -> i32 {
    guard(robot, "cy_session_log", |r| {
        crate::session::log(r, text_in(line, line_length, "line")?)?;
        Ok(CY_OK)
    })
}

/// Hands the memory port a line, for tests and event-loop runtimes.
#[unsafe(no_mangle)]
pub extern "C" fn cy_session_feed(robot: *mut Robot, line: *const u8, line_length: u32) -> i32 {
    guard(robot, "cy_session_feed", |r| {
        crate::session::feed(r, text_in(line, line_length, "line")?)?;
        Ok(CY_OK)
    })
}

/// Takes the memory port's next written line, if any.
#[unsafe(no_mangle)]
pub extern "C" fn cy_session_reply(robot: *mut Robot, buf: *mut u8, capacity: u32, length: *mut u32) -> i32 {
    guard(robot, "cy_session_reply", |r| {
        match crate::session::reply(&*r)? {
            Some(text) => {
                let status = write_text(buf, capacity, length, &text)?;
                if status == CY_OK {
                    crate::session::take_reply(r);
                }
                Ok(status)
            }
            None => Ok(CY_NONE),
        }
    })
}

/// Fills the DSL words record (section 4.2 of the design). Outside the robot API.
#[unsafe(no_mangle)]
pub extern "C" fn cy_dsl_fill_v1(robot: *mut Robot, out: *mut DslWordsV1) -> i32 {
    guard(robot, "cy_dsl_fill_v1", |r| {
        let words = super::words::fill(r)?;
        write(out, words)
    })
}
