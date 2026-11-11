// Generated from protocol/v2/api.json by protocol/v2/codegen/rust.js; do not edit. Run `make api`.
//! Fills the DSL words record from the core's facts, by the dsl: entries and words in protocol/v2/api.js.

#![allow(clippy::redundant_closure)]

use super::records::*;
use crate::runtime::Robot;

fn flag(value: bool) -> f64 {
    if value { 1.0 } else { 0.0 }
}

/// One call a turn: every word, from one state.
pub fn fill(r: &Robot) -> Result<DslWordsV1, Error> {
    let q_arena_gravity = crate::arena::gravity(r)?;
    let q_arena_size = crate::arena::size(r)?;
    let q_arena_terrain_kind = crate::arena::terrain_kind(r)?;
    let q_battery_capacity = crate::battery::capacity(r)?;
    let q_battery_charge = crate::battery::charge(r)?;
    let q_clock_remaining_ticks = crate::clock::remaining_ticks(r)?;
    let q_clock_seconds = crate::clock::seconds(r)?;
    let q_clock_tick = crate::clock::tick(r)?;
    let q_gun_aim_now = crate::gun::aim_now(r)?;
    let q_gun_cooldown_ticks = crate::gun::cooldown_ticks(r)?;
    let q_match_control = crate::r#match::control(r)?;
    let q_match_is_3d = crate::r#match::is_3d(r)?;
    let q_match_ruleset = crate::r#match::ruleset(r)?;
    let q_me_altitude = crate::me::altitude(r)?;
    let q_me_angular_velocity = crate::me::angular_velocity(r)?;
    let q_me_attitude = crate::me::attitude(r)?;
    let q_me_grounded = crate::me::grounded(r)?;
    let q_me_heading = crate::me::heading(r)?;
    let q_me_health = crate::me::health(r)?;
    let q_me_pitch = crate::me::pitch(r)?;
    let q_me_position = crate::me::position(r)?;
    let q_me_velocity = crate::me::velocity(r)?;
    let q_nav_hill = crate::nav::hill(r)?;
    let q_nav_storm = crate::nav::storm(r)?;
    let q_nav_zone = crate::nav::zone(r)?;
    let q_scanner_last = crate::scanner::last(r)?;
    let q_score_best = crate::score::best(r)?;
    let q_score_me = crate::score::me(r)?;
    let q_sensors_nearest = crate::sensors::nearest(r)?;
    let q_tracker_cue = crate::tracker::cue(r)?;
    let q_tracker_nearest_bearing_rate = crate::tracker::nearest_bearing_rate(r)?;
    Ok(DslWordsV1 {
        ruleset: f64::from(q_match_ruleset as i32),
        physics_mode: flag(q_match_ruleset != Ruleset::Classic),
        zero_g: flag(q_match_ruleset == Ruleset::ZeroG),
        control_model: f64::from(q_match_control as i32),
        inertial: flag(q_match_control == Control::Forces),
        mode3: flag(q_match_is_3d),
        tick: f64::from(q_clock_tick),
        time: q_clock_seconds,
        remaining: f64::from(q_clock_remaining_ticks),
        width: q_arena_size.x,
        height: q_arena_size.y,
        depth: q_arena_size.z,
        gravity_x: q_arena_gravity.x,
        gravity_y: q_arena_gravity.y,
        gravity_z: q_arena_gravity.z,
        terrain_kind: f64::from(q_arena_terrain_kind as i32),
        hills: flag(q_arena_terrain_kind == TerrainKind::Hills),
        health: q_me_health,
        x: q_me_position.x,
        y: q_me_position.y,
        z: q_me_position.z,
        vx: q_me_velocity.x,
        vy: q_me_velocity.y,
        vz: q_me_velocity.z,
        heading: q_me_heading,
        pitch: q_me_pitch,
        yaw_rate: q_me_angular_velocity.yaw,
        pitch_rate: q_me_angular_velocity.pitch,
        roll_rate: q_me_angular_velocity.roll,
        forward_x: q_me_attitude.forward.x,
        forward_y: q_me_attitude.forward.y,
        forward_z: q_me_attitude.forward.z,
        right_x: q_me_attitude.right.x,
        right_y: q_me_attitude.right.y,
        right_z: q_me_attitude.right.z,
        up_x: q_me_attitude.up.x,
        up_y: q_me_attitude.up.y,
        up_z: q_me_attitude.up.z,
        grounded: flag(q_me_grounded),
        altitude: q_me_altitude,
        battery: q_battery_charge,
        battery_capacity: q_battery_capacity,
        aim: q_gun_aim_now.heading,
        aim_pitch: q_gun_aim_now.pitch,
        cooldown: f64::from(q_gun_cooldown_ticks),
        scan_heading: q_scanner_last.heading,
        scan_pitch: q_scanner_last.pitch,
        scan_arc: q_scanner_last.arc,
        scan_range: q_scanner_last.range,
        contact: flag(q_sensors_nearest.is_some()),
        range: q_sensors_nearest.map_or(1000000000000.0, |v| v.range),
        bearing: q_sensors_nearest.map_or(q_me_heading, |v| v.bearing),
        elevation: q_sensors_nearest.map_or(0.0, |v| v.elevation),
        cue_present: flag(q_tracker_cue.is_some()),
        cue_error: q_tracker_cue.map_or(0.0, |v| v.bearing_error),
        bearing_rate: q_tracker_nearest_bearing_rate,
        zone_active: flag(q_nav_zone.is_some()),
        zone_radius: q_nav_zone.map_or(0.0, |v| v.radius),
        storm_active: flag(q_nav_storm.is_some()),
        storm_radius: q_nav_storm.map_or(0.0, |v| v.radius),
        hill_active: flag(q_nav_hill.is_some()),
        hill_radius: q_nav_hill.map_or(0.0, |v| v.radius),
        score: q_score_me.map_or(0.0, |v| v),
        score_best: q_score_best.map_or(0.0, |v| v),
        speed: crate::dsl::speed(r)?,
        gravity: flag(crate::dsl::gravity(r)?),
        terrain_height: crate::raw::number(r, "state.self.groundHeight")?.unwrap_or(0.0),
        battery_fraction: crate::dsl::battery_fraction(r)?,
        aim_error: crate::dsl::aim_error(r)?,
        power_min: crate::raw::number(r, "hello.gun.power.min")?.unwrap_or(0.0),
        power_max: crate::raw::number(r, "hello.gun.power.max")?.unwrap_or(0.0),
        shot_speed_base: crate::raw::number(r, "hello.gun.shotSpeed.base")?.unwrap_or(0.0),
        shot_speed_per_power: crate::raw::number(r, "hello.gun.shotSpeed.perPower")?.unwrap_or(0.0),
        shot_cost_base: crate::raw::number(r, "hello.gun.cost.base")?.unwrap_or(0.0),
        shot_cost_per_power: crate::raw::number(r, "hello.gun.cost.perPower")?.unwrap_or(0.0),
        zone_distance: crate::dsl::zone_distance(r)?,
        zone_bearing: crate::dsl::zone_bearing(r)?,
        zone_elevation: crate::dsl::zone_elevation(r)?,
        storm_distance: crate::dsl::storm_distance(r)?,
        storm_bearing: crate::dsl::storm_bearing(r)?,
        storm_heading: crate::dsl::storm_heading(r)?,
        cue_muzzle: flag(crate::dsl::cue_muzzle(r)?),
        cue_bearing: crate::dsl::cue_bearing(r)?,
        cue_elevation: crate::dsl::cue_elevation(r)?,
        cue_range: crate::dsl::cue_range(r)?,
        cue_age: crate::dsl::cue_age(r)?,
        pickup_present: flag(crate::dsl::pickup_present(r)?),
        pickup_bearing: crate::dsl::pickup_bearing(r)?,
        pickup_elevation: crate::dsl::pickup_elevation(r)?,
        pickup_range: crate::dsl::pickup_range(r)?,
        on_hill: flag(crate::dsl::on_hill(r)?),
        hill_held: flag(crate::dsl::hill_held(r)?),
        hill_bearing: crate::dsl::hill_bearing(r)?,
        hill_elevation: crate::dsl::hill_elevation(r)?,
        hill_range: crate::dsl::hill_range(r)?,
        hill_time: crate::dsl::hill_time(r)?,
        hill_to_win: crate::dsl::hill_to_win(r)?,
    })
}
