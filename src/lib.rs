#![allow(clippy::inherent_to_string)]

use std::fmt::Debug;
use std::io::Cursor;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::speedscope::MethodSample;

mod data;
mod speedscope;

/// Converts the content of a JFR file to a Vec of stacktraces.
///
/// The stacktraces are sorted by execution order.
#[wasm_bindgen]
pub fn interpret_jfr(input: Vec<u8>) -> Result<JsValue, String> {
    let export = interpret_jfr_internal(input);
    serde_wasm_bindgen::to_value(&export).map_err(to_str)
}

pub fn interpret_jfr_internal(input: Vec<u8>) -> Result<Vec<MethodSample>, String> {
    let cursor = Cursor::new(input);
    speedscope::export(cursor).map_err(to_str)
}

fn to_str(t: impl Debug) -> String {
    format!("{t:?}")
}

#[cfg(test)]
mod tests {
    wasm_bindgen_test_configure!(run_in_browser);

    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    use crate::interpret_jfr;

    const HEAVY: &'static [u8] = include_bytes!("../test-data/heavy.jfr");

    #[wasm_bindgen_test]
    fn interpret_heavy() {
        interpret_jfr(HEAVY.into()).unwrap();
        interpret_jfr(HEAVY.into()).unwrap();
    }
}
