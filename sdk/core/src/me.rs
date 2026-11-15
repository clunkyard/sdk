//! Your robot, from state.self.

use crate::generated::*;
use crate::runtime::Robot;

pub fn health(r: &Robot) -> Result<f64, Error> {
    Ok(r.current()?.me.health)
}

pub fn position(r: &Robot) -> Result<Vec3, Error> {
    Ok(r.current()?.me.position)
}

pub fn velocity(r: &Robot) -> Result<Vec3, Error> {
    Ok(r.current()?.me.velocity)
}

pub fn heading(r: &Robot) -> Result<f64, Error> {
    Ok(r.current()?.me.heading)
}

pub fn pitch(r: &Robot) -> Result<f64, Error> {
    Ok(r.current()?.me.pitch)
}

pub fn roll(r: &Robot) -> Result<f64, Error> {
    Ok(r.current()?.me.roll)
}

pub fn angular_velocity(r: &Robot) -> Result<Rotation, Error> {
    Ok(r.current()?.me.angular_velocity)
}

pub fn attitude(r: &Robot) -> Result<AttitudeV1, Error> {
    Ok(r.current()?.me.attitude)
}

pub fn grounded(r: &Robot) -> Result<bool, Error> {
    Ok(r.current()?.me.grounded)
}

pub fn altitude(r: &Robot) -> Result<f64, Error> {
    Ok(r.current()?.me.altitude)
}

pub fn applied(r: &Robot) -> Result<AppliedV1, Error> {
    Ok(r.current()?.me.applied)
}

/// The mount frame: chassis angles under hull mounts, world angles otherwise.
pub fn hull_mounts(r: &Robot) -> Result<bool, Error> {
    Ok(r.hello()?.hull_mounts)
}

pub fn to_mount(r: &Robot, heading: f64, pitch: f64) -> Result<Direction, Error> {
    if !r.hello()?.hull_mounts {
        return Ok(Direction { heading, pitch });
    }
    crate::angles::to_chassis(heading, pitch, &r.current()?.me.attitude)
}

pub fn from_mount(r: &Robot, heading: f64, pitch: f64) -> Result<Direction, Error> {
    if !r.hello()?.hull_mounts {
        return Ok(Direction { heading, pitch });
    }
    crate::angles::from_chassis(heading, pitch, &r.current()?.me.attitude)
}

pub fn mount_toward(r: &Robot, point: &Vec3) -> Result<Direction, Error> {
    let from = r.current()?.me.position;
    to_mount(r, crate::angles::bearing_to(&from, point)?, crate::angles::elevation_to(&from, point)?)
}
