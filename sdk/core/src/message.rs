//! The hello and the state, decoded once each into what the subsystems read.
//! The parsed message stays beside them for raw reads.

use crate::formula::Formula;
use crate::generated::*;
use crate::json::Value;

fn number(value: &Value, path: &str) -> f64 {
    value.at(path).and_then(Value::as_f64).unwrap_or(0.0)
}

fn optional(value: &Value, path: &str) -> Option<f64> {
    value.at(path).and_then(Value::as_f64)
}

fn vec3(value: Option<&Value>) -> Vec3 {
    let Some(value) = value else { return Vec3::default() };
    Vec3 { x: number(value, "x"), y: number(value, "y"), z: number(value, "z") }
}

fn wire<T>(value: Option<&Value>, from: fn(&str) -> Option<T>) -> Option<T> {
    value.and_then(Value::as_str).and_then(from)
}

/// The formulas the hello carries, by name.
#[derive(Clone, Debug, Default)]
pub struct Formulas {
    pub shot_speed: Option<Formula>,
    pub shot_cost: Option<Formula>,
    pub reload_ticks: Option<Formula>,
    pub shot_damage: Option<Formula>,
    pub power_for_speed: Option<Formula>,
    pub reach: Option<Formula>,
    pub arc_for_reach: Option<Formula>,
    pub drive_cost: Option<Formula>,
}

#[derive(Clone, Debug)]
pub enum Terrain {
    None,
    Flat { ground_z: f64 },
    Hills { cols: f64, rows: f64, heights: Vec<f64> },
}

/// What the core reads from the hello.
#[derive(Clone, Debug)]
pub struct Hello {
    pub raw: Value,
    pub id: String,
    pub mode: Mode,
    pub ruleset: Ruleset,
    pub control: Control,
    pub tick_rate: f64,
    pub max_ticks: f64,
    pub robots: f64,
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub gravity: Vec3,
    pub terrain_kind: TerrainKind,
    pub terrain: Terrain,
    pub max_pitch: f64,
    pub capacity: f64,
    pub regen: f64,
    pub compute: Option<ComputeV1>,
    pub sustainable_drive: f64,
    pub hover_lift: f64,
    pub power_min: f64,
    pub power_max: f64,
    pub muzzle_distance: f64,
    pub shot_range: f64,
    pub proximity_range: f64,
    pub min_arc: f64,
    pub max_arc: f64,
    pub shape: Shape,
    pub win_ticks: f64,
    pub formulas: Formulas,
}

impl Hello {
    /// A protocol 2 hello, or the reason it is not one.
    pub fn decode(raw: Value) -> Result<Hello, Error> {
        // A protocol 1 hello carries "v":2 under rules v2, so its shape decides.
        if raw.get("v").and_then(Value::as_f64) != Some(2.0) || raw.get("type").and_then(Value::as_str) != Some("hello") || !raw.get("match").is_some_and(Value::is_object) {
            return Err(Error::version("this core speaks protocol 2; add \"protocol\": 2 to bot.json"));
        }
        let id = raw.at("self.id").and_then(Value::as_str).unwrap_or("").to_string();
        let terrain = raw.at("arena.terrain");
        let terrain_kind = wire(terrain.and_then(|t| t.get("kind")), TerrainKind::from_wire).unwrap_or(TerrainKind::None);
        let terrain = match terrain_kind {
            TerrainKind::None => Terrain::None,
            TerrainKind::Flat => Terrain::Flat { ground_z: number(&raw, "arena.terrain.groundZ") },
            TerrainKind::Hills => Terrain::Hills {
                cols: number(&raw, "arena.terrain.cols"),
                rows: number(&raw, "arena.terrain.rows"),
                heights: raw.at("arena.terrain.heights").and_then(Value::as_array).unwrap_or(&[]).iter().map(|h| h.as_f64().unwrap_or(0.0)).collect(),
            },
        };
        let compute = raw.at("battery.compute").filter(|c| c.is_object()).map(|c| ComputeV1 {
            k: number(c, "k"),
            quanta: number(c, "quanta") as i32,
            unit: wire(c.get("unit"), ComputeUnit::from_wire).map_or(0, |unit| unit as i32),
        });
        let formula = |name: &str| raw.at("formulas").and_then(|f| f.get(name)).and_then(Formula::from_json);
        let formulas = Formulas {
            shot_speed: formula("shotSpeed"),
            shot_cost: formula("shotCost"),
            reload_ticks: formula("reloadTicks"),
            shot_damage: formula("shotDamage"),
            power_for_speed: formula("powerForSpeed"),
            reach: formula("reach"),
            arc_for_reach: formula("arcForReach"),
            drive_cost: formula("driveCost"),
        };
        Ok(Hello {
            id,
            mode: wire(raw.at("match.mode"), Mode::from_wire).unwrap_or(Mode::TwoD),
            ruleset: wire(raw.at("match.ruleset"), Ruleset::from_wire).unwrap_or(Ruleset::Classic),
            control: wire(raw.at("match.control"), Control::from_wire).unwrap_or(Control::Direct),
            tick_rate: optional(&raw, "match.tickRate").unwrap_or(20.0),
            max_ticks: number(&raw, "match.maxTicks"),
            robots: number(&raw, "match.robots"),
            width: number(&raw, "arena.width"),
            height: number(&raw, "arena.height"),
            depth: number(&raw, "arena.depth"),
            gravity: vec3(raw.at("arena.gravity")),
            terrain_kind,
            terrain,
            max_pitch: number(&raw, "robot.maxPitch"),
            capacity: number(&raw, "battery.capacity"),
            regen: number(&raw, "battery.regen"),
            compute,
            sustainable_drive: number(&raw, "derived.sustainableDrive"),
            hover_lift: number(&raw, "derived.hoverLift"),
            power_min: number(&raw, "gun.power.min"),
            power_max: number(&raw, "gun.power.max"),
            muzzle_distance: number(&raw, "gun.muzzleDistance"),
            shot_range: number(&raw, "gun.shotRange"),
            proximity_range: number(&raw, "scanner.proximityRange"),
            min_arc: number(&raw, "scanner.minArc"),
            max_arc: number(&raw, "scanner.maxArc"),
            shape: wire(raw.at("suddenDeath.shape"), Shape::from_wire).unwrap_or(Shape::Circle),
            win_ticks: number(&raw, "objectives.hill.winTicks"),
            formulas,
            raw,
        })
    }
}

/// A contact's id: a robot's for a precise contact, null for a passive one.
#[derive(Clone, Debug, PartialEq)]
pub enum Id {
    Absent,
    Null,
    Text(String),
}

impl Id {
    fn of(value: Option<&Value>) -> Id {
        match value {
            None => Id::Absent,
            Some(Value::String(text)) => Id::Text(text.clone()),
            Some(Value::Null) => Id::Null,
            Some(_) => Id::Absent,
        }
    }

    /// A precise contact names its robot.
    pub fn robot(&self) -> Option<&str> {
        match self {
            Id::Text(text) if !text.is_empty() => Some(text),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SeenContact {
    pub id: Id,
    pub source: i32,
    pub range: f64,
    pub bearing: f64,
    pub elevation: f64,
    pub bearing_error: Option<f64>,
    pub range_error: Option<f64>,
}

#[derive(Clone, Copy, Debug)]
pub struct Region {
    pub shape: i32,
    pub center: Vec3,
    pub radius: f64,
}

#[derive(Clone, Debug)]
pub struct HillState {
    pub record: HillV1,
    pub holder: Option<String>,
    pub held_ticks: f64,
}

#[derive(Clone, Debug)]
pub struct Me {
    pub health: f64,
    pub position: Vec3,
    pub velocity: Vec3,
    pub heading: f64,
    pub pitch: f64,
    pub roll: f64,
    pub angular_velocity: Rotation,
    pub attitude: AttitudeV1,
    pub grounded: bool,
    pub altitude: f64,
    pub applied: AppliedV1,
    pub battery: f64,
    pub aim: Direction,
    pub cooldown_ticks: f64,
    pub spread: f64,
}

/// What the core reads from a state.
#[derive(Clone, Debug)]
pub struct State {
    pub raw: Value,
    pub tick: f64,
    pub me: Me,
    pub scan: ScanV1,
    pub contacts: Vec<SeenContact>,
    pub zone: Option<Region>,
    pub storm: Option<StormV1>,
    pub hill: Option<HillState>,
    pub pickups: Vec<PickupV1>,
    pub score_me: Option<f64>,
    pub score_best: Option<f64>,
    pub armour: Option<f64>,
    pub max_power: Option<f64>,
    pub damage_scale: Option<f64>,
}

impl State {
    pub fn decode(raw: Value) -> State {
        let me = raw.get("self").cloned().unwrap_or(Value::Null);
        let rotation = |value: Option<&Value>| {
            let value = value.unwrap_or(&Value::Null);
            Rotation { yaw: number(value, "yaw"), pitch: number(value, "pitch"), roll: number(value, "roll") }
        };
        let me = Me {
            health: number(&me, "health"),
            position: vec3(me.get("position")),
            velocity: vec3(me.get("velocity")),
            heading: number(&me, "heading"),
            pitch: number(&me, "pitch"),
            roll: number(&me, "roll"),
            angular_velocity: rotation(me.get("angularVelocity")),
            attitude: AttitudeV1 { forward: vec3(me.at("attitude.forward")), right: vec3(me.at("attitude.right")), up: vec3(me.at("attitude.up")) },
            grounded: me.get("grounded").and_then(Value::as_bool).unwrap_or(false),
            altitude: number(&me, "altitude"),
            applied: AppliedV1 { force: vec3(me.at("applied.force")), torque: rotation(me.at("applied.torque")), brake: number(&me, "applied.brake") },
            battery: number(&me, "battery"),
            aim: Direction { heading: number(&me, "aim.heading"), pitch: number(&me, "aim.pitch") },
            cooldown_ticks: number(&me, "cooldownTicks"),
            spread: number(&me, "spread"),
        };
        let scan = raw.at("sensors.scan").unwrap_or(&Value::Null);
        let scan = ScanV1 { heading: number(scan, "heading"), pitch: number(scan, "pitch"), arc: number(scan, "arc"), range: number(scan, "range") };
        let contacts = raw
            .at("sensors.contacts")
            .and_then(Value::as_array)
            .unwrap_or(&[])
            .iter()
            .map(|c| SeenContact {
                id: Id::of(c.get("id")),
                source: wire(c.get("source"), Source::from_wire).map_or(0, |s| s as i32),
                range: number(c, "range"),
                bearing: number(c, "bearing"),
                elevation: number(c, "elevation"),
                bearing_error: optional(c, "bearingError"),
                range_error: optional(c, "rangeError"),
            })
            .collect();
        let shape = |value: &Value| wire(value.get("shape"), Shape::from_wire).map_or(0, |s| s as i32);
        let zone = raw.get("zone").filter(|z| z.is_object()).map(|z| Region { shape: shape(z), center: vec3(z.get("center")), radius: number(z, "radius") });
        let storm = raw.get("storm").filter(|s| s.is_object()).map(|s| StormV1 {
            shape: shape(s),
            reserved: 0,
            radius: number(s, "radius"),
            dps: number(s, "dps"),
            center: vec3(s.get("center")),
            velocity: vec3(s.get("velocity")),
        });
        let objectives = raw.get("objectives").unwrap_or(&Value::Null);
        let hill = objectives.get("hill").filter(|h| h.is_object()).map(|h| HillState {
            record: HillV1 {
                shape: shape(h),
                holder: 0,
                contested: i32::from(h.get("contested").and_then(Value::as_bool).unwrap_or(false)),
                moves_tick: number(h, "movesTick") as i32,
                held_ticks: number(h, "heldTicks") as i32,
                best_held_ticks: number(h, "bestHeldTicks") as i32,
                radius: number(h, "radius"),
                center: vec3(h.get("center")),
                next: vec3(h.get("next")),
            },
            holder: h.get("holder").and_then(Value::as_str).filter(|id| !id.is_empty()).map(str::to_string),
            held_ticks: number(h, "heldTicks"),
        });
        let pickups = objectives
            .get("pickups")
            .and_then(Value::as_array)
            .unwrap_or(&[])
            .iter()
            .map(|p| PickupV1 {
                kind: wire(p.get("kind"), PickupKind::from_wire).map_or(0, |k| k as i32),
                expires_tick: number(p, "expiresTick") as i32,
                amount: number(p, "amount"),
                position: vec3(p.get("position")),
            })
            .collect();
        State {
            tick: number(&raw, "tick"),
            me,
            scan,
            contacts,
            zone,
            storm,
            hill,
            pickups,
            score_me: optional(objectives, "score.self"),
            score_best: optional(objectives, "score.best"),
            armour: optional(objectives, "armour"),
            max_power: optional(&raw, "derived.self.maxPower"),
            damage_scale: optional(&raw, "derived.gun.damageScale"),
            raw,
        }
    }
}
