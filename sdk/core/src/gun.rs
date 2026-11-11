//! The gun: aim, cooldown, spread, the battery's heaviest shot, damage
//! scale, can_fire, the formulas, power_for_flight_time and lead.

use crate::angles::point;
use crate::generated::*;
use crate::lead::{lead as solve, Shot};
use crate::maths::max;
use crate::runtime::Robot;

pub fn aim_now(r: &Robot) -> Result<Direction, Error> {
    Ok(r.current()?.me.aim)
}

pub fn cooldown_ticks(r: &Robot) -> Result<i32, Error> {
    Ok(r.current()?.me.cooldown_ticks as i32)
}

pub fn spread(r: &Robot) -> Result<f64, Error> {
    Ok(r.current()?.me.spread)
}

/// The engine's own figure, from its unrounded battery and clock, so a shot
/// at max_power is always paid for.
pub fn max_power(r: &Robot) -> Result<f64, Error> {
    r.current()?.max_power.ok_or_else(|| Error::phase("this state has no derived.self.maxPower"))
}

pub fn damage_scale(r: &Robot) -> Result<f64, Error> {
    r.current()?.damage_scale.ok_or_else(|| Error::phase("this state has no derived.gun.damageScale"))
}

/// The cooldown is 0 and the battery pays for a shot at this power and
/// still holds reserve.
pub fn can_fire(r: &Robot, power: f64, reserve: f64) -> Result<bool, Error> {
    let cost = shot_cost(r, power)?;
    let me = &r.current()?.me;
    Ok(me.cooldown_ticks == 0.0 && me.battery >= cost + reserve)
}

pub fn shot_speed(r: &Robot, power: f64) -> Result<f64, Error> {
    r.formula("shotSpeed", |f| f.shot_speed, power)
}

pub fn shot_cost(r: &Robot, power: f64) -> Result<f64, Error> {
    r.formula("shotCost", |f| f.shot_cost, power)
}

pub fn reload_ticks(r: &Robot, power: f64) -> Result<f64, Error> {
    r.formula("reloadTicks", |f| f.reload_ticks, power)
}

pub fn shot_damage(r: &Robot, power: f64) -> Result<f64, Error> {
    r.formula("shotDamage", |f| f.shot_damage, power)
}

pub fn power_for_speed(r: &Robot, speed: f64) -> Result<f64, Error> {
    r.formula("powerForSpeed", |f| f.power_for_speed, speed)
}

/// The power whose shot covers range in seconds: power_for_speed(range / seconds).
pub fn power_for_flight_time(r: &Robot, range: f64, seconds: f64) -> Result<f64, Error> {
    if range < 0.0 {
        return Err(Error::range("range is negative"));
    }
    if seconds <= 0.0 {
        return Err(Error::range("seconds must be above 0"));
    }
    power_for_speed(r, range / seconds)
}

/// lead from where you are, at a shot of this power: your velocity counts
/// in the force rulesets, where shots inherit it.
fn lead_from(r: &Robot, target: Vec3, velocity: Vec3, power: f64) -> Result<Option<SolutionV1>, Error> {
    let hello = r.hello()?;
    let me = &r.current()?.me;
    let speed = shot_speed(r, power)?;
    let shot = Shot {
        shooter_velocity: if hello.control == Control::Forces { me.velocity } else { Vec3::default() },
        gravity: hello.gravity,
        muzzle: hello.muzzle_distance,
        max_time: hello.shot_range / max(1.0, speed),
    };
    Ok(solve(me.position, target, velocity, speed, &shot))
}

/// Aim at a contact, which holds its tracked velocity (at rest for a
/// passive contact or an untracked robot).
pub fn lead(r: &Robot, contact: &ContactV1, power: f64) -> Result<Option<SolutionV1>, Error> {
    let target = point(&contact.position, "contact.position")?;
    let id = if contact.slot > 0 { r.slot_id(contact.slot)? } else { None };
    let velocity = id.and_then(|id| r.tracker.tracks.get(id)).map_or(Vec3::default(), |track| track.velocity);
    lead_from(r, target, velocity, power)
}

pub fn lead_point(r: &Robot, position: &Vec3, velocity: &Vec3, power: f64) -> Result<Option<SolutionV1>, Error> {
    lead_from(r, point(position, "position")?, point(velocity, "velocity")?, power)
}
