/* The hurt robot (sdk/bindings/hurt.js): names the cause that took the most
 * health, and reverses while the boundary hurts it. */
#include <string.h>
#include "clunkyard.h"

static void turn(cy_robot *robot) {
  cy_hurt hurt = {0};
  cy_me_hurt(robot, &hurt);
  const char *names[] = {"SHOT", "BURST", "COLLISION", "BOUNDARY"};
  double taken[] = {hurt.shot, hurt.burst, hurt.collision, hurt.boundary};
  double total = 0;
  int worst = 0;
  for (int i = 0; i < 4; i++) {
    total += taken[i];
    if (taken[i] > taken[worst]) worst = i;
  }
  const char *text = total > 0 ? names[worst] : "NONE";
  cy_status_text(robot, text, (uint32_t)strlen(text));
  cy_status_value(robot, total);
  cy_drive_throttle(robot, hurt.boundary > 0 ? -1 : 0);
}

int main(void) {
  cy_robot *robot = 0;
  if (cy_open(CY_API_MAJOR, CY_API_MINOR, "clunkyard-c", 11, CY_API_VERSION, sizeof CY_API_VERSION - 1, &robot) < 0) return 1;
  int32_t event;
  for (;;) {
    if (cy_session_next(robot, &event) != CY_OK || event == CY_EVENT_END) break;
    if (event == CY_EVENT_STATE) turn(robot);
    cy_session_send(robot);
  }
  cy_close(robot);
  return 0;
}
