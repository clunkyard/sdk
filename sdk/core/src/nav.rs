//! The zone, the storm and the hill, with margin, contains and
//! distance_from; the pickups, nearest_pickup and field_distance.
//!
//! The zone, the hill and the field count distance by suddenDeath.shape: a
//! sphere in all three axes, a circle or a column in x and y only. The
//! storm is planar in every mode. A region query is absent while its region
//! is.

use crate::angles::point;
use crate::generated::*;
use crate::maths::{max, min, planar_distance, space_distance};
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

/// The gaps from the robot's edge to the walls, from its position, its
/// radius and the arena size. On each axis the nearer wall counts; z is
/// FAR in 2D, and under gravity its floor is the ground, at the altitude.
pub fn wall_margin(r: &Robot) -> Result<WallMarginV1, Error> {
    let hello = r.hello()?;
    let me = &r.current()?.me;
    let radius = hello.radius;
    // The gap and the unit normal of the nearer wall on an axis: the wall
    // at 0 when the gaps tie.
    let axis = |at: f64, size: f64| {
        let (low, high) = (at - radius, size - radius - at);
        if low <= high { (max(0.0, low), 1.0) } else { (max(0.0, high), -1.0) }
    };
    let (x, nx) = axis(me.position.x, hello.width);
    let (y, ny) = axis(me.position.y, hello.height);
    let mut best = (x, Vec3 { x: nx, y: 0.0, z: 0.0 });
    if y < best.0 {
        best = (y, Vec3 { x: 0.0, y: ny, z: 0.0 });
    }
    let mut z = FAR;
    if hello.mode == Mode::ThreeD {
        let ceiling = max(0.0, hello.depth - radius - me.position.z);
        let floor = max(0.0, if hello.gravity.z != 0.0 { me.altitude } else { me.position.z - radius });
        let normal = if floor <= ceiling { 1.0 } else { -1.0 };
        z = min(floor, ceiling);
        if z < best.0 {
            best = (z, Vec3 { x: 0.0, y: 0.0, z: normal });
        }
    }
    Ok(WallMarginV1 { x, y, z, nearest: best.0, normal: best.1 })
}
