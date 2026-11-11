# The Rust binding

The generated Rust binding wraps the core's C ABI with value records,
subsystem objects and `Result` and `Option` results. Every record has its
checked C layout. Tracking, geometry, protocol parsing and act serialization
remain in the core. Controls take optional fields, so later calls merge with
earlier ones; typed getters read pending fields back.

Install with `make install SDK=/opt/clunkyard/sdk`. Build with
`rustc --edition=2021 -O --extern clunkyard=/opt/clunkyard/sdk/bindings/rust/libclunkyard.rlib
-L native=/opt/clunkyard/sdk/lib -C link-arg=-lpthread -C link-arg=-ldl
-C link-arg=-lm bot.rs -o bot`. The native core is statically linked.

Implement `Strategy`, then call `clunkyard::run(strategy)`. A returned error
or panic logs its trace and sends the controls already set.
`Error::forfeit(code, message)` reports a robot forfeit.
`clunkyard::play(strategy, lines)` uses a memory session for tests.

## A tiny robot

```rust
use clunkyard::{Error, Robot, Strategy};
struct Tiny;
impl Strategy for Tiny {
    fn on_turn(&mut self, robot: &Robot) -> Result<(), Error> {
        let Some(target) = robot.sensors.nearest()? else {
            robot.drive.set(Some(0.75), Some(0.25), None)?;
            robot.scanner.set(Some(robot.me.heading()?), Some(40.0), None)?;
            robot.status.set(Some("SEARCH"), Some(robot.scanner.last()?.arc))?;
            return Ok(());
        };
        let power = robot.gun.power_for_flight_time(target.range, 1.0)?;
        let aim = robot.gun.lead(&target, power)?;
        let (heading, pitch) = aim.map(|aim| (aim.heading, aim.pitch)).unwrap_or((target.bearing, target.elevation));
        robot.gun.aim(Some(heading), Some(pitch))?;
        if robot.gun.can_fire(power, 10.0)? { robot.gun.fire(Some(power))?; }
        robot.drive.set(Some(0.5), Some(robot.angles.clamp(robot.angles.angle_diff(target.bearing, robot.me.heading()?)?, -1.0, 1.0)?), None)?;
        robot.scanner.set(Some(target.bearing), Some(10.0), None)?;
        robot.status.set(Some("TRACK"), Some(target.range))?;
        Ok(())
    }
}
fn main() -> Result<(), Error> { clunkyard::run(Tiny) }
```

`make test` runs the complete core conformance transcript through the safe
Rust wrapper. It checks exact floating-point bits, layouts, optional records,
text, controls and act lines. The tiny robot is also compared across bindings.
