// Generated from protocol/v2/api.json; do not edit. Run make api.
#pragma once
#include "clunkyard.h"
#include <optional>
#include <vector>
#include <string>
#include <string_view>
#include <stdexcept>
#include <cstdio>
#include <functional>
namespace clunkyard {
class Error:public std::runtime_error {public:int32_t status;Error(int32_t code,const std::string &message):std::runtime_error(message),status(code){}};
inline void check(cy_robot *robot,int32_t status){if(status<0){char message[4096];uint32_t length=0;cy_last_error(robot,message,sizeof message,&length);throw Error(status,std::string(message,length));}};
using Vec3 = cy_vec3;
using Rotation = cy_rotation;
using Direction = cy_direction;
using Attitude = cy_attitude_v1;
using Applied = cy_applied_v1;
using Hurt = cy_hurt_v1;
using Scan = cy_scan_v1;
using Contact = cy_contact_v1;
using Cue = cy_cue_v1;
using Sighting = cy_sighting_v1;
using WallMargin = cy_wall_margin_v1;
using Solution = cy_solution_v1;
struct Zone:public cy_zone_v1{cy_robot *handle;Zone(cy_robot *h,const cy_zone_v1 &value):cy_zone_v1(value),handle(h){}
  std::optional<double> margin(const Vec3 & point) const { double out{}; int32_t status = cy_nav_zone_margin(handle, &point, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::optional<bool> contains(const Vec3 & point, double margin = 0) const { int32_t out{}; int32_t status = cy_nav_zone_contains(handle, &point, margin, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out != 0; }
  std::optional<double> distance_from(const Vec3 & point) const { double out{}; int32_t status = cy_nav_zone_distance_from(handle, &point, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
};
struct Storm:public cy_storm_v1{cy_robot *handle;Storm(cy_robot *h,const cy_storm_v1 &value):cy_storm_v1(value),handle(h){}
  std::optional<double> margin(const Vec3 & point) const { double out{}; int32_t status = cy_nav_storm_margin(handle, &point, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::optional<bool> contains(const Vec3 & point, double margin = 0) const { int32_t out{}; int32_t status = cy_nav_storm_contains(handle, &point, margin, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out != 0; }
  std::optional<double> distance_from(const Vec3 & point) const { double out{}; int32_t status = cy_nav_storm_distance_from(handle, &point, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
};
struct Hill:public cy_hill_v1{cy_robot *handle;Hill(cy_robot *h,const cy_hill_v1 &value):cy_hill_v1(value),handle(h){}
  std::optional<double> margin(const Vec3 & point) const { double out{}; int32_t status = cy_nav_hill_margin(handle, &point, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::optional<bool> contains(const Vec3 & point, double margin = 0) const { int32_t out{}; int32_t status = cy_nav_hill_contains(handle, &point, margin, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out != 0; }
  std::optional<double> distance_from(const Vec3 & point) const { double out{}; int32_t status = cy_nav_hill_distance_from(handle, &point, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
};
using Pickup = cy_pickup_v1;
using Report = cy_report_v1;
using DroneSighting = cy_drone_sighting_v1;
using Compute = cy_compute_v1;
struct Match {cy_robot *&handle; explicit Match(cy_robot *&robot):handle(robot){}
  int32_t mode() const { int32_t out{}; int32_t status = cy_match_mode(handle, &out); check(handle,status); return out; }
  int32_t ruleset() const { int32_t out{}; int32_t status = cy_match_ruleset(handle, &out); check(handle,status); return out; }
  int32_t control() const { int32_t out{}; int32_t status = cy_match_control(handle, &out); check(handle,status); return out; }
  int32_t robots() const { int32_t out{}; int32_t status = cy_match_robots(handle, &out); check(handle,status); return out; }
  bool is_3d() const { int32_t out{}; int32_t status = cy_match_is_3d(handle, &out); check(handle,status); return out != 0; }
  int32_t tick_rate() const { int32_t out{}; int32_t status = cy_match_tick_rate(handle, &out); check(handle,status); return out; }
  int32_t max_ticks() const { int32_t out{}; int32_t status = cy_match_max_ticks(handle, &out); check(handle,status); return out; }
  std::string robot_id(int32_t slot) const { std::string out(256,'\0');uint32_t count=0; int32_t status = cy_robot_id(handle, slot, out.data(), (uint32_t)out.size(), &count); while(status==CY_SHORT){out.resize(count);status=cy_robot_id(handle, slot, out.data(), (uint32_t)out.size(), &count);} check(handle,status); out.resize(count);return out; }
};
struct Clock {cy_robot *&handle; explicit Clock(cy_robot *&robot):handle(robot){}
  int32_t tick() const { int32_t out{}; int32_t status = cy_clock_tick(handle, &out); check(handle,status); return out; }
  double seconds() const { double out{}; int32_t status = cy_clock_seconds(handle, &out); check(handle,status); return out; }
  int32_t remaining_ticks() const { int32_t out{}; int32_t status = cy_clock_remaining_ticks(handle, &out); check(handle,status); return out; }
};
struct Arena {cy_robot *&handle; explicit Arena(cy_robot *&robot):handle(robot){}
  Vec3 size() const { cy_vec3 out{}; int32_t status = cy_arena_size(handle, &out); check(handle,status); return out; }
  Vec3 gravity() const { cy_vec3 out{}; int32_t status = cy_arena_gravity(handle, &out); check(handle,status); return out; }
  int32_t terrain_kind() const { int32_t out{}; int32_t status = cy_arena_terrain_kind(handle, &out); check(handle,status); return out; }
  double ground_height(double x, double y) const { double out{}; int32_t status = cy_arena_ground_height(handle, x, y, &out); check(handle,status); return out; }
};
struct Me {cy_robot *&handle; explicit Me(cy_robot *&robot):handle(robot){}
  double health() const { double out{}; int32_t status = cy_me_health(handle, &out); check(handle,status); return out; }
  Vec3 position() const { cy_vec3 out{}; int32_t status = cy_me_position(handle, &out); check(handle,status); return out; }
  Vec3 velocity() const { cy_vec3 out{}; int32_t status = cy_me_velocity(handle, &out); check(handle,status); return out; }
  double heading() const { double out{}; int32_t status = cy_me_heading(handle, &out); check(handle,status); return out; }
  double pitch() const { double out{}; int32_t status = cy_me_pitch(handle, &out); check(handle,status); return out; }
  double roll() const { double out{}; int32_t status = cy_me_roll(handle, &out); check(handle,status); return out; }
  Rotation angular_velocity() const { cy_rotation out{}; int32_t status = cy_me_angular_velocity(handle, &out); check(handle,status); return out; }
  Attitude attitude() const { cy_attitude_v1 out{}; int32_t status = cy_me_attitude_v1(handle, &out); check(handle,status); return out; }
  bool grounded() const { int32_t out{}; int32_t status = cy_me_grounded(handle, &out); check(handle,status); return out != 0; }
  double altitude() const { double out{}; int32_t status = cy_me_altitude(handle, &out); check(handle,status); return out; }
  Vec3 wall_contact() const { cy_vec3 out{}; int32_t status = cy_me_wall_contact(handle, &out); check(handle,status); return out; }
  Hurt hurt() const { cy_hurt_v1 out{}; int32_t status = cy_me_hurt_v1(handle, &out); check(handle,status); return out; }
  Applied applied() const { cy_applied_v1 out{}; int32_t status = cy_me_applied_v1(handle, &out); check(handle,status); return out; }
  bool hull_mounts() const { int32_t out{}; int32_t status = cy_me_hull_mounts(handle, &out); check(handle,status); return out != 0; }
  Direction to_mount(double heading, double pitch) const { cy_direction out{}; int32_t status = cy_me_to_mount(handle, heading, pitch, &out); check(handle,status); return out; }
  Direction from_mount(double heading, double pitch) const { cy_direction out{}; int32_t status = cy_me_from_mount(handle, heading, pitch, &out); check(handle,status); return out; }
  Direction mount_toward(const Vec3 & point) const { cy_direction out{}; int32_t status = cy_me_mount_toward(handle, &point, &out); check(handle,status); return out; }
};
struct Battery {cy_robot *&handle; explicit Battery(cy_robot *&robot):handle(robot){}
  double charge() const { double out{}; int32_t status = cy_battery_charge(handle, &out); check(handle,status); return out; }
  double capacity() const { double out{}; int32_t status = cy_battery_capacity(handle, &out); check(handle,status); return out; }
  double regen() const { double out{}; int32_t status = cy_battery_regen(handle, &out); check(handle,status); return out; }
  double sustainable_drive() const { double out{}; int32_t status = cy_battery_sustainable_drive(handle, &out); check(handle,status); return out; }
  std::optional<Compute> compute() const { cy_compute_v1 out{}; int32_t status = cy_battery_compute_v1(handle, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  double drive_cost(double fraction) const { double out{}; int32_t status = cy_battery_drive_cost(handle, fraction, &out); check(handle,status); return out; }
};
struct Drive {cy_robot *&handle; explicit Drive(cy_robot *&robot):handle(robot){}
  double hover_lift() const { double out{}; int32_t status = cy_drive_hover_lift(handle, &out); check(handle,status); return out; }
  void set(std::optional<double> throttle = std::nullopt, std::optional<double> turn = std::nullopt, std::optional<double> pitch_turn = std::nullopt) {if(throttle)check(handle,cy_drive_throttle(handle,*throttle));if(turn)check(handle,cy_drive_turn(handle,*turn));if(pitch_turn)check(handle,cy_drive_pitch_turn(handle,*pitch_turn));}
  void forces(std::optional<double> thrust = std::nullopt, std::optional<double> strafe = std::nullopt, std::optional<double> lift = std::nullopt, std::optional<double> yaw_torque = std::nullopt, std::optional<double> pitch_torque = std::nullopt, std::optional<double> roll_torque = std::nullopt, std::optional<double> brake = std::nullopt) {if(thrust)check(handle,cy_drive_thrust(handle,*thrust));if(strafe)check(handle,cy_drive_strafe(handle,*strafe));if(lift)check(handle,cy_drive_lift(handle,*lift));if(yaw_torque)check(handle,cy_drive_yaw_torque(handle,*yaw_torque));if(pitch_torque)check(handle,cy_drive_pitch_torque(handle,*pitch_torque));if(roll_torque)check(handle,cy_drive_roll_torque(handle,*roll_torque));if(brake)check(handle,cy_drive_brake(handle,*brake));}
  std::optional<double> get_throttle() const { double out{}; int32_t status = cy_drive_get_throttle(handle, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::optional<double> get_turn() const { double out{}; int32_t status = cy_drive_get_turn(handle, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::optional<double> get_pitch_turn() const { double out{}; int32_t status = cy_drive_get_pitch_turn(handle, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::optional<double> get_thrust() const { double out{}; int32_t status = cy_drive_get_thrust(handle, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::optional<double> get_strafe() const { double out{}; int32_t status = cy_drive_get_strafe(handle, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::optional<double> get_lift() const { double out{}; int32_t status = cy_drive_get_lift(handle, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::optional<double> get_yaw_torque() const { double out{}; int32_t status = cy_drive_get_yaw_torque(handle, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::optional<double> get_pitch_torque() const { double out{}; int32_t status = cy_drive_get_pitch_torque(handle, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::optional<double> get_roll_torque() const { double out{}; int32_t status = cy_drive_get_roll_torque(handle, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::optional<double> get_brake() const { double out{}; int32_t status = cy_drive_get_brake(handle, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
};
struct Gun {cy_robot *&handle; explicit Gun(cy_robot *&robot):handle(robot){}
  Direction aim_now() const { cy_direction out{}; int32_t status = cy_gun_aim_now(handle, &out); check(handle,status); return out; }
  int32_t cooldown_ticks() const { int32_t out{}; int32_t status = cy_gun_cooldown_ticks(handle, &out); check(handle,status); return out; }
  double spread() const { double out{}; int32_t status = cy_gun_spread(handle, &out); check(handle,status); return out; }
  double max_power() const { double out{}; int32_t status = cy_gun_max_power(handle, &out); check(handle,status); return out; }
  double damage_scale() const { double out{}; int32_t status = cy_gun_damage_scale(handle, &out); check(handle,status); return out; }
  bool can_fire(double power, double reserve) const { int32_t out{}; int32_t status = cy_gun_can_fire(handle, power, reserve, &out); check(handle,status); return out != 0; }
  double shot_speed(double power) const { double out{}; int32_t status = cy_gun_shot_speed(handle, power, &out); check(handle,status); return out; }
  double shot_cost(double power) const { double out{}; int32_t status = cy_gun_shot_cost(handle, power, &out); check(handle,status); return out; }
  double reload_ticks(double power) const { double out{}; int32_t status = cy_gun_reload_ticks(handle, power, &out); check(handle,status); return out; }
  double shot_damage(double power) const { double out{}; int32_t status = cy_gun_shot_damage(handle, power, &out); check(handle,status); return out; }
  double power_for_speed(double speed) const { double out{}; int32_t status = cy_gun_power_for_speed(handle, speed, &out); check(handle,status); return out; }
  double power_for_flight_time(double range, double seconds) const { double out{}; int32_t status = cy_gun_power_for_flight_time(handle, range, seconds, &out); check(handle,status); return out; }
  std::optional<Solution> lead(const Contact & contact, double power) const { cy_solution_v1 out{}; int32_t status = cy_gun_lead_v1(handle, &contact, power, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::optional<Solution> lead_point(const Vec3 & position, const Vec3 & velocity, double power) const { cy_solution_v1 out{}; int32_t status = cy_gun_lead_point_v1(handle, &position, &velocity, power, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  void aim(std::optional<double> heading = std::nullopt, std::optional<double> pitch = std::nullopt) {if(heading)check(handle,cy_gun_aim_heading(handle,*heading));if(pitch)check(handle,cy_gun_aim_pitch(handle,*pitch));}
  void fire(std::optional<double> power = std::nullopt) {check(handle,cy_gun_fire(handle));if(power)check(handle,cy_gun_power(handle,*power));}
  std::optional<double> get_heading() const { double out{}; int32_t status = cy_gun_get_aim_heading(handle, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::optional<double> get_pitch() const { double out{}; int32_t status = cy_gun_get_aim_pitch(handle, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::optional<double> get_fire() const { double out{}; int32_t status = cy_gun_get_fire(handle, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::optional<double> get_power() const { double out{}; int32_t status = cy_gun_get_power(handle, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
};
struct Scanner {cy_robot *&handle; explicit Scanner(cy_robot *&robot):handle(robot){}
  Scan last() const { cy_scan_v1 out{}; int32_t status = cy_scanner_last_v1(handle, &out); check(handle,status); return out; }
  double proximity_range() const { double out{}; int32_t status = cy_scanner_proximity_range(handle, &out); check(handle,status); return out; }
  double reach(double arc) const { double out{}; int32_t status = cy_scanner_reach(handle, arc, &out); check(handle,status); return out; }
  double arc_for_reach(double range) const { double out{}; int32_t status = cy_scanner_arc_for_reach(handle, range, &out); check(handle,status); return out; }
  void set(std::optional<double> heading = std::nullopt, std::optional<double> arc = std::nullopt, std::optional<double> pitch = std::nullopt) {if(heading)check(handle,cy_scanner_heading(handle,*heading));if(arc)check(handle,cy_scanner_arc(handle,*arc));if(pitch)check(handle,cy_scanner_pitch(handle,*pitch));}
  bool is_on() const { int32_t out{}; int32_t status = cy_scanner_is_on(handle, &out); check(handle,status); return out != 0; }
  void power(std::optional<double> on = std::nullopt) {if(on)check(handle,cy_scanner_on(handle,*on));}
  std::optional<double> get_heading() const { double out{}; int32_t status = cy_scanner_get_heading(handle, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::optional<double> get_arc() const { double out{}; int32_t status = cy_scanner_get_arc(handle, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::optional<double> get_pitch() const { double out{}; int32_t status = cy_scanner_get_pitch(handle, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::optional<double> get_on() const { double out{}; int32_t status = cy_scanner_get_on(handle, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
};
struct Sensors {cy_robot *&handle; explicit Sensors(cy_robot *&robot):handle(robot){}
  std::vector<Contact> contacts() const { std::vector<cy_contact_v1> out(16);uint32_t count=0; int32_t status = cy_sensors_contacts_v1(handle, out.data(), (uint32_t)out.size(), &count); while(status==CY_SHORT){out.resize(count);status=cy_sensors_contacts_v1(handle, out.data(), (uint32_t)out.size(), &count);} check(handle,status); out.resize(count);return out; }
  std::optional<Contact> nearest() const { cy_contact_v1 out{}; int32_t status = cy_sensors_nearest_v1(handle, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::vector<Contact> cues() const { std::vector<cy_contact_v1> out(16);uint32_t count=0; int32_t status = cy_sensors_cues_v1(handle, out.data(), (uint32_t)out.size(), &count); while(status==CY_SHORT){out.resize(count);status=cy_sensors_cues_v1(handle, out.data(), (uint32_t)out.size(), &count);} check(handle,status); out.resize(count);return out; }
  std::optional<Contact> contact(int32_t slot) const { cy_contact_v1 out{}; int32_t status = cy_sensors_contact_v1(handle, slot, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  Vec3 position_of(const Contact & contact) const { cy_vec3 out{}; int32_t status = cy_sensors_position_of_v1(handle, &contact, &out); check(handle,status); return out; }
};
struct Tracker {cy_robot *&handle; explicit Tracker(cy_robot *&robot):handle(robot){}
  std::optional<Cue> cue() const { cy_cue_v1 out{}; int32_t status = cy_tracker_cue_v1(handle, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  double nearest_bearing_rate() const { double out{}; int32_t status = cy_tracker_nearest_bearing_rate(handle, &out); check(handle,status); return out; }
  Vec3 velocity(int32_t slot) const { cy_vec3 out{}; int32_t status = cy_tracker_velocity(handle, slot, &out); check(handle,status); return out; }
  std::optional<Vec3> position(int32_t slot) const { cy_vec3 out{}; int32_t status = cy_tracker_position(handle, slot, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::vector<Sighting> history(int32_t slot) const { std::vector<cy_sighting_v1> out(16);uint32_t count=0; int32_t status = cy_tracker_history_v1(handle, slot, out.data(), (uint32_t)out.size(), &count); while(status==CY_SHORT){out.resize(count);status=cy_tracker_history_v1(handle, slot, out.data(), (uint32_t)out.size(), &count);} check(handle,status); out.resize(count);return out; }
  std::optional<int32_t> last_seen(int32_t slot) const { int32_t out{}; int32_t status = cy_tracker_last_seen(handle, slot, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  void configure(double smoothing = 0.5, int32_t forget_ticks = 40, int32_t cue_ticks = 60) { int32_t status = cy_tracker_configure(handle, smoothing, forget_ticks, cue_ticks); check(handle,status); }
};
struct Nav {cy_robot *&handle; explicit Nav(cy_robot *&robot):handle(robot){}
  std::optional<Zone> zone() const { cy_zone_v1 out{}; int32_t status = cy_nav_zone_v1(handle, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return Zone(handle,out); }
  std::optional<Storm> storm() const { cy_storm_v1 out{}; int32_t status = cy_nav_storm_v1(handle, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return Storm(handle,out); }
  std::optional<Hill> hill() const { cy_hill_v1 out{}; int32_t status = cy_nav_hill_v1(handle, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return Hill(handle,out); }
  WallMargin wall_margin() const { cy_wall_margin_v1 out{}; int32_t status = cy_nav_wall_margin_v1(handle, &out); check(handle,status); return out; }
  std::vector<Pickup> pickups() const { std::vector<cy_pickup_v1> out(16);uint32_t count=0; int32_t status = cy_nav_pickups_v1(handle, out.data(), (uint32_t)out.size(), &count); while(status==CY_SHORT){out.resize(count);status=cy_nav_pickups_v1(handle, out.data(), (uint32_t)out.size(), &count);} check(handle,status); out.resize(count);return out; }
  std::optional<Pickup> nearest_pickup(int32_t kind = 0) const { cy_pickup_v1 out{}; int32_t status = cy_nav_nearest_pickup_v1(handle, kind, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  double field_distance(const Vec3 & a, const Vec3 & b) const { double out{}; int32_t status = cy_nav_field_distance(handle, &a, &b, &out); check(handle,status); return out; }
  std::optional<double> zone_margin(const Vec3 & point) const { double out{}; int32_t status = cy_nav_zone_margin(handle, &point, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::optional<bool> zone_contains(const Vec3 & point, double margin = 0) const { int32_t out{}; int32_t status = cy_nav_zone_contains(handle, &point, margin, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out != 0; }
  std::optional<double> zone_distance_from(const Vec3 & point) const { double out{}; int32_t status = cy_nav_zone_distance_from(handle, &point, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::optional<double> storm_margin(const Vec3 & point) const { double out{}; int32_t status = cy_nav_storm_margin(handle, &point, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::optional<bool> storm_contains(const Vec3 & point, double margin = 0) const { int32_t out{}; int32_t status = cy_nav_storm_contains(handle, &point, margin, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out != 0; }
  std::optional<double> storm_distance_from(const Vec3 & point) const { double out{}; int32_t status = cy_nav_storm_distance_from(handle, &point, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::optional<double> hill_margin(const Vec3 & point) const { double out{}; int32_t status = cy_nav_hill_margin(handle, &point, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::optional<bool> hill_contains(const Vec3 & point, double margin = 0) const { int32_t out{}; int32_t status = cy_nav_hill_contains(handle, &point, margin, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out != 0; }
  std::optional<double> hill_distance_from(const Vec3 & point) const { double out{}; int32_t status = cy_nav_hill_distance_from(handle, &point, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
};
struct Score {cy_robot *&handle; explicit Score(cy_robot *&robot):handle(robot){}
  std::optional<double> me() const { double out{}; int32_t status = cy_score_me(handle, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::optional<double> best() const { double out{}; int32_t status = cy_score_best(handle, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::optional<double> armour() const { double out{}; int32_t status = cy_score_armour(handle, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
};
struct Drone {cy_robot *&handle; explicit Drone(cy_robot *&robot):handle(robot){}
  bool has() const { int32_t out{}; int32_t status = cy_drone_has(handle, &out); check(handle,status); return out != 0; }
  bool ready() const { int32_t out{}; int32_t status = cy_drone_ready(handle, &out); check(handle,status); return out != 0; }
  bool in_flight() const { int32_t out{}; int32_t status = cy_drone_in_flight(handle, &out); check(handle,status); return out != 0; }
  int32_t launches_left() const { int32_t out{}; int32_t status = cy_drone_launches_left(handle, &out); check(handle,status); return out; }
  std::optional<Report> report() const { cy_report_v1 out{}; int32_t status = cy_drone_report_v1(handle, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::vector<DroneSighting> sightings() const { std::vector<cy_drone_sighting_v1> out(16);uint32_t count=0; int32_t status = cy_drone_sightings_v1(handle, out.data(), (uint32_t)out.size(), &count); while(status==CY_SHORT){out.resize(count);status=cy_drone_sightings_v1(handle, out.data(), (uint32_t)out.size(), &count);} check(handle,status); out.resize(count);return out; }
  void launch(std::optional<double> heading = std::nullopt, std::optional<double> pitch = std::nullopt, std::optional<double> param = std::nullopt) {check(handle,cy_drone_launch(handle));if(heading)check(handle,cy_drone_launch_heading(handle,*heading));if(pitch)check(handle,cy_drone_launch_pitch(handle,*pitch));if(param)check(handle,cy_drone_launch_param(handle,*param));}
  void brief(int32_t index, double value) { int32_t status = cy_drone_brief(handle, index, value); check(handle,status); }
  void program(std::string_view language, std::string_view source) { int32_t status = cy_drone_program(handle, language.data(), (uint32_t)language.size(), source.data(), (uint32_t)source.size()); check(handle,status); }
  std::optional<double> get_launch() const { double out{}; int32_t status = cy_drone_get_launch(handle, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::optional<double> get_heading() const { double out{}; int32_t status = cy_drone_get_launch_heading(handle, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::optional<double> get_pitch() const { double out{}; int32_t status = cy_drone_get_launch_pitch(handle, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::optional<double> get_param() const { double out{}; int32_t status = cy_drone_get_launch_param(handle, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
};
struct Angles {cy_robot *&handle; explicit Angles(cy_robot *&robot):handle(robot){}
  double normalize(double degrees) const { double out{}; int32_t status = cy_angles_normalize(degrees, &out); check(handle,status); return out; }
  double angle_diff(double target, double current) const { double out{}; int32_t status = cy_angles_angle_diff(target, current, &out); check(handle,status); return out; }
  double bearing_to(const Vec3 & from, const Vec3 & to) const { double out{}; int32_t status = cy_angles_bearing_to(&from, &to, &out); check(handle,status); return out; }
  double elevation_to(const Vec3 & from, const Vec3 & to) const { double out{}; int32_t status = cy_angles_elevation_to(&from, &to, &out); check(handle,status); return out; }
  Vec3 direction(double heading, double pitch) const { cy_vec3 out{}; int32_t status = cy_angles_direction(heading, pitch, &out); check(handle,status); return out; }
  double distance(const Vec3 & a, const Vec3 & b) const { double out{}; int32_t status = cy_angles_distance(&a, &b, &out); check(handle,status); return out; }
  Direction to_chassis(double heading, double pitch, const Attitude & attitude) const { cy_direction out{}; int32_t status = cy_angles_to_chassis_v1(heading, pitch, &attitude, &out); check(handle,status); return out; }
  Direction from_chassis(double heading, double pitch, const Attitude & attitude) const { cy_direction out{}; int32_t status = cy_angles_from_chassis_v1(heading, pitch, &attitude, &out); check(handle,status); return out; }
  double clamp(double value, double low = -1, double high = 1) const { double out{}; int32_t status = cy_angles_clamp(value, low, high, &out); check(handle,status); return out; }
};
struct Status {cy_robot *&handle; explicit Status(cy_robot *&robot):handle(robot){}
  void set(std::optional<std::string_view> text = std::nullopt, std::optional<double> value = std::nullopt) {if(text)check(handle,cy_status_text(handle,text->data(),(uint32_t)text->size()));if(value)check(handle,cy_status_value(handle,*value));}
  std::optional<std::string> get_text() const { std::string out(256,'\0');uint32_t count=0; int32_t status = cy_status_get_text(handle, out.data(), (uint32_t)out.size(), &count); while(status==CY_SHORT){out.resize(count);status=cy_status_get_text(handle, out.data(), (uint32_t)out.size(), &count);} if(status==CY_NONE)return std::nullopt; check(handle,status); out.resize(count);return out; }
  std::optional<double> get_value() const { double out{}; int32_t status = cy_status_get_value(handle, &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
};
struct Debug {cy_robot *&handle; explicit Debug(cy_robot *&robot):handle(robot){}
  void log(std::string_view line) { int32_t status = cy_debug_log(handle, line.data(), (uint32_t)line.size()); check(handle,status); }
};
struct Raw {cy_robot *&handle; explicit Raw(cy_robot *&robot):handle(robot){}
  std::optional<double> number(std::string_view path) const { double out{}; int32_t status = cy_raw_number(handle, path.data(), (uint32_t)path.size(), &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::optional<std::string> text(std::string_view path) const { std::string out(256,'\0');uint32_t count=0; int32_t status = cy_raw_text(handle, path.data(), (uint32_t)path.size(), out.data(), (uint32_t)out.size(), &count); while(status==CY_SHORT){out.resize(count);status=cy_raw_text(handle, path.data(), (uint32_t)path.size(), out.data(), (uint32_t)out.size(), &count);} if(status==CY_NONE)return std::nullopt; check(handle,status); out.resize(count);return out; }
  bool has(std::string_view path) const { int32_t out{}; int32_t status = cy_raw_has(handle, path.data(), (uint32_t)path.size(), &out); check(handle,status); return out != 0; }
  std::optional<bool> flag(std::string_view path) const { int32_t out{}; int32_t status = cy_raw_flag(handle, path.data(), (uint32_t)path.size(), &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out != 0; }
  std::optional<int32_t> count(std::string_view path) const { int32_t out{}; int32_t status = cy_raw_count(handle, path.data(), (uint32_t)path.size(), &out); if(status==CY_NONE)return std::nullopt; check(handle,status); return out; }
  std::string line() const { std::string out(256,'\0');uint32_t count=0; int32_t status = cy_raw_line(handle, out.data(), (uint32_t)out.size(), &count); while(status==CY_SHORT){out.resize(count);status=cy_raw_line(handle, out.data(), (uint32_t)out.size(), &count);} check(handle,status); out.resize(count);return out; }
  void extra(std::string_view json) { int32_t status = cy_raw_extra(handle, json.data(), (uint32_t)json.size()); check(handle,status); }
};
struct Session {cy_robot *&handle; explicit Session(cy_robot *&robot):handle(robot){}
  int32_t event() const { int32_t out{}; int32_t status = cy_session_event(handle, &out); check(handle,status); return out; }
  int32_t next() { int32_t out{}; int32_t status = cy_session_next(handle, &out); check(handle,status); return out; }
  void send() { int32_t status = cy_session_send(handle); check(handle,status); }
  void forfeit(std::string_view code, std::string_view message) { int32_t status = cy_session_forfeit(handle, code.data(), (uint32_t)code.size(), message.data(), (uint32_t)message.size()); check(handle,status); }
  void log(std::string_view line) { int32_t status = cy_session_log(handle, line.data(), (uint32_t)line.size()); check(handle,status); }
  void feed(std::string_view line) { int32_t status = cy_session_feed(handle, line.data(), (uint32_t)line.size()); check(handle,status); }
  std::optional<std::string> reply() { std::string out(256,'\0');uint32_t count=0; int32_t status = cy_session_reply(handle, out.data(), (uint32_t)out.size(), &count); while(status==CY_SHORT){out.resize(count);status=cy_session_reply(handle, out.data(), (uint32_t)out.size(), &count);} if(status==CY_NONE)return std::nullopt; check(handle,status); out.resize(count);return out; }
};
class Forfeit:public std::exception {public:std::string code,message;Forfeit(std::string c,std::string m=""):code(c),message(m){}};
class Robot {cy_robot *handle=nullptr;bool owned=true;void open(bool memory){if(handle&&owned)cy_close(handle);handle=nullptr;check(nullptr,(memory?cy_open_memory:cy_open)(CY_API_MAJOR,CY_API_MINOR,"clunkyard-cpp",13,CY_API_VERSION,sizeof CY_API_VERSION-1,&handle));}
void loop(){for(;;){int32_t event;int32_t status=cy_session_next(handle,&event);if(status==CY_E_INTERNAL){session.send();continue;}check(handle,status);if(event==CY_EVENT_END)return;try{if(event==CY_EVENT_HELLO)on_start();else on_turn();}catch(const Forfeit &failure){session.forfeit(failure.code,failure.message);continue;}catch(const std::exception &failure){session.log(failure.what());}session.send();}}
public:
  Match match;
  Clock clock;
  Arena arena;
  Me me;
  Battery battery;
  Drive drive;
  Gun gun;
  Scanner scanner;
  Sensors sensors;
  Tracker tracker;
  Nav nav;
  Score score;
  Drone drone;
  Angles angles;
  Status status;
  Debug debug;
  Raw raw;
  Session session;
  Robot():match(handle),clock(handle),arena(handle),me(handle),battery(handle),drive(handle),gun(handle),scanner(handle),sensors(handle),tracker(handle),nav(handle),score(handle),drone(handle),angles(handle),status(handle),debug(handle),raw(handle),session(handle) {}
  explicit Robot(cy_robot *h):handle(h),owned(false),match(handle),clock(handle),arena(handle),me(handle),battery(handle),drive(handle),gun(handle),scanner(handle),sensors(handle),tracker(handle),nav(handle),score(handle),drone(handle),angles(handle),status(handle),debug(handle),raw(handle),session(handle) {}
  Robot(const Robot &)=delete;Robot &operator=(const Robot &)=delete;virtual ~Robot(){if(owned)cy_close(handle);}
  virtual void on_start(){} virtual void on_turn(){}
  void run(){open(false);loop();}
  std::vector<std::string> play(const std::vector<std::string> &lines){open(true);for(const auto &line:lines)session.feed(line);loop();std::vector<std::string> out;while(auto line=session.reply())out.push_back(*line);return out;}
};
}
