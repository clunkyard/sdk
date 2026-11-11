/* Generated from protocol/v2/api.json; do not edit. Run make api. */
#define _POSIX_C_SOURCE 200809L
#include "clunkyard.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <inttypes.h>
#include <math.h>
#define CAP 4096
static cy_robot *robot;
static char *cursor;
static char *token(void){char *at=cursor,*tab=strchr(at,'\t');if(tab){*tab=0;cursor=tab+1;}else cursor=at+strlen(at);return at;}
static int32_t integer(void){return (int32_t)strtol(token()+1,NULL,10);}
static double number(void){char *value=token();if(*value=='i')return strtod(value+1,NULL);uint64_t raw=strtoull(value+1,NULL,16);double out;memcpy(&out,&raw,8);return out;}
static char *string(uint32_t *length){char *value=token();*length=(uint32_t)(strlen(value+1)/2);for(uint32_t i=0;i<*length;i++){char pair[3]={value[1+2*i],value[2+2*i],0};value[i]=(char)strtoul(pair,NULL,16);}return value;}
static void bits(double value){uint64_t raw;memcpy(&raw,&value,8);if(isnan(value))raw=UINT64_C(0x7ff8000000000000);printf("%016" PRIx64,raw);}
static void text(const char *value,uint32_t length){putchar('s');for(uint32_t i=0;i<length;i++)printf("%02x",(unsigned char)value[i]);}
static void enum_source(int32_t value){switch(value){case 1:text("scan",4);break;case 2:text("proximity",9);break;case 3:text("faint",5);break;case 4:text("beam",4);break;case 5:text("muzzle",6);break;case 6:text("motion",6);break;case 7:text("ping",4);break;default:putchar('n');}}
static void enum_mode(int32_t value){switch(value){case 1:text("2d",2);break;case 2:text("3d",2);break;default:putchar('n');}}
static void enum_ruleset(int32_t value){switch(value){case 1:text("classic",7);break;case 2:text("inertial-2d",11);break;case 3:text("gravity-flat",12);break;case 4:text("gravity-hills",13);break;case 5:text("zero-g",6);break;default:putchar('n');}}
static void enum_control(int32_t value){switch(value){case 1:text("direct",6);break;case 2:text("forces",6);break;default:putchar('n');}}
static void enum_terrain_kind(int32_t value){switch(value){case 1:text("none",4);break;case 2:text("flat",4);break;case 3:text("hills",5);break;default:putchar('n');}}
static void enum_shape(int32_t value){switch(value){case 1:text("circle",6);break;case 2:text("sphere",6);break;case 3:text("column",6);break;default:putchar('n');}}
static void enum_pickup_kind(int32_t value){switch(value){case 1:text("energy",6);break;case 2:text("health",6);break;case 3:text("armour",6);break;default:putchar('n');}}
static void enum_compute_unit(int32_t value){switch(value){case 1:text("operations",10);break;case 2:text("fuel",4);break;case 3:text("cpu-ms",6);break;default:putchar('n');}}
static void enum_event(int32_t value){switch(value){case 1:text("hello",5);break;case 2:text("state",5);break;case 3:text("end",3);break;default:putchar('n');}}
static void read_vec3(cy_vec3 *out){out->x = number();out->y = number();out->z = number();}
static void print_vec3(const cy_vec3 *out){putchar('{');fputs("x=",stdout);bits((double)out->x);putchar(',');fputs("y=",stdout);bits((double)out->y);putchar(',');fputs("z=",stdout);bits((double)out->z);putchar('}');}
static void read_rotation(cy_rotation *out){out->yaw = number();out->pitch = number();out->roll = number();}
static void print_rotation(const cy_rotation *out){putchar('{');fputs("yaw=",stdout);bits((double)out->yaw);putchar(',');fputs("pitch=",stdout);bits((double)out->pitch);putchar(',');fputs("roll=",stdout);bits((double)out->roll);putchar('}');}
static void read_direction(cy_direction *out){out->heading = number();out->pitch = number();}
static void print_direction(const cy_direction *out){putchar('{');fputs("heading=",stdout);bits((double)out->heading);putchar(',');fputs("pitch=",stdout);bits((double)out->pitch);putchar('}');}
static void read_attitude(cy_attitude_v1 *out){read_vec3(&out->forward);read_vec3(&out->right);read_vec3(&out->up);}
static void print_attitude(const cy_attitude_v1 *out){putchar('{');fputs("forward=",stdout);print_vec3(&out->forward);putchar(',');fputs("right=",stdout);print_vec3(&out->right);putchar(',');fputs("up=",stdout);print_vec3(&out->up);putchar('}');}
static void read_applied(cy_applied_v1 *out){read_vec3(&out->force);read_rotation(&out->torque);out->brake = number();}
static void print_applied(const cy_applied_v1 *out){putchar('{');fputs("force=",stdout);print_vec3(&out->force);putchar(',');fputs("torque=",stdout);print_rotation(&out->torque);putchar(',');fputs("brake=",stdout);bits((double)out->brake);putchar('}');}
static void read_scan(cy_scan_v1 *out){out->heading = number();out->pitch = number();out->arc = number();out->range = number();}
static void print_scan(const cy_scan_v1 *out){putchar('{');fputs("heading=",stdout);bits((double)out->heading);putchar(',');fputs("pitch=",stdout);bits((double)out->pitch);putchar(',');fputs("arc=",stdout);bits((double)out->arc);putchar(',');fputs("range=",stdout);bits((double)out->range);putchar('}');}
static void read_contact(cy_contact_v1 *out){out->slot = integer();out->source = integer();out->range = number();out->bearing = number();out->elevation = number();out->bearing_error = number();out->range_error = number();read_vec3(&out->position);}
static void print_contact(const cy_contact_v1 *out){putchar('{');fputs("slot=",stdout);bits((double)out->slot);putchar(',');fputs("source=",stdout);enum_source(out->source);putchar(',');fputs("range=",stdout);bits((double)out->range);putchar(',');fputs("bearing=",stdout);bits((double)out->bearing);putchar(',');fputs("elevation=",stdout);bits((double)out->elevation);putchar(',');fputs("bearing_error=",stdout);bits((double)out->bearing_error);putchar(',');fputs("range_error=",stdout);bits((double)out->range_error);putchar(',');fputs("position=",stdout);print_vec3(&out->position);putchar('}');}
static void read_cue(cy_cue_v1 *out){out->source = integer();out->tick = integer();out->bearing_error = number();read_vec3(&out->position);}
static void print_cue(const cy_cue_v1 *out){putchar('{');fputs("source=",stdout);enum_source(out->source);putchar(',');fputs("tick=",stdout);bits((double)out->tick);putchar(',');fputs("bearing_error=",stdout);bits((double)out->bearing_error);putchar(',');fputs("position=",stdout);print_vec3(&out->position);putchar('}');}
static void read_solution(cy_solution_v1 *out){out->heading = number();out->pitch = number();out->time = number();}
static void print_solution(const cy_solution_v1 *out){putchar('{');fputs("heading=",stdout);bits((double)out->heading);putchar(',');fputs("pitch=",stdout);bits((double)out->pitch);putchar(',');fputs("time=",stdout);bits((double)out->time);putchar('}');}
static void read_zone(cy_zone_v1 *out){out->shape = integer();out->reserved = integer();out->radius = number();read_vec3(&out->center);}
static void print_zone(const cy_zone_v1 *out){putchar('{');fputs("shape=",stdout);enum_shape(out->shape);putchar(',');fputs("radius=",stdout);bits((double)out->radius);putchar(',');fputs("center=",stdout);print_vec3(&out->center);putchar('}');}
static void read_storm(cy_storm_v1 *out){out->shape = integer();out->reserved = integer();out->radius = number();out->dps = number();read_vec3(&out->center);read_vec3(&out->velocity);}
static void print_storm(const cy_storm_v1 *out){putchar('{');fputs("shape=",stdout);enum_shape(out->shape);putchar(',');fputs("radius=",stdout);bits((double)out->radius);putchar(',');fputs("dps=",stdout);bits((double)out->dps);putchar(',');fputs("center=",stdout);print_vec3(&out->center);putchar(',');fputs("velocity=",stdout);print_vec3(&out->velocity);putchar('}');}
static void read_hill(cy_hill_v1 *out){out->shape = integer();out->holder = integer();out->contested = integer();out->moves_tick = integer();out->held_ticks = integer();out->best_held_ticks = integer();out->radius = number();read_vec3(&out->center);read_vec3(&out->next);}
static void print_hill(const cy_hill_v1 *out){putchar('{');fputs("shape=",stdout);enum_shape(out->shape);putchar(',');fputs("holder=",stdout);bits((double)out->holder);putchar(',');fputs("contested=",stdout);fputs(out->contested?"t":"f",stdout);putchar(',');fputs("moves_tick=",stdout);bits((double)out->moves_tick);putchar(',');fputs("held_ticks=",stdout);bits((double)out->held_ticks);putchar(',');fputs("best_held_ticks=",stdout);bits((double)out->best_held_ticks);putchar(',');fputs("radius=",stdout);bits((double)out->radius);putchar(',');fputs("center=",stdout);print_vec3(&out->center);putchar(',');fputs("next=",stdout);print_vec3(&out->next);putchar('}');}
static void read_pickup(cy_pickup_v1 *out){out->kind = integer();out->expires_tick = integer();out->amount = number();read_vec3(&out->position);}
static void print_pickup(const cy_pickup_v1 *out){putchar('{');fputs("kind=",stdout);enum_pickup_kind(out->kind);putchar(',');fputs("expires_tick=",stdout);bits((double)out->expires_tick);putchar(',');fputs("amount=",stdout);bits((double)out->amount);putchar(',');fputs("position=",stdout);print_vec3(&out->position);putchar('}');}
static void read_compute(cy_compute_v1 *out){out->k = number();out->quanta = integer();out->unit = integer();}
static void print_compute(const cy_compute_v1 *out){putchar('{');fputs("k=",stdout);bits((double)out->k);putchar(',');fputs("quanta=",stdout);bits((double)out->quanta);putchar(',');fputs("unit=",stdout);enum_compute_unit(out->unit);putchar('}');}
extern int32_t binding_cy_match_mode(cy_robot *robot,int32_t *out);
extern int32_t binding_cy_match_ruleset(cy_robot *robot,int32_t *out);
extern int32_t binding_cy_match_control(cy_robot *robot,int32_t *out);
extern int32_t binding_cy_match_robots(cy_robot *robot,int32_t *out);
extern int32_t binding_cy_match_is_3d(cy_robot *robot,int32_t *out);
extern int32_t binding_cy_match_tick_rate(cy_robot *robot,int32_t *out);
extern int32_t binding_cy_match_max_ticks(cy_robot *robot,int32_t *out);
extern int32_t binding_cy_robot_id(cy_robot *robot,int32_t slot,char *buf,uint32_t capacity,uint32_t *length);
extern int32_t binding_cy_clock_tick(cy_robot *robot,int32_t *out);
extern int32_t binding_cy_clock_seconds(cy_robot *robot,double *out);
extern int32_t binding_cy_clock_remaining_ticks(cy_robot *robot,int32_t *out);
extern int32_t binding_cy_arena_size(cy_robot *robot,cy_vec3 *out);
extern int32_t binding_cy_arena_gravity(cy_robot *robot,cy_vec3 *out);
extern int32_t binding_cy_arena_terrain_kind(cy_robot *robot,int32_t *out);
extern int32_t binding_cy_arena_ground_height(cy_robot *robot,double x,double y,double *out);
extern int32_t binding_cy_me_health(cy_robot *robot,double *out);
extern int32_t binding_cy_me_position(cy_robot *robot,cy_vec3 *out);
extern int32_t binding_cy_me_velocity(cy_robot *robot,cy_vec3 *out);
extern int32_t binding_cy_me_heading(cy_robot *robot,double *out);
extern int32_t binding_cy_me_pitch(cy_robot *robot,double *out);
extern int32_t binding_cy_me_roll(cy_robot *robot,double *out);
extern int32_t binding_cy_me_angular_velocity(cy_robot *robot,cy_rotation *out);
extern int32_t binding_cy_me_attitude_v1(cy_robot *robot,cy_attitude_v1 *out);
extern int32_t binding_cy_me_grounded(cy_robot *robot,int32_t *out);
extern int32_t binding_cy_me_altitude(cy_robot *robot,double *out);
extern int32_t binding_cy_me_applied_v1(cy_robot *robot,cy_applied_v1 *out);
extern int32_t binding_cy_battery_charge(cy_robot *robot,double *out);
extern int32_t binding_cy_battery_capacity(cy_robot *robot,double *out);
extern int32_t binding_cy_battery_regen(cy_robot *robot,double *out);
extern int32_t binding_cy_battery_sustainable_drive(cy_robot *robot,double *out);
extern int32_t binding_cy_battery_compute_v1(cy_robot *robot,cy_compute_v1 *out);
extern int32_t binding_cy_battery_drive_cost(cy_robot *robot,double fraction,double *out);
extern int32_t binding_cy_drive_hover_lift(cy_robot *robot,double *out);
extern int32_t binding_cy_drive_throttle(cy_robot *robot,double value);
extern int32_t binding_cy_drive_turn(cy_robot *robot,double value);
extern int32_t binding_cy_drive_pitch_turn(cy_robot *robot,double value);
extern int32_t binding_cy_drive_thrust(cy_robot *robot,double value);
extern int32_t binding_cy_drive_strafe(cy_robot *robot,double value);
extern int32_t binding_cy_drive_lift(cy_robot *robot,double value);
extern int32_t binding_cy_drive_yaw_torque(cy_robot *robot,double value);
extern int32_t binding_cy_drive_pitch_torque(cy_robot *robot,double value);
extern int32_t binding_cy_drive_roll_torque(cy_robot *robot,double value);
extern int32_t binding_cy_drive_brake(cy_robot *robot,double value);
extern int32_t binding_cy_drive_get_throttle(cy_robot *robot,double *out);
extern int32_t binding_cy_drive_get_turn(cy_robot *robot,double *out);
extern int32_t binding_cy_drive_get_pitch_turn(cy_robot *robot,double *out);
extern int32_t binding_cy_drive_get_thrust(cy_robot *robot,double *out);
extern int32_t binding_cy_drive_get_strafe(cy_robot *robot,double *out);
extern int32_t binding_cy_drive_get_lift(cy_robot *robot,double *out);
extern int32_t binding_cy_drive_get_yaw_torque(cy_robot *robot,double *out);
extern int32_t binding_cy_drive_get_pitch_torque(cy_robot *robot,double *out);
extern int32_t binding_cy_drive_get_roll_torque(cy_robot *robot,double *out);
extern int32_t binding_cy_drive_get_brake(cy_robot *robot,double *out);
extern int32_t binding_cy_gun_aim_now(cy_robot *robot,cy_direction *out);
extern int32_t binding_cy_gun_cooldown_ticks(cy_robot *robot,int32_t *out);
extern int32_t binding_cy_gun_spread(cy_robot *robot,double *out);
extern int32_t binding_cy_gun_max_power(cy_robot *robot,double *out);
extern int32_t binding_cy_gun_damage_scale(cy_robot *robot,double *out);
extern int32_t binding_cy_gun_can_fire(cy_robot *robot,double power,double reserve,int32_t *out);
extern int32_t binding_cy_gun_shot_speed(cy_robot *robot,double power,double *out);
extern int32_t binding_cy_gun_shot_cost(cy_robot *robot,double power,double *out);
extern int32_t binding_cy_gun_reload_ticks(cy_robot *robot,double power,double *out);
extern int32_t binding_cy_gun_shot_damage(cy_robot *robot,double power,double *out);
extern int32_t binding_cy_gun_power_for_speed(cy_robot *robot,double speed,double *out);
extern int32_t binding_cy_gun_power_for_flight_time(cy_robot *robot,double range,double seconds,double *out);
extern int32_t binding_cy_gun_lead_v1(cy_robot *robot,const cy_contact_v1 *contact,double power,cy_solution_v1 *out);
extern int32_t binding_cy_gun_lead_point_v1(cy_robot *robot,const cy_vec3 *position,const cy_vec3 *velocity,double power,cy_solution_v1 *out);
extern int32_t binding_cy_gun_aim_heading(cy_robot *robot,double value);
extern int32_t binding_cy_gun_aim_pitch(cy_robot *robot,double value);
extern int32_t binding_cy_gun_fire(cy_robot *robot);
extern int32_t binding_cy_gun_power(cy_robot *robot,double value);
extern int32_t binding_cy_gun_get_aim_heading(cy_robot *robot,double *out);
extern int32_t binding_cy_gun_get_aim_pitch(cy_robot *robot,double *out);
extern int32_t binding_cy_gun_get_fire(cy_robot *robot,double *out);
extern int32_t binding_cy_gun_get_power(cy_robot *robot,double *out);
extern int32_t binding_cy_scanner_last_v1(cy_robot *robot,cy_scan_v1 *out);
extern int32_t binding_cy_scanner_proximity_range(cy_robot *robot,double *out);
extern int32_t binding_cy_scanner_reach(cy_robot *robot,double arc,double *out);
extern int32_t binding_cy_scanner_arc_for_reach(cy_robot *robot,double range,double *out);
extern int32_t binding_cy_scanner_heading(cy_robot *robot,double value);
extern int32_t binding_cy_scanner_arc(cy_robot *robot,double value);
extern int32_t binding_cy_scanner_pitch(cy_robot *robot,double value);
extern int32_t binding_cy_scanner_get_heading(cy_robot *robot,double *out);
extern int32_t binding_cy_scanner_get_arc(cy_robot *robot,double *out);
extern int32_t binding_cy_scanner_get_pitch(cy_robot *robot,double *out);
extern int32_t binding_cy_sensors_contacts_v1(cy_robot *robot,cy_contact_v1 *out,uint32_t capacity,uint32_t *count);
extern int32_t binding_cy_sensors_nearest_v1(cy_robot *robot,cy_contact_v1 *out);
extern int32_t binding_cy_sensors_cues_v1(cy_robot *robot,cy_contact_v1 *out,uint32_t capacity,uint32_t *count);
extern int32_t binding_cy_sensors_contact_v1(cy_robot *robot,int32_t slot,cy_contact_v1 *out);
extern int32_t binding_cy_sensors_position_of_v1(cy_robot *robot,const cy_contact_v1 *contact,cy_vec3 *out);
extern int32_t binding_cy_tracker_cue_v1(cy_robot *robot,cy_cue_v1 *out);
extern int32_t binding_cy_tracker_nearest_bearing_rate(cy_robot *robot,double *out);
extern int32_t binding_cy_tracker_velocity(cy_robot *robot,int32_t slot,cy_vec3 *out);
extern int32_t binding_cy_tracker_position(cy_robot *robot,int32_t slot,cy_vec3 *out);
extern int32_t binding_cy_tracker_last_seen(cy_robot *robot,int32_t slot,int32_t *out);
extern int32_t binding_cy_tracker_configure(cy_robot *robot,double smoothing,int32_t forget_ticks,int32_t cue_ticks);
extern int32_t binding_cy_nav_zone_v1(cy_robot *robot,cy_zone_v1 *out);
extern int32_t binding_cy_nav_zone_margin(cy_robot *robot,const cy_vec3 *point,double *out);
extern int32_t binding_cy_nav_zone_contains(cy_robot *robot,const cy_vec3 *point,double margin,int32_t *out);
extern int32_t binding_cy_nav_zone_distance_from(cy_robot *robot,const cy_vec3 *point,double *out);
extern int32_t binding_cy_nav_storm_v1(cy_robot *robot,cy_storm_v1 *out);
extern int32_t binding_cy_nav_storm_margin(cy_robot *robot,const cy_vec3 *point,double *out);
extern int32_t binding_cy_nav_storm_contains(cy_robot *robot,const cy_vec3 *point,double margin,int32_t *out);
extern int32_t binding_cy_nav_storm_distance_from(cy_robot *robot,const cy_vec3 *point,double *out);
extern int32_t binding_cy_nav_hill_v1(cy_robot *robot,cy_hill_v1 *out);
extern int32_t binding_cy_nav_hill_margin(cy_robot *robot,const cy_vec3 *point,double *out);
extern int32_t binding_cy_nav_hill_contains(cy_robot *robot,const cy_vec3 *point,double margin,int32_t *out);
extern int32_t binding_cy_nav_hill_distance_from(cy_robot *robot,const cy_vec3 *point,double *out);
extern int32_t binding_cy_nav_pickups_v1(cy_robot *robot,cy_pickup_v1 *out,uint32_t capacity,uint32_t *count);
extern int32_t binding_cy_nav_nearest_pickup_v1(cy_robot *robot,int32_t kind,cy_pickup_v1 *out);
extern int32_t binding_cy_nav_field_distance(cy_robot *robot,const cy_vec3 *a,const cy_vec3 *b,double *out);
extern int32_t binding_cy_score_me(cy_robot *robot,double *out);
extern int32_t binding_cy_score_best(cy_robot *robot,double *out);
extern int32_t binding_cy_score_armour(cy_robot *robot,double *out);
extern int32_t binding_cy_angles_normalize(double degrees,double *out);
extern int32_t binding_cy_angles_angle_diff(double target,double current,double *out);
extern int32_t binding_cy_angles_bearing_to(const cy_vec3 *from,const cy_vec3 *to,double *out);
extern int32_t binding_cy_angles_elevation_to(const cy_vec3 *from,const cy_vec3 *to,double *out);
extern int32_t binding_cy_angles_direction(double heading,double pitch,cy_vec3 *out);
extern int32_t binding_cy_angles_distance(const cy_vec3 *a,const cy_vec3 *b,double *out);
extern int32_t binding_cy_angles_clamp(double value,double low,double high,double *out);
extern int32_t binding_cy_status_text(cy_robot *robot,const char *text,uint32_t text_length);
extern int32_t binding_cy_status_value(cy_robot *robot,double value);
extern int32_t binding_cy_status_get_text(cy_robot *robot,char *buf,uint32_t capacity,uint32_t *length);
extern int32_t binding_cy_status_get_value(cy_robot *robot,double *out);
extern int32_t binding_cy_raw_number(cy_robot *robot,const char *path,uint32_t path_length,double *out);
extern int32_t binding_cy_raw_text(cy_robot *robot,const char *path,uint32_t path_length,char *buf,uint32_t capacity,uint32_t *length);
extern int32_t binding_cy_raw_has(cy_robot *robot,const char *path,uint32_t path_length,int32_t *out);
extern int32_t binding_cy_raw_flag(cy_robot *robot,const char *path,uint32_t path_length,int32_t *out);
extern int32_t binding_cy_raw_count(cy_robot *robot,const char *path,uint32_t path_length,int32_t *out);
extern int32_t binding_cy_raw_line(cy_robot *robot,char *buf,uint32_t capacity,uint32_t *length);
extern int32_t binding_cy_raw_extra(cy_robot *robot,const char *json,uint32_t json_length);
static void invoke(const char *op,const char *label,const char *name,const char *field){int answer=strcmp(label,"-")!=0;if(!strcmp(op,"call")&&!strcmp(name,"match.mode")){int32_t out=0;
int32_t status=binding_cy_match_mode(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');enum_mode(out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"match.ruleset")){int32_t out=0;
int32_t status=binding_cy_match_ruleset(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');enum_ruleset(out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"match.control")){int32_t out=0;
int32_t status=binding_cy_match_control(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');enum_control(out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"match.robots")){int32_t out=0;
int32_t status=binding_cy_match_robots(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"match.is_3d")){int32_t out=0;
int32_t status=binding_cy_match_is_3d(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');fputs(out?"t":"f",stdout);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"match.tick_rate")){int32_t out=0;
int32_t status=binding_cy_match_tick_rate(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"match.max_ticks")){int32_t out=0;
int32_t status=binding_cy_match_max_ticks(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"match.robot_id")){int32_t slot = integer();
char buf[CAP]; uint32_t count=0;
int32_t status=binding_cy_robot_id(robot,slot,buf,CAP,&count);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');text(buf, count);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"clock.tick")){int32_t out=0;
int32_t status=binding_cy_clock_tick(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"clock.seconds")){double out=0;
int32_t status=binding_cy_clock_seconds(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"clock.remaining_ticks")){int32_t out=0;
int32_t status=binding_cy_clock_remaining_ticks(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"arena.size")){cy_vec3 out={0};
int32_t status=binding_cy_arena_size(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');print_vec3(&out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"arena.gravity")){cy_vec3 out={0};
int32_t status=binding_cy_arena_gravity(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');print_vec3(&out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"arena.terrain_kind")){int32_t out=0;
int32_t status=binding_cy_arena_terrain_kind(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');enum_terrain_kind(out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"arena.ground_height")){double x = number();
double y = number();
double out=0;
int32_t status=binding_cy_arena_ground_height(robot,x,y,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"me.health")){double out=0;
int32_t status=binding_cy_me_health(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"me.position")){cy_vec3 out={0};
int32_t status=binding_cy_me_position(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');print_vec3(&out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"me.velocity")){cy_vec3 out={0};
int32_t status=binding_cy_me_velocity(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');print_vec3(&out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"me.heading")){double out=0;
int32_t status=binding_cy_me_heading(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"me.pitch")){double out=0;
int32_t status=binding_cy_me_pitch(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"me.roll")){double out=0;
int32_t status=binding_cy_me_roll(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"me.angular_velocity")){cy_rotation out={0};
int32_t status=binding_cy_me_angular_velocity(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');print_rotation(&out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"me.attitude")){cy_attitude_v1 out={0};
int32_t status=binding_cy_me_attitude_v1(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');print_attitude(&out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"me.grounded")){int32_t out=0;
int32_t status=binding_cy_me_grounded(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');fputs(out?"t":"f",stdout);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"me.altitude")){double out=0;
int32_t status=binding_cy_me_altitude(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"me.applied")){cy_applied_v1 out={0};
int32_t status=binding_cy_me_applied_v1(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');print_applied(&out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"battery.charge")){double out=0;
int32_t status=binding_cy_battery_charge(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"battery.capacity")){double out=0;
int32_t status=binding_cy_battery_capacity(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"battery.regen")){double out=0;
int32_t status=binding_cy_battery_regen(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"battery.sustainable_drive")){double out=0;
int32_t status=binding_cy_battery_sustainable_drive(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"battery.compute")){cy_compute_v1 out={0};
int32_t status=binding_cy_battery_compute_v1(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');print_compute(&out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"battery.drive_cost")){double fraction = number();
double out=0;
int32_t status=binding_cy_battery_drive_cost(robot,fraction,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"drive.hover_lift")){double out=0;
int32_t status=binding_cy_drive_hover_lift(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"set")&&!strcmp(name,"drive.set")&&!strcmp(field,"throttle")){double value = number();
int32_t status=binding_cy_drive_throttle(robot,value);
if(answer){printf("%s %d",label,status);if(status==CY_OK && 0){putchar(' ');}putchar('\n');}return;}
if(!strcmp(op,"set")&&!strcmp(name,"drive.set")&&!strcmp(field,"turn")){double value = number();
int32_t status=binding_cy_drive_turn(robot,value);
if(answer){printf("%s %d",label,status);if(status==CY_OK && 0){putchar(' ');}putchar('\n');}return;}
if(!strcmp(op,"set")&&!strcmp(name,"drive.set")&&!strcmp(field,"pitch_turn")){double value = number();
int32_t status=binding_cy_drive_pitch_turn(robot,value);
if(answer){printf("%s %d",label,status);if(status==CY_OK && 0){putchar(' ');}putchar('\n');}return;}
if(!strcmp(op,"set")&&!strcmp(name,"drive.forces")&&!strcmp(field,"thrust")){double value = number();
int32_t status=binding_cy_drive_thrust(robot,value);
if(answer){printf("%s %d",label,status);if(status==CY_OK && 0){putchar(' ');}putchar('\n');}return;}
if(!strcmp(op,"set")&&!strcmp(name,"drive.forces")&&!strcmp(field,"strafe")){double value = number();
int32_t status=binding_cy_drive_strafe(robot,value);
if(answer){printf("%s %d",label,status);if(status==CY_OK && 0){putchar(' ');}putchar('\n');}return;}
if(!strcmp(op,"set")&&!strcmp(name,"drive.forces")&&!strcmp(field,"lift")){double value = number();
int32_t status=binding_cy_drive_lift(robot,value);
if(answer){printf("%s %d",label,status);if(status==CY_OK && 0){putchar(' ');}putchar('\n');}return;}
if(!strcmp(op,"set")&&!strcmp(name,"drive.forces")&&!strcmp(field,"yaw_torque")){double value = number();
int32_t status=binding_cy_drive_yaw_torque(robot,value);
if(answer){printf("%s %d",label,status);if(status==CY_OK && 0){putchar(' ');}putchar('\n');}return;}
if(!strcmp(op,"set")&&!strcmp(name,"drive.forces")&&!strcmp(field,"pitch_torque")){double value = number();
int32_t status=binding_cy_drive_pitch_torque(robot,value);
if(answer){printf("%s %d",label,status);if(status==CY_OK && 0){putchar(' ');}putchar('\n');}return;}
if(!strcmp(op,"set")&&!strcmp(name,"drive.forces")&&!strcmp(field,"roll_torque")){double value = number();
int32_t status=binding_cy_drive_roll_torque(robot,value);
if(answer){printf("%s %d",label,status);if(status==CY_OK && 0){putchar(' ');}putchar('\n');}return;}
if(!strcmp(op,"set")&&!strcmp(name,"drive.forces")&&!strcmp(field,"brake")){double value = number();
int32_t status=binding_cy_drive_brake(robot,value);
if(answer){printf("%s %d",label,status);if(status==CY_OK && 0){putchar(' ');}putchar('\n');}return;}
if(!strcmp(op,"get")&&!strcmp(name,"drive.get")&&!strcmp(field,"throttle")){double out=0;
int32_t status=binding_cy_drive_get_throttle(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"get")&&!strcmp(name,"drive.get")&&!strcmp(field,"turn")){double out=0;
int32_t status=binding_cy_drive_get_turn(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"get")&&!strcmp(name,"drive.get")&&!strcmp(field,"pitch_turn")){double out=0;
int32_t status=binding_cy_drive_get_pitch_turn(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"get")&&!strcmp(name,"drive.get")&&!strcmp(field,"thrust")){double out=0;
int32_t status=binding_cy_drive_get_thrust(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"get")&&!strcmp(name,"drive.get")&&!strcmp(field,"strafe")){double out=0;
int32_t status=binding_cy_drive_get_strafe(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"get")&&!strcmp(name,"drive.get")&&!strcmp(field,"lift")){double out=0;
int32_t status=binding_cy_drive_get_lift(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"get")&&!strcmp(name,"drive.get")&&!strcmp(field,"yaw_torque")){double out=0;
int32_t status=binding_cy_drive_get_yaw_torque(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"get")&&!strcmp(name,"drive.get")&&!strcmp(field,"pitch_torque")){double out=0;
int32_t status=binding_cy_drive_get_pitch_torque(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"get")&&!strcmp(name,"drive.get")&&!strcmp(field,"roll_torque")){double out=0;
int32_t status=binding_cy_drive_get_roll_torque(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"get")&&!strcmp(name,"drive.get")&&!strcmp(field,"brake")){double out=0;
int32_t status=binding_cy_drive_get_brake(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"gun.aim_now")){cy_direction out={0};
int32_t status=binding_cy_gun_aim_now(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');print_direction(&out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"gun.cooldown_ticks")){int32_t out=0;
int32_t status=binding_cy_gun_cooldown_ticks(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"gun.spread")){double out=0;
int32_t status=binding_cy_gun_spread(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"gun.max_power")){double out=0;
int32_t status=binding_cy_gun_max_power(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"gun.damage_scale")){double out=0;
int32_t status=binding_cy_gun_damage_scale(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"gun.can_fire")){double power = number();
double reserve = number();
int32_t out=0;
int32_t status=binding_cy_gun_can_fire(robot,power,reserve,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');fputs(out?"t":"f",stdout);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"gun.shot_speed")){double power = number();
double out=0;
int32_t status=binding_cy_gun_shot_speed(robot,power,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"gun.shot_cost")){double power = number();
double out=0;
int32_t status=binding_cy_gun_shot_cost(robot,power,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"gun.reload_ticks")){double power = number();
double out=0;
int32_t status=binding_cy_gun_reload_ticks(robot,power,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"gun.shot_damage")){double power = number();
double out=0;
int32_t status=binding_cy_gun_shot_damage(robot,power,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"gun.power_for_speed")){double speed = number();
double out=0;
int32_t status=binding_cy_gun_power_for_speed(robot,speed,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"gun.power_for_flight_time")){double range = number();
double seconds = number();
double out=0;
int32_t status=binding_cy_gun_power_for_flight_time(robot,range,seconds,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"gun.lead")){cy_contact_v1 contact; read_contact(&contact);
double power = number();
cy_solution_v1 out={0};
int32_t status=binding_cy_gun_lead_v1(robot,&contact,power,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');print_solution(&out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"gun.lead_point")){cy_vec3 position; read_vec3(&position);
cy_vec3 velocity; read_vec3(&velocity);
double power = number();
cy_solution_v1 out={0};
int32_t status=binding_cy_gun_lead_point_v1(robot,&position,&velocity,power,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');print_solution(&out);}putchar('\n');}return;}
if(!strcmp(op,"set")&&!strcmp(name,"gun.aim")&&!strcmp(field,"heading")){double value = number();
int32_t status=binding_cy_gun_aim_heading(robot,value);
if(answer){printf("%s %d",label,status);if(status==CY_OK && 0){putchar(' ');}putchar('\n');}return;}
if(!strcmp(op,"set")&&!strcmp(name,"gun.aim")&&!strcmp(field,"pitch")){double value = number();
int32_t status=binding_cy_gun_aim_pitch(robot,value);
if(answer){printf("%s %d",label,status);if(status==CY_OK && 0){putchar(' ');}putchar('\n');}return;}
if(!strcmp(op,"set")&&!strcmp(name,"gun.fire")&&!strcmp(field,"fire")){
int32_t status=binding_cy_gun_fire(robot);
if(answer){printf("%s %d",label,status);if(status==CY_OK && 0){putchar(' ');}putchar('\n');}return;}
if(!strcmp(op,"set")&&!strcmp(name,"gun.fire")&&!strcmp(field,"power")){double value = number();
int32_t status=binding_cy_gun_power(robot,value);
if(answer){printf("%s %d",label,status);if(status==CY_OK && 0){putchar(' ');}putchar('\n');}return;}
if(!strcmp(op,"get")&&!strcmp(name,"gun.get")&&!strcmp(field,"heading")){double out=0;
int32_t status=binding_cy_gun_get_aim_heading(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"get")&&!strcmp(name,"gun.get")&&!strcmp(field,"pitch")){double out=0;
int32_t status=binding_cy_gun_get_aim_pitch(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"get")&&!strcmp(name,"gun.get")&&!strcmp(field,"fire")){double out=0;
int32_t status=binding_cy_gun_get_fire(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"get")&&!strcmp(name,"gun.get")&&!strcmp(field,"power")){double out=0;
int32_t status=binding_cy_gun_get_power(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"scanner.last")){cy_scan_v1 out={0};
int32_t status=binding_cy_scanner_last_v1(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');print_scan(&out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"scanner.proximity_range")){double out=0;
int32_t status=binding_cy_scanner_proximity_range(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"scanner.reach")){double arc = number();
double out=0;
int32_t status=binding_cy_scanner_reach(robot,arc,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"scanner.arc_for_reach")){double range = number();
double out=0;
int32_t status=binding_cy_scanner_arc_for_reach(robot,range,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"set")&&!strcmp(name,"scanner.set")&&!strcmp(field,"heading")){double value = number();
int32_t status=binding_cy_scanner_heading(robot,value);
if(answer){printf("%s %d",label,status);if(status==CY_OK && 0){putchar(' ');}putchar('\n');}return;}
if(!strcmp(op,"set")&&!strcmp(name,"scanner.set")&&!strcmp(field,"arc")){double value = number();
int32_t status=binding_cy_scanner_arc(robot,value);
if(answer){printf("%s %d",label,status);if(status==CY_OK && 0){putchar(' ');}putchar('\n');}return;}
if(!strcmp(op,"set")&&!strcmp(name,"scanner.set")&&!strcmp(field,"pitch")){double value = number();
int32_t status=binding_cy_scanner_pitch(robot,value);
if(answer){printf("%s %d",label,status);if(status==CY_OK && 0){putchar(' ');}putchar('\n');}return;}
if(!strcmp(op,"get")&&!strcmp(name,"scanner.get")&&!strcmp(field,"heading")){double out=0;
int32_t status=binding_cy_scanner_get_heading(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"get")&&!strcmp(name,"scanner.get")&&!strcmp(field,"arc")){double out=0;
int32_t status=binding_cy_scanner_get_arc(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"get")&&!strcmp(name,"scanner.get")&&!strcmp(field,"pitch")){double out=0;
int32_t status=binding_cy_scanner_get_pitch(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"sensors.contacts")){cy_contact_v1 out[CAP]; uint32_t count=0;
int32_t status=binding_cy_sensors_contacts_v1(robot,out,CAP,&count);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');putchar('[');for(uint32_t i=0;i<count;i++){if(i)putchar(',');print_contact(&out[i]);}putchar(']');}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"sensors.nearest")){cy_contact_v1 out={0};
int32_t status=binding_cy_sensors_nearest_v1(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');print_contact(&out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"sensors.cues")){cy_contact_v1 out[CAP]; uint32_t count=0;
int32_t status=binding_cy_sensors_cues_v1(robot,out,CAP,&count);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');putchar('[');for(uint32_t i=0;i<count;i++){if(i)putchar(',');print_contact(&out[i]);}putchar(']');}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"sensors.contact")){int32_t slot = integer();
cy_contact_v1 out={0};
int32_t status=binding_cy_sensors_contact_v1(robot,slot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');print_contact(&out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"sensors.position_of")){cy_contact_v1 contact; read_contact(&contact);
cy_vec3 out={0};
int32_t status=binding_cy_sensors_position_of_v1(robot,&contact,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');print_vec3(&out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"tracker.cue")){cy_cue_v1 out={0};
int32_t status=binding_cy_tracker_cue_v1(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');print_cue(&out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"tracker.nearest_bearing_rate")){double out=0;
int32_t status=binding_cy_tracker_nearest_bearing_rate(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"tracker.velocity")){int32_t slot = integer();
cy_vec3 out={0};
int32_t status=binding_cy_tracker_velocity(robot,slot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');print_vec3(&out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"tracker.position")){int32_t slot = integer();
cy_vec3 out={0};
int32_t status=binding_cy_tracker_position(robot,slot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');print_vec3(&out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"tracker.last_seen")){int32_t slot = integer();
int32_t out=0;
int32_t status=binding_cy_tracker_last_seen(robot,slot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"tracker.configure")){double smoothing = number();
int32_t forget_ticks = integer();
int32_t cue_ticks = integer();
int32_t status=binding_cy_tracker_configure(robot,smoothing,forget_ticks,cue_ticks);
if(answer){printf("%s %d",label,status);if(status==CY_OK && 0){putchar(' ');}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"nav.zone")){cy_zone_v1 out={0};
int32_t status=binding_cy_nav_zone_v1(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');print_zone(&out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"nav.zone.margin")){cy_vec3 point; read_vec3(&point);
double out=0;
int32_t status=binding_cy_nav_zone_margin(robot,&point,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"nav.zone.contains")){cy_vec3 point; read_vec3(&point);
double margin = number();
int32_t out=0;
int32_t status=binding_cy_nav_zone_contains(robot,&point,margin,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');fputs(out?"t":"f",stdout);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"nav.zone.distance_from")){cy_vec3 point; read_vec3(&point);
double out=0;
int32_t status=binding_cy_nav_zone_distance_from(robot,&point,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"nav.storm")){cy_storm_v1 out={0};
int32_t status=binding_cy_nav_storm_v1(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');print_storm(&out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"nav.storm.margin")){cy_vec3 point; read_vec3(&point);
double out=0;
int32_t status=binding_cy_nav_storm_margin(robot,&point,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"nav.storm.contains")){cy_vec3 point; read_vec3(&point);
double margin = number();
int32_t out=0;
int32_t status=binding_cy_nav_storm_contains(robot,&point,margin,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');fputs(out?"t":"f",stdout);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"nav.storm.distance_from")){cy_vec3 point; read_vec3(&point);
double out=0;
int32_t status=binding_cy_nav_storm_distance_from(robot,&point,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"nav.hill")){cy_hill_v1 out={0};
int32_t status=binding_cy_nav_hill_v1(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');print_hill(&out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"nav.hill.margin")){cy_vec3 point; read_vec3(&point);
double out=0;
int32_t status=binding_cy_nav_hill_margin(robot,&point,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"nav.hill.contains")){cy_vec3 point; read_vec3(&point);
double margin = number();
int32_t out=0;
int32_t status=binding_cy_nav_hill_contains(robot,&point,margin,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');fputs(out?"t":"f",stdout);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"nav.hill.distance_from")){cy_vec3 point; read_vec3(&point);
double out=0;
int32_t status=binding_cy_nav_hill_distance_from(robot,&point,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"nav.pickups")){cy_pickup_v1 out[CAP]; uint32_t count=0;
int32_t status=binding_cy_nav_pickups_v1(robot,out,CAP,&count);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');putchar('[');for(uint32_t i=0;i<count;i++){if(i)putchar(',');print_pickup(&out[i]);}putchar(']');}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"nav.nearest_pickup")){int32_t kind = integer();
cy_pickup_v1 out={0};
int32_t status=binding_cy_nav_nearest_pickup_v1(robot,kind,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');print_pickup(&out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"nav.field_distance")){cy_vec3 a; read_vec3(&a);
cy_vec3 b; read_vec3(&b);
double out=0;
int32_t status=binding_cy_nav_field_distance(robot,&a,&b,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"score.me")){double out=0;
int32_t status=binding_cy_score_me(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"score.best")){double out=0;
int32_t status=binding_cy_score_best(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"score.armour")){double out=0;
int32_t status=binding_cy_score_armour(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"angles.normalize")){double degrees = number();
double out=0;
int32_t status=binding_cy_angles_normalize(degrees,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"angles.angle_diff")){double target = number();
double current = number();
double out=0;
int32_t status=binding_cy_angles_angle_diff(target,current,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"angles.bearing_to")){cy_vec3 from; read_vec3(&from);
cy_vec3 to; read_vec3(&to);
double out=0;
int32_t status=binding_cy_angles_bearing_to(&from,&to,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"angles.elevation_to")){cy_vec3 from; read_vec3(&from);
cy_vec3 to; read_vec3(&to);
double out=0;
int32_t status=binding_cy_angles_elevation_to(&from,&to,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"angles.direction")){double heading = number();
double pitch = number();
cy_vec3 out={0};
int32_t status=binding_cy_angles_direction(heading,pitch,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');print_vec3(&out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"angles.distance")){cy_vec3 a; read_vec3(&a);
cy_vec3 b; read_vec3(&b);
double out=0;
int32_t status=binding_cy_angles_distance(&a,&b,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"angles.clamp")){double value = number();
double low = number();
double high = number();
double out=0;
int32_t status=binding_cy_angles_clamp(value,low,high,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"set")&&!strcmp(name,"status.set")&&!strcmp(field,"text")){uint32_t text_length; char *text=string(&text_length);
int32_t status=binding_cy_status_text(robot,text,text_length);
if(answer){printf("%s %d",label,status);if(status==CY_OK && 0){putchar(' ');}putchar('\n');}return;}
if(!strcmp(op,"set")&&!strcmp(name,"status.set")&&!strcmp(field,"value")){double value = number();
int32_t status=binding_cy_status_value(robot,value);
if(answer){printf("%s %d",label,status);if(status==CY_OK && 0){putchar(' ');}putchar('\n');}return;}
if(!strcmp(op,"get")&&!strcmp(name,"status.get")&&!strcmp(field,"text")){char buf[CAP]; uint32_t count=0;
int32_t status=binding_cy_status_get_text(robot,buf,CAP,&count);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');text(buf, count);}putchar('\n');}return;}
if(!strcmp(op,"get")&&!strcmp(name,"status.get")&&!strcmp(field,"value")){double out=0;
int32_t status=binding_cy_status_get_value(robot,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"raw.number")){uint32_t path_length; char *path=string(&path_length);
double out=0;
int32_t status=binding_cy_raw_number(robot,path,path_length,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"raw.text")){uint32_t path_length; char *path=string(&path_length);
char buf[CAP]; uint32_t count=0;
int32_t status=binding_cy_raw_text(robot,path,path_length,buf,CAP,&count);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');text(buf, count);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"raw.has")){uint32_t path_length; char *path=string(&path_length);
int32_t out=0;
int32_t status=binding_cy_raw_has(robot,path,path_length,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');fputs(out?"t":"f",stdout);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"raw.flag")){uint32_t path_length; char *path=string(&path_length);
int32_t out=0;
int32_t status=binding_cy_raw_flag(robot,path,path_length,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');fputs(out?"t":"f",stdout);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"raw.count")){uint32_t path_length; char *path=string(&path_length);
int32_t out=0;
int32_t status=binding_cy_raw_count(robot,path,path_length,&out);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');bits((double)out);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"raw.line")){char buf[CAP]; uint32_t count=0;
int32_t status=binding_cy_raw_line(robot,buf,CAP,&count);
if(answer){printf("%s %d",label,status);if(status==CY_OK){putchar(' ');text(buf, count);}putchar('\n');}return;}
if(!strcmp(op,"call")&&!strcmp(name,"raw.extra")){uint32_t json_length; char *json=string(&json_length);
int32_t status=binding_cy_raw_extra(robot,json,json_length);
if(answer){printf("%s %d",label,status);if(status==CY_OK && 0){putchar(' ');}putchar('\n');}return;}fprintf(stderr,"no call: %s %s %s\n",op,name,field);exit(2);}
int main(void){char *line=NULL;size_t capacity=0;while(getline(&line,&capacity,stdin)>=0){line[strcspn(line,"\r\n")]=0;cursor=line;char *op=token();
if(!strcmp(op,"open")){if(cy_open_memory(2,0,"clunkyard-c-test",16,"2.0.0",5,&robot)!=CY_OK)return 2;continue;}
if(!strcmp(op,"close")){cy_close(robot);robot=NULL;continue;}
if(!strcmp(op,"feed")){uint32_t length;char *value=string(&length);if(cy_session_feed(robot,value,length)<0)return 2;continue;}
if(!strcmp(op,"next")){int32_t event;if(cy_session_next(robot,&event)<0)return 2;continue;}
if(!strcmp(op,"send")){if(cy_session_send(robot)<0)return 2;continue;}
if(!strcmp(op,"reply")){char buf[CAP];uint32_t length;cy_session_reply(robot,buf,CAP,&length);continue;}
char *label=token();
if(!strcmp(op,"act")){char buf[CAP];uint32_t length;cy_session_send(robot);int status=cy_session_reply(robot,buf,CAP,&length);if(status!=CY_OK)return 2;printf("%s ",label);text(buf,length);putchar('\n');continue;}
if(!strcmp(op,"slots")){int32_t count;binding_cy_match_robots(robot,&count);printf("%s [",label);for(int32_t i=1;i<=count;i++){char buf[CAP];uint32_t length;binding_cy_robot_id(robot,i,buf,CAP,&length);if(i>1)putchar(',');text(buf,length);}puts("]");continue;}
char *name=token();const char *field="";
if(!strcmp(op,"set")||!strcmp(op,"get")){field=token();if(!*field){if(!strcmp(name,"gun.fire")){binding_cy_gun_fire(robot);if(strcmp(label,"-"))printf("%s 0\n",label);continue;}}}
invoke(op,label,name,field);
if(!strcmp(op,"set")){if(!strcmp(name,"gun.fire")){binding_cy_gun_fire(robot);}}
}free(line);cy_close(robot);fflush(stdout);return 0;}
