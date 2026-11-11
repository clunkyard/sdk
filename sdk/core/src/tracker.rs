//! The Tracker (reference.js Tracker), run once on each state inside the
//! core, and the tracker subsystem's queries.
//!
//! It estimates each precise contact's position and velocity from the ticks
//! it was seen, and remembers the most accurate passive contact (smallest
//! bearing error, the first on a tie) for cue_ticks. A contact seen again
//! within forget_ticks updates its track: its first velocity is the
//! measured one, later ones are smoothed as old × smoothing + measured ×
//! (1 − smoothing). A contact seen after longer starts a new track at rest.

use crate::generated::*;
use crate::maths::{add, contact_position, scale, sub};
use crate::message::{Id, SeenContact};
use crate::runtime::Robot;
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    pub smoothing: f64,
    pub forget_ticks: f64,
    pub cue_ticks: f64,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings { smoothing: 0.5, forget_ticks: 40.0, cue_ticks: f64::from(CUE_TICKS) }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Track {
    pub tick: f64,
    pub position: Vec3,
    pub velocity: Vec3,
    pub seen: u32,
}

/// The passive contact heard most recently, as heard.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Heard {
    pub position: Vec3,
    pub tick: f64,
    pub source: i32,
    pub bearing_error: f64,
    pub range: f64,
    pub bearing: f64,
    pub elevation: f64,
}

#[derive(Clone, Debug, Default)]
pub struct Tracker {
    pub settings: Settings,
    pub tracks: BTreeMap<String, Track>,
    pub latest: Option<Heard>,
    pub tick: f64,
}

impl Tracker {
    pub fn new(settings: Settings) -> Tracker {
        Tracker { settings, ..Tracker::default() }
    }

    pub fn update(&mut self, tick: f64, origin: Vec3, contacts: &[SeenContact], tick_rate: f64) {
        self.tick = tick;
        let mut best: Option<&SeenContact> = None;
        for contact in contacts {
            // Only a null id is a passive contact here, as in reference.js.
            if contact.id == Id::Null && best.is_none_or(|b| bearing_error(contact) < bearing_error(b)) {
                best = Some(contact);
            }
        }
        if let Some(best) = best {
            self.latest = Some(Heard {
                position: contact_position(origin, best.range, best.bearing, best.elevation),
                tick,
                source: best.source,
                bearing_error: bearing_error(best),
                range: best.range,
                bearing: best.bearing,
                elevation: best.elevation,
            });
        }
        for contact in contacts {
            let Some(id) = contact.id.robot() else { continue };
            let position = contact_position(origin, contact.range, contact.bearing, contact.elevation);
            let track = match self.tracks.get(id) {
                Some(old) if tick > old.tick && tick - old.tick <= self.settings.forget_ticks => {
                    let measured = scale(sub(position, old.position), tick_rate / (tick - old.tick));
                    let velocity = if old.seen > 1 {
                        add(scale(old.velocity, self.settings.smoothing), scale(measured, 1.0 - self.settings.smoothing))
                    } else {
                        measured
                    };
                    Track { tick, position, velocity, seen: old.seen + 1 }
                }
                _ => Track { tick, position, velocity: Vec3::default(), seen: 1 },
            };
            self.tracks.insert(id.to_string(), track);
        }
    }

    /// The freshest passive contact heard within cue_ticks.
    pub fn cue(&self) -> Option<&Heard> {
        self.latest.as_ref().filter(|cue| self.tick - cue.tick <= self.settings.cue_ticks)
    }
}

// A passive contact's bearingError; a missing one compares as JavaScript's
// undefined does, never smaller.
fn bearing_error(contact: &SeenContact) -> f64 {
    contact.bearing_error.unwrap_or(f64::NAN)
}

// ---- the subsystem ----

pub fn cue(r: &Robot) -> Result<Option<CueV1>, Error> {
    r.current()?;
    Ok(r.tracker.cue().map(|cue| CueV1 { source: cue.source, tick: cue.tick as i32, bearing_error: cue.bearing_error, position: cue.position }))
}

pub fn nearest_bearing_rate(r: &Robot) -> Result<f64, Error> {
    r.current()?;
    Ok(r.bearing_rate)
}

fn track(r: &Robot, slot: i32) -> Result<Option<&Track>, Error> {
    Ok(r.slot_id(slot)?.and_then(|id| r.tracker.tracks.get(id)))
}

pub fn velocity(r: &Robot, slot: i32) -> Result<Vec3, Error> {
    Ok(track(r, slot)?.map_or(Vec3::default(), |t| t.velocity))
}

pub fn position(r: &Robot, slot: i32) -> Result<Option<Vec3>, Error> {
    Ok(track(r, slot)?.map(|t| t.position))
}

pub fn last_seen(r: &Robot, slot: i32) -> Result<Option<i32>, Error> {
    Ok(track(r, slot)?.map(|t| t.tick as i32))
}

/// The Tracker's parameters, before the first state only.
pub fn configure(r: &mut Robot, smoothing: f64, forget_ticks: i32, cue_ticks: i32) -> Result<(), Error> {
    if r.state.is_some() {
        return Err(Error::phase("configure the Tracker before the first state"));
    }
    if !(0.0..=1.0).contains(&smoothing) {
        return Err(Error::range("smoothing is 0 to 1"));
    }
    if forget_ticks < 0 || cue_ticks < 0 {
        return Err(Error::range("tick counts are whole and not negative"));
    }
    let settings = Settings { smoothing, forget_ticks: f64::from(forget_ticks), cue_ticks: f64::from(cue_ticks) };
    r.tracker = Tracker::new(settings);
    r.tracker_settings = settings;
    Ok(())
}
