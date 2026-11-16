//! The pending act (reference.js Act).
//!
//! Controls are set one field at a time. A later write to a field replaces
//! the earlier one and the other fields keep theirs. get reads back exactly
//! what was set, before clamping. The act carries only the fields that were
//! set, clamped as the arena would clamp them: drive fields to their schema
//! limits, headings by normalize, pitches to ±robot.maxPitch, the scan arc
//! to scanner.minArc to maxArc, the power to gun.power.min to max, the
//! status value to ±1,000,000,000 and its text to 32 code points, and the
//! scanner switch is written true for any non-zero value. fire is
//! sent only after gun.fire, with the power if gun.power was set. The
//! first four debug lines of a turn are kept, each with control characters
//! as spaces and cut to 120 bytes ending in an ellipsis (src/robot-log.js).
//! The members come in a fixed order: v, type, tick, drive, forces, aim,
//! scan, fire, status, log, then the extra fields in the order they were
//! first given.

use crate::generated::*;
use crate::json::Value;
use crate::maths::{between, normalize};
use crate::message::Hello;
use crate::runtime::Robot;

const STATUS_TEXT: usize = 32;
const STATUS_VALUE: f64 = 1e9;
const LOG_LINES: usize = 4;
const LOG_BYTES: usize = 120;
const ELLIPSIS: char = '…';
/// The members the API writes, which raw.extra cannot set.
pub const RESERVED: [&str; 10] = ["v", "type", "tick", "drive", "forces", "aim", "scan", "fire", "status", "log"];

/// Each group's fields in schema order.
const GROUPS: [(&str, &[Field]); 6] = [
    ("drive", &[Field::DriveThrottle, Field::DriveTurn, Field::DrivePitchTurn]),
    (
        "forces",
        &[Field::DriveThrust, Field::DriveStrafe, Field::DriveLift, Field::DriveYawTorque, Field::DrivePitchTorque, Field::DriveRollTorque, Field::DriveBrake],
    ),
    ("aim", &[Field::GunAimHeading, Field::GunAimPitch]),
    ("scan", &[Field::ScannerHeading, Field::ScannerPitch, Field::ScannerArc, Field::ScannerOn]),
    ("fire", &[Field::GunPower]),
    ("status", &[Field::StatusText, Field::StatusValue]),
];

#[derive(Clone, Debug, Default)]
pub struct Act {
    numbers: [Option<f64>; Field::ALL.len()],
    text: Option<String>,
    firing: bool,
    lines: Vec<String>,
    extras: Vec<(String, Value)>,
}

fn index(field: Field) -> usize {
    Field::ALL.iter().position(|item| *item == field).unwrap_or(0)
}

impl Act {
    pub fn set_number(&mut self, field: Field, value: f64) {
        self.numbers[index(field)] = Some(value);
    }

    pub fn number(&self, field: Field) -> Option<f64> {
        self.numbers[index(field)]
    }

    /// A debug line, as robot-log.js keeps it; past four a turn it is dropped.
    pub fn log(&mut self, line: &str) {
        if self.lines.len() >= LOG_LINES {
            return;
        }
        let line: String = line.chars().map(|c| if c < ' ' || c == '\u{7f}' { ' ' } else { c }).collect();
        if line.len() <= LOG_BYTES {
            self.lines.push(line);
            return;
        }
        let room = LOG_BYTES - ELLIPSIS.len_utf8();
        let mut kept = String::new();
        for c in line.chars() {
            if kept.len() + c.len_utf8() > room {
                break;
            }
            kept.push(c);
        }
        kept.push(ELLIPSIS);
        self.lines.push(kept);
    }

    /// Fields the API does not model, from a JSON object; none may be one
    /// of the API's own members.
    pub fn extra(&mut self, json: &str) -> Result<(), Error> {
        let Ok(Value::Object(members)) = crate::json::parse(json) else {
            return Err(Error::argument("extra is not a JSON object"));
        };
        if let Some((key, _)) = members.iter().find(|(key, _)| RESERVED.contains(&key.as_str())) {
            return Err(Error::argument(format!("extra cannot set {key}")));
        }
        let mut extras = Value::Object(std::mem::take(&mut self.extras));
        for (key, value) in members {
            extras.insert(&key, value);
        }
        if let Value::Object(members) = extras {
            self.extras = members;
        }
        Ok(())
    }

    fn clamped(field: Field, value: f64, hello: &Hello) -> f64 {
        if let Some((low, high)) = field.limits() {
            return between(value, low, high) + 0.0;
        }
        match field {
            Field::GunAimHeading | Field::ScannerHeading => normalize(value),
            Field::GunAimPitch | Field::ScannerPitch => between(value, -hello.max_pitch, hello.max_pitch) + 0.0,
            Field::ScannerArc => between(value, hello.min_arc, hello.max_arc),
            Field::GunPower => between(value, hello.power_min, hello.power_max),
            Field::StatusValue => between(value, -STATUS_VALUE, STATUS_VALUE) + 0.0,
            _ => value,
        }
    }

    /// The act line for tick.
    pub fn line(&self, tick: f64, hello: &Hello) -> String {
        let mut message = empty_act(tick);
        for (group, fields) in GROUPS {
            if group == "fire" && !self.firing {
                continue;
            }
            let mut block = Value::Object(Vec::new());
            for field in fields {
                let (_, Some(name)) = field.wire() else { continue };
                if *field == Field::StatusText {
                    if let Some(text) = &self.text {
                        block.insert(name, Value::String(text.chars().take(STATUS_TEXT).collect()));
                    }
                } else if let Some(value) = self.number(*field) {
                    // The scanner switch is a boolean on the wire: any non-zero value is on.
                    let value = if *field == Field::ScannerOn { Value::Bool(value != 0.0) } else { Value::Number(Act::clamped(*field, value, hello)) };
                    block.insert(name, value);
                }
            }
            if group == "fire" || matches!(&block, Value::Object(members) if !members.is_empty()) {
                message.insert(group, block);
            }
        }
        if !self.lines.is_empty() {
            message.insert("log", Value::Array(self.lines.iter().cloned().map(Value::String).collect()));
        }
        for (key, value) in &self.extras {
            message.insert(key, value.clone());
        }
        message.to_json()
    }
}

pub fn empty_act(tick: f64) -> Value {
    Value::Object(vec![("v".into(), Value::Number(2.0)), ("type".into(), Value::String("act".into())), ("tick".into(), Value::Number(tick))])
}

fn expect(field: Field, kind: FieldKind) -> Result<(), Error> {
    if field.kind() == kind {
        Ok(())
    } else {
        Err(Error::argument(format!("{field:?} is not that kind of field")))
    }
}

pub fn set_number(r: &mut Robot, field: Field, value: f64) -> Result<(), Error> {
    expect(field, FieldKind::Number)?;
    if !value.is_finite() {
        return Err(Error::not_finite(format!("{field:?} is not finite")));
    }
    r.act.set_number(field, value);
    Ok(())
}

pub fn set_flag(r: &mut Robot, field: Field) -> Result<(), Error> {
    expect(field, FieldKind::Flag)?;
    r.act.firing = true;
    Ok(())
}

pub fn set_text(r: &mut Robot, field: Field, text: &str) -> Result<(), Error> {
    expect(field, FieldKind::Text)?;
    r.act.text = Some(text.to_string());
    Ok(())
}

pub fn get(r: &Robot, field: Field) -> Result<Option<f64>, Error> {
    match field.kind() {
        FieldKind::Number => Ok(r.act.number(field)),
        FieldKind::Flag => Ok(r.act.firing.then_some(1.0)),
        FieldKind::Text => Err(Error::argument("read a text field with get_text")),
    }
}

pub fn get_text(r: &Robot, field: Field) -> Result<Option<String>, Error> {
    expect(field, FieldKind::Text)?;
    Ok(r.act.text.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Control characters become spaces, as the referee keeps them; the
    // vectors leave them out because not every language's literals carry them.
    #[test]
    fn a_log_line_turns_control_characters_into_spaces() {
        let mut act = Act::default();
        act.log("tab\tand\nnewline\u{7f}");
        assert_eq!(act.lines, ["tab and newline "]);
    }
}
