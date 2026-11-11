/* Generated from protocol/v2/api.json by protocol/v2/codegen/c.js; do not edit. Run `make api`.
 *
 * libclunkyard: the Clunkyard robot core, API 2.0.0 (separate from wire protocol 2).
 * One opaque handle per robot. Every function returns a CY_ status:
 * CY_OK, CY_NONE for absent (not an error), CY_SHORT for a buffer too
 * short, or a negative error, whose message cy_last_error gives. Results
 * come back through out-parameters; records are copied out into memory
 * the caller owns, and nothing the caller holds points into the core.
 * Text is UTF-8, passed as a pointer and a length. A list or text that
 * does not fit returns CY_SHORT with the count or length needed.
 *
 * Exports carry their record version (cy_sensors_nearest_v1, filling a
 * cy_contact_v1); the plain names at the end alias the latest. A handle
 * belongs to one thread at a time; the core starts no threads and never
 * calls back. docs/design/robot-core.md, section 1.4, has the rules.
 */
#ifndef CLUNKYARD_H
#define CLUNKYARD_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#ifndef CY_API
#define CY_API extern
#endif

#define CY_API_VERSION "2.0.0"
#define CY_API_MAJOR 2
#define CY_API_MINOR 0
#define CY_API_PATCH 0
/* The Tracker keeps a passive cue this many ticks unless configured otherwise. */
#define CY_CUE_TICKS 60

/* Statuses. Every function returns one. A binding treats an unknown negative one as a generic error. */
#define CY_OK 0
#define CY_NONE 1
#define CY_SHORT 2
#define CY_E_NOT_FINITE -1
#define CY_E_RANGE -2
#define CY_E_ARGUMENT -3
#define CY_E_PHASE -4
#define CY_E_VERSION -5
#define CY_E_INTERNAL -6

/* What detected a contact. 0 is none. */
#define CY_SOURCE_NONE 0
#define CY_SOURCE_SCAN 1 /* scan */
#define CY_SOURCE_PROXIMITY 2 /* proximity */
#define CY_SOURCE_FAINT 3 /* faint */
#define CY_SOURCE_BEAM 4 /* beam */
#define CY_SOURCE_MUZZLE 5 /* muzzle */
#define CY_SOURCE_MOTION 6 /* motion */
#define CY_SOURCE_PING 7 /* ping */

/* The match's dimensions. 0 is none. */
#define CY_MODE_NONE 0
#define CY_MODE_TWO_D 1 /* 2d */
#define CY_MODE_THREE_D 2 /* 3d */

/* The ruleset. 0 is none. */
#define CY_RULESET_NONE 0
#define CY_RULESET_CLASSIC 1 /* classic */
#define CY_RULESET_INERTIAL_2D 2 /* inertial-2d */
#define CY_RULESET_GRAVITY_FLAT 3 /* gravity-flat */
#define CY_RULESET_GRAVITY_HILLS 4 /* gravity-hills */
#define CY_RULESET_ZERO_G 5 /* zero-g */

/*
 * What the drive controls mean: direct speeds in classic, forces elsewhere. 0
 * is none.
 */
#define CY_CONTROL_NONE 0
#define CY_CONTROL_DIRECT 1 /* direct */
#define CY_CONTROL_FORCES 2 /* forces */

/* The ground. 0 is none. */
#define CY_TERRAIN_KIND_NONE 1 /* none */
#define CY_TERRAIN_KIND_FLAT 2 /* flat */
#define CY_TERRAIN_KIND_HILLS 3 /* hills */

/*
 * A region's shape: circle in 2D, sphere in 3D classic and zero-g, column
 * (unbounded in height) under gravity. 0 is none.
 */
#define CY_SHAPE_NONE 0
#define CY_SHAPE_CIRCLE 1 /* circle */
#define CY_SHAPE_SPHERE 2 /* sphere */
#define CY_SHAPE_COLUMN 3 /* column */

/* What a pickup gives. 0 asks for any kind. 0 is any. */
#define CY_PICKUP_KIND_ANY 0
#define CY_PICKUP_KIND_ENERGY 1 /* energy */
#define CY_PICKUP_KIND_HEALTH 2 /* health */
#define CY_PICKUP_KIND_ARMOUR 3 /* armour */

/* The meter compute is charged by; 0 when turns are not metered. 0 is none. */
#define CY_COMPUTE_UNIT_NONE 0
#define CY_COMPUTE_UNIT_OPERATIONS 1 /* operations */
#define CY_COMPUTE_UNIT_FUEL 2 /* fuel */
#define CY_COMPUTE_UNIT_CPU_MS 3 /* cpu-ms */

/* What session.next read: the hello, a state, or the end of input. 0 is none. */
#define CY_EVENT_NONE 0
#define CY_EVENT_HELLO 1 /* hello */
#define CY_EVENT_STATE 2 /* state */
#define CY_EVENT_END 3 /* end */

typedef struct cy_robot cy_robot;

/* A vector in world axes. 24 bytes. */
typedef struct cy_vec3 {
  double x; /* x component, east. */
  double y; /* y component, north. */
  double z; /* z component, up; 0 in 2D. */
} cy_vec3;

/* A rotation rate or torque about the robot's own axes. 24 bytes. */
typedef struct cy_rotation {
  double yaw; /* About the up axis. */
  double pitch; /* About the right axis; 0 in 2D. */
  double roll; /* About the forward axis; 0 in 2D. */
} cy_rotation;

/* A direction as a heading and an elevation. 16 bytes. */
typedef struct cy_direction {
  double heading; /* Heading from the +x axis, degrees, 0 to 360. */
  double pitch; /* Elevation, degrees, positive upward; 0 in 2D. */
} cy_direction;

/* The chassis axes as unit vectors in world axes. 72 bytes. */
typedef struct cy_attitude_v1 {
  cy_vec3 forward; /* The chassis forward axis. */
  cy_vec3 right; /* The chassis right axis. */
  cy_vec3 up; /* The chassis up axis. */
} cy_attitude_v1;

/*
 * What the previous tick applied to you, after the battery scaled the drive.
 * 56 bytes.
 */
typedef struct cy_applied_v1 {
  cy_vec3 force; /* Total force, gravity included. */
  cy_rotation torque; /* Torque. */
  double brake; /* Brake, 0 to 1. */
} cy_applied_v1;

/*
 * The scan that produced this tick's contacts: the one your previous act asked
 * for. 32 bytes.
 */
typedef struct cy_scan_v1 {
  double heading; /* Scan heading. */
  double pitch; /* Scan pitch; 0 in 2D. */
  double arc; /* Full cone width. */
  double range; /* Reach at this arc. */
} cy_scan_v1;

/* A robot the sensors detect this tick. 72 bytes. */
typedef struct cy_contact_v1 {
  int32_t slot; /* The robot: you are 1, and the others are numbered from 2 by first sighting (contacts first, then the hill holder), stable all match; 0 for a passive contact, which does not say which robot it heard. */
  int32_t source; /* CY_SOURCE_*. What detected it: scan, inside the scan cone and its reach; proximity, within scanner.proximityRange in any direction. A robot both inside the cone and near reports scan. With hello.sensing, passive sources give a noisy direction and no id: faint, inside the scan cone beyond its reach; beam, a robot whose scan swept over you; muzzle, a robot that fired; motion, a robot moving fast; ping, the periodic ping of every robot. Later sensors add sources; ignore ones you do not know. */
  double range; /* Straight-line distance between centres. For a passive source, noisy and rounded to sensing.rangeBin. */
  double bearing; /* Heading from you to it. For a passive source, noisy. */
  double elevation; /* Elevation from you to it; 0 in 2D. For a passive source, noisy. */
  double bearing_error; /* Passive sources only: the standard deviation of bearing and elevation, degrees; 0 for scan and proximity. */
  double range_error; /* Passive sources only: the standard deviation of the natural log of range; 0 for scan and proximity. */
  cy_vec3 position; /* Where it is, in world axes: your position plus its range along its bearing and elevation. */
} cy_contact_v1;

/*
 * The most accurate passive contact the Tracker holds: where to look, not
 * where to shoot. 40 bytes.
 */
typedef struct cy_cue_v1 {
  int32_t source; /* CY_SOURCE_*. What heard it. */
  int32_t tick; /* The tick it was heard. */
  double bearing_error; /* The standard deviation of its bearing and elevation, degrees. */
  cy_vec3 position; /* Where it placed the robot it heard, in world axes. */
} cy_cue_v1;

/*
 * Where to aim for a shot fired now to meet a target, and when it meets it. 24
 * bytes.
 */
typedef struct cy_solution_v1 {
  double heading; /* Turret heading, degrees, 0 to 360. */
  double pitch; /* Turret pitch, degrees; 0 in 2D. */
  double time; /* Seconds of flight until the shot meets the target. */
} cy_solution_v1;

/* The sudden-death safe zone. 40 bytes. */
typedef struct cy_zone_v1 {
  int32_t shape; /* CY_SHAPE_*. circle in 2D; sphere in 3D classic and zero-g; column, unbounded in height, in the gravity rulesets. */
  int32_t reserved; /* Reserved; always 0. It keeps the record free of hidden padding. */
  double radius; /* The radius now. It shrinks evenly from suddenDeath.radiusStart to radiusEnd at suddenDeath.endTick (the last tick when absent). */
  cy_vec3 center; /* The zone's centre. A column's centre has z 0 and its height does not matter. */
} cy_zone_v1;

/* The storm now. 72 bytes. */
typedef struct cy_storm_v1 {
  int32_t shape; /* CY_SHAPE_*. circle in 2D, column in 3D. */
  int32_t reserved; /* Reserved; always 0. It keeps the record free of hidden padding. */
  double radius; /* Its radius now. */
  double dps; /* Health lost per second inside it. */
  cy_vec3 center; /* Its centre; z is 0. */
  cy_vec3 velocity; /* Its velocity, units/s. */
} cy_storm_v1;

/* The king-of-the-hill zone now. 80 bytes. */
typedef struct cy_hill_v1 {
  int32_t shape; /* CY_SHAPE_*. As zone.shape. */
  int32_t holder; /* The slot of the robot alone inside it (1 is you), or 0 for nobody. */
  int32_t contested; /* 1 or 0. More than one robot is inside it. */
  int32_t moves_tick; /* When it moves. */
  int32_t held_ticks; /* Ticks you have held it. */
  int32_t best_held_ticks; /* Most ticks any other robot has held it. */
  double radius; /* Its radius. */
  cy_vec3 center; /* Its centre. */
  cy_vec3 next; /* Where it moves at movesTick. */
} cy_hill_v1;

/* A cell on the field. 40 bytes. */
typedef struct cy_pickup_v1 {
  int32_t kind; /* CY_PICKUP_KIND_*. energy refills the battery, health repairs, armour absorbs shot damage before health. */
  int32_t expires_tick; /* The tick it disappears if nobody takes it. */
  double amount; /* What it gives. */
  cy_vec3 position; /* Where it is. Under gravity it lies on the ground and only x and y count. */
} cy_pickup_v1;

/* How the match charges compute against the battery. 16 bytes. */
typedef struct cy_compute_v1 {
  double k; /* The charge for a turn that uses its whole budget. */
  int32_t quanta; /* A trusted CPU turn is charged in steps of 1/quanta of its budget: k × q / quanta, where q = min(quanta, floor(quanta × cpu / budget)). */
  int32_t unit; /* CY_COMPUTE_UNIT_*. The meter you are charged by: DSL operations and WebAssembly fuel exactly, k × used / limit; trusted CPU in quanta. null when your turns are not metered, so thinking costs you nothing. */
} cy_compute_v1;

/* ---- the handle ---- */
/* The core's API version. */
CY_API int32_t cy_version(int32_t *major, int32_t *minor, int32_t *patch);
/*
 * Opens a robot on standard input and output. CY_E_VERSION when api_major
 * differs from the core's or api_minor is newer. binding and
 * binding_version name the binding in the ready reply's sdk, as
 * clunkyard-python 2.0.0.
 */
CY_API int32_t cy_open(int32_t api_major, int32_t api_minor, const char *binding, uint32_t binding_length, const char *binding_version, uint32_t binding_version_length, cy_robot **out);
/* As cy_open, on a memory port: cy_session_feed gives it lines and cy_session_reply takes its own. */
CY_API int32_t cy_open_memory(int32_t api_major, int32_t api_minor, const char *binding, uint32_t binding_length, const char *binding_version, uint32_t binding_version_length, cy_robot **out);
/* Frees a robot; a null robot is fine. */
CY_API int32_t cy_close(cy_robot *robot);
/* The message of the robot's last error. */
CY_API int32_t cy_last_error(cy_robot *robot, char *buf, uint32_t capacity, uint32_t *length);

/* ---- match: The match, fixed from the hello ---- */
/* match.mode: The match is 2D or 3D. */
CY_API int32_t cy_match_mode(cy_robot *robot, int32_t *out);
/* match.ruleset: The ruleset in play. */
CY_API int32_t cy_match_ruleset(cy_robot *robot, int32_t *out);
/* match.control: Whether the drive sets speeds (direct) or forces. */
CY_API int32_t cy_match_control(cy_robot *robot, int32_t *out);
/* match.robots: Robots in the match, you included; slots run 1 to this. */
CY_API int32_t cy_match_robots(cy_robot *robot, int32_t *out);
/* match.is_3d: The match is 3D. */
CY_API int32_t cy_match_is_3d(cy_robot *robot, int32_t *out);
/* match.tick_rate: Ticks per second. */
CY_API int32_t cy_match_tick_rate(cy_robot *robot, int32_t *out);
/* match.max_ticks: The last tick the match can reach. */
CY_API int32_t cy_match_max_ticks(cy_robot *robot, int32_t *out);
/* match.robot_id: The id the protocol gives the robot in a slot. */
CY_API int32_t cy_robot_id(cy_robot *robot, int32_t slot, char *buf, uint32_t capacity, uint32_t *length);

/* ---- clock: Where the match is in time ---- */
/* clock.tick: This tick, from 0. */
CY_API int32_t cy_clock_tick(cy_robot *robot, int32_t *out);
/* clock.seconds: The match time in seconds. */
CY_API int32_t cy_clock_seconds(cy_robot *robot, double *out);
/* clock.remaining_ticks: Ticks left: match.max_ticks less this tick. */
CY_API int32_t cy_clock_remaining_ticks(cy_robot *robot, int32_t *out);

/* ---- arena: The space robots move in, fixed from the hello ---- */
/* arena.size: The arena runs from 0 to this on each axis. */
CY_API int32_t cy_arena_size(cy_robot *robot, cy_vec3 *out);
/* arena.gravity: The acceleration of gravity, units/s². */
CY_API int32_t cy_arena_gravity(cy_robot *robot, cy_vec3 *out);
/* arena.terrain_kind: The kind of ground: none, flat or hills. */
CY_API int32_t cy_arena_terrain_kind(cy_robot *robot, int32_t *out);
/*
 * arena.ground_height: The height of the ground at a point, interpolated as
 * the arena does.
 */
CY_API int32_t cy_arena_ground_height(cy_robot *robot, double x, double y, double *out);

/* ---- me: Your robot, after the previous tick resolved ---- */
/* me.health: Health left. */
CY_API int32_t cy_me_health(cy_robot *robot, double *out);
/* me.position: Your centre. */
CY_API int32_t cy_me_position(cy_robot *robot, cy_vec3 *out);
/* me.velocity: Your velocity, units/s. */
CY_API int32_t cy_me_velocity(cy_robot *robot, cy_vec3 *out);
/* me.heading: Your chassis heading. */
CY_API int32_t cy_me_heading(cy_robot *robot, double *out);
/* me.pitch: Your chassis pitch; 0 in 2D. */
CY_API int32_t cy_me_pitch(cy_robot *robot, double *out);
/* me.roll: Your chassis roll; 0 in 2D and classic. */
CY_API int32_t cy_me_roll(cy_robot *robot, double *out);
/* me.angular_velocity: Your rotation rates, degrees/s. */
CY_API int32_t cy_me_angular_velocity(cy_robot *robot, cy_rotation *out);
/* me.attitude: Your chassis axes in world axes. */
CY_API int32_t cy_me_attitude_v1(cy_robot *robot, cy_attitude_v1 *out);
/* me.grounded: You rest on the ground, under gravity. */
CY_API int32_t cy_me_grounded(cy_robot *robot, int32_t *out);
/* me.altitude: In 3D, the height of your underside above the ground. */
CY_API int32_t cy_me_altitude(cy_robot *robot, double *out);
/* me.applied: What the previous tick applied to you. */
CY_API int32_t cy_me_applied_v1(cy_robot *robot, cy_applied_v1 *out);

/* ---- battery: The battery that driving and shooting share ---- */
/* battery.charge: Charge now, before this tick's regeneration and driving. */
CY_API int32_t cy_battery_charge(cy_robot *robot, double *out);
/* battery.capacity: Charge when full. */
CY_API int32_t cy_battery_capacity(cy_robot *robot, double *out);
/* battery.regen: Charge regained each second. */
CY_API int32_t cy_battery_regen(cy_robot *robot, double *out);
/* battery.sustainable_drive: The drive fraction regeneration pays for forever. */
CY_API int32_t cy_battery_sustainable_drive(cy_robot *robot, double *out);
/*
 * battery.compute: How compute is charged, when the match charges it. CY_NONE
 * when absent.
 */
CY_API int32_t cy_battery_compute_v1(cy_robot *robot, cy_compute_v1 *out);
/*
 * battery.drive_cost: What holding a drive fraction costs each second, by
 * hello.formulas.driveCost.
 */
CY_API int32_t cy_battery_drive_cost(cy_robot *robot, double fraction, double *out);

/* ---- drive: The chassis controls ---- */
/*
 * drive.hover_lift: The forces.lift that cancels gravity for a level robot; 0
 * without gravity.
 */
CY_API int32_t cy_drive_hover_lift(cy_robot *robot, double *out);
/*
 * drive.set (throttle): Sets drive.throttle. The act write clamps it to -1..1.
 * CY_E_NOT_FINITE keeps it as it was.
 */
CY_API int32_t cy_drive_throttle(cy_robot *robot, double value);
/*
 * drive.set (turn): Sets drive.turn. The act write clamps it to -1..1.
 * CY_E_NOT_FINITE keeps it as it was.
 */
CY_API int32_t cy_drive_turn(cy_robot *robot, double value);
/*
 * drive.set (pitch_turn): Sets drive.pitchTurn. The act write clamps it to
 * -1..1. CY_E_NOT_FINITE keeps it as it was.
 */
CY_API int32_t cy_drive_pitch_turn(cy_robot *robot, double value);
/*
 * drive.forces (thrust): Sets forces.thrust. The act write clamps it to -1..1.
 * CY_E_NOT_FINITE keeps it as it was.
 */
CY_API int32_t cy_drive_thrust(cy_robot *robot, double value);
/*
 * drive.forces (strafe): Sets forces.strafe. The act write clamps it to -1..1.
 * CY_E_NOT_FINITE keeps it as it was.
 */
CY_API int32_t cy_drive_strafe(cy_robot *robot, double value);
/*
 * drive.forces (lift): Sets forces.lift. The act write clamps it to -1..1.
 * CY_E_NOT_FINITE keeps it as it was.
 */
CY_API int32_t cy_drive_lift(cy_robot *robot, double value);
/*
 * drive.forces (yaw_torque): Sets forces.yawTorque. The act write clamps it to
 * -1..1. CY_E_NOT_FINITE keeps it as it was.
 */
CY_API int32_t cy_drive_yaw_torque(cy_robot *robot, double value);
/*
 * drive.forces (pitch_torque): Sets forces.pitchTorque. The act write clamps
 * it to -1..1. CY_E_NOT_FINITE keeps it as it was.
 */
CY_API int32_t cy_drive_pitch_torque(cy_robot *robot, double value);
/*
 * drive.forces (roll_torque): Sets forces.rollTorque. The act write clamps it
 * to -1..1. CY_E_NOT_FINITE keeps it as it was.
 */
CY_API int32_t cy_drive_roll_torque(cy_robot *robot, double value);
/*
 * drive.forces (brake): Sets forces.brake. The act write clamps it to 0..1.
 * CY_E_NOT_FINITE keeps it as it was.
 */
CY_API int32_t cy_drive_brake(cy_robot *robot, double value);
/*
 * drive.get (throttle): Reads back drive.throttle. The value as set, before
 * clamping; CY_NONE while unset.
 */
CY_API int32_t cy_drive_get_throttle(cy_robot *robot, double *out);
/*
 * drive.get (turn): Reads back drive.turn. The value as set, before clamping;
 * CY_NONE while unset.
 */
CY_API int32_t cy_drive_get_turn(cy_robot *robot, double *out);
/*
 * drive.get (pitch_turn): Reads back drive.pitchTurn. The value as set, before
 * clamping; CY_NONE while unset.
 */
CY_API int32_t cy_drive_get_pitch_turn(cy_robot *robot, double *out);
/*
 * drive.get (thrust): Reads back forces.thrust. The value as set, before
 * clamping; CY_NONE while unset.
 */
CY_API int32_t cy_drive_get_thrust(cy_robot *robot, double *out);
/*
 * drive.get (strafe): Reads back forces.strafe. The value as set, before
 * clamping; CY_NONE while unset.
 */
CY_API int32_t cy_drive_get_strafe(cy_robot *robot, double *out);
/*
 * drive.get (lift): Reads back forces.lift. The value as set, before clamping;
 * CY_NONE while unset.
 */
CY_API int32_t cy_drive_get_lift(cy_robot *robot, double *out);
/*
 * drive.get (yaw_torque): Reads back forces.yawTorque. The value as set,
 * before clamping; CY_NONE while unset.
 */
CY_API int32_t cy_drive_get_yaw_torque(cy_robot *robot, double *out);
/*
 * drive.get (pitch_torque): Reads back forces.pitchTorque. The value as set,
 * before clamping; CY_NONE while unset.
 */
CY_API int32_t cy_drive_get_pitch_torque(cy_robot *robot, double *out);
/*
 * drive.get (roll_torque): Reads back forces.rollTorque. The value as set,
 * before clamping; CY_NONE while unset.
 */
CY_API int32_t cy_drive_get_roll_torque(cy_robot *robot, double *out);
/*
 * drive.get (brake): Reads back forces.brake. The value as set, before
 * clamping; CY_NONE while unset.
 */
CY_API int32_t cy_drive_get_brake(cy_robot *robot, double *out);

/* ---- gun: The turret and its shots ---- */
/* gun.aim_now: Where the turret points now; a shot leaves along it. */
CY_API int32_t cy_gun_aim_now(cy_robot *robot, cy_direction *out);
/*
 * gun.cooldown_ticks: Ticks until the gun can fire; 0 means it can fire this
 * tick.
 */
CY_API int32_t cy_gun_cooldown_ticks(cy_robot *robot, int32_t *out);
/* gun.spread: The aim spread a shot would have at your speed now. */
CY_API int32_t cy_gun_spread(cy_robot *robot, double *out);
/*
 * gun.max_power: The heaviest shot the battery pays for now; 0 when it cannot
 * pay for the lightest.
 */
CY_API int32_t cy_gun_max_power(cy_robot *robot, double *out);
/* gun.damage_scale: The escalation multiplier on hit damage this tick. */
CY_API int32_t cy_gun_damage_scale(cy_robot *robot, double *out);
/*
 * gun.can_fire: The cooldown is 0 and the battery pays for a shot at this
 * power and still holds the reserve.
 */
CY_API int32_t cy_gun_can_fire(cy_robot *robot, double power, double reserve, int32_t *out);
/*
 * gun.shot_speed: How fast a shot at this power flies, units/s, by
 * hello.formulas.shotSpeed.
 */
CY_API int32_t cy_gun_shot_speed(cy_robot *robot, double power, double *out);
/*
 * gun.shot_cost: What a shot at this power costs the battery, by
 * hello.formulas.shotCost.
 */
CY_API int32_t cy_gun_shot_cost(cy_robot *robot, double power, double *out);
/*
 * gun.reload_ticks: The reload after a shot at this power, ticks, by
 * hello.formulas.reloadTicks.
 */
CY_API int32_t cy_gun_reload_ticks(cy_robot *robot, double power, double *out);
/*
 * gun.shot_damage: The damage of a hit at this power before escalation, by
 * hello.formulas.shotDamage.
 */
CY_API int32_t cy_gun_shot_damage(cy_robot *robot, double power, double *out);
/*
 * gun.power_for_speed: The power whose shot flies at this speed, within the
 * legal powers, by hello.formulas.powerForSpeed.
 */
CY_API int32_t cy_gun_power_for_speed(cy_robot *robot, double speed, double *out);
/*
 * gun.power_for_flight_time: The power whose shot covers this range in this
 * time, within the legal powers.
 */
CY_API int32_t cy_gun_power_for_flight_time(cy_robot *robot, double range, double seconds, double *out);
/*
 * gun.lead: The aim at which a shot fired now meets the contact if it holds
 * its tracked velocity; absent when no shot reaches it. CY_NONE when absent.
 */
CY_API int32_t cy_gun_lead_v1(cy_robot *robot, const cy_contact_v1 *contact, double power, cy_solution_v1 *out);
/*
 * gun.lead_point: The aim at which a shot fired now meets a point moving at
 * this velocity; absent when no shot reaches it. CY_NONE when absent.
 */
CY_API int32_t cy_gun_lead_point_v1(cy_robot *robot, const cy_vec3 *position, const cy_vec3 *velocity, double power, cy_solution_v1 *out);
/* gun.aim (heading): Sets aim.heading. CY_E_NOT_FINITE keeps it as it was. */
CY_API int32_t cy_gun_aim_heading(cy_robot *robot, double value);
/* gun.aim (pitch): Sets aim.pitch. CY_E_NOT_FINITE keeps it as it was. */
CY_API int32_t cy_gun_aim_pitch(cy_robot *robot, double value);
/* gun.fire (fire): Sets fire. */
CY_API int32_t cy_gun_fire(cy_robot *robot);
/* gun.fire (power): Sets fire.power. CY_E_NOT_FINITE keeps it as it was. */
CY_API int32_t cy_gun_power(cy_robot *robot, double value);
/*
 * gun.get (heading): Reads back aim.heading. The value as set, before
 * clamping; CY_NONE while unset.
 */
CY_API int32_t cy_gun_get_aim_heading(cy_robot *robot, double *out);
/*
 * gun.get (pitch): Reads back aim.pitch. The value as set, before clamping;
 * CY_NONE while unset.
 */
CY_API int32_t cy_gun_get_aim_pitch(cy_robot *robot, double *out);
/*
 * gun.get (fire): Reads back fire. The value as set, before clamping; CY_NONE
 * while unset.
 */
CY_API int32_t cy_gun_get_fire(cy_robot *robot, double *out);
/*
 * gun.get (power): Reads back fire.power. The value as set, before clamping;
 * CY_NONE while unset.
 */
CY_API int32_t cy_gun_get_power(cy_robot *robot, double *out);

/* ---- scanner: The scanner ---- */
/* scanner.last: The scan that produced this tick's contacts. */
CY_API int32_t cy_scanner_last_v1(cy_robot *robot, cy_scan_v1 *out);
/* scanner.proximity_range: Robots this close are detected in any direction. */
CY_API int32_t cy_scanner_proximity_range(cy_robot *robot, double *out);
/* scanner.reach: How far a scan of this arc reaches, by hello.formulas.reach. */
CY_API int32_t cy_scanner_reach(cy_robot *robot, double arc, double *out);
/*
 * scanner.arc_for_reach: The widest arc that still reaches this far, by
 * hello.formulas.arcForReach.
 */
CY_API int32_t cy_scanner_arc_for_reach(cy_robot *robot, double range, double *out);
/*
 * scanner.set (heading): Sets scan.heading. CY_E_NOT_FINITE keeps it as it
 * was.
 */
CY_API int32_t cy_scanner_heading(cy_robot *robot, double value);
/* scanner.set (arc): Sets scan.arc. CY_E_NOT_FINITE keeps it as it was. */
CY_API int32_t cy_scanner_arc(cy_robot *robot, double value);
/* scanner.set (pitch): Sets scan.pitch. CY_E_NOT_FINITE keeps it as it was. */
CY_API int32_t cy_scanner_pitch(cy_robot *robot, double value);
/*
 * scanner.get (heading): Reads back scan.heading. The value as set, before
 * clamping; CY_NONE while unset.
 */
CY_API int32_t cy_scanner_get_heading(cy_robot *robot, double *out);
/*
 * scanner.get (arc): Reads back scan.arc. The value as set, before clamping;
 * CY_NONE while unset.
 */
CY_API int32_t cy_scanner_get_arc(cy_robot *robot, double *out);
/*
 * scanner.get (pitch): Reads back scan.pitch. The value as set, before
 * clamping; CY_NONE while unset.
 */
CY_API int32_t cy_scanner_get_pitch(cy_robot *robot, double *out);

/* ---- sensors: What the sensors detect this tick ---- */
/*
 * sensors.contacts: Every contact this tick: scan and proximity first, nearest
 * first, then passive ones.
 */
CY_API int32_t cy_sensors_contacts_v1(cy_robot *robot, cy_contact_v1 *out, uint32_t capacity, uint32_t *count);
/*
 * sensors.nearest: The nearest precise contact this tick. Passive contacts are
 * never returned. CY_NONE when absent.
 */
CY_API int32_t cy_sensors_nearest_v1(cy_robot *robot, cy_contact_v1 *out);
/*
 * sensors.cues: This tick's passive contacts, most accurate (smallest bearing
 * error) first.
 */
CY_API int32_t cy_sensors_cues_v1(cy_robot *robot, cy_contact_v1 *out, uint32_t capacity, uint32_t *count);
/*
 * sensors.contact: The precise contact for a robot this tick, if it was
 * detected. CY_NONE when absent.
 */
CY_API int32_t cy_sensors_contact_v1(cy_robot *robot, int32_t slot, cy_contact_v1 *out);
/*
 * sensors.position_of: Where a contact places its robot: your position plus
 * its range along its bearing and elevation.
 */
CY_API int32_t cy_sensors_position_of_v1(cy_robot *robot, const cy_contact_v1 *contact, cy_vec3 *out);

/* ---- tracker: Estimates from past sightings, updated once a state inside the core ---- */
/*
 * tracker.cue: The most accurate passive contact heard in the last cue_ticks
 * (60 by default), or absent. CY_NONE when absent.
 */
CY_API int32_t cy_tracker_cue_v1(cy_robot *robot, cy_cue_v1 *out);
/*
 * tracker.nearest_bearing_rate: How fast the nearest contact's bearing turns,
 * from two sightings.
 */
CY_API int32_t cy_tracker_nearest_bearing_rate(cy_robot *robot, double *out);
/* tracker.velocity: The robot's tracked velocity; zero when it has no track. */
CY_API int32_t cy_tracker_velocity(cy_robot *robot, int32_t slot, cy_vec3 *out);
/*
 * tracker.position: Where the robot was last seen, if it was. CY_NONE when
 * absent.
 */
CY_API int32_t cy_tracker_position(cy_robot *robot, int32_t slot, cy_vec3 *out);
/*
 * tracker.last_seen: The tick the robot was last seen, if it was. CY_NONE when
 * absent.
 */
CY_API int32_t cy_tracker_last_seen(cy_robot *robot, int32_t slot, int32_t *out);
/*
 * tracker.configure: Sets the Tracker's parameters, in the robot's start only;
 * after the first state it refuses with E_PHASE.
 */
CY_API int32_t cy_tracker_configure(cy_robot *robot, double smoothing, int32_t forget_ticks, int32_t cue_ticks);

/* ---- nav: The regions and objects on the field ---- */
/*
 * nav.zone: The sudden-death safe zone, once sudden death starts. CY_NONE when
 * absent.
 */
CY_API int32_t cy_nav_zone_v1(cy_robot *robot, cy_zone_v1 *out);
/*
 * nav.zone.margin: How far inside the zone a point is, by its shape's
 * distance; absent while there is none. CY_NONE when absent.
 */
CY_API int32_t cy_nav_zone_margin(cy_robot *robot, const cy_vec3 *point, double *out);
/*
 * nav.zone.contains: The point is inside the zone by at least the margin;
 * absent while there is none. CY_NONE when absent.
 */
CY_API int32_t cy_nav_zone_contains(cy_robot *robot, const cy_vec3 *point, double margin, int32_t *out);
/*
 * nav.zone.distance_from: How far a point is from the zone's edge, 0 inside;
 * absent while there is none. CY_NONE when absent.
 */
CY_API int32_t cy_nav_zone_distance_from(cy_robot *robot, const cy_vec3 *point, double *out);
/* nav.storm: The storm, once it enters. CY_NONE when absent. */
CY_API int32_t cy_nav_storm_v1(cy_robot *robot, cy_storm_v1 *out);
/*
 * nav.storm.margin: How far inside the storm a point is, by its shape's
 * distance; absent while there is none. CY_NONE when absent.
 */
CY_API int32_t cy_nav_storm_margin(cy_robot *robot, const cy_vec3 *point, double *out);
/*
 * nav.storm.contains: The point is inside the storm by at least the margin;
 * absent while there is none. CY_NONE when absent.
 */
CY_API int32_t cy_nav_storm_contains(cy_robot *robot, const cy_vec3 *point, double margin, int32_t *out);
/*
 * nav.storm.distance_from: How far a point is from the storm's edge, 0 inside;
 * absent while there is none. CY_NONE when absent.
 */
CY_API int32_t cy_nav_storm_distance_from(cy_robot *robot, const cy_vec3 *point, double *out);
/* nav.hill: The hill, once it appears. CY_NONE when absent. */
CY_API int32_t cy_nav_hill_v1(cy_robot *robot, cy_hill_v1 *out);
/*
 * nav.hill.margin: How far inside the hill a point is, by its shape's
 * distance; absent while there is none. CY_NONE when absent.
 */
CY_API int32_t cy_nav_hill_margin(cy_robot *robot, const cy_vec3 *point, double *out);
/*
 * nav.hill.contains: The point is inside the hill by at least the margin;
 * absent while there is none. CY_NONE when absent.
 */
CY_API int32_t cy_nav_hill_contains(cy_robot *robot, const cy_vec3 *point, double margin, int32_t *out);
/*
 * nav.hill.distance_from: How far a point is from the hill's edge, 0 inside;
 * absent while there is none. CY_NONE when absent.
 */
CY_API int32_t cy_nav_hill_distance_from(cy_robot *robot, const cy_vec3 *point, double *out);
/* nav.pickups: Every pickup on the field. */
CY_API int32_t cy_nav_pickups_v1(cy_robot *robot, cy_pickup_v1 *out, uint32_t capacity, uint32_t *count);
/*
 * nav.nearest_pickup: The pickup of this kind nearest you by field distance,
 * if there is one. CY_NONE when absent.
 */
CY_API int32_t cy_nav_nearest_pickup_v1(cy_robot *robot, int32_t kind, cy_pickup_v1 *out);
/*
 * nav.field_distance: The distance between two points as the field measures
 * it: in the plane under gravity, in space otherwise.
 */
CY_API int32_t cy_nav_field_distance(cy_robot *robot, const cy_vec3 *a, const cy_vec3 *b, double *out);

/* ---- score: Time-limit scoring, when the match scores ---- */
/*
 * score.me: Your score; absent when the match does not score. CY_NONE when
 * absent.
 */
CY_API int32_t cy_score_me(cy_robot *robot, double *out);
/*
 * score.best: The best other robot's score; absent when the match does not
 * score. CY_NONE when absent.
 */
CY_API int32_t cy_score_best(cy_robot *robot, double *out);
/*
 * score.armour: Your armour; absent when the match has no objectives. CY_NONE
 * when absent.
 */
CY_API int32_t cy_score_armour(cy_robot *robot, double *out);

/* ---- angles: Angle and vector arithmetic in the core's maths, so every language agrees bit for bit ---- */
/*
 * angles.normalize: An angle brought into 0 to 360, exactly: degrees % 360,
 * plus 360 when that is negative.
 */
CY_API int32_t cy_angles_normalize(double degrees, double *out);
/*
 * angles.angle_diff: The signed turn from current to target: normalize(target
 * - current + 540) - 180.
 */
CY_API int32_t cy_angles_angle_diff(double target, double current, double *out);
/* angles.bearing_to: The heading from one point to another. */
CY_API int32_t cy_angles_bearing_to(const cy_vec3 *from, const cy_vec3 *to, double *out);
/* angles.elevation_to: The elevation from one point to another. */
CY_API int32_t cy_angles_elevation_to(const cy_vec3 *from, const cy_vec3 *to, double *out);
/* angles.direction: The unit vector along a heading and pitch. */
CY_API int32_t cy_angles_direction(double heading, double pitch, cy_vec3 *out);
/* angles.distance: The straight-line distance between two points. */
CY_API int32_t cy_angles_distance(const cy_vec3 *a, const cy_vec3 *b, double *out);
/* angles.clamp: A value held within low and high. */
CY_API int32_t cy_angles_clamp(double value, double low, double high, double *out);

/* ---- status: The status line spectators see ---- */
/* status.set (text): Sets status.text. */
CY_API int32_t cy_status_text(cy_robot *robot, const char *text, uint32_t text_length);
/* status.set (value): Sets status.value. CY_E_NOT_FINITE keeps it as it was. */
CY_API int32_t cy_status_value(cy_robot *robot, double value);
/*
 * status.get (text): Reads back status.text. The value as set, before
 * clamping; CY_NONE while unset.
 */
CY_API int32_t cy_status_get_text(cy_robot *robot, char *buf, uint32_t capacity, uint32_t *length);
/*
 * status.get (value): Reads back status.value. The value as set, before
 * clamping; CY_NONE while unset.
 */
CY_API int32_t cy_status_get_value(cy_robot *robot, double *out);

/* ---- raw: The messages as the arena sent them, for what the API does not cover ---- */
/* raw.number: The number at a path, if there is one. CY_NONE when absent. */
CY_API int32_t cy_raw_number(cy_robot *robot, const char *path, uint32_t path_length, double *out);
/* raw.text: The string at a path, if there is one. CY_NONE when absent. */
CY_API int32_t cy_raw_text(cy_robot *robot, const char *path, uint32_t path_length, char *buf, uint32_t capacity, uint32_t *length);
/* raw.has: Something is at the path. */
CY_API int32_t cy_raw_has(cy_robot *robot, const char *path, uint32_t path_length, int32_t *out);
/* raw.flag: The boolean at a path, if there is one. CY_NONE when absent. */
CY_API int32_t cy_raw_flag(cy_robot *robot, const char *path, uint32_t path_length, int32_t *out);
/*
 * raw.count: The length of the array at a path, if there is one. CY_NONE when
 * absent.
 */
CY_API int32_t cy_raw_count(cy_robot *robot, const char *path, uint32_t path_length, int32_t *out);
/* raw.line: The last line the session read: the hello, or the latest state. */
CY_API int32_t cy_raw_line(cy_robot *robot, char *buf, uint32_t capacity, uint32_t *length);
/*
 * raw.extra: Adds fields the API does not cover to this tick's act; a key the
 * API writes (v, type, tick, drive, forces, aim, scan, fire, status) is
 * refused with E_ARGUMENT.
 */
CY_API int32_t cy_raw_extra(cy_robot *robot, const char *json, uint32_t json_length);

/* ---- session: The protocol session: hello, ready, state, act and forfeit ---- */
/* session.event: What the last next() read. */
CY_API int32_t cy_session_event(cy_robot *robot, int32_t *out);
/* session.next: Reads the next message and decodes it. */
CY_API int32_t cy_session_next(cy_robot *robot, int32_t *out);
/*
 * session.send: Writes the ready reply after a hello, or this tick's act after
 * a state, and clears the pending act.
 */
CY_API int32_t cy_session_send(cy_robot *robot);
/* session.forfeit: Writes a forfeit in place of this tick's act. */
CY_API int32_t cy_session_forfeit(cy_robot *robot, const char *code, uint32_t code_length, const char *message, uint32_t message_length);
/* session.log: Writes a line to standard error, never to the protocol. */
CY_API int32_t cy_session_log(cy_robot *robot, const char *line, uint32_t line_length);
/*
 * session.feed: Hands the memory port a line, for tests and event-loop
 * runtimes.
 */
CY_API int32_t cy_session_feed(cy_robot *robot, const char *line, uint32_t line_length);
/*
 * session.reply: Takes the memory port's next written line, if any. CY_NONE
 * when absent.
 */
CY_API int32_t cy_session_reply(cy_robot *robot, char *buf, uint32_t capacity, uint32_t *length);

/* ---- the DSL words: outside the robot API, for the DSL interpreters and -wasm builds ---- */
/* One double per word (booleans 1 or 0, enums their CY_ value), in protocol/v2/api.json's word order. 776 bytes. */
typedef struct cy_dsl_words_v1 {
  double ruleset; /* ruleset */
  double physics_mode; /* physics-mode */
  double zero_g; /* zero-g */
  double control_model; /* control-model */
  double inertial; /* inertial */
  double mode3; /* mode3 */
  double tick; /* tick */
  double time; /* time */
  double remaining; /* remaining */
  double width; /* width */
  double height; /* height */
  double depth; /* depth */
  double gravity_x; /* gravity-x */
  double gravity_y; /* gravity-y */
  double gravity_z; /* gravity-z */
  double terrain_kind; /* terrain-kind */
  double hills; /* hills */
  double health; /* health */
  double x; /* x */
  double y; /* y */
  double z; /* z */
  double vx; /* vx */
  double vy; /* vy */
  double vz; /* vz */
  double heading; /* heading */
  double pitch; /* pitch */
  double yaw_rate; /* yaw-rate */
  double pitch_rate; /* pitch-rate */
  double roll_rate; /* roll-rate */
  double forward_x; /* forward-x */
  double forward_y; /* forward-y */
  double forward_z; /* forward-z */
  double right_x; /* right-x */
  double right_y; /* right-y */
  double right_z; /* right-z */
  double up_x; /* up-x */
  double up_y; /* up-y */
  double up_z; /* up-z */
  double grounded; /* grounded */
  double altitude; /* altitude */
  double battery; /* battery */
  double battery_capacity; /* battery-capacity */
  double aim; /* aim */
  double aim_pitch; /* aim-pitch */
  double cooldown; /* cooldown */
  double scan_heading; /* scan-heading */
  double scan_pitch; /* scan-pitch */
  double scan_arc; /* scan-arc */
  double scan_range; /* scan-range */
  double contact; /* contact */
  double range; /* range */
  double bearing; /* bearing */
  double elevation; /* elevation */
  double cue_present; /* cue-present */
  double cue_error; /* cue-error */
  double bearing_rate; /* bearing-rate */
  double zone_active; /* zone-active */
  double zone_radius; /* zone-radius */
  double storm_active; /* storm-active */
  double storm_radius; /* storm-radius */
  double hill_active; /* hill-active */
  double hill_radius; /* hill-radius */
  double score; /* score */
  double score_best; /* score-best */
  double speed; /* speed */
  double gravity; /* gravity */
  double terrain_height; /* terrain-height */
  double battery_fraction; /* battery-fraction */
  double aim_error; /* aim-error */
  double power_min; /* power-min */
  double power_max; /* power-max */
  double shot_speed_base; /* shot-speed-base */
  double shot_speed_per_power; /* shot-speed-per-power */
  double shot_cost_base; /* shot-cost-base */
  double shot_cost_per_power; /* shot-cost-per-power */
  double zone_distance; /* zone-distance */
  double zone_bearing; /* zone-bearing */
  double zone_elevation; /* zone-elevation */
  double storm_distance; /* storm-distance */
  double storm_bearing; /* storm-bearing */
  double storm_heading; /* storm-heading */
  double cue_muzzle; /* cue-muzzle */
  double cue_bearing; /* cue-bearing */
  double cue_elevation; /* cue-elevation */
  double cue_range; /* cue-range */
  double cue_age; /* cue-age */
  double pickup_present; /* pickup-present */
  double pickup_bearing; /* pickup-bearing */
  double pickup_elevation; /* pickup-elevation */
  double pickup_range; /* pickup-range */
  double on_hill; /* on-hill */
  double hill_held; /* hill-held */
  double hill_bearing; /* hill-bearing */
  double hill_elevation; /* hill-elevation */
  double hill_range; /* hill-range */
  double hill_time; /* hill-time */
  double hill_to_win; /* hill-to-win */
} cy_dsl_words_v1;
CY_API int32_t cy_dsl_fill_v1(cy_robot *robot, cy_dsl_words_v1 *out);

/* Source-level names: always the latest version. */
typedef cy_attitude_v1 cy_attitude;
typedef cy_applied_v1 cy_applied;
typedef cy_scan_v1 cy_scan;
typedef cy_contact_v1 cy_contact;
typedef cy_cue_v1 cy_cue;
typedef cy_solution_v1 cy_solution;
typedef cy_zone_v1 cy_zone;
typedef cy_storm_v1 cy_storm;
typedef cy_hill_v1 cy_hill;
typedef cy_pickup_v1 cy_pickup;
typedef cy_compute_v1 cy_compute;
#define cy_me_attitude cy_me_attitude_v1
#define cy_me_applied cy_me_applied_v1
#define cy_battery_compute cy_battery_compute_v1
#define cy_gun_lead cy_gun_lead_v1
#define cy_gun_lead_point cy_gun_lead_point_v1
#define cy_scanner_last cy_scanner_last_v1
#define cy_sensors_contacts cy_sensors_contacts_v1
#define cy_sensors_nearest cy_sensors_nearest_v1
#define cy_sensors_cues cy_sensors_cues_v1
#define cy_sensors_contact cy_sensors_contact_v1
#define cy_sensors_position_of cy_sensors_position_of_v1
#define cy_tracker_cue cy_tracker_cue_v1
#define cy_nav_zone cy_nav_zone_v1
#define cy_nav_storm cy_nav_storm_v1
#define cy_nav_hill cy_nav_hill_v1
#define cy_nav_pickups cy_nav_pickups_v1
#define cy_nav_nearest_pickup cy_nav_nearest_pickup_v1

#ifdef __cplusplus
}
#endif

#endif
