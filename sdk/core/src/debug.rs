//! Debug lines for the owner's trace (docs/design/robot-trace.md).

use crate::generated::Error;
use crate::runtime::Robot;

pub fn log(r: &mut Robot, line: &str) -> Result<(), Error> {
    r.act.log(line);
    Ok(())
}
