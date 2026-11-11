// Generated from protocol/v2/api.json by protocol/v2/codegen/rust.js; do not edit. Run `make api`.
//! The core's generated surface, from protocol/v2/api.json (API 2.0.0).
//!
//! records.rs holds the statuses, enums, records (with size and offset
//! asserts) and the act fields; ffi.rs every C export; words.rs the DSL
//! words filler. Include it with `mod generated;` in lib.rs.
//!
//! The exports call the hand-written core by a fixed rule: the entry
//! subsystem.member is crate::subsystem::member (a rename: {rust} in
//! api.js changes the name; nav.zone.margin is crate::nav::zone_margin).
//! Queries take &Robot, commands &mut Robot, and the stateless angles none.
//! Every function returns Result<T, Error>: Option<T> for an optional
//! result (Ok(None) is CY_NONE, absent and not an error) and Vec<T> for a
//! list. A text argument arrives as &str, already checked as UTF-8; a record
//! argument as a reference; a non-finite number argument never arrives
//! (the export returns CY_E_NOT_FINITE first); an enum whose 0 means "any"
//! arrives as Option. The exports check every pointer, catch every panic,
//! and copy every result out. A non-finite control value is refused with
//! CY_E_NOT_FINITE before act::set_number, so the field keeps what it had.
//! The act write clamps (Field::limits() for the schema's ranges; headings
//! by normalize, pitches, the scan arc, the power and the status from the
//! hello), writes -0 as 0, sends fire only after a fire op, and orders
//! members v, type, tick, drive, forces, aim, scan, fire, status, extras
//! (protocol/v2/vectors/api/controls.json).
//!
//! The hand-written core provides:
//!
//!     crate::runtime::Robot                     what a cy_robot * points to
//!     crate::runtime::PortKind                  Stdio or Memory
//!     crate::runtime::open(port: PortKind, binding: &str, version: &str) -> Result<Robot, Error>
//!     crate::runtime::last_error(r: &Robot) -> &str
//!     crate::runtime::record_error(r: &mut Robot, error: &Error)     keep the message for cy_last_error
//!     crate::runtime::record_panic(r: &mut Robot, symbol: &'static str)   log once, mark for reset, count
//!     crate::act::set_number(r: &mut Robot, field: Field, value: f64) -> Result<(), Error>   kept as set; the act write clamps
//!     crate::act::set_flag(r: &mut Robot, field: Field) -> Result<(), Error>
//!     crate::act::set_text(r: &mut Robot, field: Field, text: &str) -> Result<(), Error>
//!     crate::act::get(r: &Robot, field: Field) -> Result<Option<f64>, Error>      as set, before clamping; 1.0 for a set flag
//!     crate::act::get_text(r: &Robot, field: Field) -> Result<Option<String>, Error>
//!     crate::raw::number(r: &Robot, path: &str) -> Result<Option<f64>, Error>    (also below)
//!
//! and one function per entry:
//!
//!     crate::r#match::mode(r: &Robot) -> Result<Mode, Error>
//!     crate::r#match::ruleset(r: &Robot) -> Result<Ruleset, Error>
//!     crate::r#match::control(r: &Robot) -> Result<Control, Error>
//!     crate::r#match::robots(r: &Robot) -> Result<i32, Error>
//!     crate::r#match::is_3d(r: &Robot) -> Result<bool, Error>
//!     crate::r#match::tick_rate(r: &Robot) -> Result<i32, Error>
//!     crate::r#match::max_ticks(r: &Robot) -> Result<i32, Error>
//!     crate::r#match::robot_id(r: &Robot, slot: i32) -> Result<String, Error>
//!     crate::clock::tick(r: &Robot) -> Result<i32, Error>
//!     crate::clock::seconds(r: &Robot) -> Result<f64, Error>
//!     crate::clock::remaining_ticks(r: &Robot) -> Result<i32, Error>
//!     crate::arena::size(r: &Robot) -> Result<Vec3, Error>
//!     crate::arena::gravity(r: &Robot) -> Result<Vec3, Error>
//!     crate::arena::terrain_kind(r: &Robot) -> Result<TerrainKind, Error>
//!     crate::arena::ground_height(r: &Robot, x: f64, y: f64) -> Result<f64, Error>
//!     crate::me::health(r: &Robot) -> Result<f64, Error>
//!     crate::me::position(r: &Robot) -> Result<Vec3, Error>
//!     crate::me::velocity(r: &Robot) -> Result<Vec3, Error>
//!     crate::me::heading(r: &Robot) -> Result<f64, Error>
//!     crate::me::pitch(r: &Robot) -> Result<f64, Error>
//!     crate::me::roll(r: &Robot) -> Result<f64, Error>
//!     crate::me::angular_velocity(r: &Robot) -> Result<Rotation, Error>
//!     crate::me::attitude(r: &Robot) -> Result<AttitudeV1, Error>
//!     crate::me::grounded(r: &Robot) -> Result<bool, Error>
//!     crate::me::altitude(r: &Robot) -> Result<f64, Error>
//!     crate::me::applied(r: &Robot) -> Result<AppliedV1, Error>
//!     crate::battery::charge(r: &Robot) -> Result<f64, Error>
//!     crate::battery::capacity(r: &Robot) -> Result<f64, Error>
//!     crate::battery::regen(r: &Robot) -> Result<f64, Error>
//!     crate::battery::sustainable_drive(r: &Robot) -> Result<f64, Error>
//!     crate::battery::compute(r: &Robot) -> Result<Option<ComputeV1>, Error>
//!     crate::battery::drive_cost(r: &Robot, fraction: f64) -> Result<f64, Error>
//!     crate::drive::hover_lift(r: &Robot) -> Result<f64, Error>
//!     crate::gun::aim_now(r: &Robot) -> Result<Direction, Error>
//!     crate::gun::cooldown_ticks(r: &Robot) -> Result<i32, Error>
//!     crate::gun::spread(r: &Robot) -> Result<f64, Error>
//!     crate::gun::max_power(r: &Robot) -> Result<f64, Error>
//!     crate::gun::damage_scale(r: &Robot) -> Result<f64, Error>
//!     crate::gun::can_fire(r: &Robot, power: f64, reserve: f64) -> Result<bool, Error>
//!     crate::gun::shot_speed(r: &Robot, power: f64) -> Result<f64, Error>
//!     crate::gun::shot_cost(r: &Robot, power: f64) -> Result<f64, Error>
//!     crate::gun::reload_ticks(r: &Robot, power: f64) -> Result<f64, Error>
//!     crate::gun::shot_damage(r: &Robot, power: f64) -> Result<f64, Error>
//!     crate::gun::power_for_speed(r: &Robot, speed: f64) -> Result<f64, Error>
//!     crate::gun::power_for_flight_time(r: &Robot, range: f64, seconds: f64) -> Result<f64, Error>
//!     crate::gun::lead(r: &Robot, contact: &ContactV1, power: f64) -> Result<Option<SolutionV1>, Error>
//!     crate::gun::lead_point(r: &Robot, position: &Vec3, velocity: &Vec3, power: f64) -> Result<Option<SolutionV1>, Error>
//!     crate::scanner::last(r: &Robot) -> Result<ScanV1, Error>
//!     crate::scanner::proximity_range(r: &Robot) -> Result<f64, Error>
//!     crate::scanner::reach(r: &Robot, arc: f64) -> Result<f64, Error>
//!     crate::scanner::arc_for_reach(r: &Robot, range: f64) -> Result<f64, Error>
//!     crate::sensors::contacts(r: &Robot) -> Result<Vec<ContactV1>, Error>
//!     crate::sensors::nearest(r: &Robot) -> Result<Option<ContactV1>, Error>
//!     crate::sensors::cues(r: &Robot) -> Result<Vec<ContactV1>, Error>
//!     crate::sensors::contact(r: &Robot, slot: i32) -> Result<Option<ContactV1>, Error>
//!     crate::sensors::position_of(r: &Robot, contact: &ContactV1) -> Result<Vec3, Error>
//!     crate::tracker::cue(r: &Robot) -> Result<Option<CueV1>, Error>
//!     crate::tracker::nearest_bearing_rate(r: &Robot) -> Result<f64, Error>
//!     crate::tracker::velocity(r: &Robot, slot: i32) -> Result<Vec3, Error>
//!     crate::tracker::position(r: &Robot, slot: i32) -> Result<Option<Vec3>, Error>
//!     crate::tracker::last_seen(r: &Robot, slot: i32) -> Result<Option<i32>, Error>
//!     crate::tracker::configure(r: &mut Robot, smoothing: f64, forget_ticks: i32, cue_ticks: i32) -> Result<(), Error>
//!     crate::nav::zone(r: &Robot) -> Result<Option<ZoneV1>, Error>
//!     crate::nav::zone_margin(r: &Robot, point: &Vec3) -> Result<Option<f64>, Error>
//!     crate::nav::zone_contains(r: &Robot, point: &Vec3, margin: f64) -> Result<Option<bool>, Error>
//!     crate::nav::zone_distance_from(r: &Robot, point: &Vec3) -> Result<Option<f64>, Error>
//!     crate::nav::storm(r: &Robot) -> Result<Option<StormV1>, Error>
//!     crate::nav::storm_margin(r: &Robot, point: &Vec3) -> Result<Option<f64>, Error>
//!     crate::nav::storm_contains(r: &Robot, point: &Vec3, margin: f64) -> Result<Option<bool>, Error>
//!     crate::nav::storm_distance_from(r: &Robot, point: &Vec3) -> Result<Option<f64>, Error>
//!     crate::nav::hill(r: &Robot) -> Result<Option<HillV1>, Error>
//!     crate::nav::hill_margin(r: &Robot, point: &Vec3) -> Result<Option<f64>, Error>
//!     crate::nav::hill_contains(r: &Robot, point: &Vec3, margin: f64) -> Result<Option<bool>, Error>
//!     crate::nav::hill_distance_from(r: &Robot, point: &Vec3) -> Result<Option<f64>, Error>
//!     crate::nav::pickups(r: &Robot) -> Result<Vec<PickupV1>, Error>
//!     crate::nav::nearest_pickup(r: &Robot, kind: Option<PickupKind>) -> Result<Option<PickupV1>, Error>
//!     crate::nav::field_distance(r: &Robot, a: &Vec3, b: &Vec3) -> Result<f64, Error>
//!     crate::score::me(r: &Robot) -> Result<Option<f64>, Error>
//!     crate::score::best(r: &Robot) -> Result<Option<f64>, Error>
//!     crate::score::armour(r: &Robot) -> Result<Option<f64>, Error>
//!     crate::angles::normalize(degrees: f64) -> Result<f64, Error>
//!     crate::angles::angle_diff(target: f64, current: f64) -> Result<f64, Error>
//!     crate::angles::bearing_to(from: &Vec3, to: &Vec3) -> Result<f64, Error>
//!     crate::angles::elevation_to(from: &Vec3, to: &Vec3) -> Result<f64, Error>
//!     crate::angles::direction(heading: f64, pitch: f64) -> Result<Vec3, Error>
//!     crate::angles::distance(a: &Vec3, b: &Vec3) -> Result<f64, Error>
//!     crate::angles::clamp(value: f64, low: f64, high: f64) -> Result<f64, Error>
//!     crate::raw::number(r: &Robot, path: &str) -> Result<Option<f64>, Error>
//!     crate::raw::text(r: &Robot, path: &str) -> Result<Option<String>, Error>
//!     crate::raw::has(r: &Robot, path: &str) -> Result<bool, Error>
//!     crate::raw::flag(r: &Robot, path: &str) -> Result<Option<bool>, Error>
//!     crate::raw::count(r: &Robot, path: &str) -> Result<Option<i32>, Error>
//!     crate::raw::line(r: &Robot) -> Result<String, Error>
//!     crate::raw::extra(r: &mut Robot, json: &str) -> Result<(), Error>
//!     crate::session::event(r: &Robot) -> Result<Event, Error>
//!     crate::session::next(r: &mut Robot) -> Result<Event, Error>
//!     crate::session::send(r: &mut Robot) -> Result<(), Error>
//!     crate::session::forfeit(r: &mut Robot, code: &str, message: &str) -> Result<(), Error>
//!     crate::session::log(r: &mut Robot, line: &str) -> Result<(), Error>
//!     crate::session::feed(r: &mut Robot, line: &str) -> Result<(), Error>
//!     crate::session::reply(r: &Robot) -> Result<Option<String>, Error>
//!     crate::session::take_reply(r: &mut Robot)     the line reply gave, once copied out
//!
//! and the derived DSL words (section 4.2), each from core facts:
//!
//!     crate::dsl::speed(r: &Robot) -> Result<f64, Error>
//!     crate::dsl::gravity(r: &Robot) -> Result<bool, Error>
//!     crate::dsl::battery_fraction(r: &Robot) -> Result<f64, Error>
//!     crate::dsl::aim_error(r: &Robot) -> Result<f64, Error>
//!     crate::dsl::zone_distance(r: &Robot) -> Result<f64, Error>
//!     crate::dsl::zone_bearing(r: &Robot) -> Result<f64, Error>
//!     crate::dsl::zone_elevation(r: &Robot) -> Result<f64, Error>
//!     crate::dsl::storm_distance(r: &Robot) -> Result<f64, Error>
//!     crate::dsl::storm_bearing(r: &Robot) -> Result<f64, Error>
//!     crate::dsl::storm_heading(r: &Robot) -> Result<f64, Error>
//!     crate::dsl::cue_muzzle(r: &Robot) -> Result<bool, Error>
//!     crate::dsl::cue_bearing(r: &Robot) -> Result<f64, Error>
//!     crate::dsl::cue_elevation(r: &Robot) -> Result<f64, Error>
//!     crate::dsl::cue_range(r: &Robot) -> Result<f64, Error>
//!     crate::dsl::cue_age(r: &Robot) -> Result<f64, Error>
//!     crate::dsl::pickup_present(r: &Robot) -> Result<bool, Error>
//!     crate::dsl::pickup_bearing(r: &Robot) -> Result<f64, Error>
//!     crate::dsl::pickup_elevation(r: &Robot) -> Result<f64, Error>
//!     crate::dsl::pickup_range(r: &Robot) -> Result<f64, Error>
//!     crate::dsl::on_hill(r: &Robot) -> Result<bool, Error>
//!     crate::dsl::hill_held(r: &Robot) -> Result<bool, Error>
//!     crate::dsl::hill_bearing(r: &Robot) -> Result<f64, Error>
//!     crate::dsl::hill_elevation(r: &Robot) -> Result<f64, Error>
//!     crate::dsl::hill_range(r: &Robot) -> Result<f64, Error>
//!     crate::dsl::hill_time(r: &Robot) -> Result<f64, Error>
//!     crate::dsl::hill_to_win(r: &Robot) -> Result<f64, Error>

pub mod ffi;
pub mod records;
pub mod words;

pub use records::*;
