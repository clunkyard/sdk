//! Raw reads of the latest hello or state, by path, and raw act fields.
//!
//! A path is "hello" or "state", then .key for a member and [n] for an
//! array element, as in state.calibration.work or
//! state.sensors.contacts[0].range.

use crate::generated::*;
use crate::json::Value;
use crate::runtime::Robot;

enum Step<'a> {
    Key(&'a str),
    Index(&'a str),
}

fn steps(path: &str) -> Result<(bool, Vec<Step<'_>>), Error> {
    let bad = || Error::argument(format!("not a raw path: {path}"));
    let (hello, mut rest) = if let Some(rest) = path.strip_prefix("hello") {
        (true, rest)
    } else if let Some(rest) = path.strip_prefix("state") {
        (false, rest)
    } else {
        return Err(bad());
    };
    let mut steps = Vec::new();
    while !rest.is_empty() {
        if let Some(after) = rest.strip_prefix('.') {
            let end = after.find(['.', '[']).unwrap_or(after.len());
            let key = &after[..end];
            let mut chars = key.chars();
            if !chars.next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_') || !chars.all(|c| c.is_ascii_alphanumeric() || c == '_') {
                return Err(bad());
            }
            steps.push(Step::Key(key));
            rest = &after[end..];
        } else if let Some(after) = rest.strip_prefix('[') {
            let end = after.find(']').ok_or_else(bad)?;
            let digits = &after[..end];
            if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
                return Err(bad());
            }
            steps.push(Step::Index(digits));
            rest = &after[end + 1..];
        } else {
            return Err(bad());
        }
    }
    Ok((hello, steps))
}

fn value<'a>(r: &'a Robot, path: &str) -> Result<Option<&'a Value>, Error> {
    let (hello, steps) = steps(path)?;
    let mut value = if hello { &r.hello()?.raw } else { &r.current()?.raw };
    for step in steps {
        let next = match (step, value) {
            (Step::Index(digits), Value::Array(items)) => digits.parse::<usize>().ok().and_then(|i| items.get(i)),
            (Step::Key(key), Value::Object(_)) => value.get(key),
            _ => None,
        };
        match next {
            Some(next) => value = next,
            None => return Ok(None),
        }
    }
    Ok(Some(value))
}

pub fn number(r: &Robot, path: &str) -> Result<Option<f64>, Error> {
    Ok(value(r, path)?.and_then(Value::as_f64))
}

pub fn text(r: &Robot, path: &str) -> Result<Option<String>, Error> {
    Ok(value(r, path)?.and_then(Value::as_str).map(str::to_string))
}

pub fn has(r: &Robot, path: &str) -> Result<bool, Error> {
    Ok(value(r, path)?.is_some())
}

pub fn flag(r: &Robot, path: &str) -> Result<Option<bool>, Error> {
    Ok(value(r, path)?.and_then(Value::as_bool))
}

pub fn count(r: &Robot, path: &str) -> Result<Option<i32>, Error> {
    Ok(value(r, path)?.and_then(Value::as_array).map(|items| items.len() as i32))
}

/// The hello's line, or the latest state's, exactly as it arrived.
pub fn line(r: &Robot) -> Result<String, Error> {
    r.hello()?;
    Ok(r.line.clone().unwrap_or_default())
}

pub fn extra(r: &mut Robot, json: &str) -> Result<(), Error> {
    r.act.extra(json)
}
