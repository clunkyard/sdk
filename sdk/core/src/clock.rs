//! The tick, in ticks and seconds, and the ticks left.

use crate::generated::*;
use crate::maths::max;
use crate::runtime::Robot;

pub fn tick(r: &Robot) -> Result<i32, Error> {
    Ok(r.current()?.tick as i32)
}

pub fn seconds(r: &Robot) -> Result<f64, Error> {
    let tick = r.current()?.tick;
    Ok(tick / r.hello()?.tick_rate)
}

pub fn remaining_ticks(r: &Robot) -> Result<i32, Error> {
    let tick = r.current()?.tick;
    Ok(max(0.0, r.hello()?.max_ticks - tick) as i32)
}
