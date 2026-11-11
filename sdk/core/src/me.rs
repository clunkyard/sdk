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
