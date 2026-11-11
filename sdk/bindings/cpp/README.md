# The C++ binding

The generated C++ API wraps the core's C ABI with subsystem objects,
value records, `std::optional` results and exceptions for negative statuses.
There is no protocol parser, tracker or physics implementation in the binding.
Controls take optional fields and merge with earlier controls in the turn.
Typed getters such as `drive.get_throttle()` read a pending field back.

Install with `make install SDK=/opt/clunkyard/sdk`. Build with
`c++ -std=c++20 -O2 -I/opt/clunkyard/sdk/include
-I/opt/clunkyard/sdk/bindings/cpp bot.cpp
/opt/clunkyard/sdk/lib/libclunkyard.a -lpthread -ldl -lm -o bot`.
The core is linked statically. The artifact carries the robot executable.

## A tiny robot

```cpp
#include "clunkyard.hpp"

class Tiny : public clunkyard::Robot {
  void on_turn() override {
    const auto target=sensors.nearest();
    if(!target) {
      drive.set(0.75,0.25);
      scanner.set(me.heading(),40);
      status.set("SEARCH",scanner.last().arc);
      return;
    }
    const auto power=gun.power_for_flight_time(target->range,1);
    const auto aim=gun.lead(*target,power);
    gun.aim(aim?aim->heading:target->bearing,aim?aim->pitch:target->elevation);
    if(gun.can_fire(power,10))gun.fire(power);
    drive.set(0.5,angles.clamp(angles.angle_diff(target->bearing,me.heading())));
    scanner.set(target->bearing,10);
    status.set("TRACK",target->range);
  }
};

int main() { Tiny robot; robot.run(); }
```

Override `on_start()` and `on_turn()`, then call `run()`. An exception logs
its message and sends the controls already set. Throw
`clunkyard::Forfeit(code, message)` to forfeit. `play(lines)` opens a memory
session, runs the same callbacks and returns the core's replies for tests.

`make test` checks the idiomatic generated wrapper against the complete
core conformance transcript, including exact floating-point bits, optional
records, controls and serialized acts.
