// Generated from protocol/v2/api.json by protocol/v2/codegen/rust.js; do not edit. Run `make api`.
//! The API's statuses, enums, records and act fields, laid out as the C header has them.

use core::mem::{offset_of, size_of};

/// The core's API version (protocol/v2/api.js), separate from the wire protocol.
pub const API_VERSION: &str = "2.0.0";
pub const API_MAJOR: i32 = 2;
pub const API_MINOR: i32 = 0;
pub const API_PATCH: i32 = 0;
/// The Tracker keeps a passive cue this many ticks unless configured otherwise.
pub const CUE_TICKS: i32 = 60;
/// What a DSL distance reads when the thing it measures to is absent.
pub const FAR: f64 = 1000000000000.0;

/// Done.
pub const CY_OK: i32 = 0;
/// Absent, which is not an error: no precise contact, a mechanic that is off, a shot that cannot reach, or a field that is unset.
pub const CY_NONE: i32 = 1;
/// The buffer is too short; the length or count says how much is needed.
pub const CY_SHORT: i32 = 2;
/// A number is NaN or infinite; the field is left unset.
pub const CY_E_NOT_FINITE: i32 = -1;
/// A number is outside what the call accepts.
pub const CY_E_RANGE: i32 = -2;
/// An argument is malformed: a null pointer, a slot out of range, text that is not UTF-8.
pub const CY_E_ARGUMENT: i32 = -3;
/// The wrong point in the session, such as a state query before the first state.
pub const CY_E_PHASE: i32 = -4;
/// The binding's API version is newer than the core's, or a different major.
pub const CY_E_VERSION: i32 = -5;
/// A fault in the core. It is never the robot's fault; the handle resets on the next state.
pub const CY_E_INTERNAL: i32 = -6;

/// A failed call: its status (one of the negative CY_E_ codes) and the message cy_last_error gives.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    pub status: i32,
    pub message: String,
}

impl Error {
    pub fn not_finite(message: impl Into<String>) -> Self {
        Error { status: CY_E_NOT_FINITE, message: message.into() }
    }
    pub fn range(message: impl Into<String>) -> Self {
        Error { status: CY_E_RANGE, message: message.into() }
    }
    pub fn argument(message: impl Into<String>) -> Self {
        Error { status: CY_E_ARGUMENT, message: message.into() }
    }
    pub fn phase(message: impl Into<String>) -> Self {
        Error { status: CY_E_PHASE, message: message.into() }
    }
    pub fn version(message: impl Into<String>) -> Self {
        Error { status: CY_E_VERSION, message: message.into() }
    }
    pub fn internal(message: impl Into<String>) -> Self {
        Error { status: CY_E_INTERNAL, message: message.into() }
    }
}

/// What detected a contact. In a record or an argument, 0 is none.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Source {
    Scan = 1,
    Proximity = 2,
    Faint = 3,
    Beam = 4,
    Muzzle = 5,
    Motion = 6,
    Ping = 7,
}

impl Source {
    pub const ALL: [Source; 7] = [Source::Scan, Source::Proximity, Source::Faint, Source::Beam, Source::Muzzle, Source::Motion, Source::Ping];

    /// The protocol's name for the value.
    pub fn wire(self) -> &'static str {
        match self {
            Source::Scan => "scan",
            Source::Proximity => "proximity",
            Source::Faint => "faint",
            Source::Beam => "beam",
            Source::Muzzle => "muzzle",
            Source::Motion => "motion",
            Source::Ping => "ping",
        }
    }

    pub fn from_wire(wire: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|value| value.wire() == wire)
    }

    pub fn from_i32(value: i32) -> Option<Self> {
        Self::ALL.into_iter().find(|item| *item as i32 == value)
    }
}

/// The match's dimensions. In a record or an argument, 0 is none.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Mode {
    TwoD = 1,
    ThreeD = 2,
}

impl Mode {
    pub const ALL: [Mode; 2] = [Mode::TwoD, Mode::ThreeD];

    /// The protocol's name for the value.
    pub fn wire(self) -> &'static str {
        match self {
            Mode::TwoD => "2d",
            Mode::ThreeD => "3d",
        }
    }

    pub fn from_wire(wire: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|value| value.wire() == wire)
    }

    pub fn from_i32(value: i32) -> Option<Self> {
        Self::ALL.into_iter().find(|item| *item as i32 == value)
    }
}

/// The ruleset. In a record or an argument, 0 is none.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Ruleset {
    Classic = 1,
    Inertial2d = 2,
    GravityFlat = 3,
    GravityHills = 4,
    ZeroG = 5,
}

impl Ruleset {
    pub const ALL: [Ruleset; 5] = [Ruleset::Classic, Ruleset::Inertial2d, Ruleset::GravityFlat, Ruleset::GravityHills, Ruleset::ZeroG];

    /// The protocol's name for the value.
    pub fn wire(self) -> &'static str {
        match self {
            Ruleset::Classic => "classic",
            Ruleset::Inertial2d => "inertial-2d",
            Ruleset::GravityFlat => "gravity-flat",
            Ruleset::GravityHills => "gravity-hills",
            Ruleset::ZeroG => "zero-g",
        }
    }

    pub fn from_wire(wire: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|value| value.wire() == wire)
    }

    pub fn from_i32(value: i32) -> Option<Self> {
        Self::ALL.into_iter().find(|item| *item as i32 == value)
    }
}

/// What the drive controls mean: direct speeds in classic, forces elsewhere. In a record or an argument, 0 is none.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Control {
    Direct = 1,
    Forces = 2,
}

impl Control {
    pub const ALL: [Control; 2] = [Control::Direct, Control::Forces];

    /// The protocol's name for the value.
    pub fn wire(self) -> &'static str {
        match self {
            Control::Direct => "direct",
            Control::Forces => "forces",
        }
    }

    pub fn from_wire(wire: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|value| value.wire() == wire)
    }

    pub fn from_i32(value: i32) -> Option<Self> {
        Self::ALL.into_iter().find(|item| *item as i32 == value)
    }
}

/// The ground. In a record or an argument, 0 is none.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TerrainKind {
    None = 1,
    Flat = 2,
    Hills = 3,
}

impl TerrainKind {
    pub const ALL: [TerrainKind; 3] = [TerrainKind::None, TerrainKind::Flat, TerrainKind::Hills];

    /// The protocol's name for the value.
    pub fn wire(self) -> &'static str {
        match self {
            TerrainKind::None => "none",
            TerrainKind::Flat => "flat",
            TerrainKind::Hills => "hills",
        }
    }

    pub fn from_wire(wire: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|value| value.wire() == wire)
    }

    pub fn from_i32(value: i32) -> Option<Self> {
        Self::ALL.into_iter().find(|item| *item as i32 == value)
    }
}

/// A region's shape: circle in 2D, sphere in 3D classic and zero-g, column (unbounded in height) under gravity. In a record or an argument, 0 is none.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Shape {
    Circle = 1,
    Sphere = 2,
    Column = 3,
}

impl Shape {
    pub const ALL: [Shape; 3] = [Shape::Circle, Shape::Sphere, Shape::Column];

    /// The protocol's name for the value.
    pub fn wire(self) -> &'static str {
        match self {
            Shape::Circle => "circle",
            Shape::Sphere => "sphere",
            Shape::Column => "column",
        }
    }

    pub fn from_wire(wire: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|value| value.wire() == wire)
    }

    pub fn from_i32(value: i32) -> Option<Self> {
        Self::ALL.into_iter().find(|item| *item as i32 == value)
    }
}

/// What a pickup gives. 0 asks for any kind. In a record or an argument, 0 is any.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PickupKind {
    Energy = 1,
    Health = 2,
    Armour = 3,
}

impl PickupKind {
    pub const ALL: [PickupKind; 3] = [PickupKind::Energy, PickupKind::Health, PickupKind::Armour];

    /// The protocol's name for the value.
    pub fn wire(self) -> &'static str {
        match self {
            PickupKind::Energy => "energy",
            PickupKind::Health => "health",
            PickupKind::Armour => "armour",
        }
    }

    pub fn from_wire(wire: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|value| value.wire() == wire)
    }

    pub fn from_i32(value: i32) -> Option<Self> {
        Self::ALL.into_iter().find(|item| *item as i32 == value)
    }
}

/// The meter compute is charged by; 0 when turns are not metered. In a record or an argument, 0 is none.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ComputeUnit {
    Operations = 1,
    Fuel = 2,
    CpuMs = 3,
}

impl ComputeUnit {
    pub const ALL: [ComputeUnit; 3] = [ComputeUnit::Operations, ComputeUnit::Fuel, ComputeUnit::CpuMs];

    /// The protocol's name for the value.
    pub fn wire(self) -> &'static str {
        match self {
            ComputeUnit::Operations => "operations",
            ComputeUnit::Fuel => "fuel",
            ComputeUnit::CpuMs => "cpu-ms",
        }
    }

    pub fn from_wire(wire: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|value| value.wire() == wire)
    }

    pub fn from_i32(value: i32) -> Option<Self> {
        Self::ALL.into_iter().find(|item| *item as i32 == value)
    }
}

/// What session.next read: the hello, a state, or the end of input. In a record or an argument, 0 is none.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Event {
    Hello = 1,
    State = 2,
    End = 3,
}

impl Event {
    pub const ALL: [Event; 3] = [Event::Hello, Event::State, Event::End];

    /// The protocol's name for the value.
    pub fn wire(self) -> &'static str {
        match self {
            Event::Hello => "hello",
            Event::State => "state",
            Event::End => "end",
        }
    }

    pub fn from_wire(wire: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|value| value.wire() == wire)
    }

    pub fn from_i32(value: i32) -> Option<Self> {
        Self::ALL.into_iter().find(|item| *item as i32 == value)
    }
}

/// A vector in world axes. 24 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    /// x component, east.
    pub x: f64,
    /// y component, north.
    pub y: f64,
    /// z component, up; 0 in 2D.
    pub z: f64,
}
const _: () = assert!(size_of::<Vec3>() == 24);
const _: () = assert!(offset_of!(Vec3, x) == 0);
const _: () = assert!(offset_of!(Vec3, y) == 8);
const _: () = assert!(offset_of!(Vec3, z) == 16);

/// A rotation rate or torque about the robot's own axes. 24 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rotation {
    /// About the up axis.
    pub yaw: f64,
    /// About the right axis; 0 in 2D.
    pub pitch: f64,
    /// About the forward axis; 0 in 2D.
    pub roll: f64,
}
const _: () = assert!(size_of::<Rotation>() == 24);
const _: () = assert!(offset_of!(Rotation, yaw) == 0);
const _: () = assert!(offset_of!(Rotation, pitch) == 8);
const _: () = assert!(offset_of!(Rotation, roll) == 16);

/// A direction as a heading and an elevation. 16 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Direction {
    /// Heading from the +x axis, degrees, 0 to 360.
    pub heading: f64,
    /// Elevation, degrees, positive upward; 0 in 2D.
    pub pitch: f64,
}
const _: () = assert!(size_of::<Direction>() == 16);
const _: () = assert!(offset_of!(Direction, heading) == 0);
const _: () = assert!(offset_of!(Direction, pitch) == 8);

/// The chassis axes as unit vectors in world axes. 72 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AttitudeV1 {
    /// The chassis forward axis.
    pub forward: Vec3,
    /// The chassis right axis.
    pub right: Vec3,
    /// The chassis up axis.
    pub up: Vec3,
}
/// The latest version of AttitudeV1.
pub type Attitude = AttitudeV1;
const _: () = assert!(size_of::<AttitudeV1>() == 72);
const _: () = assert!(offset_of!(AttitudeV1, forward) == 0);
const _: () = assert!(offset_of!(AttitudeV1, right) == 24);
const _: () = assert!(offset_of!(AttitudeV1, up) == 48);

/// What the previous tick applied to you, after the battery scaled the drive. 56 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AppliedV1 {
    /// Total force, gravity included.
    pub force: Vec3,
    /// Torque.
    pub torque: Rotation,
    /// Brake, 0 to 1.
    pub brake: f64,
}
/// The latest version of AppliedV1.
pub type Applied = AppliedV1;
const _: () = assert!(size_of::<AppliedV1>() == 56);
const _: () = assert!(offset_of!(AppliedV1, force) == 0);
const _: () = assert!(offset_of!(AppliedV1, torque) == 24);
const _: () = assert!(offset_of!(AppliedV1, brake) == 48);

/// The scan that produced this tick's contacts: the one your previous act asked for. 32 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ScanV1 {
    /// Scan heading.
    pub heading: f64,
    /// Scan pitch; 0 in 2D.
    pub pitch: f64,
    /// Full cone width.
    pub arc: f64,
    /// Reach at this arc.
    pub range: f64,
}
/// The latest version of ScanV1.
pub type Scan = ScanV1;
const _: () = assert!(size_of::<ScanV1>() == 32);
const _: () = assert!(offset_of!(ScanV1, heading) == 0);
const _: () = assert!(offset_of!(ScanV1, pitch) == 8);
const _: () = assert!(offset_of!(ScanV1, arc) == 16);
const _: () = assert!(offset_of!(ScanV1, range) == 24);

/// A robot the sensors detect this tick. 72 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ContactV1 {
    /// The robot: you are 1, and the others are numbered from 2 by first sighting (contacts first, then the hill holder), stable all match; 0 for a passive contact, which does not say which robot it heard.
    pub slot: i32,
    /// What detected it: scan, inside the scan cone and its reach; proximity, within scanner.proximityRange in any direction. A robot both inside the cone and near reports scan. With hello.sensing, passive sources give a noisy direction and no id: faint, inside the scan cone beyond its reach; beam, a robot whose scan swept over you; muzzle, a robot that fired; motion, a robot moving fast; ping, the periodic ping of every robot. Later sensors add sources; ignore ones you do not know. A Source as i32.
    pub source: i32,
    /// Straight-line distance between centres. For a passive source, noisy and rounded to sensing.rangeBin.
    pub range: f64,
    /// Heading from you to it. For a passive source, noisy.
    pub bearing: f64,
    /// Elevation from you to it; 0 in 2D. For a passive source, noisy.
    pub elevation: f64,
    /// Passive sources only: the standard deviation of bearing and elevation, degrees; 0 for scan and proximity.
    pub bearing_error: f64,
    /// Passive sources only: the standard deviation of the natural log of range; 0 for scan and proximity.
    pub range_error: f64,
    /// Where it is, in world axes: your position plus its range along its bearing and elevation.
    pub position: Vec3,
}
/// The latest version of ContactV1.
pub type Contact = ContactV1;
const _: () = assert!(size_of::<ContactV1>() == 72);
const _: () = assert!(offset_of!(ContactV1, slot) == 0);
const _: () = assert!(offset_of!(ContactV1, source) == 4);
const _: () = assert!(offset_of!(ContactV1, range) == 8);
const _: () = assert!(offset_of!(ContactV1, bearing) == 16);
const _: () = assert!(offset_of!(ContactV1, elevation) == 24);
const _: () = assert!(offset_of!(ContactV1, bearing_error) == 32);
const _: () = assert!(offset_of!(ContactV1, range_error) == 40);
const _: () = assert!(offset_of!(ContactV1, position) == 48);

/// The most accurate passive contact the Tracker holds: where to look, not where to shoot. 40 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CueV1 {
    /// What heard it. A Source as i32.
    pub source: i32,
    /// The tick it was heard.
    pub tick: i32,
    /// The standard deviation of its bearing and elevation, degrees.
    pub bearing_error: f64,
    /// Where it placed the robot it heard, in world axes.
    pub position: Vec3,
}
/// The latest version of CueV1.
pub type Cue = CueV1;
const _: () = assert!(size_of::<CueV1>() == 40);
const _: () = assert!(offset_of!(CueV1, source) == 0);
const _: () = assert!(offset_of!(CueV1, tick) == 4);
const _: () = assert!(offset_of!(CueV1, bearing_error) == 8);
const _: () = assert!(offset_of!(CueV1, position) == 16);

/// Where to aim for a shot fired now to meet a target, and when it meets it. 24 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SolutionV1 {
    /// Turret heading, degrees, 0 to 360.
    pub heading: f64,
    /// Turret pitch, degrees; 0 in 2D.
    pub pitch: f64,
    /// Seconds of flight until the shot meets the target.
    pub time: f64,
}
/// The latest version of SolutionV1.
pub type Solution = SolutionV1;
const _: () = assert!(size_of::<SolutionV1>() == 24);
const _: () = assert!(offset_of!(SolutionV1, heading) == 0);
const _: () = assert!(offset_of!(SolutionV1, pitch) == 8);
const _: () = assert!(offset_of!(SolutionV1, time) == 16);

/// The sudden-death safe zone. 40 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ZoneV1 {
    /// circle in 2D; sphere in 3D classic and zero-g; column, unbounded in height, in the gravity rulesets. A Shape as i32.
    pub shape: i32,
    /// Reserved; always 0. It keeps the record free of hidden padding.
    pub reserved: i32,
    /// The radius now. It shrinks evenly from suddenDeath.radiusStart to radiusEnd at suddenDeath.endTick (the last tick when absent).
    pub radius: f64,
    /// The zone's centre. A column's centre has z 0 and its height does not matter.
    pub center: Vec3,
}
/// The latest version of ZoneV1.
pub type Zone = ZoneV1;
const _: () = assert!(size_of::<ZoneV1>() == 40);
const _: () = assert!(offset_of!(ZoneV1, shape) == 0);
const _: () = assert!(offset_of!(ZoneV1, reserved) == 4);
const _: () = assert!(offset_of!(ZoneV1, radius) == 8);
const _: () = assert!(offset_of!(ZoneV1, center) == 16);

/// The storm now. 72 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StormV1 {
    /// circle in 2D, column in 3D. A Shape as i32.
    pub shape: i32,
    /// Reserved; always 0. It keeps the record free of hidden padding.
    pub reserved: i32,
    /// Its radius now.
    pub radius: f64,
    /// Health lost per second inside it.
    pub dps: f64,
    /// Its centre; z is 0.
    pub center: Vec3,
    /// Its velocity, units/s.
    pub velocity: Vec3,
}
/// The latest version of StormV1.
pub type Storm = StormV1;
const _: () = assert!(size_of::<StormV1>() == 72);
const _: () = assert!(offset_of!(StormV1, shape) == 0);
const _: () = assert!(offset_of!(StormV1, reserved) == 4);
const _: () = assert!(offset_of!(StormV1, radius) == 8);
const _: () = assert!(offset_of!(StormV1, dps) == 16);
const _: () = assert!(offset_of!(StormV1, center) == 24);
const _: () = assert!(offset_of!(StormV1, velocity) == 48);

/// The king-of-the-hill zone now. 80 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HillV1 {
    /// As zone.shape. A Shape as i32.
    pub shape: i32,
    /// The slot of the robot alone inside it (1 is you), or 0 for nobody.
    pub holder: i32,
    /// More than one robot is inside it. 1 for true, 0 for false.
    pub contested: i32,
    /// When it moves.
    pub moves_tick: i32,
    /// Ticks you have held it.
    pub held_ticks: i32,
    /// Most ticks any other robot has held it.
    pub best_held_ticks: i32,
    /// Its radius.
    pub radius: f64,
    /// Its centre.
    pub center: Vec3,
    /// Where it moves at movesTick.
    pub next: Vec3,
}
/// The latest version of HillV1.
pub type Hill = HillV1;
const _: () = assert!(size_of::<HillV1>() == 80);
const _: () = assert!(offset_of!(HillV1, shape) == 0);
const _: () = assert!(offset_of!(HillV1, holder) == 4);
const _: () = assert!(offset_of!(HillV1, contested) == 8);
const _: () = assert!(offset_of!(HillV1, moves_tick) == 12);
const _: () = assert!(offset_of!(HillV1, held_ticks) == 16);
const _: () = assert!(offset_of!(HillV1, best_held_ticks) == 20);
const _: () = assert!(offset_of!(HillV1, radius) == 24);
const _: () = assert!(offset_of!(HillV1, center) == 32);
const _: () = assert!(offset_of!(HillV1, next) == 56);

/// A cell on the field. 40 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PickupV1 {
    /// energy refills the battery, health repairs, armour absorbs shot damage before health. A PickupKind as i32.
    pub kind: i32,
    /// The tick it disappears if nobody takes it.
    pub expires_tick: i32,
    /// What it gives.
    pub amount: f64,
    /// Where it is. Under gravity it lies on the ground and only x and y count.
    pub position: Vec3,
}
/// The latest version of PickupV1.
pub type Pickup = PickupV1;
const _: () = assert!(size_of::<PickupV1>() == 40);
const _: () = assert!(offset_of!(PickupV1, kind) == 0);
const _: () = assert!(offset_of!(PickupV1, expires_tick) == 4);
const _: () = assert!(offset_of!(PickupV1, amount) == 8);
const _: () = assert!(offset_of!(PickupV1, position) == 16);

/// How the match charges compute against the battery. 16 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ComputeV1 {
    /// The charge for a turn that uses its whole budget.
    pub k: f64,
    /// A trusted CPU turn is charged in steps of 1/quanta of its budget: k × q / quanta, where q = min(quanta, floor(quanta × cpu / budget)).
    pub quanta: i32,
    /// The meter you are charged by: DSL operations and WebAssembly fuel exactly, k × used / limit; trusted CPU in quanta. null when your turns are not metered, so thinking costs you nothing. A ComputeUnit as i32.
    pub unit: i32,
}
/// The latest version of ComputeV1.
pub type Compute = ComputeV1;
const _: () = assert!(size_of::<ComputeV1>() == 16);
const _: () = assert!(offset_of!(ComputeV1, k) == 0);
const _: () = assert!(offset_of!(ComputeV1, quanta) == 8);
const _: () = assert!(offset_of!(ComputeV1, unit) == 12);

/// The DSL words, one f64 each (booleans 0 or 1, enums their value), filled once a turn by words::fill.
/// Outside the robot API: no binding sees it.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DslWordsV1 {
    /// The ruleset in play.
    pub ruleset: f64,
    /// The ruleset in play.
    pub physics_mode: f64,
    /// The ruleset in play.
    pub zero_g: f64,
    /// Whether the drive sets speeds (direct) or forces.
    pub control_model: f64,
    /// Whether the drive sets speeds (direct) or forces.
    pub inertial: f64,
    /// The match is 3D.
    pub mode3: f64,
    /// This tick, from 0.
    pub tick: f64,
    /// The match time in seconds.
    pub time: f64,
    /// Ticks left: match.max_ticks less this tick.
    pub remaining: f64,
    /// The arena runs from 0 to this on each axis.
    pub width: f64,
    /// The arena runs from 0 to this on each axis.
    pub height: f64,
    /// The arena runs from 0 to this on each axis.
    pub depth: f64,
    /// The acceleration of gravity, units/s².
    pub gravity_x: f64,
    /// The acceleration of gravity, units/s².
    pub gravity_y: f64,
    /// The acceleration of gravity, units/s².
    pub gravity_z: f64,
    /// The kind of ground: none, flat or hills.
    pub terrain_kind: f64,
    /// The kind of ground: none, flat or hills.
    pub hills: f64,
    /// Health left.
    pub health: f64,
    /// Your centre.
    pub x: f64,
    /// Your centre.
    pub y: f64,
    /// Your centre.
    pub z: f64,
    /// Your velocity, units/s.
    pub vx: f64,
    /// Your velocity, units/s.
    pub vy: f64,
    /// Your velocity, units/s.
    pub vz: f64,
    /// Your chassis heading.
    pub heading: f64,
    /// Your chassis pitch; 0 in 2D.
    pub pitch: f64,
    /// Your rotation rates, degrees/s.
    pub yaw_rate: f64,
    /// Your rotation rates, degrees/s.
    pub pitch_rate: f64,
    /// Your rotation rates, degrees/s.
    pub roll_rate: f64,
    /// Your chassis axes in world axes.
    pub forward_x: f64,
    /// Your chassis axes in world axes.
    pub forward_y: f64,
    /// Your chassis axes in world axes.
    pub forward_z: f64,
    /// Your chassis axes in world axes.
    pub right_x: f64,
    /// Your chassis axes in world axes.
    pub right_y: f64,
    /// Your chassis axes in world axes.
    pub right_z: f64,
    /// Your chassis axes in world axes.
    pub up_x: f64,
    /// Your chassis axes in world axes.
    pub up_y: f64,
    /// Your chassis axes in world axes.
    pub up_z: f64,
    /// You rest on the ground, under gravity.
    pub grounded: f64,
    /// In 3D, the height of your underside above the ground.
    pub altitude: f64,
    /// Charge now, before this tick's regeneration and driving.
    pub battery: f64,
    /// Charge when full.
    pub battery_capacity: f64,
    /// Where the turret points now; a shot leaves along it.
    pub aim: f64,
    /// Where the turret points now; a shot leaves along it.
    pub aim_pitch: f64,
    /// Ticks until the gun can fire; 0 means it can fire this tick.
    pub cooldown: f64,
    /// The scan that produced this tick's contacts.
    pub scan_heading: f64,
    /// The scan that produced this tick's contacts.
    pub scan_pitch: f64,
    /// The scan that produced this tick's contacts.
    pub scan_arc: f64,
    /// The scan that produced this tick's contacts.
    pub scan_range: f64,
    /// The nearest precise contact this tick. Passive contacts are never returned.
    pub contact: f64,
    /// The nearest precise contact this tick. Passive contacts are never returned.
    pub range: f64,
    /// The nearest precise contact this tick. Passive contacts are never returned.
    pub bearing: f64,
    /// The nearest precise contact this tick. Passive contacts are never returned.
    pub elevation: f64,
    /// The most accurate passive contact heard in the last cue_ticks (60 by default), or absent.
    pub cue_present: f64,
    /// The most accurate passive contact heard in the last cue_ticks (60 by default), or absent.
    pub cue_error: f64,
    /// How fast the nearest contact's bearing turns, from two sightings.
    pub bearing_rate: f64,
    /// The sudden-death safe zone, once sudden death starts.
    pub zone_active: f64,
    /// The sudden-death safe zone, once sudden death starts.
    pub zone_radius: f64,
    /// The storm, once it enters.
    pub storm_active: f64,
    /// The storm, once it enters.
    pub storm_radius: f64,
    /// The hill, once it appears.
    pub hill_active: f64,
    /// The hill, once it appears.
    pub hill_radius: f64,
    /// Your score; absent when the match does not score.
    pub score: f64,
    /// The best other robot's score; absent when the match does not score.
    pub score_best: f64,
    /// Your speed: the length of me.velocity.
    pub speed: f64,
    /// Gravity is on: arena.gravity.z is not 0.
    pub gravity: f64,
    /// The ground height under you.
    pub terrain_height: f64,
    /// battery.charge over battery.capacity; 0 with no capacity.
    pub battery_fraction: f64,
    /// The nearest contact's bearing less gun.aim_now's heading, -180 to 180; 0 with no contact.
    pub aim_error: f64,
    /// The lowest shot power.
    pub power_min: f64,
    /// The highest shot power.
    pub power_max: f64,
    /// Shot speed at power 0.
    pub shot_speed_base: f64,
    /// Shot speed added per unit of power.
    pub shot_speed_per_power: f64,
    /// Shot cost at power 0.
    pub shot_cost_base: f64,
    /// Shot cost added per unit of power.
    pub shot_cost_per_power: f64,
    /// The distance from you to the zone's centre; 0 before sudden death.
    pub zone_distance: f64,
    /// The heading from you to the zone's centre; your heading before sudden death.
    pub zone_bearing: f64,
    /// The elevation from you to the zone's centre; 0 before sudden death.
    pub zone_elevation: f64,
    /// The planar distance from you to the storm's edge, negative inside; 1000000000000 with no storm.
    pub storm_distance: f64,
    /// The heading from you to the storm's centre; your heading with no storm.
    pub storm_bearing: f64,
    /// The heading the storm moves along; 0 at rest or with no storm.
    pub storm_heading: f64,
    /// The cue came from a shot.
    pub cue_muzzle: f64,
    /// The heading from you to where the cue placed its robot; your heading with no cue.
    pub cue_bearing: f64,
    /// The elevation from you to where the cue placed its robot; 0 with no cue.
    pub cue_elevation: f64,
    /// The distance from you to where the cue placed its robot; 1000000000000 with no cue.
    pub cue_range: f64,
    /// Ticks since the cue was heard; 0 with no cue.
    pub cue_age: f64,
    /// A pickup of any kind is on the field.
    pub pickup_present: f64,
    /// The heading to the nearest pickup; your heading with none.
    pub pickup_bearing: f64,
    /// The elevation to the nearest pickup in 3D without gravity; 0 otherwise.
    pub pickup_elevation: f64,
    /// The field distance to the nearest pickup; 1000000000000 with none.
    pub pickup_range: f64,
    /// You are inside the hill.
    pub on_hill: f64,
    /// You hold the hill.
    pub hill_held: f64,
    /// The heading to the hill's centre; your heading with no hill.
    pub hill_bearing: f64,
    /// The elevation to the hill's centre in 3D without gravity; 0 otherwise.
    pub hill_elevation: f64,
    /// The distance to the hill's centre; 1000000000000 with no hill.
    pub hill_range: f64,
    /// Seconds you have held the hill.
    pub hill_time: f64,
    /// Seconds of holding still needed to win; 0 when holding never wins.
    pub hill_to_win: f64,
}
const _: () = assert!(size_of::<DslWordsV1>() == 776);

/// What an act field holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldKind {
    Number,
    Flag,
    Text,
}

/// Every act field a control sets, one C symbol each.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Field {
    DriveThrottle,
    DriveTurn,
    DrivePitchTurn,
    DriveThrust,
    DriveStrafe,
    DriveLift,
    DriveYawTorque,
    DrivePitchTorque,
    DriveRollTorque,
    DriveBrake,
    GunAimHeading,
    GunAimPitch,
    GunFire,
    GunPower,
    ScannerHeading,
    ScannerArc,
    ScannerPitch,
    StatusText,
    StatusValue,
}

impl Field {
    pub const ALL: [Field; 19] = [Field::DriveThrottle, Field::DriveTurn, Field::DrivePitchTurn, Field::DriveThrust, Field::DriveStrafe, Field::DriveLift, Field::DriveYawTorque, Field::DrivePitchTorque, Field::DriveRollTorque, Field::DriveBrake, Field::GunAimHeading, Field::GunAimPitch, Field::GunFire, Field::GunPower, Field::ScannerHeading, Field::ScannerArc, Field::ScannerPitch, Field::StatusText, Field::StatusValue];

    /// The act group and field it writes: ("drive", Some("throttle")); a flag names only its group.
    pub fn wire(self) -> (&'static str, Option<&'static str>) {
        match self {
            Field::DriveThrottle => ("drive", Some("throttle")),
            Field::DriveTurn => ("drive", Some("turn")),
            Field::DrivePitchTurn => ("drive", Some("pitchTurn")),
            Field::DriveThrust => ("forces", Some("thrust")),
            Field::DriveStrafe => ("forces", Some("strafe")),
            Field::DriveLift => ("forces", Some("lift")),
            Field::DriveYawTorque => ("forces", Some("yawTorque")),
            Field::DrivePitchTorque => ("forces", Some("pitchTorque")),
            Field::DriveRollTorque => ("forces", Some("rollTorque")),
            Field::DriveBrake => ("forces", Some("brake")),
            Field::GunAimHeading => ("aim", Some("heading")),
            Field::GunAimPitch => ("aim", Some("pitch")),
            Field::GunFire => ("fire", None),
            Field::GunPower => ("fire", Some("power")),
            Field::ScannerHeading => ("scan", Some("heading")),
            Field::ScannerArc => ("scan", Some("arc")),
            Field::ScannerPitch => ("scan", Some("pitch")),
            Field::StatusText => ("status", Some("text")),
            Field::StatusValue => ("status", Some("value")),
        }
    }

    pub fn kind(self) -> FieldKind {
        match self {
            Field::DriveThrottle => FieldKind::Number,
            Field::DriveTurn => FieldKind::Number,
            Field::DrivePitchTurn => FieldKind::Number,
            Field::DriveThrust => FieldKind::Number,
            Field::DriveStrafe => FieldKind::Number,
            Field::DriveLift => FieldKind::Number,
            Field::DriveYawTorque => FieldKind::Number,
            Field::DrivePitchTorque => FieldKind::Number,
            Field::DriveRollTorque => FieldKind::Number,
            Field::DriveBrake => FieldKind::Number,
            Field::GunAimHeading => FieldKind::Number,
            Field::GunAimPitch => FieldKind::Number,
            Field::GunFire => FieldKind::Flag,
            Field::GunPower => FieldKind::Number,
            Field::ScannerHeading => FieldKind::Number,
            Field::ScannerArc => FieldKind::Number,
            Field::ScannerPitch => FieldKind::Number,
            Field::StatusText => FieldKind::Text,
            Field::StatusValue => FieldKind::Number,
        }
    }

    /// The range the schema gives the field, which the act write clamps to; None where the clamp comes from the hello or there is none.
    pub fn limits(self) -> Option<(f64, f64)> {
        match self {
            Field::DriveThrottle => Some((-1.0, 1.0)),
            Field::DriveTurn => Some((-1.0, 1.0)),
            Field::DrivePitchTurn => Some((-1.0, 1.0)),
            Field::DriveThrust => Some((-1.0, 1.0)),
            Field::DriveStrafe => Some((-1.0, 1.0)),
            Field::DriveLift => Some((-1.0, 1.0)),
            Field::DriveYawTorque => Some((-1.0, 1.0)),
            Field::DrivePitchTorque => Some((-1.0, 1.0)),
            Field::DriveRollTorque => Some((-1.0, 1.0)),
            Field::DriveBrake => Some((0.0, 1.0)),
            Field::GunAimHeading => None,
            Field::GunAimPitch => None,
            Field::GunFire => None,
            Field::GunPower => None,
            Field::ScannerHeading => None,
            Field::ScannerArc => None,
            Field::ScannerPitch => None,
            Field::StatusText => None,
            Field::StatusValue => None,
        }
    }
}
