//! The handle behind cy_robot *: the session, the decoded messages, the
//! robot slots, the Tracker, the pending act and the panic count.
//!
//! A panic in any export marks the handle for reset: the tick it happened
//! on is answered with an empty act, and the next state rebuilds the
//! session from the kept hello, with an empty Tracker and a cleared act.
//! Slots stay, since they are stable for the match. After three panics in a
//! match the handle is poisoned and answers every later state with an
//! empty act (sdk/README.md, "Resilience").

use crate::act::Act;
use crate::generated::*;
use crate::maths::angle_diff;
use crate::formula::Formula;
use crate::message::{Formulas, Hello, State};
use crate::port::{MemoryPort, Port, StdioPort};
use crate::tracker::{Settings, Tracker};

/// Panics in a match before the handle is poisoned.
pub const POISON_AFTER: u32 = 3;

pub enum PortKind {
    Stdio,
    Memory,
}

pub struct Robot {
    pub port: Box<dyn Port>,
    pub binding: String,
    pub binding_version: String,
    pub hello: Option<Hello>,
    pub state: Option<State>,
    /// The hello's line, or the latest state's, exactly as it arrived.
    pub line: Option<String>,
    pub event: Option<Event>,
    /// Whether the current hello or state has had its reply.
    pub answered: bool,
    /// A successful forfeit ends the handle even when the port has more frames.
    pub(crate) forfeited: bool,
    /// Robot ids by slot: slot 1 is ids[0], you.
    pub ids: Vec<String>,
    pub tracker_settings: Settings,
    pub tracker: Tracker,
    /// The nearest precise contact's tick and bearing, for the bearing rate.
    pub sighting: Option<(f64, f64)>,
    pub bearing_rate: f64,
    pub act: Act,
    pub last_error: String,
    pub panics: u32,
    pub reset: bool,
    /// The tick a trapped instance was on (wasm32 recovery), before any state.
    pub fault_tick: Option<f64>,
}

pub fn open(port: PortKind, binding: &str, version: &str) -> Result<Robot, Error> {
    let port: Box<dyn Port> = match port {
        PortKind::Stdio => Box::new(StdioPort::open()),
        PortKind::Memory => Box::new(MemoryPort::default()),
    };
    Ok(Robot {
        port,
        binding: binding.to_string(),
        binding_version: version.to_string(),
        hello: None,
        state: None,
        line: None,
        event: None,
        answered: true,
        forfeited: false,
        ids: Vec::new(),
        tracker_settings: Settings::default(),
        tracker: Tracker::default(),
        sighting: None,
        bearing_rate: 0.0,
        act: Act::default(),
        last_error: String::new(),
        panics: 0,
        reset: false,
        fault_tick: None,
    })
}

pub fn last_error(r: &Robot) -> &str {
    &r.last_error
}

pub fn record_error(r: &mut Robot, error: &Error) {
    r.last_error.clone_from(&error.message);
}

pub fn record_panic(r: &mut Robot, symbol: &'static str) {
    r.panics += 1;
    r.reset = true;
    r.last_error = format!("a fault in the core ({symbol}); it is not the robot's");
    let then = if r.poisoned() { "the core answers every later state with an empty act" } else { "the core resets on the next state" };
    crate::session::log_line(&format!("clunkyard: {} (fault {} of {POISON_AFTER}); {then}", r.last_error, r.panics));
}

impl Robot {
    pub fn poisoned(&self) -> bool {
        self.panics >= POISON_AFTER
    }

    pub fn hello(&self) -> Result<&Hello, Error> {
        self.hello.as_ref().ok_or_else(|| Error::phase("no hello yet"))
    }

    pub fn current(&self) -> Result<&State, Error> {
        self.state.as_ref().ok_or_else(|| Error::phase("no state yet"))
    }

    /// One of the hello's formulas at x.
    pub fn formula(&self, name: &str, pick: fn(&Formulas) -> Option<Formula>, x: f64) -> Result<f64, Error> {
        if !x.is_finite() {
            return Err(Error::not_finite("value is not finite"));
        }
        let formula = pick(&self.hello()?.formulas).ok_or_else(|| Error::argument(format!("the hello has no formulas.{name}")))?;
        formula.evaluate(x).ok_or_else(|| Error::internal(format!("formulas.{name} has an exponent the core does not know")))
    }

    /// The slot of a robot id, 0 for none; ids get slots as they appear.
    pub fn slot_of(&mut self, id: Option<&str>) -> i32 {
        let Some(id) = id.filter(|id| !id.is_empty()) else { return 0 };
        let index = match self.ids.iter().position(|known| known == id) {
            Some(index) => index,
            None => {
                self.ids.push(id.to_string());
                self.ids.len() - 1
            }
        };
        index as i32 + 1
    }

    /// The slot of an id already seen, 0 for none.
    pub fn slot(&self, id: Option<&str>) -> i32 {
        id.and_then(|id| self.ids.iter().position(|known| known == id)).map_or(0, |index| index as i32 + 1)
    }

    /// The id in a slot argument, if that robot has been seen; E_RANGE
    /// outside 1 to match.robots.
    pub fn slot_id(&self, slot: i32) -> Result<Option<&str>, Error> {
        let robots = self.hello()?.robots;
        if slot < 1 || f64::from(slot) > robots {
            return Err(Error::range(format!("slot {slot} is not 1 to {robots}")));
        }
        Ok(self.ids.get(slot as usize - 1).map(String::as_str))
    }

    /// Starts the match from a hello.
    pub fn begin(&mut self, hello: Hello, line: String) {
        self.ids = vec![hello.id.clone()];
        self.hello = Some(hello);
        self.state = None;
        self.line = Some(line);
        self.restart();
        self.panics = 0;
        self.reset = false;
    }

    /// The session as the hello left it: an empty Tracker and act.
    pub fn restart(&mut self) {
        self.tracker = Tracker::new(self.tracker_settings);
        self.sighting = None;
        self.bearing_rate = 0.0;
        self.act = Act::default();
    }

    /// The core receiving one state.
    pub fn update(&mut self, state: State, line: String) {
        if self.reset && !self.poisoned() {
            self.restart();
            self.reset = false;
        }
        for contact in &state.contacts {
            let id = contact.id.robot().map(str::to_string);
            self.slot_of(id.as_deref());
        }
        if let Some(holder) = state.hill.as_ref().and_then(|hill| hill.holder.clone()) {
            self.slot_of(Some(&holder));
        }
        let tick_rate = self.hello.as_ref().map_or(20.0, |hello| hello.tick_rate);
        self.tracker.update(state.tick, state.me.position, &state.contacts, tick_rate);
        // The nearest precise contact's bearing rate, degrees a second, from
        // this state and the one just before; 0 unless both had one.
        let nearest = crate::sensors::nearest_seen(&state).map(|contact| contact.bearing);
        self.bearing_rate = match (nearest, self.sighting) {
            (Some(bearing), Some((tick, last))) if tick == state.tick - 1.0 => angle_diff(bearing, last) * tick_rate,
            _ => 0.0,
        };
        self.sighting = nearest.map(|bearing| (state.tick, bearing));
        self.state = Some(state);
        self.line = Some(line);
        self.fault_tick = None;
    }
}

impl Robot {
    /// A fresh handle standing in for one whose instance trapped (wasm32),
    /// just fed the kept hello: the panics so far, and the tick that
    /// faulted, which send answers with an empty act.
    pub fn recover(&mut self, panics: u32, tick: f64) {
        self.panics = panics;
        self.reset = true;
        if tick >= 0.0 {
            self.fault_tick = Some(tick);
            self.event = Some(Event::State);
            self.answered = false;
        }
    }
}
