//! The scanner: the last scan, the proximity range, reach and arc_for_reach.

use crate::generated::*;
use crate::runtime::Robot;

pub fn last(r: &Robot) -> Result<ScanV1, Error> {
    Ok(r.current()?.scan)
}

pub fn proximity_range(r: &Robot) -> Result<f64, Error> {
    Ok(r.hello()?.proximity_range)
}

pub fn reach(r: &Robot, arc: f64) -> Result<f64, Error> {
    r.formula("reach", |f| f.reach, arc)
}

pub fn arc_for_reach(r: &Robot, range: f64) -> Result<f64, Error> {
    r.formula("arcForReach", |f| f.arc_for_reach, range)
}
