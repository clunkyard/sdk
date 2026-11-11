//! Contacts as records. nearest is the nearest precise contact, the first
//! on a tie; cues are the passive contacts, smallest bearing_error first.

use crate::generated::*;
use crate::maths::contact_position;
use crate::message::{SeenContact, State};
use crate::runtime::Robot;

/// The nearest precise contact this tick.
pub fn nearest_seen(state: &State) -> Option<&SeenContact> {
    let mut best: Option<&SeenContact> = None;
    for contact in &state.contacts {
        if contact.id.robot().is_some() && best.is_none_or(|b| contact.range < b.range) {
            best = Some(contact);
        }
    }
    best
}

/// A contact as its record has it.
pub fn record(r: &Robot, state: &State, contact: &SeenContact) -> ContactV1 {
    let precise = contact.id.robot();
    ContactV1 {
        slot: r.slot(precise),
        source: contact.source,
        range: contact.range,
        bearing: contact.bearing,
        elevation: contact.elevation,
        bearing_error: if precise.is_some() { 0.0 } else { contact.bearing_error.unwrap_or(0.0) },
        range_error: if precise.is_some() { 0.0 } else { contact.range_error.unwrap_or(0.0) },
        position: contact_position(state.me.position, contact.range, contact.bearing, contact.elevation),
    }
}

pub fn contacts(r: &Robot) -> Result<Vec<ContactV1>, Error> {
    let state = r.current()?;
    Ok(state.contacts.iter().map(|contact| record(r, state, contact)).collect())
}

pub fn nearest(r: &Robot) -> Result<Option<ContactV1>, Error> {
    let state = r.current()?;
    Ok(nearest_seen(state).map(|contact| record(r, state, contact)))
}

/// Passive contacts, most accurate first; a stable sort.
pub fn cues(r: &Robot) -> Result<Vec<ContactV1>, Error> {
    let state = r.current()?;
    let mut cues: Vec<ContactV1> = state.contacts.iter().filter(|contact| contact.id.robot().is_none()).map(|contact| record(r, state, contact)).collect();
    cues.sort_by(|a, b| (a.bearing_error - b.bearing_error).partial_cmp(&0.0).unwrap_or(core::cmp::Ordering::Equal));
    Ok(cues)
}

pub fn contact(r: &Robot, slot: i32) -> Result<Option<ContactV1>, Error> {
    let id = r.slot_id(slot)?;
    let state = r.current()?;
    Ok(id.and_then(|id| state.contacts.iter().find(|contact| contact.id.robot() == Some(id))).map(|contact| record(r, state, contact)))
}

pub fn position_of(r: &Robot, contact: &ContactV1) -> Result<Vec3, Error> {
    let state = r.current()?;
    for (name, value) in [("range", contact.range), ("bearing", contact.bearing), ("elevation", contact.elevation)] {
        if !value.is_finite() {
            return Err(Error::not_finite(format!("{name} is not finite")));
        }
    }
    Ok(contact_position(state.me.position, contact.range, contact.bearing, contact.elevation))
}
