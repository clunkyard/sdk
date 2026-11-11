// Generated from protocol/v2/api.json; do not edit. Run make api.
#![allow(dead_code,unused_mut)]
use std::ffi::c_void;
use std::fmt;
use std::ops::Deref;
pub const OK:i32=0;
pub const NONE:i32=1;
pub const SHORT:i32=2;
pub const E_NOT_FINITE:i32=-1;
pub const E_RANGE:i32=-2;
pub const E_ARGUMENT:i32=-3;
pub const E_PHASE:i32=-4;
pub const E_VERSION:i32=-5;
pub const E_INTERNAL:i32=-6;
pub const SOURCE_SCAN:i32=1;
pub const SOURCE_PROXIMITY:i32=2;
pub const SOURCE_FAINT:i32=3;
pub const SOURCE_BEAM:i32=4;
pub const SOURCE_MUZZLE:i32=5;
pub const SOURCE_MOTION:i32=6;
pub const SOURCE_PING:i32=7;
pub const MODE_TWO_D:i32=1;
pub const MODE_THREE_D:i32=2;
pub const RULESET_CLASSIC:i32=1;
pub const RULESET_INERTIAL_2D:i32=2;
pub const RULESET_GRAVITY_FLAT:i32=3;
pub const RULESET_GRAVITY_HILLS:i32=4;
pub const RULESET_ZERO_G:i32=5;
pub const CONTROL_DIRECT:i32=1;
pub const CONTROL_FORCES:i32=2;
pub const TERRAIN_KIND_NONE:i32=1;
pub const TERRAIN_KIND_FLAT:i32=2;
pub const TERRAIN_KIND_HILLS:i32=3;
pub const SHAPE_CIRCLE:i32=1;
pub const SHAPE_SPHERE:i32=2;
pub const SHAPE_COLUMN:i32=3;
pub const PICKUP_KIND_ENERGY:i32=1;
pub const PICKUP_KIND_HEALTH:i32=2;
pub const PICKUP_KIND_ARMOUR:i32=3;
pub const COMPUTE_UNIT_OPERATIONS:i32=1;
pub const COMPUTE_UNIT_FUEL:i32=2;
pub const COMPUTE_UNIT_CPU_MS:i32=3;
pub const EVENT_HELLO:i32=1;
pub const EVENT_STATE:i32=2;
pub const EVENT_END:i32=3;
pub const VERSION:&str="2.0.0";
#[repr(C)] #[derive(Default,Clone,Copy,Debug,PartialEq)]
pub struct RawVec3 {pub x:f64,pub y:f64,pub z:f64,}
pub type Vec3=RawVec3;
#[repr(C)] #[derive(Default,Clone,Copy,Debug,PartialEq)]
pub struct RawRotation {pub yaw:f64,pub pitch:f64,pub roll:f64,}
pub type Rotation=RawRotation;
#[repr(C)] #[derive(Default,Clone,Copy,Debug,PartialEq)]
pub struct RawDirection {pub heading:f64,pub pitch:f64,}
pub type Direction=RawDirection;
#[repr(C)] #[derive(Default,Clone,Copy,Debug,PartialEq)]
pub struct RawAttitude {pub forward:RawVec3,pub right:RawVec3,pub up:RawVec3,}
pub type Attitude=RawAttitude;
#[repr(C)] #[derive(Default,Clone,Copy,Debug,PartialEq)]
pub struct RawApplied {pub force:RawVec3,pub torque:RawRotation,pub brake:f64,}
pub type Applied=RawApplied;
#[repr(C)] #[derive(Default,Clone,Copy,Debug,PartialEq)]
pub struct RawScan {pub heading:f64,pub pitch:f64,pub arc:f64,pub range:f64,}
pub type Scan=RawScan;
#[repr(C)] #[derive(Default,Clone,Copy,Debug,PartialEq)]
pub struct RawContact {pub slot:i32,pub source:i32,pub range:f64,pub bearing:f64,pub elevation:f64,pub bearing_error:f64,pub range_error:f64,pub position:RawVec3,}
pub type Contact=RawContact;
#[repr(C)] #[derive(Default,Clone,Copy,Debug,PartialEq)]
pub struct RawCue {pub source:i32,pub tick:i32,pub bearing_error:f64,pub position:RawVec3,}
pub type Cue=RawCue;
#[repr(C)] #[derive(Default,Clone,Copy,Debug,PartialEq)]
pub struct RawSolution {pub heading:f64,pub pitch:f64,pub time:f64,}
pub type Solution=RawSolution;
#[repr(C)] #[derive(Default,Clone,Copy,Debug,PartialEq)]
pub struct RawZone {pub shape:i32,pub reserved:i32,pub radius:f64,pub center:RawVec3,}
#[derive(Clone,Copy,Debug)] pub struct Zone{pub raw:RawZone,handle:*mut c_void}
impl Deref for Zone {type Target=RawZone;fn deref(&self)->&Self::Target{&self.raw}}
impl Zone{
pub fn margin(&self,point: &Vec3) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_nav_zone_margin(self.handle,point,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn contains(&self,point: &Vec3,margin: f64) -> Result<Option<bool>,Error> {let mut out=i32::default();let mut status=unsafe{ffi::cy_nav_zone_contains(self.handle,point,margin,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out!=0))}
pub fn distance_from(&self,point: &Vec3) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_nav_zone_distance_from(self.handle,point,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
}
#[repr(C)] #[derive(Default,Clone,Copy,Debug,PartialEq)]
pub struct RawStorm {pub shape:i32,pub reserved:i32,pub radius:f64,pub dps:f64,pub center:RawVec3,pub velocity:RawVec3,}
#[derive(Clone,Copy,Debug)] pub struct Storm{pub raw:RawStorm,handle:*mut c_void}
impl Deref for Storm {type Target=RawStorm;fn deref(&self)->&Self::Target{&self.raw}}
impl Storm{
pub fn margin(&self,point: &Vec3) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_nav_storm_margin(self.handle,point,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn contains(&self,point: &Vec3,margin: f64) -> Result<Option<bool>,Error> {let mut out=i32::default();let mut status=unsafe{ffi::cy_nav_storm_contains(self.handle,point,margin,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out!=0))}
pub fn distance_from(&self,point: &Vec3) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_nav_storm_distance_from(self.handle,point,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
}
#[repr(C)] #[derive(Default,Clone,Copy,Debug,PartialEq)]
pub struct RawHill {pub shape:i32,pub holder:i32,pub contested:i32,pub moves_tick:i32,pub held_ticks:i32,pub best_held_ticks:i32,pub radius:f64,pub center:RawVec3,pub next:RawVec3,}
#[derive(Clone,Copy,Debug)] pub struct Hill{pub raw:RawHill,handle:*mut c_void}
impl Deref for Hill {type Target=RawHill;fn deref(&self)->&Self::Target{&self.raw}}
impl Hill{
pub fn margin(&self,point: &Vec3) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_nav_hill_margin(self.handle,point,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn contains(&self,point: &Vec3,margin: f64) -> Result<Option<bool>,Error> {let mut out=i32::default();let mut status=unsafe{ffi::cy_nav_hill_contains(self.handle,point,margin,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out!=0))}
pub fn distance_from(&self,point: &Vec3) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_nav_hill_distance_from(self.handle,point,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
}
#[repr(C)] #[derive(Default,Clone,Copy,Debug,PartialEq)]
pub struct RawPickup {pub kind:i32,pub expires_tick:i32,pub amount:f64,pub position:RawVec3,}
pub type Pickup=RawPickup;
#[repr(C)] #[derive(Default,Clone,Copy,Debug,PartialEq)]
pub struct RawCompute {pub k:f64,pub quanta:i32,pub unit:i32,}
pub type Compute=RawCompute;
pub mod ffi {use super::*;#[link(name="clunkyard",kind="static")] extern "C" {
pub fn cy_open(major:i32,minor:i32,name:*const u8,name_length:u32,version:*const u8,version_length:u32,out:*mut *mut c_void)->i32;
pub fn cy_open_memory(major:i32,minor:i32,name:*const u8,name_length:u32,version:*const u8,version_length:u32,out:*mut *mut c_void)->i32;
pub fn cy_close(robot:*mut c_void);pub fn cy_last_error(robot:*mut c_void,out:*mut u8,capacity:u32,length:*mut u32)->i32;
pub fn cy_match_mode(robot:*mut std::ffi::c_void,out:*mut i32)->i32;
pub fn cy_match_ruleset(robot:*mut std::ffi::c_void,out:*mut i32)->i32;
pub fn cy_match_control(robot:*mut std::ffi::c_void,out:*mut i32)->i32;
pub fn cy_match_robots(robot:*mut std::ffi::c_void,out:*mut i32)->i32;
pub fn cy_match_is_3d(robot:*mut std::ffi::c_void,out:*mut i32)->i32;
pub fn cy_match_tick_rate(robot:*mut std::ffi::c_void,out:*mut i32)->i32;
pub fn cy_match_max_ticks(robot:*mut std::ffi::c_void,out:*mut i32)->i32;
pub fn cy_robot_id(robot:*mut std::ffi::c_void,slot:i32,buf:*mut u8,capacity:u32,length:*mut u32)->i32;
pub fn cy_clock_tick(robot:*mut std::ffi::c_void,out:*mut i32)->i32;
pub fn cy_clock_seconds(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_clock_remaining_ticks(robot:*mut std::ffi::c_void,out:*mut i32)->i32;
pub fn cy_arena_size(robot:*mut std::ffi::c_void,out:*mut RawVec3)->i32;
pub fn cy_arena_gravity(robot:*mut std::ffi::c_void,out:*mut RawVec3)->i32;
pub fn cy_arena_terrain_kind(robot:*mut std::ffi::c_void,out:*mut i32)->i32;
pub fn cy_arena_ground_height(robot:*mut std::ffi::c_void,x:f64,y:f64,out:*mut f64)->i32;
pub fn cy_me_health(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_me_position(robot:*mut std::ffi::c_void,out:*mut RawVec3)->i32;
pub fn cy_me_velocity(robot:*mut std::ffi::c_void,out:*mut RawVec3)->i32;
pub fn cy_me_heading(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_me_pitch(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_me_roll(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_me_angular_velocity(robot:*mut std::ffi::c_void,out:*mut RawRotation)->i32;
pub fn cy_me_attitude_v1(robot:*mut std::ffi::c_void,out:*mut RawAttitude)->i32;
pub fn cy_me_grounded(robot:*mut std::ffi::c_void,out:*mut i32)->i32;
pub fn cy_me_altitude(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_me_applied_v1(robot:*mut std::ffi::c_void,out:*mut RawApplied)->i32;
pub fn cy_battery_charge(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_battery_capacity(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_battery_regen(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_battery_sustainable_drive(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_battery_compute_v1(robot:*mut std::ffi::c_void,out:*mut RawCompute)->i32;
pub fn cy_battery_drive_cost(robot:*mut std::ffi::c_void,fraction:f64,out:*mut f64)->i32;
pub fn cy_drive_hover_lift(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_drive_throttle(robot:*mut std::ffi::c_void,value:f64)->i32;
pub fn cy_drive_turn(robot:*mut std::ffi::c_void,value:f64)->i32;
pub fn cy_drive_pitch_turn(robot:*mut std::ffi::c_void,value:f64)->i32;
pub fn cy_drive_thrust(robot:*mut std::ffi::c_void,value:f64)->i32;
pub fn cy_drive_strafe(robot:*mut std::ffi::c_void,value:f64)->i32;
pub fn cy_drive_lift(robot:*mut std::ffi::c_void,value:f64)->i32;
pub fn cy_drive_yaw_torque(robot:*mut std::ffi::c_void,value:f64)->i32;
pub fn cy_drive_pitch_torque(robot:*mut std::ffi::c_void,value:f64)->i32;
pub fn cy_drive_roll_torque(robot:*mut std::ffi::c_void,value:f64)->i32;
pub fn cy_drive_brake(robot:*mut std::ffi::c_void,value:f64)->i32;
pub fn cy_drive_get_throttle(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_drive_get_turn(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_drive_get_pitch_turn(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_drive_get_thrust(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_drive_get_strafe(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_drive_get_lift(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_drive_get_yaw_torque(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_drive_get_pitch_torque(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_drive_get_roll_torque(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_drive_get_brake(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_gun_aim_now(robot:*mut std::ffi::c_void,out:*mut RawDirection)->i32;
pub fn cy_gun_cooldown_ticks(robot:*mut std::ffi::c_void,out:*mut i32)->i32;
pub fn cy_gun_spread(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_gun_max_power(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_gun_damage_scale(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_gun_can_fire(robot:*mut std::ffi::c_void,power:f64,reserve:f64,out:*mut i32)->i32;
pub fn cy_gun_shot_speed(robot:*mut std::ffi::c_void,power:f64,out:*mut f64)->i32;
pub fn cy_gun_shot_cost(robot:*mut std::ffi::c_void,power:f64,out:*mut f64)->i32;
pub fn cy_gun_reload_ticks(robot:*mut std::ffi::c_void,power:f64,out:*mut f64)->i32;
pub fn cy_gun_shot_damage(robot:*mut std::ffi::c_void,power:f64,out:*mut f64)->i32;
pub fn cy_gun_power_for_speed(robot:*mut std::ffi::c_void,speed:f64,out:*mut f64)->i32;
pub fn cy_gun_power_for_flight_time(robot:*mut std::ffi::c_void,range:f64,seconds:f64,out:*mut f64)->i32;
pub fn cy_gun_lead_v1(robot:*mut std::ffi::c_void,contact:*const RawContact,power:f64,out:*mut RawSolution)->i32;
pub fn cy_gun_lead_point_v1(robot:*mut std::ffi::c_void,position:*const RawVec3,velocity:*const RawVec3,power:f64,out:*mut RawSolution)->i32;
pub fn cy_gun_aim_heading(robot:*mut std::ffi::c_void,value:f64)->i32;
pub fn cy_gun_aim_pitch(robot:*mut std::ffi::c_void,value:f64)->i32;
pub fn cy_gun_fire(robot:*mut std::ffi::c_void)->i32;
pub fn cy_gun_power(robot:*mut std::ffi::c_void,value:f64)->i32;
pub fn cy_gun_get_aim_heading(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_gun_get_aim_pitch(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_gun_get_fire(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_gun_get_power(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_scanner_last_v1(robot:*mut std::ffi::c_void,out:*mut RawScan)->i32;
pub fn cy_scanner_proximity_range(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_scanner_reach(robot:*mut std::ffi::c_void,arc:f64,out:*mut f64)->i32;
pub fn cy_scanner_arc_for_reach(robot:*mut std::ffi::c_void,range:f64,out:*mut f64)->i32;
pub fn cy_scanner_heading(robot:*mut std::ffi::c_void,value:f64)->i32;
pub fn cy_scanner_arc(robot:*mut std::ffi::c_void,value:f64)->i32;
pub fn cy_scanner_pitch(robot:*mut std::ffi::c_void,value:f64)->i32;
pub fn cy_scanner_get_heading(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_scanner_get_arc(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_scanner_get_pitch(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_sensors_contacts_v1(robot:*mut std::ffi::c_void,out:*mut RawContact,capacity:u32,count:*mut u32)->i32;
pub fn cy_sensors_nearest_v1(robot:*mut std::ffi::c_void,out:*mut RawContact)->i32;
pub fn cy_sensors_cues_v1(robot:*mut std::ffi::c_void,out:*mut RawContact,capacity:u32,count:*mut u32)->i32;
pub fn cy_sensors_contact_v1(robot:*mut std::ffi::c_void,slot:i32,out:*mut RawContact)->i32;
pub fn cy_sensors_position_of_v1(robot:*mut std::ffi::c_void,contact:*const RawContact,out:*mut RawVec3)->i32;
pub fn cy_tracker_cue_v1(robot:*mut std::ffi::c_void,out:*mut RawCue)->i32;
pub fn cy_tracker_nearest_bearing_rate(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_tracker_velocity(robot:*mut std::ffi::c_void,slot:i32,out:*mut RawVec3)->i32;
pub fn cy_tracker_position(robot:*mut std::ffi::c_void,slot:i32,out:*mut RawVec3)->i32;
pub fn cy_tracker_last_seen(robot:*mut std::ffi::c_void,slot:i32,out:*mut i32)->i32;
pub fn cy_tracker_configure(robot:*mut std::ffi::c_void,smoothing:f64,forget_ticks:i32,cue_ticks:i32)->i32;
pub fn cy_nav_zone_v1(robot:*mut std::ffi::c_void,out:*mut RawZone)->i32;
pub fn cy_nav_zone_margin(robot:*mut std::ffi::c_void,point:*const RawVec3,out:*mut f64)->i32;
pub fn cy_nav_zone_contains(robot:*mut std::ffi::c_void,point:*const RawVec3,margin:f64,out:*mut i32)->i32;
pub fn cy_nav_zone_distance_from(robot:*mut std::ffi::c_void,point:*const RawVec3,out:*mut f64)->i32;
pub fn cy_nav_storm_v1(robot:*mut std::ffi::c_void,out:*mut RawStorm)->i32;
pub fn cy_nav_storm_margin(robot:*mut std::ffi::c_void,point:*const RawVec3,out:*mut f64)->i32;
pub fn cy_nav_storm_contains(robot:*mut std::ffi::c_void,point:*const RawVec3,margin:f64,out:*mut i32)->i32;
pub fn cy_nav_storm_distance_from(robot:*mut std::ffi::c_void,point:*const RawVec3,out:*mut f64)->i32;
pub fn cy_nav_hill_v1(robot:*mut std::ffi::c_void,out:*mut RawHill)->i32;
pub fn cy_nav_hill_margin(robot:*mut std::ffi::c_void,point:*const RawVec3,out:*mut f64)->i32;
pub fn cy_nav_hill_contains(robot:*mut std::ffi::c_void,point:*const RawVec3,margin:f64,out:*mut i32)->i32;
pub fn cy_nav_hill_distance_from(robot:*mut std::ffi::c_void,point:*const RawVec3,out:*mut f64)->i32;
pub fn cy_nav_pickups_v1(robot:*mut std::ffi::c_void,out:*mut RawPickup,capacity:u32,count:*mut u32)->i32;
pub fn cy_nav_nearest_pickup_v1(robot:*mut std::ffi::c_void,kind:i32,out:*mut RawPickup)->i32;
pub fn cy_nav_field_distance(robot:*mut std::ffi::c_void,a:*const RawVec3,b:*const RawVec3,out:*mut f64)->i32;
pub fn cy_score_me(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_score_best(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_score_armour(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_angles_normalize(degrees:f64,out:*mut f64)->i32;
pub fn cy_angles_angle_diff(target:f64,current:f64,out:*mut f64)->i32;
pub fn cy_angles_bearing_to(from:*const RawVec3,to:*const RawVec3,out:*mut f64)->i32;
pub fn cy_angles_elevation_to(from:*const RawVec3,to:*const RawVec3,out:*mut f64)->i32;
pub fn cy_angles_direction(heading:f64,pitch:f64,out:*mut RawVec3)->i32;
pub fn cy_angles_distance(a:*const RawVec3,b:*const RawVec3,out:*mut f64)->i32;
pub fn cy_angles_clamp(value:f64,low:f64,high:f64,out:*mut f64)->i32;
pub fn cy_status_text(robot:*mut std::ffi::c_void,text:*const u8,text_length:u32)->i32;
pub fn cy_status_value(robot:*mut std::ffi::c_void,value:f64)->i32;
pub fn cy_status_get_text(robot:*mut std::ffi::c_void,buf:*mut u8,capacity:u32,length:*mut u32)->i32;
pub fn cy_status_get_value(robot:*mut std::ffi::c_void,out:*mut f64)->i32;
pub fn cy_raw_number(robot:*mut std::ffi::c_void,path:*const u8,path_length:u32,out:*mut f64)->i32;
pub fn cy_raw_text(robot:*mut std::ffi::c_void,path:*const u8,path_length:u32,buf:*mut u8,capacity:u32,length:*mut u32)->i32;
pub fn cy_raw_has(robot:*mut std::ffi::c_void,path:*const u8,path_length:u32,out:*mut i32)->i32;
pub fn cy_raw_flag(robot:*mut std::ffi::c_void,path:*const u8,path_length:u32,out:*mut i32)->i32;
pub fn cy_raw_count(robot:*mut std::ffi::c_void,path:*const u8,path_length:u32,out:*mut i32)->i32;
pub fn cy_raw_line(robot:*mut std::ffi::c_void,buf:*mut u8,capacity:u32,length:*mut u32)->i32;
pub fn cy_raw_extra(robot:*mut std::ffi::c_void,json:*const u8,json_length:u32)->i32;
pub fn cy_session_event(robot:*mut std::ffi::c_void,out:*mut i32)->i32;
pub fn cy_session_next(robot:*mut std::ffi::c_void,out:*mut i32)->i32;
pub fn cy_session_send(robot:*mut std::ffi::c_void)->i32;
pub fn cy_session_forfeit(robot:*mut std::ffi::c_void,code:*const u8,code_length:u32,message:*const u8,message_length:u32)->i32;
pub fn cy_session_log(robot:*mut std::ffi::c_void,line:*const u8,line_length:u32)->i32;
pub fn cy_session_feed(robot:*mut std::ffi::c_void,line:*const u8,line_length:u32)->i32;
pub fn cy_session_reply(robot:*mut std::ffi::c_void,buf:*mut u8,capacity:u32,length:*mut u32)->i32;
}}

#[derive(Debug)] pub struct Error{pub status:i32,pub message:String,forfeit:Option<(String,String)>}
impl Error{pub fn new(status:i32,message:&str)->Self{Self{status,message:message.into(),forfeit:None}}pub fn forfeit(code:&str,message:&str)->Self{Self{status:0,message:message.into(),forfeit:Some((code.into(),message.into()))}}}
impl fmt::Display for Error{fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{write!(f,"{}",self.message)}}impl std::error::Error for Error{}
fn check(handle:*mut c_void,status:i32)->Result<(),Error>{if status>=0{return Ok(());}let mut bytes=vec![0u8;4096];let mut length=0;unsafe{ffi::cy_last_error(handle,bytes.as_mut_ptr(),bytes.len() as u32,&mut length);}bytes.truncate(length as usize);Err(Error::new(status,&String::from_utf8_lossy(&bytes)))}

#[derive(Clone,Copy)] pub struct Match {handle:*mut c_void} impl Match {
pub fn mode(&self) -> Result<i32,Error> {let mut out=i32::default();let mut status=unsafe{ffi::cy_match_mode(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn ruleset(&self) -> Result<i32,Error> {let mut out=i32::default();let mut status=unsafe{ffi::cy_match_ruleset(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn control(&self) -> Result<i32,Error> {let mut out=i32::default();let mut status=unsafe{ffi::cy_match_control(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn robots(&self) -> Result<i32,Error> {let mut out=i32::default();let mut status=unsafe{ffi::cy_match_robots(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn is_3d(&self) -> Result<bool,Error> {let mut out=i32::default();let mut status=unsafe{ffi::cy_match_is_3d(self.handle,&mut out)};check(self.handle,status)?;Ok(out!=0)}
pub fn tick_rate(&self) -> Result<i32,Error> {let mut out=i32::default();let mut status=unsafe{ffi::cy_match_tick_rate(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn max_ticks(&self) -> Result<i32,Error> {let mut out=i32::default();let mut status=unsafe{ffi::cy_match_max_ticks(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn robot_id(&self,slot: i32) -> Result<String,Error> {let mut out=vec![0u8;256];let mut count=0;let mut status=unsafe{ffi::cy_robot_id(self.handle,slot,out.as_mut_ptr(),out.len() as u32,&mut count)};while status==SHORT {out.resize(count as usize,0);status=unsafe{ffi::cy_robot_id(self.handle,slot,out.as_mut_ptr(),out.len() as u32,&mut count)};}check(self.handle,status)?;out.truncate(count as usize);Ok(String::from_utf8(out).map_err(|_|Error::new(E_ARGUMENT,"core returned invalid UTF-8"))?)}
}
#[derive(Clone,Copy)] pub struct Clock {handle:*mut c_void} impl Clock {
pub fn tick(&self) -> Result<i32,Error> {let mut out=i32::default();let mut status=unsafe{ffi::cy_clock_tick(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn seconds(&self) -> Result<f64,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_clock_seconds(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn remaining_ticks(&self) -> Result<i32,Error> {let mut out=i32::default();let mut status=unsafe{ffi::cy_clock_remaining_ticks(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
}
#[derive(Clone,Copy)] pub struct Arena {handle:*mut c_void} impl Arena {
pub fn size(&self) -> Result<Vec3,Error> {let mut out=RawVec3::default();let mut status=unsafe{ffi::cy_arena_size(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn gravity(&self) -> Result<Vec3,Error> {let mut out=RawVec3::default();let mut status=unsafe{ffi::cy_arena_gravity(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn terrain_kind(&self) -> Result<i32,Error> {let mut out=i32::default();let mut status=unsafe{ffi::cy_arena_terrain_kind(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn ground_height(&self,x: f64,y: f64) -> Result<f64,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_arena_ground_height(self.handle,x,y,&mut out)};check(self.handle,status)?;Ok(out)}
}
#[derive(Clone,Copy)] pub struct Me {handle:*mut c_void} impl Me {
pub fn health(&self) -> Result<f64,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_me_health(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn position(&self) -> Result<Vec3,Error> {let mut out=RawVec3::default();let mut status=unsafe{ffi::cy_me_position(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn velocity(&self) -> Result<Vec3,Error> {let mut out=RawVec3::default();let mut status=unsafe{ffi::cy_me_velocity(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn heading(&self) -> Result<f64,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_me_heading(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn pitch(&self) -> Result<f64,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_me_pitch(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn roll(&self) -> Result<f64,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_me_roll(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn angular_velocity(&self) -> Result<Rotation,Error> {let mut out=RawRotation::default();let mut status=unsafe{ffi::cy_me_angular_velocity(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn attitude(&self) -> Result<Attitude,Error> {let mut out=RawAttitude::default();let mut status=unsafe{ffi::cy_me_attitude_v1(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn grounded(&self) -> Result<bool,Error> {let mut out=i32::default();let mut status=unsafe{ffi::cy_me_grounded(self.handle,&mut out)};check(self.handle,status)?;Ok(out!=0)}
pub fn altitude(&self) -> Result<f64,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_me_altitude(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn applied(&self) -> Result<Applied,Error> {let mut out=RawApplied::default();let mut status=unsafe{ffi::cy_me_applied_v1(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
}
#[derive(Clone,Copy)] pub struct Battery {handle:*mut c_void} impl Battery {
pub fn charge(&self) -> Result<f64,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_battery_charge(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn capacity(&self) -> Result<f64,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_battery_capacity(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn regen(&self) -> Result<f64,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_battery_regen(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn sustainable_drive(&self) -> Result<f64,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_battery_sustainable_drive(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn compute(&self) -> Result<Option<Compute>,Error> {let mut out=RawCompute::default();let mut status=unsafe{ffi::cy_battery_compute_v1(self.handle,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn drive_cost(&self,fraction: f64) -> Result<f64,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_battery_drive_cost(self.handle,fraction,&mut out)};check(self.handle,status)?;Ok(out)}
}
#[derive(Clone,Copy)] pub struct Drive {handle:*mut c_void} impl Drive {
pub fn hover_lift(&self) -> Result<f64,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_drive_hover_lift(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn set(&self,throttle:Option<f64>,turn:Option<f64>,pitch_turn:Option<f64>)->Result<(),Error>{if let Some(value)=throttle{check(self.handle,unsafe{ffi::cy_drive_throttle(self.handle,value)})?;}if let Some(value)=turn{check(self.handle,unsafe{ffi::cy_drive_turn(self.handle,value)})?;}if let Some(value)=pitch_turn{check(self.handle,unsafe{ffi::cy_drive_pitch_turn(self.handle,value)})?;}Ok(())}
pub fn forces(&self,thrust:Option<f64>,strafe:Option<f64>,lift:Option<f64>,yaw_torque:Option<f64>,pitch_torque:Option<f64>,roll_torque:Option<f64>,brake:Option<f64>)->Result<(),Error>{if let Some(value)=thrust{check(self.handle,unsafe{ffi::cy_drive_thrust(self.handle,value)})?;}if let Some(value)=strafe{check(self.handle,unsafe{ffi::cy_drive_strafe(self.handle,value)})?;}if let Some(value)=lift{check(self.handle,unsafe{ffi::cy_drive_lift(self.handle,value)})?;}if let Some(value)=yaw_torque{check(self.handle,unsafe{ffi::cy_drive_yaw_torque(self.handle,value)})?;}if let Some(value)=pitch_torque{check(self.handle,unsafe{ffi::cy_drive_pitch_torque(self.handle,value)})?;}if let Some(value)=roll_torque{check(self.handle,unsafe{ffi::cy_drive_roll_torque(self.handle,value)})?;}if let Some(value)=brake{check(self.handle,unsafe{ffi::cy_drive_brake(self.handle,value)})?;}Ok(())}
pub fn get_throttle(&self) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_drive_get_throttle(self.handle,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn get_turn(&self) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_drive_get_turn(self.handle,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn get_pitch_turn(&self) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_drive_get_pitch_turn(self.handle,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn get_thrust(&self) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_drive_get_thrust(self.handle,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn get_strafe(&self) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_drive_get_strafe(self.handle,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn get_lift(&self) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_drive_get_lift(self.handle,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn get_yaw_torque(&self) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_drive_get_yaw_torque(self.handle,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn get_pitch_torque(&self) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_drive_get_pitch_torque(self.handle,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn get_roll_torque(&self) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_drive_get_roll_torque(self.handle,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn get_brake(&self) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_drive_get_brake(self.handle,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
}
#[derive(Clone,Copy)] pub struct Gun {handle:*mut c_void} impl Gun {
pub fn aim_now(&self) -> Result<Direction,Error> {let mut out=RawDirection::default();let mut status=unsafe{ffi::cy_gun_aim_now(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn cooldown_ticks(&self) -> Result<i32,Error> {let mut out=i32::default();let mut status=unsafe{ffi::cy_gun_cooldown_ticks(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn spread(&self) -> Result<f64,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_gun_spread(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn max_power(&self) -> Result<f64,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_gun_max_power(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn damage_scale(&self) -> Result<f64,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_gun_damage_scale(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn can_fire(&self,power: f64,reserve: f64) -> Result<bool,Error> {let mut out=i32::default();let mut status=unsafe{ffi::cy_gun_can_fire(self.handle,power,reserve,&mut out)};check(self.handle,status)?;Ok(out!=0)}
pub fn shot_speed(&self,power: f64) -> Result<f64,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_gun_shot_speed(self.handle,power,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn shot_cost(&self,power: f64) -> Result<f64,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_gun_shot_cost(self.handle,power,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn reload_ticks(&self,power: f64) -> Result<f64,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_gun_reload_ticks(self.handle,power,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn shot_damage(&self,power: f64) -> Result<f64,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_gun_shot_damage(self.handle,power,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn power_for_speed(&self,speed: f64) -> Result<f64,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_gun_power_for_speed(self.handle,speed,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn power_for_flight_time(&self,range: f64,seconds: f64) -> Result<f64,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_gun_power_for_flight_time(self.handle,range,seconds,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn lead(&self,contact: &Contact,power: f64) -> Result<Option<Solution>,Error> {let mut out=RawSolution::default();let mut status=unsafe{ffi::cy_gun_lead_v1(self.handle,contact,power,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn lead_point(&self,position: &Vec3,velocity: &Vec3,power: f64) -> Result<Option<Solution>,Error> {let mut out=RawSolution::default();let mut status=unsafe{ffi::cy_gun_lead_point_v1(self.handle,position,velocity,power,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn aim(&self,heading:Option<f64>,pitch:Option<f64>)->Result<(),Error>{if let Some(value)=heading{check(self.handle,unsafe{ffi::cy_gun_aim_heading(self.handle,value)})?;}if let Some(value)=pitch{check(self.handle,unsafe{ffi::cy_gun_aim_pitch(self.handle,value)})?;}Ok(())}
pub fn fire(&self,power:Option<f64>)->Result<(),Error>{check(self.handle,unsafe{ffi::cy_gun_fire(self.handle)})?;if let Some(value)=power{check(self.handle,unsafe{ffi::cy_gun_power(self.handle,value)})?;}Ok(())}
pub fn get_heading(&self) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_gun_get_aim_heading(self.handle,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn get_pitch(&self) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_gun_get_aim_pitch(self.handle,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn get_fire(&self) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_gun_get_fire(self.handle,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn get_power(&self) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_gun_get_power(self.handle,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
}
#[derive(Clone,Copy)] pub struct Scanner {handle:*mut c_void} impl Scanner {
pub fn last(&self) -> Result<Scan,Error> {let mut out=RawScan::default();let mut status=unsafe{ffi::cy_scanner_last_v1(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn proximity_range(&self) -> Result<f64,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_scanner_proximity_range(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn reach(&self,arc: f64) -> Result<f64,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_scanner_reach(self.handle,arc,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn arc_for_reach(&self,range: f64) -> Result<f64,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_scanner_arc_for_reach(self.handle,range,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn set(&self,heading:Option<f64>,arc:Option<f64>,pitch:Option<f64>)->Result<(),Error>{if let Some(value)=heading{check(self.handle,unsafe{ffi::cy_scanner_heading(self.handle,value)})?;}if let Some(value)=arc{check(self.handle,unsafe{ffi::cy_scanner_arc(self.handle,value)})?;}if let Some(value)=pitch{check(self.handle,unsafe{ffi::cy_scanner_pitch(self.handle,value)})?;}Ok(())}
pub fn get_heading(&self) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_scanner_get_heading(self.handle,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn get_arc(&self) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_scanner_get_arc(self.handle,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn get_pitch(&self) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_scanner_get_pitch(self.handle,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
}
#[derive(Clone,Copy)] pub struct Sensors {handle:*mut c_void} impl Sensors {
pub fn contacts(&self) -> Result<Vec<Contact>,Error> {let mut out=vec![RawContact::default();16];let mut count=0;let mut status=unsafe{ffi::cy_sensors_contacts_v1(self.handle,out.as_mut_ptr(),out.len() as u32,&mut count)};while status==SHORT {out.resize(count as usize,RawContact::default());status=unsafe{ffi::cy_sensors_contacts_v1(self.handle,out.as_mut_ptr(),out.len() as u32,&mut count)};}check(self.handle,status)?;out.truncate(count as usize);Ok(out)}
pub fn nearest(&self) -> Result<Option<Contact>,Error> {let mut out=RawContact::default();let mut status=unsafe{ffi::cy_sensors_nearest_v1(self.handle,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn cues(&self) -> Result<Vec<Contact>,Error> {let mut out=vec![RawContact::default();16];let mut count=0;let mut status=unsafe{ffi::cy_sensors_cues_v1(self.handle,out.as_mut_ptr(),out.len() as u32,&mut count)};while status==SHORT {out.resize(count as usize,RawContact::default());status=unsafe{ffi::cy_sensors_cues_v1(self.handle,out.as_mut_ptr(),out.len() as u32,&mut count)};}check(self.handle,status)?;out.truncate(count as usize);Ok(out)}
pub fn contact(&self,slot: i32) -> Result<Option<Contact>,Error> {let mut out=RawContact::default();let mut status=unsafe{ffi::cy_sensors_contact_v1(self.handle,slot,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn position_of(&self,contact: &Contact) -> Result<Vec3,Error> {let mut out=RawVec3::default();let mut status=unsafe{ffi::cy_sensors_position_of_v1(self.handle,contact,&mut out)};check(self.handle,status)?;Ok(out)}
}
#[derive(Clone,Copy)] pub struct Tracker {handle:*mut c_void} impl Tracker {
pub fn cue(&self) -> Result<Option<Cue>,Error> {let mut out=RawCue::default();let mut status=unsafe{ffi::cy_tracker_cue_v1(self.handle,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn nearest_bearing_rate(&self) -> Result<f64,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_tracker_nearest_bearing_rate(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn velocity(&self,slot: i32) -> Result<Vec3,Error> {let mut out=RawVec3::default();let mut status=unsafe{ffi::cy_tracker_velocity(self.handle,slot,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn position(&self,slot: i32) -> Result<Option<Vec3>,Error> {let mut out=RawVec3::default();let mut status=unsafe{ffi::cy_tracker_position(self.handle,slot,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn last_seen(&self,slot: i32) -> Result<Option<i32>,Error> {let mut out=i32::default();let mut status=unsafe{ffi::cy_tracker_last_seen(self.handle,slot,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn configure(&self,smoothing: f64,forget_ticks: i32,cue_ticks: i32) -> Result<(),Error> {let mut status=unsafe{ffi::cy_tracker_configure(self.handle,smoothing,forget_ticks,cue_ticks)};check(self.handle,status)?;Ok(())}
}
#[derive(Clone,Copy)] pub struct Nav {handle:*mut c_void} impl Nav {
pub fn zone(&self) -> Result<Option<Zone>,Error> {let mut out=RawZone::default();let mut status=unsafe{ffi::cy_nav_zone_v1(self.handle,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(Zone{raw:out,handle:self.handle}))}
pub fn storm(&self) -> Result<Option<Storm>,Error> {let mut out=RawStorm::default();let mut status=unsafe{ffi::cy_nav_storm_v1(self.handle,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(Storm{raw:out,handle:self.handle}))}
pub fn hill(&self) -> Result<Option<Hill>,Error> {let mut out=RawHill::default();let mut status=unsafe{ffi::cy_nav_hill_v1(self.handle,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(Hill{raw:out,handle:self.handle}))}
pub fn pickups(&self) -> Result<Vec<Pickup>,Error> {let mut out=vec![RawPickup::default();16];let mut count=0;let mut status=unsafe{ffi::cy_nav_pickups_v1(self.handle,out.as_mut_ptr(),out.len() as u32,&mut count)};while status==SHORT {out.resize(count as usize,RawPickup::default());status=unsafe{ffi::cy_nav_pickups_v1(self.handle,out.as_mut_ptr(),out.len() as u32,&mut count)};}check(self.handle,status)?;out.truncate(count as usize);Ok(out)}
pub fn nearest_pickup(&self,kind: i32) -> Result<Option<Pickup>,Error> {let mut out=RawPickup::default();let mut status=unsafe{ffi::cy_nav_nearest_pickup_v1(self.handle,kind,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn field_distance(&self,a: &Vec3,b: &Vec3) -> Result<f64,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_nav_field_distance(self.handle,a,b,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn zone_margin(&self,point: &Vec3) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_nav_zone_margin(self.handle,point,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn zone_contains(&self,point: &Vec3,margin: f64) -> Result<Option<bool>,Error> {let mut out=i32::default();let mut status=unsafe{ffi::cy_nav_zone_contains(self.handle,point,margin,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out!=0))}
pub fn zone_distance_from(&self,point: &Vec3) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_nav_zone_distance_from(self.handle,point,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn storm_margin(&self,point: &Vec3) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_nav_storm_margin(self.handle,point,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn storm_contains(&self,point: &Vec3,margin: f64) -> Result<Option<bool>,Error> {let mut out=i32::default();let mut status=unsafe{ffi::cy_nav_storm_contains(self.handle,point,margin,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out!=0))}
pub fn storm_distance_from(&self,point: &Vec3) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_nav_storm_distance_from(self.handle,point,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn hill_margin(&self,point: &Vec3) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_nav_hill_margin(self.handle,point,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn hill_contains(&self,point: &Vec3,margin: f64) -> Result<Option<bool>,Error> {let mut out=i32::default();let mut status=unsafe{ffi::cy_nav_hill_contains(self.handle,point,margin,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out!=0))}
pub fn hill_distance_from(&self,point: &Vec3) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_nav_hill_distance_from(self.handle,point,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
}
#[derive(Clone,Copy)] pub struct Score {handle:*mut c_void} impl Score {
pub fn me(&self) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_score_me(self.handle,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn best(&self) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_score_best(self.handle,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn armour(&self) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_score_armour(self.handle,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
}
#[derive(Clone,Copy)] pub struct Angles {handle:*mut c_void} impl Angles {
pub fn normalize(&self,degrees: f64) -> Result<f64,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_angles_normalize(degrees,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn angle_diff(&self,target: f64,current: f64) -> Result<f64,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_angles_angle_diff(target,current,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn bearing_to(&self,from: &Vec3,to: &Vec3) -> Result<f64,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_angles_bearing_to(from,to,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn elevation_to(&self,from: &Vec3,to: &Vec3) -> Result<f64,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_angles_elevation_to(from,to,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn direction(&self,heading: f64,pitch: f64) -> Result<Vec3,Error> {let mut out=RawVec3::default();let mut status=unsafe{ffi::cy_angles_direction(heading,pitch,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn distance(&self,a: &Vec3,b: &Vec3) -> Result<f64,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_angles_distance(a,b,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn clamp(&self,value: f64,low: f64,high: f64) -> Result<f64,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_angles_clamp(value,low,high,&mut out)};check(self.handle,status)?;Ok(out)}
}
#[derive(Clone,Copy)] pub struct Status {handle:*mut c_void} impl Status {
pub fn set(&self,text:Option<&str>,value:Option<f64>)->Result<(),Error>{if let Some(value)=text{check(self.handle,unsafe{ffi::cy_status_text(self.handle,value.as_ptr(),value.len() as u32)})?;}if let Some(value)=value{check(self.handle,unsafe{ffi::cy_status_value(self.handle,value)})?;}Ok(())}
pub fn get_text(&self) -> Result<Option<String>,Error> {let mut out=vec![0u8;256];let mut count=0;let mut status=unsafe{ffi::cy_status_get_text(self.handle,out.as_mut_ptr(),out.len() as u32,&mut count)};while status==SHORT {out.resize(count as usize,0);status=unsafe{ffi::cy_status_get_text(self.handle,out.as_mut_ptr(),out.len() as u32,&mut count)};}if status==NONE{return Ok(None);}check(self.handle,status)?;out.truncate(count as usize);Ok(Some(String::from_utf8(out).map_err(|_|Error::new(E_ARGUMENT,"core returned invalid UTF-8"))?))}
pub fn get_value(&self) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_status_get_value(self.handle,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
}
#[derive(Clone,Copy)] pub struct Raw {handle:*mut c_void} impl Raw {
pub fn number(&self,path: &str) -> Result<Option<f64>,Error> {let mut out=f64::default();let mut status=unsafe{ffi::cy_raw_number(self.handle,path.as_ptr(),path.len() as u32,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn text(&self,path: &str) -> Result<Option<String>,Error> {let mut out=vec![0u8;256];let mut count=0;let mut status=unsafe{ffi::cy_raw_text(self.handle,path.as_ptr(),path.len() as u32,out.as_mut_ptr(),out.len() as u32,&mut count)};while status==SHORT {out.resize(count as usize,0);status=unsafe{ffi::cy_raw_text(self.handle,path.as_ptr(),path.len() as u32,out.as_mut_ptr(),out.len() as u32,&mut count)};}if status==NONE{return Ok(None);}check(self.handle,status)?;out.truncate(count as usize);Ok(Some(String::from_utf8(out).map_err(|_|Error::new(E_ARGUMENT,"core returned invalid UTF-8"))?))}
pub fn has(&self,path: &str) -> Result<bool,Error> {let mut out=i32::default();let mut status=unsafe{ffi::cy_raw_has(self.handle,path.as_ptr(),path.len() as u32,&mut out)};check(self.handle,status)?;Ok(out!=0)}
pub fn flag(&self,path: &str) -> Result<Option<bool>,Error> {let mut out=i32::default();let mut status=unsafe{ffi::cy_raw_flag(self.handle,path.as_ptr(),path.len() as u32,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out!=0))}
pub fn count(&self,path: &str) -> Result<Option<i32>,Error> {let mut out=i32::default();let mut status=unsafe{ffi::cy_raw_count(self.handle,path.as_ptr(),path.len() as u32,&mut out)};if status==NONE{return Ok(None);}check(self.handle,status)?;Ok(Some(out))}
pub fn line(&self) -> Result<String,Error> {let mut out=vec![0u8;256];let mut count=0;let mut status=unsafe{ffi::cy_raw_line(self.handle,out.as_mut_ptr(),out.len() as u32,&mut count)};while status==SHORT {out.resize(count as usize,0);status=unsafe{ffi::cy_raw_line(self.handle,out.as_mut_ptr(),out.len() as u32,&mut count)};}check(self.handle,status)?;out.truncate(count as usize);Ok(String::from_utf8(out).map_err(|_|Error::new(E_ARGUMENT,"core returned invalid UTF-8"))?)}
pub fn extra(&self,json: &str) -> Result<(),Error> {let mut status=unsafe{ffi::cy_raw_extra(self.handle,json.as_ptr(),json.len() as u32)};check(self.handle,status)?;Ok(())}
}
#[derive(Clone,Copy)] pub struct Session {handle:*mut c_void} impl Session {
pub fn event(&self) -> Result<i32,Error> {let mut out=i32::default();let mut status=unsafe{ffi::cy_session_event(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn next(&self) -> Result<i32,Error> {let mut out=i32::default();let mut status=unsafe{ffi::cy_session_next(self.handle,&mut out)};check(self.handle,status)?;Ok(out)}
pub fn send(&self) -> Result<(),Error> {let mut status=unsafe{ffi::cy_session_send(self.handle)};check(self.handle,status)?;Ok(())}
pub fn forfeit(&self,code: &str,message: &str) -> Result<(),Error> {let mut status=unsafe{ffi::cy_session_forfeit(self.handle,code.as_ptr(),code.len() as u32,message.as_ptr(),message.len() as u32)};check(self.handle,status)?;Ok(())}
pub fn log(&self,line: &str) -> Result<(),Error> {let mut status=unsafe{ffi::cy_session_log(self.handle,line.as_ptr(),line.len() as u32)};check(self.handle,status)?;Ok(())}
pub fn feed(&self,line: &str) -> Result<(),Error> {let mut status=unsafe{ffi::cy_session_feed(self.handle,line.as_ptr(),line.len() as u32)};check(self.handle,status)?;Ok(())}
pub fn reply(&self) -> Result<Option<String>,Error> {let mut out=vec![0u8;256];let mut count=0;let mut status=unsafe{ffi::cy_session_reply(self.handle,out.as_mut_ptr(),out.len() as u32,&mut count)};while status==SHORT {out.resize(count as usize,0);status=unsafe{ffi::cy_session_reply(self.handle,out.as_mut_ptr(),out.len() as u32,&mut count)};}if status==NONE{return Ok(None);}check(self.handle,status)?;out.truncate(count as usize);Ok(Some(String::from_utf8(out).map_err(|_|Error::new(E_ARGUMENT,"core returned invalid UTF-8"))?))}
}
pub struct Robot {handle:*mut c_void,owned:bool,pub r#match:Match,pub clock:Clock,pub arena:Arena,pub me:Me,pub battery:Battery,pub drive:Drive,pub gun:Gun,pub scanner:Scanner,pub sensors:Sensors,pub tracker:Tracker,pub nav:Nav,pub score:Score,pub angles:Angles,pub status:Status,pub raw:Raw,pub session:Session,}
impl Robot {pub unsafe fn from_raw(handle:*mut c_void)->Self{Self{handle,owned:false,r#match:Match{handle},clock:Clock{handle},arena:Arena{handle},me:Me{handle},battery:Battery{handle},drive:Drive{handle},gun:Gun{handle},scanner:Scanner{handle},sensors:Sensors{handle},tracker:Tracker{handle},nav:Nav{handle},score:Score{handle},angles:Angles{handle},status:Status{handle},raw:Raw{handle},session:Session{handle},}}
fn open(memory:bool)->Result<Self,Error>{let mut handle=std::ptr::null_mut();let name=b"clunkyard-rust";let status=unsafe{if memory{ffi::cy_open_memory(2,0,name.as_ptr(),name.len() as u32,VERSION.as_ptr(),VERSION.len() as u32,&mut handle)}else{ffi::cy_open(2,0,name.as_ptr(),name.len() as u32,VERSION.as_ptr(),VERSION.len() as u32,&mut handle)}};check(handle,status)?;let mut robot=unsafe{Self::from_raw(handle)};robot.owned=true;Ok(robot)}}
impl Drop for Robot{fn drop(&mut self){if self.owned{unsafe{ffi::cy_close(self.handle)}}}}

pub trait Strategy{fn on_start(&mut self,_robot:&Robot)->Result<(),Error>{Ok(())}fn on_turn(&mut self,robot:&Robot)->Result<(),Error>;}
fn turn<S:Strategy>(strategy:&mut S,robot:&Robot,event:i32)->Result<(),Error>{
  let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||if event==EVENT_HELLO{strategy.on_start(robot)}else{strategy.on_turn(robot)}));
  match result{Ok(Err(error))=>{if let Some((code,message))=error.forfeit{robot.session.forfeit(&code,&message)?;return Ok(());}robot.session.log(&error.message)?;},Err(_)=>{robot.session.log("robot panicked")?;},_=>{}}
  robot.session.send()
}
pub fn run<S:Strategy>(mut strategy:S)->Result<(),Error>{let robot=Robot::open(false)?;loop{let event=match robot.session.next(){Ok(event)=>event,Err(error) if error.status==E_INTERNAL=>{robot.session.send()?;continue;},Err(error)=>return Err(error)};if event==EVENT_END{return Ok(());}turn(&mut strategy,&robot,event)?;}}
pub fn play<S:Strategy>(mut strategy:S,lines:&[String])->Result<Vec<String>,Error>{let robot=Robot::open(true)?;for line in lines{robot.session.feed(line)?;}loop{let event=match robot.session.next(){Ok(event)=>event,Err(error) if error.status==E_INTERNAL=>{robot.session.send()?;continue;},Err(error)=>return Err(error)};if event==EVENT_END{break;}turn(&mut strategy,&robot,event)?;}let mut out=Vec::new();while let Some(line)=robot.session.reply()?{out.push(line);}Ok(out)}
