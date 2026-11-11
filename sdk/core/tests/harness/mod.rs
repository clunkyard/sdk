//! Calls the core through its C exports, by api.json, as a binding does:
//! arguments are encoded and results decoded by the records' layouts in
//! protocol/v2/api.json, the same way test/core-wasm.test.js does through
//! the wasm32 build's memory.

#![allow(dead_code)]

use clunkyard::generated::ffi::*;
use clunkyard::generated::*;
use clunkyard::json::{self, Value};
use clunkyard::runtime::Robot;
use std::path::PathBuf;

mod dispatch;
pub use dispatch::dispatch;

pub fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

pub fn read(path: &str) -> String {
    let at = root().join(path);
    std::fs::read_to_string(&at).unwrap_or_else(|error| panic!("{}: {error}", at.display()))
}

pub fn load(path: &str) -> Value {
    json::parse(&read(path)).unwrap_or_else(|error| panic!("{path}: {error}"))
}

pub fn text(value: &Value, key: &str) -> String {
    value.get(key).and_then(Value::as_str).unwrap_or_else(|| panic!("no text {key}")).to_string()
}

pub fn list<'a>(value: &'a Value, key: &str) -> &'a [Value] {
    value.get(key).and_then(Value::as_array).unwrap_or(&[])
}

/// A number as the vectors write one: {"f64": "NaN"} for a non-finite one.
pub fn number(value: &Value) -> Option<f64> {
    match value {
        Value::Number(x) => Some(*x),
        Value::Object(_) => match value.get("f64").and_then(Value::as_str)? {
            "NaN" => Some(f64::NAN),
            "Infinity" => Some(f64::INFINITY),
            "-Infinity" => Some(f64::NEG_INFINITY),
            "-0" => Some(-0.0),
            _ => None,
        },
        _ => None,
    }
}

/// protocol/v2/api.json.
pub struct Api {
    pub json: Value,
}

impl Api {
    pub fn load() -> Api {
        Api { json: load("protocol/v2/api.json") }
    }

    fn named<'a>(&'a self, list: &str, name: &str) -> &'a Value {
        self::list(&self.json, list).iter().find(|item| item.get("name").and_then(Value::as_str) == Some(name)).unwrap_or_else(|| panic!("no {list} {name}"))
    }

    pub fn entry(&self, name: &str) -> &Value {
        self.named("entries", name)
    }

    /// An entry's export, or a control field's setter or getter.
    pub fn has_symbol(&self, symbol: &str) -> bool {
        let is = |value: Option<&Value>| value.and_then(Value::as_str) == Some(symbol);
        list(&self.json, "entries").iter().any(|entry| is(entry.get("symbol")) || list(entry, "fields").iter().any(|field| is(field.get("symbol")) || is(field.get("getter"))))
    }

    pub fn record(&self, name: &str) -> &Value {
        self.named("records", name)
    }

    pub fn status(&self, name: &str) -> i32 {
        self.named("statuses", name).get("value").and_then(Value::as_f64).unwrap() as i32
    }

    pub fn status_name(&self, status: i32) -> String {
        self::list(&self.json, "statuses").iter().find(|item| item.get("value").and_then(Value::as_f64) == Some(f64::from(status))).map_or(format!("{status}"), |item| text(item, "name"))
    }

    /// An enum's value for a wire name; an unknown name is 99, a value the
    /// core refuses.
    pub fn enum_value(&self, name: &str, wire: &Value) -> i32 {
        let Some(wire) = wire.as_str() else { return 0 };
        self::list(self.named("enums", name), "values").iter().find(|item| item.get("wire").and_then(Value::as_str) == Some(wire)).map_or(99, |item| item.get("value").and_then(Value::as_f64).unwrap() as i32)
    }

    pub fn enum_wire(&self, name: &str, value: i32) -> Value {
        self::list(self.named("enums", name), "values")
            .iter()
            .find(|item| item.get("value").and_then(Value::as_f64) == Some(f64::from(value)))
            .and_then(|item| item.get("wire").cloned())
            .unwrap_or(Value::Null)
    }

    pub fn size(&self, record: &str) -> usize {
        self.record(record).get("size").and_then(Value::as_f64).unwrap() as usize
    }

    /// Encodes a record argument; Err when the C ABI cannot carry the value.
    pub fn encode(&self, record: &str, value: &Value, bytes: &mut [u8], base: usize) -> Result<(), String> {
        for field in list(self.record(record), "fields") {
            let name = text(field, "name");
            let kind = field.at("type.kind").and_then(Value::as_str).unwrap();
            let at = base + field.get("offset").and_then(Value::as_f64).unwrap() as usize;
            let item = value.get(&name).unwrap_or(&Value::Null);
            match kind {
                "f64" => bytes[at..at + 8].copy_from_slice(&number(item).unwrap_or(0.0).to_le_bytes()),
                "i32" => {
                    let n = match item {
                        Value::Null => 0.0,
                        _ => number(item).ok_or_else(|| format!("{record}.{name} is not a number"))?,
                    };
                    if n.fract() != 0.0 || !(f64::from(i32::MIN)..=f64::from(i32::MAX)).contains(&n) {
                        return Err(format!("{record}.{name} is not an i32"));
                    }
                    bytes[at..at + 4].copy_from_slice(&(n as i32).to_le_bytes());
                }
                "bool" => bytes[at..at + 4].copy_from_slice(&i32::from(item.as_bool() == Some(true)).to_le_bytes()),
                "enum" => {
                    let value = self.enum_value(field.at("type.enum").and_then(Value::as_str).unwrap(), item);
                    bytes[at..at + 4].copy_from_slice(&value.to_le_bytes());
                }
                "record" => self.encode(field.at("type.record").and_then(Value::as_str).unwrap(), item, bytes, at)?,
                other => panic!("field kind {other}"),
            }
        }
        Ok(())
    }

    /// Decodes a record as the vectors write it: enums by name, booleans as
    /// booleans, and no reserved fields.
    pub fn decode(&self, record: &str, bytes: &[u8], base: usize) -> Value {
        let mut members = Vec::new();
        for field in list(self.record(record), "fields") {
            let name = text(field, "name");
            if name == "reserved" {
                continue;
            }
            let at = base + field.get("offset").and_then(Value::as_f64).unwrap() as usize;
            let int = || i32::from_le_bytes(bytes[at..at + 4].try_into().unwrap());
            let value = match field.at("type.kind").and_then(Value::as_str).unwrap() {
                "f64" => Value::Number(f64::from_le_bytes(bytes[at..at + 8].try_into().unwrap())),
                "i32" => Value::Number(f64::from(int())),
                "bool" => Value::Bool(int() != 0),
                "enum" => self.enum_wire(field.at("type.enum").and_then(Value::as_str).unwrap(), int()),
                "record" => self.decode(field.at("type.record").and_then(Value::as_str).unwrap(), bytes, at),
                other => panic!("field kind {other}"),
            };
            members.push((name, value));
        }
        Value::Object(members)
    }

    /// A scalar or record result of a return type.
    pub fn result(&self, kind: &Value, bytes: &[u8], count: usize) -> Value {
        let int = || i32::from_le_bytes(bytes[0..4].try_into().unwrap());
        if kind.get("list").and_then(Value::as_bool) == Some(true) {
            let record = text(kind, "record");
            let size = self.size(&record);
            return Value::Array((0..count).map(|i| self.decode(&record, bytes, i * size)).collect());
        }
        match kind.get("kind").and_then(Value::as_str).unwrap() {
            "f64" => Value::Number(f64::from_le_bytes(bytes[0..8].try_into().unwrap())),
            "i32" => Value::Number(f64::from(int())),
            "bool" => Value::Bool(int() != 0),
            "enum" => self.enum_wire(&text(kind, "enum"), int()),
            "record" => self.decode(&text(kind, "record"), bytes, 0),
            "text" => Value::String(String::from_utf8(bytes[..count].to_vec()).unwrap()),
            other => panic!("return kind {other}"),
        }
    }
}

/// Arguments, encoded by position.
#[derive(Default)]
pub struct Args {
    numbers: Vec<f64>,
    ints: Vec<i32>,
    bytes: Vec<Vec<u64>>,
    lengths: Vec<u32>,
}

impl Args {
    pub fn f(&self, i: usize) -> f64 {
        self.numbers[i]
    }
    pub fn i(&self, i: usize) -> i32 {
        self.ints[i]
    }
    pub fn p<T>(&self, i: usize) -> *const T {
        self.bytes[i].as_ptr() as *const T
    }
    pub fn n(&self, i: usize) -> u32 {
        self.lengths[i]
    }

    fn push(&mut self, number: f64, int: i32, bytes: Vec<u64>, length: u32) {
        self.numbers.push(number);
        self.ints.push(int);
        self.bytes.push(bytes);
        self.lengths.push(length);
    }

    pub fn number(&mut self, value: f64) {
        self.push(value, 0, vec![0], 0);
    }

    pub fn int(&mut self, value: i32) {
        self.push(0.0, value, vec![0], 0);
    }

    pub fn text(&mut self, value: &str) {
        let mut words = vec![0u64; value.len() / 8 + 1];
        // SAFETY: words holds at least value.len() bytes.
        unsafe { std::ptr::copy_nonoverlapping(value.as_ptr(), words.as_mut_ptr() as *mut u8, value.len()) };
        self.push(0.0, 0, words, value.len() as u32);
    }

    pub fn record(&mut self, bytes: &[u8]) {
        let mut words = vec![0u64; bytes.len() / 8 + 1];
        // SAFETY: words holds at least bytes.len() bytes.
        unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), words.as_mut_ptr() as *mut u8, bytes.len()) };
        self.push(0.0, 0, words, 0);
    }
}

/// The out-parameters: an aligned buffer, its capacity in items, and the
/// count or length the core writes.
pub struct Out {
    pub words: Vec<u64>,
    pub capacity: u32,
    pub count: u32,
}

impl Out {
    pub fn new(bytes: usize, capacity: u32) -> Out {
        Out { words: vec![0; bytes / 8 + 1], capacity, count: 0 }
    }
    pub fn ptr<T>(&mut self) -> *mut T {
        self.words.as_mut_ptr() as *mut T
    }
    pub fn cap(&self) -> u32 {
        self.capacity
    }
    pub fn len(&mut self) -> *mut u32 {
        &mut self.count
    }
    pub fn bytes(&self) -> &[u8] {
        // SAFETY: the words' own bytes.
        unsafe { std::slice::from_raw_parts(self.words.as_ptr() as *const u8, self.words.len() * 8) }
    }
}

/// A robot on a memory port.
pub struct Core {
    pub robot: *mut Robot,
}

impl Core {
    pub fn open(binding: &str, version: &str) -> Core {
        let mut robot = std::ptr::null_mut();
        let status = cy_open_memory(API_MAJOR, API_MINOR, binding.as_ptr(), binding.len() as u32, version.as_ptr(), version.len() as u32, &mut robot);
        assert_eq!(status, CY_OK, "cy_open_memory");
        Core { robot }
    }

    pub fn feed(&self, line: &str) {
        assert_eq!(cy_session_feed(self.robot, line.as_ptr(), line.len() as u32), CY_OK, "feed");
    }

    pub fn next(&self) -> i32 {
        let mut event = 0;
        let status = cy_session_next(self.robot, &mut event);
        if status != CY_OK {
            return status;
        }
        event
    }

    pub fn send(&self) -> i32 {
        cy_session_send(self.robot)
    }

    pub fn reply(&self) -> Option<String> {
        let mut buf = vec![0u8; 16];
        let mut length = 0;
        let mut status = cy_session_reply(self.robot, buf.as_mut_ptr(), buf.len() as u32, &mut length);
        if status == CY_SHORT {
            buf = vec![0u8; length as usize];
            status = cy_session_reply(self.robot, buf.as_mut_ptr(), buf.len() as u32, &mut length);
        }
        match status {
            CY_OK => Some(String::from_utf8(buf[..length as usize].to_vec()).unwrap()),
            CY_NONE => None,
            other => panic!("reply gave {other}"),
        }
    }

    pub fn last_error(&self) -> String {
        let mut buf = vec![0u8; 4096];
        let mut length = 0;
        cy_last_error(self.robot, buf.as_mut_ptr(), buf.len() as u32, &mut length);
        String::from_utf8_lossy(&buf[..length as usize]).into_owned()
    }
}

impl Drop for Core {
    fn drop(&mut self) {
        cy_close(self.robot);
    }
}

/// What a call gave: its status and, on CY_OK, its result.
pub enum Outcome {
    Done(i32, Option<Value>),
    /// The C ABI cannot carry an argument.
    Unrepresentable(String),
}

/// A call by its api.json entry, with arguments as the vectors write them.
pub fn call(api: &Api, robot: *mut Robot, name: &str, args: &[Value]) -> Outcome {
    let entry = api.entry(name);
    let symbol = text(entry, "symbol");
    let mut encoded = Args::default();
    for (i, arg) in list(entry, "args").iter().enumerate() {
        let kind = arg.at("type.kind").and_then(Value::as_str).unwrap();
        let value = match args.get(i) {
            Some(value) => value.clone(),
            // reference.js's defaults: api.json's, and angles.direction's pitch of 0.
            None => arg.get("default").cloned().unwrap_or(Value::Number(0.0)),
        };
        match kind {
            "f64" => match number(&value) {
                Some(x) => encoded.number(x),
                None => return Outcome::Unrepresentable(format!("{name}: {} is not a number", value.to_json())),
            },
            "i32" => match number(&value) {
                Some(x) if x.fract() == 0.0 && (f64::from(i32::MIN)..=f64::from(i32::MAX)).contains(&x) => encoded.int(x as i32),
                _ => return Outcome::Unrepresentable(format!("{name}: {} is not an i32", value.to_json())),
            },
            "enum" => encoded.int(api.enum_value(arg.at("type.enum").and_then(Value::as_str).unwrap(), &value)),
            "text" => match value.as_str() {
                Some(text) => encoded.text(text),
                None => return Outcome::Unrepresentable(format!("{name}: {} is not text", value.to_json())),
            },
            "record" => {
                let record = arg.at("type.record").and_then(Value::as_str).unwrap();
                let mut bytes = vec![0u8; api.size(record)];
                if let Err(why) = api.encode(record, &value, &mut bytes, 0) {
                    return Outcome::Unrepresentable(format!("{name}: {why}"));
                }
                encoded.record(&bytes);
            }
            other => panic!("argument kind {other}"),
        }
    }
    let returns = entry.get("returns").filter(|r| !matches!(r, Value::Null));
    let (bytes, capacity) = match returns {
        Some(kind) if kind.get("list").and_then(Value::as_bool) == Some(true) => (api.size(&text(kind, "record")) * 64, 64),
        Some(kind) if kind.get("kind").and_then(Value::as_str) == Some("text") => (1 << 16, 1 << 16),
        Some(kind) if kind.get("kind").and_then(Value::as_str) == Some("record") => (api.size(&text(kind, "record")), 0),
        _ => (8, 0),
    };
    let mut out = Out::new(bytes, capacity);
    let status = dispatch(&symbol, robot, &encoded, &mut out).unwrap_or_else(|| panic!("no export {symbol}"));
    assert_ne!(status, CY_SHORT, "{name}: the test's buffer is too short");
    let value = match (status, returns) {
        (CY_OK, Some(kind)) => Some(api.result(kind, out.bytes(), out.count as usize)),
        _ => None,
    };
    Outcome::Done(status, value)
}

/// A value's exact bits, for the transcript native and wasm32 must share:
/// numbers as f64 bits (NaN canonical), text as UTF-8 hex, records by their
/// fields in order.
pub fn canon(value: &Value) -> String {
    match value {
        Value::Null => "n".into(),
        Value::Bool(true) => "t".into(),
        Value::Bool(false) => "f".into(),
        Value::Number(x) => format!("{:016x}", if x.is_nan() { 0x7ff8_0000_0000_0000 } else { x.to_bits() }),
        Value::String(text) => format!("s{}", text.bytes().map(|b| format!("{b:02x}")).collect::<String>()),
        Value::Array(items) => format!("[{}]", items.iter().map(canon).collect::<Vec<_>>().join(",")),
        Value::Object(members) => format!("{{{}}}", members.iter().map(|(key, value)| format!("{key}={}", canon(value))).collect::<Vec<_>>().join(",")),
    }
}

pub fn outcome_text(outcome: &Outcome) -> String {
    match outcome {
        Outcome::Unrepresentable(_) => "skip".into(),
        Outcome::Done(status, None) => format!("{status}"),
        Outcome::Done(status, Some(value)) => format!("{status} {}", canon(value)),
    }
}

/// |actual − expected| ≤ tolerance × max(1, |expected|), field by field;
/// everything that is not a number exactly.
pub fn close(actual: &Value, expected: &Value, tolerance: f64) -> bool {
    match (actual, expected) {
        (Value::Number(a), Value::Number(e)) => (a - e).abs() <= tolerance * if e.abs() > 1.0 { e.abs() } else { 1.0 },
        (Value::Array(a), Value::Array(e)) => a.len() == e.len() && a.iter().zip(e).all(|(a, e)| close(a, e, tolerance)),
        (Value::Object(a), Value::Object(e)) => {
            let mut keys: Vec<&String> = a.iter().map(|(key, _)| key).collect();
            let mut expected_keys: Vec<&String> = e.iter().map(|(key, _)| key).collect();
            keys.sort();
            expected_keys.sort();
            keys == expected_keys && e.iter().all(|(key, e)| close(actual.get(key).unwrap(), e, tolerance))
        }
        (a, e) => a == e,
    }
}

/// FNV-1a, 64 bits, over the transcript.
pub fn digest(lines: &[String]) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for line in lines {
        for byte in line.bytes().chain(std::iter::once(b'\n')) {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0100_0000_01b3);
        }
    }
    format!("{hash:016x}")
}
