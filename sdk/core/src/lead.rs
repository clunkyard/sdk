//! lead: the earliest flight time t at which a shot fired now meets the
//! target, |D(t) − g t² / 2| = speed × t + muzzle, where D(t) is the target's
//! position relative to the shooter at t (reference.js lead). It scans
//! [0, max_time] in 64 steps for the first t where the shot has caught up,
//! then bisects that step 40 times.

use crate::generated::{SolutionV1, Vec3};
use crate::maths::{add, degrees_of, length, normalize, planar, scale, sub};

#[derive(Clone, Copy, Debug, Default)]
pub struct Shot {
    pub shooter_velocity: Vec3,
    pub gravity: Vec3,
    pub muzzle: f64,
    pub max_time: f64,
}

pub fn lead(shooter: Vec3, target: Vec3, target_velocity: Vec3, speed: f64, shot: &Shot) -> Option<SolutionV1> {
    let offset = |t: f64| {
        sub(sub(sub(add(target, scale(target_velocity, t)), shooter), scale(shot.shooter_velocity, t)), scale(shot.gravity, 0.5 * t * t))
    };
    let miss = |t: f64| length(offset(t)) - (speed * t + shot.muzzle);
    let mut time = if miss(0.0) <= 0.0 { Some(0.0) } else { None };
    const STEPS: i32 = 64;
    let mut i = 1;
    while i <= STEPS && time.is_none() {
        let mut high = shot.max_time * f64::from(i) / f64::from(STEPS);
        // As reference.js: a NaN miss counts as caught up.
        if miss(high).partial_cmp(&0.0) != Some(core::cmp::Ordering::Greater) {
            let mut low = shot.max_time * f64::from(i - 1) / f64::from(STEPS);
            for _ in 0..40 {
                let mid = (low + high) / 2.0;
                if miss(mid) > 0.0 {
                    low = mid;
                } else {
                    high = mid;
                }
            }
            time = Some(high);
        }
        i += 1;
    }
    let time = time?;
    let d = offset(time);
    if length(d) == 0.0 {
        return Some(SolutionV1 { heading: 0.0, pitch: 0.0, time });
    }
    Some(SolutionV1 { heading: normalize(degrees_of(d.y, d.x)), pitch: degrees_of(d.z, planar(d)), time })
}
