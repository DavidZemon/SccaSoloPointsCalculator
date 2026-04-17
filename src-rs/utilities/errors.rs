//! Error-handling helpers for bridging internal `anyhow::Error` values to the
//! JS/WASM boundary.
//!
//! Internally the Rust code uses [`anyhow::Result`] with rich `.context(...)`
//! chains so that when something fails we have a clear story of _where_ and
//! _why_. At the WASM boundary we flatten the chain into a `String` (so the
//! React layer can surface it in a `react-toastify` toast), but we **also**
//! log the full debug-formatted chain (`{e:?}`) to the browser console so
//! that developers get the full "stack trace" in DevTools.

use anyhow::Error;

/// Log the full anyhow error chain to the browser console and return a
/// human-friendly single-line representation suitable for a user-facing
/// toast.
///
/// - `console.error` receives the multi-line `{e:?}` debug representation,
///   which includes every `.context(...)` frame and every `#[source]` cause
///   in the chain.
/// - The returned `String` uses `{e:#}` which renders as `top: next: root`
///   on a single line — compact enough for a toast but still showing the
///   full causal chain.
pub fn log_and_stringify(e: &Error) -> String {
    let debug = format!("{e:?}");
    // In native `cargo test` runs there is no `web-sys` console; fall back to
    // `eprintln!` so test failures still surface the chain.
    #[cfg(target_arch = "wasm32")]
    web_sys::console::error_1(&debug.into());
    #[cfg(not(target_arch = "wasm32"))]
    eprintln!("{debug}");
    format!("{e:#}")
}

/// Convenience wrapper used at the `#[wasm_bindgen]` boundary to satisfy
/// `Result<_, String>` return types while still logging the full chain.
pub fn to_js_error_string(e: Error) -> String {
    log_and_stringify(&e)
}

/// Install the `console_error_panic_hook` so that any `panic!` inside the
/// WASM module surfaces in the browser DevTools with a JS-visible message
/// and stack. Safe to call multiple times.
pub fn install_panic_hook() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
}

/// Bind `console.error` alongside the existing `console.log` binding in
/// [`crate::utilities`] so callers can emit error-level diagnostics without
/// going through `anyhow`.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    #[wasm_bindgen::prelude::wasm_bindgen(js_namespace = console, js_name = error)]
    pub fn console_error(s: &str);
}

/// Mirror of [`crate::console_log!`] that routes to `console.error`.
#[macro_export]
macro_rules! console_error {
    ($($t:tt)*) => {{
        #[cfg(target_arch = "wasm32")]
        $crate::utilities::errors::console_error(&format_args!($($t)*).to_string());
        #[cfg(not(target_arch = "wasm32"))]
        eprintln!("{}", format_args!($($t)*));
    }}
}
