//! Exports only the wasm32 build has, for the JavaScript loader
//! (src/dsl/core.js), which owns the instance's memory.

#![allow(clippy::not_unsafe_ptr_arg_deref)]

use crate::generated::*;
use crate::runtime::Robot;

/// Memory for the loader's arguments and results, 8-byte aligned.
#[unsafe(no_mangle)]
pub extern "C" fn cy_alloc(size: u32) -> *mut u8 {
    let layout = std::alloc::Layout::from_size_align(size.max(1) as usize, 8);
    // SAFETY: a non-zero size and a valid alignment.
    layout.map_or(std::ptr::null_mut(), |layout| unsafe { std::alloc::alloc(layout) })
}

/// Frees what cy_alloc gave, with the same size.
#[unsafe(no_mangle)]
pub extern "C" fn cy_free(pointer: *mut u8, size: u32) {
    if let (false, Ok(layout)) = (pointer.is_null(), std::alloc::Layout::from_size_align(size.max(1) as usize, 8)) {
        // SAFETY: pointer came from cy_alloc with this layout.
        unsafe { std::alloc::dealloc(pointer, layout) };
    }
}

/// Carries a trapped instance's faults into a fresh one, which has just been
/// fed the kept hello: the panic count, and the tick that faulted (negative
/// before the first state), which send then answers with an empty act. A
/// wasm32 panic traps rather than unwinding, so the loader stands in for
/// the native catch_unwind.
#[unsafe(no_mangle)]
pub extern "C" fn cy_wasm_recover(robot: *mut Robot, panics: u32, tick: f64) -> i32 {
    // SAFETY: the loader passes the handle cy_open_memory gave it.
    let Some(r) = (unsafe { robot.as_mut() }) else { return CY_E_ARGUMENT };
    r.recover(panics, tick);
    CY_OK
}

/// A DSL interpreter handle (src/dsl/interpreter.js): the robot's program
/// and cells, and the JSON of the last load or decision.
pub struct Interpreter {
    machine: Option<crate::ir::Machine>,
    result: String,
}

/// Decodes and checks an IR module. Always gives a handle; its result says
/// whether the module loaded.
#[unsafe(no_mangle)]
pub extern "C" fn cy_ir_load(bytes: *const u8, length: u32) -> *mut Interpreter {
    // SAFETY: the loader passes memory it wrote, of that length.
    let bytes = if bytes.is_null() { &[][..] } else { unsafe { std::slice::from_raw_parts(bytes, length as usize) } };
    let (machine, result) = match crate::ir::Module::decode(bytes) {
        Ok(module) => (Some(crate::ir::Machine::new(module)), serde_json::json!({ "ok": true }).to_string()),
        Err(message) => (None, serde_json::json!({ "error": message }).to_string()),
    };
    Box::into_raw(Box::new(Interpreter { machine, result }))
}

/// A copy with the same program and cells, for a strategist's dry run.
#[unsafe(no_mangle)]
pub extern "C" fn cy_ir_clone(handle: *const Interpreter) -> *mut Interpreter {
    // SAFETY: a handle cy_ir_load gave.
    let Some(h) = (unsafe { handle.as_ref() }) else { return std::ptr::null_mut() };
    Box::into_raw(Box::new(Interpreter { machine: h.machine.clone(), result: String::new() }))
}

#[unsafe(no_mangle)]
pub extern "C" fn cy_ir_free(handle: *mut Interpreter) {
    if !handle.is_null() {
        // SAFETY: a handle cy_ir_load or cy_ir_clone gave, freed once.
        drop(unsafe { Box::from_raw(handle) });
    }
}

/// Runs one decision on the words record (118 f64), the contact rows (10
/// f64 each; a negative count when the program reads none) and one
/// parameter value per slot. 0 when it decided, 1 when it failed; the
/// result says what it wrote or where it failed.
#[unsafe(no_mangle)]
pub extern "C" fn cy_ir_decide(handle: *mut Interpreter, words: *const f64, contacts: *const f64, contact_count: i32, params: *const f64, param_count: u32, budget: u32) -> i32 {
    // SAFETY: a handle cy_ir_load gave.
    let Some(h) = (unsafe { handle.as_mut() }) else { return CY_E_ARGUMENT };
    let Some(machine) = h.machine.as_mut() else { return CY_E_ARGUMENT };
    if words.is_null() {
        return CY_E_ARGUMENT;
    }
    // SAFETY: arrays the loader wrote, of these lengths.
    let words = unsafe { std::slice::from_raw_parts(words, crate::ir::WORDS) };
    let contacts = match contact_count {
        n if n < 0 => None,
        0 => Some(&[][..]),
        _ if contacts.is_null() => return CY_E_ARGUMENT,
        n => Some(unsafe { std::slice::from_raw_parts(contacts, n as usize * crate::ir::CONTACT_FIELDS) }),
    };
    let params = if params.is_null() { &[][..] } else { unsafe { std::slice::from_raw_parts(params, param_count as usize) } };
    let turn = crate::ir::Turn { words, contacts, params, budget };
    match machine.decide(&turn) {
        Ok(out) => {
            let mut fields = serde_json::Map::new();
            for (field, value) in crate::ir::FIELDS.iter().zip(out.numbers) {
                if let Some(value) = value {
                    fields.insert((*field).into(), serde_json::json!(value));
                }
            }
            if let Some(fire) = out.fire {
                fields.insert("fire".into(), fire.into());
            }
            if let Some(launch) = out.launch {
                fields.insert("launch".into(), launch.into());
            }
            if let Some(text) = out.text {
                fields.insert("botStatusText".into(), text.into());
            }
            h.result = serde_json::json!({ "ops": out.ops, "set": fields }).to_string();
            0
        }
        Err((fault, ops)) => {
            let (line, column) = machine.position(fault.pos);
            h.result = serde_json::json!({ "ops": ops, "fault": { "line": line, "column": column, "message": fault.message } }).to_string();
            1
        }
    }
}

/// Copies the last result's JSON out: CY_OK, or CY_SHORT with the length
/// it needs.
#[unsafe(no_mangle)]
pub extern "C" fn cy_ir_result(handle: *const Interpreter, out: *mut u8, capacity: u32, length: *mut u32) -> i32 {
    // SAFETY: a handle cy_ir_load gave, and a buffer and length the loader owns.
    let Some(h) = (unsafe { handle.as_ref() }) else { return CY_E_ARGUMENT };
    let bytes = h.result.as_bytes();
    if !length.is_null() {
        unsafe { *length = bytes.len() as u32 };
    }
    if bytes.len() > capacity as usize || out.is_null() {
        return CY_SHORT;
    }
    unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), out, bytes.len()) };
    CY_OK
}
