//! The drone: whether it can launch, whether it flies, its launches left,
//! the newest report and its sightings. With the match's drones off the
//! state has no drone block, and every query answers as for a robot with no
//! program: false, 0, no report and no sightings.

use crate::generated::*;
use crate::message::DroneState;
use crate::runtime::Robot;

fn state(r: &Robot) -> Result<Option<&DroneState>, Error> {
    Ok(r.current()?.drone.as_ref())
}

pub fn has(r: &Robot) -> Result<bool, Error> {
    Ok(state(r)?.is_some_and(|d| d.has))
}

pub fn ready(r: &Robot) -> Result<bool, Error> {
    Ok(state(r)?.is_some_and(|d| d.ready))
}

pub fn in_flight(r: &Robot) -> Result<bool, Error> {
    Ok(state(r)?.is_some_and(|d| d.in_flight))
}

pub fn launches_left(r: &Robot) -> Result<i32, Error> {
    Ok(state(r)?.map_or(0, |d| d.launches_left as i32))
}

/// The newest report, aged from this tick.
pub fn report(r: &Robot) -> Result<Option<ReportV1>, Error> {
    let tick_rate = r.hello()?.tick_rate;
    let tick = r.current()?.tick;
    Ok(state(r)?.and_then(|d| d.report).map(|report| ReportV1 { age: (tick - f64::from(report.tick)) / tick_rate, ..report }))
}

pub fn sightings(r: &Robot) -> Result<Vec<DroneSightingV1>, Error> {
    Ok(state(r)?.map_or_else(Vec::new, |d| d.sightings.clone()))
}

/// One number of the launch brief, 1 to BRIEF.
pub fn brief(r: &mut Robot, index: i32, value: f64) -> Result<(), Error> {
    if !(1..=crate::act::BRIEF as i32).contains(&index) {
        return Err(Error::range(format!("brief {index} is not 1 to {}", crate::act::BRIEF)));
    }
    r.act.brief(index as usize - 1, value);
    Ok(())
}

/// The drone program the ready reply carries, in the start only.
pub fn program(r: &mut Robot, language: &str, source: &str) -> Result<(), Error> {
    if r.state.is_some() {
        return Err(Error::phase("hand over a drone program before the first state"));
    }
    if language != "vane" && language != "cadence" {
        return Err(Error::argument(format!("{language} is not a drone language; use vane or cadence")));
    }
    if source.len() > PROGRAM_BYTES {
        return Err(Error::range(format!("the drone program is {} bytes, over {PROGRAM_BYTES}", source.len())));
    }
    r.drone_program = Some((language.to_string(), source.to_string()));
    Ok(())
}

/// The longest drone program a ready reply carries, in bytes.
pub const PROGRAM_BYTES: usize = 4096;
