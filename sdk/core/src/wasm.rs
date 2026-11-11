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
