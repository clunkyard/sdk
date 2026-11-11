//! The zone, the storm and the hill, with margin, contains and
//! distance_from; the pickups, nearest_pickup and field_distance.
//!
//! The zone, the hill and the field count distance by suddenDeath.shape: a
//! sphere in all three axes, a circle or a column in x and y only. The
//! storm is planar in every mode. A region query is absent while its region
//! is.

use crate::angles::point;
use crate::generated::*;
use crate::maths::{planar_distance, space_distance};
use crate::runtime::Robot;

pub fn region_distance(shape: Shape, a: Vec3, b: Vec3) -> f64 {
    if shape == Shape::Sphere { space_distance(a, b) } else { planar_distance(a, b) }
}

/// A region's shape for distance, centre and radius.
type Region = (Shape, Vec3, f64);

fn zone_region(r: &Robot) -> Result<Option<Region>, Error> {
    let shape = r.hello()?.shape;
    Ok(r.current()?.zone.map(|zone| (shape, zone.center, zone.radius)))
}

fn storm_region(r: &Robot) -> Result<Option<Region>, Error> {
    Ok(r.current()?.storm.map(|storm| (Shape::Circle, storm.center, storm.radius)))
}

fn hill_region(r: &Robot) -> Result<Option<Region>, Error> {
    let shape = r.hello()?.shape;
    Ok(r.current()?.hill.as_ref().map(|hill| (shape, hill.record.center, hill.record.radius)))
}

fn margin(region: Option<Region>, at: &Vec3) -> Result<Option<f64>, Error> {
    let Some((shape, center, radius)) = region else { return Ok(None) };
    Ok(Some(radius - region_distance(shape, point(at, "point")?, center)))
}

fn contains(region: Option<Region>, at: &Vec3, by: f64) -> Result<Option<bool>, Error> {
    let Some((shape, center, radius)) = region else { return Ok(None) };
    Ok(Some(region_distance(shape, point(at, "point")?, center) <= radius + by))
}

fn distance_from(region: Option<Region>, at: &Vec3) -> Result<Option<f64>, Error> {
    let Some((shape, center, _)) = region else { return Ok(None) };
    Ok(Some(region_distance(shape, point(at, "point")?, center)))
}

pub fn zone(r: &Robot) -> Result<Option<ZoneV1>, Error> {
    Ok(r.current()?.zone.map(|zone| ZoneV1 { shape: zone.shape, reserved: 0, radius: zone.radius, center: zone.center }))
}

pub fn zone_margin(r: &Robot, at: &Vec3) -> Result<Option<f64>, Error> {
    margin(zone_region(r)?, at)
}

pub fn zone_contains(r: &Robot, at: &Vec3, by: f64) -> Result<Option<bool>, Error> {
    contains(zone_region(r)?, at, by)
}

pub fn zone_distance_from(r: &Robot, at: &Vec3) -> Result<Option<f64>, Error> {
    distance_from(zone_region(r)?, at)
}

pub fn storm(r: &Robot) -> Result<Option<StormV1>, Error> {
    Ok(r.current()?.storm)
}

pub fn storm_margin(r: &Robot, at: &Vec3) -> Result<Option<f64>, Error> {
    margin(storm_region(r)?, at)
}

pub fn storm_contains(r: &Robot, at: &Vec3, by: f64) -> Result<Option<bool>, Error> {
    contains(storm_region(r)?, at, by)
}

pub fn storm_distance_from(r: &Robot, at: &Vec3) -> Result<Option<f64>, Error> {
    distance_from(storm_region(r)?, at)
}

/// The hill, its holder as a slot (0 for nobody).
pub fn hill(r: &Robot) -> Result<Option<HillV1>, Error> {
    Ok(r.current()?.hill.as_ref().map(|hill| HillV1 { holder: r.slot(hill.holder.as_deref()), ..hill.record }))
}

pub fn hill_margin(r: &Robot, at: &Vec3) -> Result<Option<f64>, Error> {
    margin(hill_region(r)?, at)
}

pub fn hill_contains(r: &Robot, at: &Vec3, by: f64) -> Result<Option<bool>, Error> {
    contains(hill_region(r)?, at, by)
}

pub fn hill_distance_from(r: &Robot, at: &Vec3) -> Result<Option<f64>, Error> {
    distance_from(hill_region(r)?, at)
}

pub fn pickups(r: &Robot) -> Result<Vec<PickupV1>, Error> {
    Ok(r.current()?.pickups.clone())
}

/// The pickup nearest you by field distance, of one kind or any; the first
/// listed on a tie.
pub fn nearest_pickup(r: &Robot, kind: Option<PickupKind>) -> Result<Option<PickupV1>, Error> {
    let shape = r.hello()?.shape;
    let state = r.current()?;
    let mut best: Option<(&PickupV1, f64)> = None;
    for item in &state.pickups {
        if kind.is_some_and(|kind| item.kind != kind as i32) {
            continue;
        }
        let d = region_distance(shape, item.position, state.me.position);
        if best.is_none_or(|(_, distance)| d < distance) {
            best = Some((item, d));
        }
    }
    Ok(best.map(|(item, _)| *item))
}

pub fn field_distance(r: &Robot, a: &Vec3, b: &Vec3) -> Result<f64, Error> {
    Ok(region_distance(r.hello()?.shape, point(a, "a")?, point(b, "b")?))
}
