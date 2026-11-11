/* Generated from protocol/v2/api.json by protocol/v2/codegen/c.js; do not edit. Run `make api`.
 * The C ABI's layout and symbols: it fails to compile when a record's size
 * or an offset differs from api.json, and to link when an export is missing.
 * Run it linked with libclunkyard: it prints the count of symbols it found.
 */
#include <stddef.h>
#include <stdio.h>
#include "clunkyard.h"

_Static_assert(sizeof(cy_vec3) == 24, "cy_vec3 size");
_Static_assert(offsetof(cy_vec3, x) == 0, "cy_vec3.x");
_Static_assert(offsetof(cy_vec3, y) == 8, "cy_vec3.y");
_Static_assert(offsetof(cy_vec3, z) == 16, "cy_vec3.z");
_Static_assert(sizeof(cy_rotation) == 24, "cy_rotation size");
_Static_assert(offsetof(cy_rotation, yaw) == 0, "cy_rotation.yaw");
_Static_assert(offsetof(cy_rotation, pitch) == 8, "cy_rotation.pitch");
_Static_assert(offsetof(cy_rotation, roll) == 16, "cy_rotation.roll");
_Static_assert(sizeof(cy_direction) == 16, "cy_direction size");
_Static_assert(offsetof(cy_direction, heading) == 0, "cy_direction.heading");
_Static_assert(offsetof(cy_direction, pitch) == 8, "cy_direction.pitch");
_Static_assert(sizeof(cy_attitude_v1) == 72, "cy_attitude_v1 size");
_Static_assert(offsetof(cy_attitude_v1, forward) == 0, "cy_attitude_v1.forward");
_Static_assert(offsetof(cy_attitude_v1, right) == 24, "cy_attitude_v1.right");
_Static_assert(offsetof(cy_attitude_v1, up) == 48, "cy_attitude_v1.up");
_Static_assert(sizeof(cy_applied_v1) == 56, "cy_applied_v1 size");
_Static_assert(offsetof(cy_applied_v1, force) == 0, "cy_applied_v1.force");
_Static_assert(offsetof(cy_applied_v1, torque) == 24, "cy_applied_v1.torque");
_Static_assert(offsetof(cy_applied_v1, brake) == 48, "cy_applied_v1.brake");
_Static_assert(sizeof(cy_scan_v1) == 32, "cy_scan_v1 size");
_Static_assert(offsetof(cy_scan_v1, heading) == 0, "cy_scan_v1.heading");
_Static_assert(offsetof(cy_scan_v1, pitch) == 8, "cy_scan_v1.pitch");
_Static_assert(offsetof(cy_scan_v1, arc) == 16, "cy_scan_v1.arc");
_Static_assert(offsetof(cy_scan_v1, range) == 24, "cy_scan_v1.range");
_Static_assert(sizeof(cy_contact_v1) == 72, "cy_contact_v1 size");
_Static_assert(offsetof(cy_contact_v1, slot) == 0, "cy_contact_v1.slot");
_Static_assert(offsetof(cy_contact_v1, source) == 4, "cy_contact_v1.source");
_Static_assert(offsetof(cy_contact_v1, range) == 8, "cy_contact_v1.range");
_Static_assert(offsetof(cy_contact_v1, bearing) == 16, "cy_contact_v1.bearing");
_Static_assert(offsetof(cy_contact_v1, elevation) == 24, "cy_contact_v1.elevation");
_Static_assert(offsetof(cy_contact_v1, bearing_error) == 32, "cy_contact_v1.bearing_error");
_Static_assert(offsetof(cy_contact_v1, range_error) == 40, "cy_contact_v1.range_error");
_Static_assert(offsetof(cy_contact_v1, position) == 48, "cy_contact_v1.position");
_Static_assert(sizeof(cy_cue_v1) == 40, "cy_cue_v1 size");
_Static_assert(offsetof(cy_cue_v1, source) == 0, "cy_cue_v1.source");
_Static_assert(offsetof(cy_cue_v1, tick) == 4, "cy_cue_v1.tick");
_Static_assert(offsetof(cy_cue_v1, bearing_error) == 8, "cy_cue_v1.bearing_error");
_Static_assert(offsetof(cy_cue_v1, position) == 16, "cy_cue_v1.position");
_Static_assert(sizeof(cy_solution_v1) == 24, "cy_solution_v1 size");
_Static_assert(offsetof(cy_solution_v1, heading) == 0, "cy_solution_v1.heading");
_Static_assert(offsetof(cy_solution_v1, pitch) == 8, "cy_solution_v1.pitch");
_Static_assert(offsetof(cy_solution_v1, time) == 16, "cy_solution_v1.time");
_Static_assert(sizeof(cy_zone_v1) == 40, "cy_zone_v1 size");
_Static_assert(offsetof(cy_zone_v1, shape) == 0, "cy_zone_v1.shape");
_Static_assert(offsetof(cy_zone_v1, reserved) == 4, "cy_zone_v1.reserved");
_Static_assert(offsetof(cy_zone_v1, radius) == 8, "cy_zone_v1.radius");
_Static_assert(offsetof(cy_zone_v1, center) == 16, "cy_zone_v1.center");
_Static_assert(sizeof(cy_storm_v1) == 72, "cy_storm_v1 size");
_Static_assert(offsetof(cy_storm_v1, shape) == 0, "cy_storm_v1.shape");
_Static_assert(offsetof(cy_storm_v1, reserved) == 4, "cy_storm_v1.reserved");
_Static_assert(offsetof(cy_storm_v1, radius) == 8, "cy_storm_v1.radius");
_Static_assert(offsetof(cy_storm_v1, dps) == 16, "cy_storm_v1.dps");
_Static_assert(offsetof(cy_storm_v1, center) == 24, "cy_storm_v1.center");
_Static_assert(offsetof(cy_storm_v1, velocity) == 48, "cy_storm_v1.velocity");
_Static_assert(sizeof(cy_hill_v1) == 80, "cy_hill_v1 size");
_Static_assert(offsetof(cy_hill_v1, shape) == 0, "cy_hill_v1.shape");
_Static_assert(offsetof(cy_hill_v1, holder) == 4, "cy_hill_v1.holder");
_Static_assert(offsetof(cy_hill_v1, contested) == 8, "cy_hill_v1.contested");
_Static_assert(offsetof(cy_hill_v1, moves_tick) == 12, "cy_hill_v1.moves_tick");
_Static_assert(offsetof(cy_hill_v1, held_ticks) == 16, "cy_hill_v1.held_ticks");
_Static_assert(offsetof(cy_hill_v1, best_held_ticks) == 20, "cy_hill_v1.best_held_ticks");
_Static_assert(offsetof(cy_hill_v1, radius) == 24, "cy_hill_v1.radius");
_Static_assert(offsetof(cy_hill_v1, center) == 32, "cy_hill_v1.center");
_Static_assert(offsetof(cy_hill_v1, next) == 56, "cy_hill_v1.next");
_Static_assert(sizeof(cy_pickup_v1) == 40, "cy_pickup_v1 size");
_Static_assert(offsetof(cy_pickup_v1, kind) == 0, "cy_pickup_v1.kind");
_Static_assert(offsetof(cy_pickup_v1, expires_tick) == 4, "cy_pickup_v1.expires_tick");
_Static_assert(offsetof(cy_pickup_v1, amount) == 8, "cy_pickup_v1.amount");
_Static_assert(offsetof(cy_pickup_v1, position) == 16, "cy_pickup_v1.position");
_Static_assert(sizeof(cy_compute_v1) == 16, "cy_compute_v1 size");
_Static_assert(offsetof(cy_compute_v1, k) == 0, "cy_compute_v1.k");
_Static_assert(offsetof(cy_compute_v1, quanta) == 8, "cy_compute_v1.quanta");
_Static_assert(offsetof(cy_compute_v1, unit) == 12, "cy_compute_v1.unit");
_Static_assert(sizeof(cy_dsl_words_v1) == 776, "cy_dsl_words_v1 size");

typedef void (*cy_any_fn)(void);
static const cy_any_fn SYMBOLS[] = {
  (cy_any_fn)cy_version,
  (cy_any_fn)cy_open,
  (cy_any_fn)cy_open_memory,
  (cy_any_fn)cy_close,
  (cy_any_fn)cy_last_error,
  (cy_any_fn)cy_match_mode,
  (cy_any_fn)cy_match_ruleset,
  (cy_any_fn)cy_match_control,
  (cy_any_fn)cy_match_robots,
  (cy_any_fn)cy_match_is_3d,
  (cy_any_fn)cy_match_tick_rate,
  (cy_any_fn)cy_match_max_ticks,
  (cy_any_fn)cy_robot_id,
  (cy_any_fn)cy_clock_tick,
  (cy_any_fn)cy_clock_seconds,
  (cy_any_fn)cy_clock_remaining_ticks,
  (cy_any_fn)cy_arena_size,
  (cy_any_fn)cy_arena_gravity,
  (cy_any_fn)cy_arena_terrain_kind,
  (cy_any_fn)cy_arena_ground_height,
  (cy_any_fn)cy_me_health,
  (cy_any_fn)cy_me_position,
  (cy_any_fn)cy_me_velocity,
  (cy_any_fn)cy_me_heading,
  (cy_any_fn)cy_me_pitch,
  (cy_any_fn)cy_me_roll,
  (cy_any_fn)cy_me_angular_velocity,
  (cy_any_fn)cy_me_attitude_v1,
  (cy_any_fn)cy_me_grounded,
  (cy_any_fn)cy_me_altitude,
  (cy_any_fn)cy_me_applied_v1,
  (cy_any_fn)cy_battery_charge,
  (cy_any_fn)cy_battery_capacity,
  (cy_any_fn)cy_battery_regen,
  (cy_any_fn)cy_battery_sustainable_drive,
  (cy_any_fn)cy_battery_compute_v1,
  (cy_any_fn)cy_battery_drive_cost,
  (cy_any_fn)cy_drive_hover_lift,
  (cy_any_fn)cy_drive_throttle,
  (cy_any_fn)cy_drive_turn,
  (cy_any_fn)cy_drive_pitch_turn,
  (cy_any_fn)cy_drive_thrust,
  (cy_any_fn)cy_drive_strafe,
  (cy_any_fn)cy_drive_lift,
  (cy_any_fn)cy_drive_yaw_torque,
  (cy_any_fn)cy_drive_pitch_torque,
  (cy_any_fn)cy_drive_roll_torque,
  (cy_any_fn)cy_drive_brake,
  (cy_any_fn)cy_drive_get_throttle,
  (cy_any_fn)cy_drive_get_turn,
  (cy_any_fn)cy_drive_get_pitch_turn,
  (cy_any_fn)cy_drive_get_thrust,
  (cy_any_fn)cy_drive_get_strafe,
  (cy_any_fn)cy_drive_get_lift,
  (cy_any_fn)cy_drive_get_yaw_torque,
  (cy_any_fn)cy_drive_get_pitch_torque,
  (cy_any_fn)cy_drive_get_roll_torque,
  (cy_any_fn)cy_drive_get_brake,
  (cy_any_fn)cy_gun_aim_now,
  (cy_any_fn)cy_gun_cooldown_ticks,
  (cy_any_fn)cy_gun_spread,
  (cy_any_fn)cy_gun_max_power,
  (cy_any_fn)cy_gun_damage_scale,
  (cy_any_fn)cy_gun_can_fire,
  (cy_any_fn)cy_gun_shot_speed,
  (cy_any_fn)cy_gun_shot_cost,
  (cy_any_fn)cy_gun_reload_ticks,
  (cy_any_fn)cy_gun_shot_damage,
  (cy_any_fn)cy_gun_power_for_speed,
  (cy_any_fn)cy_gun_power_for_flight_time,
  (cy_any_fn)cy_gun_lead_v1,
  (cy_any_fn)cy_gun_lead_point_v1,
  (cy_any_fn)cy_gun_aim_heading,
  (cy_any_fn)cy_gun_aim_pitch,
  (cy_any_fn)cy_gun_fire,
  (cy_any_fn)cy_gun_power,
  (cy_any_fn)cy_gun_get_aim_heading,
  (cy_any_fn)cy_gun_get_aim_pitch,
  (cy_any_fn)cy_gun_get_fire,
  (cy_any_fn)cy_gun_get_power,
  (cy_any_fn)cy_scanner_last_v1,
  (cy_any_fn)cy_scanner_proximity_range,
  (cy_any_fn)cy_scanner_reach,
  (cy_any_fn)cy_scanner_arc_for_reach,
  (cy_any_fn)cy_scanner_heading,
  (cy_any_fn)cy_scanner_arc,
  (cy_any_fn)cy_scanner_pitch,
  (cy_any_fn)cy_scanner_get_heading,
  (cy_any_fn)cy_scanner_get_arc,
  (cy_any_fn)cy_scanner_get_pitch,
  (cy_any_fn)cy_sensors_contacts_v1,
  (cy_any_fn)cy_sensors_nearest_v1,
  (cy_any_fn)cy_sensors_cues_v1,
  (cy_any_fn)cy_sensors_contact_v1,
  (cy_any_fn)cy_sensors_position_of_v1,
  (cy_any_fn)cy_tracker_cue_v1,
  (cy_any_fn)cy_tracker_nearest_bearing_rate,
  (cy_any_fn)cy_tracker_velocity,
  (cy_any_fn)cy_tracker_position,
  (cy_any_fn)cy_tracker_last_seen,
  (cy_any_fn)cy_tracker_configure,
  (cy_any_fn)cy_nav_zone_v1,
  (cy_any_fn)cy_nav_zone_margin,
  (cy_any_fn)cy_nav_zone_contains,
  (cy_any_fn)cy_nav_zone_distance_from,
  (cy_any_fn)cy_nav_storm_v1,
  (cy_any_fn)cy_nav_storm_margin,
  (cy_any_fn)cy_nav_storm_contains,
  (cy_any_fn)cy_nav_storm_distance_from,
  (cy_any_fn)cy_nav_hill_v1,
  (cy_any_fn)cy_nav_hill_margin,
  (cy_any_fn)cy_nav_hill_contains,
  (cy_any_fn)cy_nav_hill_distance_from,
  (cy_any_fn)cy_nav_pickups_v1,
  (cy_any_fn)cy_nav_nearest_pickup_v1,
  (cy_any_fn)cy_nav_field_distance,
  (cy_any_fn)cy_score_me,
  (cy_any_fn)cy_score_best,
  (cy_any_fn)cy_score_armour,
  (cy_any_fn)cy_angles_normalize,
  (cy_any_fn)cy_angles_angle_diff,
  (cy_any_fn)cy_angles_bearing_to,
  (cy_any_fn)cy_angles_elevation_to,
  (cy_any_fn)cy_angles_direction,
  (cy_any_fn)cy_angles_distance,
  (cy_any_fn)cy_angles_clamp,
  (cy_any_fn)cy_status_text,
  (cy_any_fn)cy_status_value,
  (cy_any_fn)cy_status_get_text,
  (cy_any_fn)cy_status_get_value,
  (cy_any_fn)cy_raw_number,
  (cy_any_fn)cy_raw_text,
  (cy_any_fn)cy_raw_has,
  (cy_any_fn)cy_raw_flag,
  (cy_any_fn)cy_raw_count,
  (cy_any_fn)cy_raw_line,
  (cy_any_fn)cy_raw_extra,
  (cy_any_fn)cy_session_event,
  (cy_any_fn)cy_session_next,
  (cy_any_fn)cy_session_send,
  (cy_any_fn)cy_session_forfeit,
  (cy_any_fn)cy_session_log,
  (cy_any_fn)cy_session_feed,
  (cy_any_fn)cy_session_reply,
  (cy_any_fn)cy_dsl_fill_v1,
};

int main(void) {
  size_t found = 0;
  for (size_t i = 0; i < sizeof SYMBOLS / sizeof SYMBOLS[0]; i += 1) found += SYMBOLS[i] != NULL;
  printf("%zu of %zu symbols\n", found, sizeof SYMBOLS / sizeof SYMBOLS[0]);
  return found == sizeof SYMBOLS / sizeof SYMBOLS[0] ? 0 : 1;
}
