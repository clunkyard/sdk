# The C binding

C uses the core's generated `clunkyard.h` directly. Every function returns a
status and copies its result into the caller's output parameter. `CY_NONE`
means absent, `CY_SHORT` asks for a larger buffer, and negative statuses are
errors. `cy_last_error` gives the error text. Records and enum constants are
declared in the header; the plain names alias the current record versions.

Build with `cc -std=c17 -O2 -I/opt/clunkyard/sdk/include bot.c
/opt/clunkyard/sdk/lib/libclunkyard.a -lpthread -ldl -lm -o bot`.
The library is linked statically, and only the robot executable is shipped.

## A tiny robot

```c
#include "clunkyard.h"

static void turn(cy_robot *robot) {
  cy_contact target={0};double heading=0;
  cy_me_heading(robot,&heading);
  if(cy_sensors_nearest(robot,&target)!=CY_OK) {
    cy_drive_throttle(robot,0.75);cy_drive_turn(robot,0.25);
    cy_scanner_heading(robot,heading);cy_scanner_arc(robot,40);
    cy_scan last={0};cy_scanner_last(robot,&last);
    cy_status_text(robot,"SEARCH",6);cy_status_value(robot,last.arc);
    return;
  }
  double power=0;cy_gun_power_for_flight_time(robot,target.range,1,&power);
  cy_solution aim={0};if(cy_gun_lead(robot,&target,power,&aim)!=CY_OK){aim.heading=target.bearing;aim.pitch=target.elevation;}
  cy_gun_aim_heading(robot,aim.heading);cy_gun_aim_pitch(robot,aim.pitch);
  int32_t can_fire=0;cy_gun_can_fire(robot,power,10,&can_fire);
  if(can_fire){cy_gun_fire(robot);cy_gun_power(robot,power);}
  double delta=0,control=0;cy_angles_angle_diff(target.bearing,heading,&delta);cy_angles_clamp(delta,-1,1,&control);
  cy_drive_throttle(robot,0.5);cy_drive_turn(robot,control);
  cy_scanner_heading(robot,target.bearing);cy_scanner_arc(robot,10);
  cy_status_text(robot,"TRACK",5);cy_status_value(robot,target.range);
}
int main(void) {
  cy_robot *robot=0;
  if(cy_open(CY_API_MAJOR,CY_API_MINOR,"clunkyard-c",11,CY_API_VERSION,sizeof CY_API_VERSION-1,&robot)<0)return 1;
  int32_t event;
  for(;;){int32_t status=cy_session_next(robot,&event);if(status==CY_E_INTERNAL){if(cy_session_send(robot)<0)break;continue;}if(status!=CY_OK||event==CY_EVENT_END)break;if(event==CY_EVENT_STATE)turn(robot);cy_session_send(robot);}
  cy_close(robot);return 0;
}
```

`make test SDK=/opt/clunkyard/sdk` runs every exposed API operation against
the core's conformance vectors. The generated test preserves floating-point
bits and compares record layouts and serialized acts.
