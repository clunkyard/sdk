//! The DSL words derived from core facts (design section 4.2), for the
//! cy_dsl_words_v1 record that generated/words.rs fills. Outside the robot
//! API: no binding sees them.
//!
//! A mechanic that is off or not yet in play reads as inactive, with
//! bearings your heading and distances FAR.

use crate::generated::*;
use crate::maths::{angle_diff, degrees_of, max, normalize, sqrt};
use crate::message::State;
use crate::runtime::Robot;

/// Distance, bearing and elevation from you to a target, counting height
/// only in a sphere-shaped field.
struct Seen {
    distance: f64,
    bearing: f64,
    elevation: f64,
}

fn sphere(r: &Robot) -> Result<bool, Error> {
    Ok(r.hello()?.shape == Shape::Sphere)
}

fn towards(state: &State, sphere: bool, target: Vec3) -> Seen {
    let at = state.me.position;
    let (dx, dy, dz) = (target.x - at.x, target.y - at.y, if sphere { target.z - at.z } else { 0.0 });
    let planar = sqrt(dx * dx + dy * dy);
    Seen { distance: sqrt(planar * planar + dz * dz), bearing: normalize(degrees_of(dy, dx)), elevation: if sphere { degrees_of(dz, planar) } else { 0.0 } }
}

pub fn speed(r: &Robot) -> Result<f64, Error> {
    let v = r.current()?.me.velocity;
    Ok(sqrt(v.x * v.x + v.y * v.y + v.z * v.z))
}

pub fn gravity(r: &Robot) -> Result<bool, Error> {
    Ok(r.hello()?.gravity.z.abs() > 0.0)
}

pub fn battery_fraction(r: &Robot) -> Result<f64, Error> {
    let capacity = r.hello()?.capacity;
    let energy = r.current()?.me.battery;
    Ok(if capacity > 0.0 { energy / capacity } else { 0.0 })
}

pub fn aim_error(r: &Robot) -> Result<f64, Error> {
    let state = r.current()?;
    Ok(crate::sensors::nearest_seen(state).map_or(0.0, |nearest| angle_diff(nearest.bearing, state.me.aim.heading)))
}

// The zone as the DSL sees it: height counts in a sphere-shaped field.
fn zone(r: &Robot) -> Result<Option<Seen>, Error> {
    let sphere = sphere(r)?;
    let state = r.current()?;
    Ok(state.zone.map(|zone| {
        let at = state.me.position;
        let (dx, dy, dz) = (zone.center.x - at.x, zone.center.y - at.y, if sphere { zone.center.z - at.z } else { 0.0 });
        let planar = sqrt(dx * dx + dy * dy);
        Seen { distance: sqrt(planar * planar + dz * dz), bearing: normalize(degrees_of(dy, dx)), elevation: degrees_of(dz, planar) }
    }))
}

pub fn zone_distance(r: &Robot) -> Result<f64, Error> {
    Ok(zone(r)?.map_or(0.0, |zone| zone.distance))
}

pub fn zone_bearing(r: &Robot) -> Result<f64, Error> {
    let heading = r.current()?.me.heading;
    Ok(zone(r)?.map_or(heading, |zone| zone.bearing))
}

pub fn zone_elevation(r: &Robot) -> Result<f64, Error> {
    Ok(zone(r)?.map_or(0.0, |zone| zone.elevation))
}

pub fn storm_distance(r: &Robot) -> Result<f64, Error> {
    let state = r.current()?;
    let at = state.me.position;
    Ok(state.storm.map_or(FAR, |storm| {
        let (dx, dy) = (storm.center.x - at.x, storm.center.y - at.y);
        sqrt(dx * dx + dy * dy) - storm.radius
    }))
}

pub fn storm_bearing(r: &Robot) -> Result<f64, Error> {
    let state = r.current()?;
    let at = state.me.position;
    Ok(state.storm.map_or(state.me.heading, |storm| normalize(degrees_of(storm.center.y - at.y, storm.center.x - at.x))))
}

pub fn storm_heading(r: &Robot) -> Result<f64, Error> {
    Ok(r.current()?.storm.map_or(0.0, |storm| {
        let (vx, vy) = (storm.velocity.x, storm.velocity.y);
        if vx == 0.0 && vy == 0.0 { 0.0 } else { normalize(degrees_of(vy, vx)) }
    }))
}

pub fn cue_muzzle(r: &Robot) -> Result<bool, Error> {
    r.current()?;
    Ok(r.tracker.cue().is_some_and(|cue| cue.source == Source::Muzzle as i32))
}

/// The cue as the DSL sees it: as heard on the tick it was heard, and after
/// that its remembered place seen from where you are now, in three axes.
fn cue(r: &Robot) -> Result<Option<(Seen, f64)>, Error> {
    let state = r.current()?;
    let Some(cue) = r.tracker.cue() else { return Ok(None) };
    let age = state.tick - cue.tick;
    if age == 0.0 {
        return Ok(Some((Seen { distance: cue.range, bearing: cue.bearing, elevation: cue.elevation }, age)));
    }
    let at = state.me.position;
    let (dx, dy, dz) = (cue.position.x - at.x, cue.position.y - at.y, cue.position.z - at.z);
    let planar = sqrt(dx * dx + dy * dy);
    Ok(Some((Seen { distance: sqrt(planar * planar + dz * dz), bearing: normalize(degrees_of(dy, dx)), elevation: degrees_of(dz, planar) }, age)))
}

pub fn cue_bearing(r: &Robot) -> Result<f64, Error> {
    let heading = r.current()?.me.heading;
    Ok(cue(r)?.map_or(heading, |(seen, _)| seen.bearing))
}

pub fn cue_elevation(r: &Robot) -> Result<f64, Error> {
    Ok(cue(r)?.map_or(0.0, |(seen, _)| seen.elevation))
}

pub fn cue_range(r: &Robot) -> Result<f64, Error> {
    Ok(cue(r)?.map_or(FAR, |(seen, _)| seen.distance))
}

pub fn cue_age(r: &Robot) -> Result<f64, Error> {
    Ok(cue(r)?.map_or(0.0, |(_, age)| age))
}

/// The nearest pickup of any kind, as the DSL measures it.
fn pickup(r: &Robot) -> Result<Option<Seen>, Error> {
    let sphere = sphere(r)?;
    let state = r.current()?;
    let mut best: Option<Seen> = None;
    for item in &state.pickups {
        let seen = towards(state, sphere, item.position);
        if best.as_ref().is_none_or(|best| seen.distance < best.distance) {
            best = Some(seen);
        }
    }
    Ok(best)
}

pub fn pickup_present(r: &Robot) -> Result<bool, Error> {
    Ok(pickup(r)?.is_some())
}

pub fn pickup_bearing(r: &Robot) -> Result<f64, Error> {
    let heading = r.current()?.me.heading;
    Ok(pickup(r)?.map_or(heading, |seen| seen.bearing))
}

pub fn pickup_elevation(r: &Robot) -> Result<f64, Error> {
    Ok(pickup(r)?.map_or(0.0, |seen| seen.elevation))
}

pub fn pickup_range(r: &Robot) -> Result<f64, Error> {
    Ok(pickup(r)?.map_or(FAR, |seen| seen.distance))
}

fn hill(r: &Robot) -> Result<Option<(Seen, f64)>, Error> {
    let sphere = sphere(r)?;
    let state = r.current()?;
    Ok(state.hill.as_ref().map(|hill| (towards(state, sphere, hill.record.center), hill.record.radius)))
}

pub fn on_hill(r: &Robot) -> Result<bool, Error> {
    Ok(hill(r)?.is_some_and(|(seen, radius)| seen.distance <= radius))
}

pub fn hill_held(r: &Robot) -> Result<bool, Error> {
    let id = &r.hello()?.id;
    Ok(r.current()?.hill.as_ref().is_some_and(|hill| hill.holder.as_ref() == Some(id)))
}

pub fn hill_bearing(r: &Robot) -> Result<f64, Error> {
    let heading = r.current()?.me.heading;
    Ok(hill(r)?.map_or(heading, |(seen, _)| seen.bearing))
}

pub fn hill_elevation(r: &Robot) -> Result<f64, Error> {
    Ok(hill(r)?.map_or(0.0, |(seen, _)| seen.elevation))
}

pub fn hill_range(r: &Robot) -> Result<f64, Error> {
    Ok(hill(r)?.map_or(FAR, |(seen, _)| seen.distance))
}

/// Seconds you have held the hill.
pub fn hill_time(r: &Robot) -> Result<f64, Error> {
    let tick_rate = r.hello()?.tick_rate;
    Ok(r.current()?.hill.as_ref().map_or(0.0, |hill| hill.held_ticks / tick_rate))
}

/// Seconds of holding the hill still needed to win it.
pub fn hill_to_win(r: &Robot) -> Result<f64, Error> {
    let hello = r.hello()?;
    Ok(r.current()?.hill.as_ref().map_or(0.0, |hill| max(0.0, hello.win_ticks / hello.tick_rate - hill.held_ticks / hello.tick_rate)))
}
