//! The arithmetic of protocol/v2/reference.js, operation for operation.
//!
//! Every transcendental call goes through the libm crate, so native and
//! wasm32 builds give the same bits (clippy.toml forbids the standard
//! library's). min, max and clamp are JavaScript's Math.min and Math.max:
//! NaN wins, and +0 is above -0.

use crate::generated::Vec3;

pub const PI: f64 = core::f64::consts::PI;
/// Math.PI / 180, as reference.js has it.
pub const DEG: f64 = PI / 180.0;

pub fn sin(x: f64) -> f64 {
    libm::sin(x)
}

pub fn cos(x: f64) -> f64 {
    libm::cos(x)
}

pub fn atan2(y: f64, x: f64) -> f64 {
    libm::atan2(y, x)
}

pub fn sqrt(x: f64) -> f64 {
    libm::sqrt(x)
}

pub fn floor(x: f64) -> f64 {
    libm::floor(x)
}

/// Math.max(a, b).
pub fn max(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else if a == b {
        if a.is_sign_negative() { b } else { a }
    } else if a > b {
        a
    } else {
        b
    }
}

/// Math.min(a, b).
pub fn min(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else if a == b {
        if a.is_sign_negative() { a } else { b }
    } else if a < b {
        a
    } else {
        b
    }
}

/// Math.min(high, Math.max(low, value)).
pub fn between(value: f64, low: f64, high: f64) -> f64 {
    min(high, max(low, value))
}

/// Math.atan2(y, x) * 180 / Math.PI.
pub fn degrees_of(y: f64, x: f64) -> f64 {
    atan2(y, x) * 180.0 / PI
}

/// A heading in [0, 360]: fmod, then 360 added to a negative remainder; -0
/// becomes 0.
pub fn normalize(degrees: f64) -> f64 {
    let r = degrees % 360.0;
    if r < 0.0 { r + 360.0 } else { r + 0.0 }
}

pub fn angle_diff(target: f64, current: f64) -> f64 {
    normalize(target - current + 540.0) - 180.0
}

pub fn direction(heading: f64, pitch: f64) -> Vec3 {
    Vec3 { x: cos(pitch * DEG) * cos(heading * DEG), y: cos(pitch * DEG) * sin(heading * DEG), z: sin(pitch * DEG) }
}

pub fn add(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 { x: a.x + b.x, y: a.y + b.y, z: a.z + b.z }
}

pub fn sub(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 { x: a.x - b.x, y: a.y - b.y, z: a.z - b.z }
}

pub fn scale(a: Vec3, s: f64) -> Vec3 {
    Vec3 { x: a.x * s, y: a.y * s, z: a.z * s }
}

pub fn length(a: Vec3) -> f64 {
    sqrt(a.x * a.x + a.y * a.y + a.z * a.z)
}

pub fn planar(a: Vec3) -> f64 {
    sqrt(a.x * a.x + a.y * a.y)
}

pub fn planar_distance(a: Vec3, b: Vec3) -> f64 {
    let (dx, dy) = (a.x - b.x, a.y - b.y);
    sqrt(dx * dx + dy * dy)
}

pub fn space_distance(a: Vec3, b: Vec3) -> f64 {
    let (dx, dy, dz) = (a.x - b.x, a.y - b.y, a.z - b.z);
    sqrt(dx * dx + dy * dy + dz * dz)
}

/// origin + direction(bearing, elevation) × range.
pub fn contact_position(origin: Vec3, range: f64, bearing: f64, elevation: f64) -> Vec3 {
    add(origin, scale(direction(bearing, elevation), range))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_and_max_are_javascript_s() {
        assert!(max(0.0, -0.0).is_sign_positive() && max(-0.0, 0.0).is_sign_positive());
        assert!(min(0.0, -0.0).is_sign_negative() && min(-0.0, 0.0).is_sign_negative());
        assert!(max(1.0, f64::NAN).is_nan() && min(f64::NAN, 1.0).is_nan());
        assert_eq!(between(5.0, -1.0, 1.0), 1.0);
        assert_eq!(normalize(-1e-15), 360.0);
        assert!(normalize(-0.0).is_sign_positive());
    }
}
