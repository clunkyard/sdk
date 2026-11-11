//! The match, fixed from the hello.

use crate::generated::*;
use crate::runtime::Robot;

pub fn mode(r: &Robot) -> Result<Mode, Error> {
    Ok(r.hello()?.mode)
}

pub fn ruleset(r: &Robot) -> Result<Ruleset, Error> {
    Ok(r.hello()?.ruleset)
}

pub fn control(r: &Robot) -> Result<Control, Error> {
    Ok(r.hello()?.control)
}

pub fn robots(r: &Robot) -> Result<i32, Error> {
    Ok(r.hello()?.robots as i32)
}

pub fn is_3d(r: &Robot) -> Result<bool, Error> {
    Ok(r.hello()?.mode == Mode::ThreeD)
}

pub fn tick_rate(r: &Robot) -> Result<i32, Error> {
    Ok(r.hello()?.tick_rate as i32)
}

pub fn max_ticks(r: &Robot) -> Result<i32, Error> {
    Ok(r.hello()?.max_ticks as i32)
}

/// The id in a slot; empty until that robot is first seen.
pub fn robot_id(r: &Robot, slot: i32) -> Result<String, Error> {
    Ok(r.slot_id(slot)?.unwrap_or("").to_string())
}
