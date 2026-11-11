//! The time-limit score and your armour; absent while the mechanic is off.

use crate::generated::*;
use crate::runtime::Robot;

pub fn me(r: &Robot) -> Result<Option<f64>, Error> {
    Ok(r.current()?.score_me)
}

pub fn best(r: &Robot) -> Result<Option<f64>, Error> {
    Ok(r.current()?.score_best)
}

pub fn armour(r: &Robot) -> Result<Option<f64>, Error> {
    Ok(r.current()?.armour)
}
