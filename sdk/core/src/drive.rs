//! The drive's facts; its controls are per-field act setters (act.rs).

use crate::generated::*;
use crate::runtime::Robot;

pub fn hover_lift(r: &Robot) -> Result<f64, Error> {
    Ok(r.hello()?.hover_lift)
}
