//! An ECMAScript WASM implementation based on `boa_engine`.
#![cfg_attr(not(test), forbid(clippy::unwrap_used))]

use std::sync::{ Mutex, Once };
use boa_engine::{ native_function::NativeFunction, Context, JsString, Source };
use getrandom as _;
use wasm_bindgen::prelude::*;

static START: Once = Once::new();
static mut ARCMUT: Vec<Mutex<Context>> = Vec::new();
static mut MESSAGING: Vec<Mutex<js_sys::Function>> = Vec::new();

#[wasm_bindgen(start)]
fn main() {
    console_error_panic_hook::set_once();
    START.call_once(|| unsafe {
        ARCMUT = vec![Mutex::new(Context::default())];
        let _unused = ARCMUT[0].lock()
            .unwrap()
            .register_global_builtin_callable(
                JsString::from("message"),
                1,
                NativeFunction::from_fn_ptr(|_, args, context| {
                    let value = args[0].to_string(context)?;
                    let messaging = &MESSAGING[0];
                    let this = JsValue::null();
                    let msg = JsValue::from_str(&value.to_std_string_escaped());
                    let _unused = messaging.lock().unwrap().call1(&this, &msg);
                    Ok(value.into())
                })
            );
    });
}

/// Register the host messaging.
///
/// # Errors
///
/// If the execution of the registering throws, returns a `JsValue` with the error string.
#[wasm_bindgen]
pub fn register_messaging(f: &js_sys::Function) {
    unsafe {
        MESSAGING = vec![Mutex::new(f.clone())];
    }
}

/// Evaluate the given ECMAScript code.
///
/// # Errors
///
/// If the execution of the script throws, returns a `JsValue` with the error string.
#[wasm_bindgen]
pub fn evaluate(src: &str) -> Result<String, JsValue> {
    // Setup the executor
    let arc_clone = unsafe { &ARCMUT[0] };
    let x = arc_clone
        .lock()
        .unwrap()
        .eval(Source::from_bytes(src))
        .map_err(|e| JsValue::from(format!("Uncaught {e}")))
        .map(|v| v.display().to_string());
    x
}

/// Evaluate the given ECMAScript code.
///
/// # Errors
///
/// If the execution of the script throws, returns a `JsValue` with the error string.
#[wasm_bindgen]
pub fn trigger(callback_id: &str) -> Result<String, JsValue> {
    let arc_clone = unsafe { &ARCMUT[0] };
    let x = arc_clone
        .lock()
        .unwrap()
        .eval(Source::from_bytes(&("trigger(".to_owned() + callback_id + ")")))
        .map_err(|e| JsValue::from(format!("Uncaught {e}")))
        .map(|v| v.display().to_string());
    x
}
