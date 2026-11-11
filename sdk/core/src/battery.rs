//! The battery: charge, capacity, regeneration, the compute charge and the
//! drive's cost.

use crate::generated::*;
use crate::runtime::Robot;

pub fn charge(r: &Robot) -> Result<f64, Error> {
    Ok(r.current()?.me.battery)
}

pub fn capacity(r: &Robot) -> Result<f64, Error> {
    Ok(r.hello()?.capacity)
}

pub fn regen(r: &Robot) -> Result<f64, Error> {
    Ok(r.hello()?.regen)
}

pub fn sustainable_drive(r: &Robot) -> Result<f64, Error> {
    Ok(r.hello()?.sustainable_drive)
}

pub fn compute(r: &Robot) -> Result<Option<ComputeV1>, Error> {
    Ok(r.hello()?.compute)
}

pub fn drive_cost(r: &Robot, fraction: f64) -> Result<f64, Error> {
    r.formula("driveCost", |f| f.drive_cost, fraction)
}
