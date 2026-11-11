//! The angle helpers, in degrees, the same in every language.
//!
//! normalize is fmod into [0, 360], adding 360 to a negative remainder,
//! with -0 as 0. angle_diff is normalize(target − current + 540) − 180.

use crate::generated::*;
use crate::maths::{self, between, degrees_of, planar_distance, space_distance};

/// A point argument: every component finite.
pub fn point(value: &Vec3, name: &str) -> Result<Vec3, Error> {
    for (axis, component) in [("x", value.x), ("y", value.y), ("z", value.z)] {
        if !component.is_finite() {
            return Err(Error::not_finite(format!("{name}.{axis} is not finite")));
        }
    }
    Ok(*value)
}

pub fn normalize(degrees: f64) -> Result<f64, Error> {
    Ok(maths::normalize(degrees))
}

pub fn angle_diff(target: f64, current: f64) -> Result<f64, Error> {
    Ok(maths::angle_diff(target, current))
}

pub fn bearing_to(from: &Vec3, to: &Vec3) -> Result<f64, Error> {
    let (a, b) = (point(from, "from")?, point(to, "to")?);
    Ok(maths::normalize(degrees_of(b.y - a.y, b.x - a.x)))
}

pub fn elevation_to(from: &Vec3, to: &Vec3) -> Result<f64, Error> {
    let (a, b) = (point(from, "from")?, point(to, "to")?);
    Ok(degrees_of(b.z - a.z, planar_distance(b, a)))
}

pub fn direction(heading: f64, pitch: f64) -> Result<Vec3, Error> {
    Ok(maths::direction(heading, pitch))
}

pub fn distance(a: &Vec3, b: &Vec3) -> Result<f64, Error> {
    Ok(space_distance(point(a, "a")?, point(b, "b")?))
}

pub fn clamp(value: f64, low: f64, high: f64) -> Result<f64, Error> {
    if low > high {
        return Err(Error::range("low is above high"));
    }
    Ok(between(value, low, high))
}
