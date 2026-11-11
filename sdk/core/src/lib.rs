//! The Clunkyard robot core (docs/design/robot-core.md): protocol 2 and the
//! robot subsystems behind one C ABI. It answers "what is true now", never
//! "what should I do". It has no clocks, no randomness, no threads and no
//! global state; one handle is one robot.
//!
//! generated/ holds the C exports, the records and the DSL words filler,
//! written from protocol/v2/api.json by `make api`. Everything else is the
//! hand-written core they call: the session and its port, the decoded
//! messages, the subsystems (one module each), the pending act, the
//! Tracker, lead and the DSL words.

#[allow(clippy::all)]
pub mod generated;

pub mod act;
pub mod angles;
pub mod arena;
pub mod battery;
pub mod clock;
pub mod drive;
pub mod dsl;
pub mod formula;
pub mod gun;
pub mod json;
pub mod lead;
pub mod r#match;
pub mod maths;
pub mod me;
pub mod message;
pub mod nav;
pub mod port;
pub mod raw;
pub mod runtime;
pub mod scanner;
pub mod score;
pub mod sensors;
pub mod session;
pub mod tracker;
#[cfg(target_family = "wasm")]
pub mod wasm;
