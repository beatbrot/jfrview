use crate::speedscope::MethodSample;
use std::fmt::Debug;
use std::io::Cursor;
use wasm_bindgen::prelude::wasm_bindgen;

mod data;
mod speedscope;

/// Converts the content of a JFR file to a Vec of stacktraces.
///
/// The stacktraces are sorted by execution order.
///
/// You can specify `include_native` to also record JNI or other calls to native methods.
#[wasm_bindgen]
pub fn interpret_jfr(input: Vec<u8>, include_native: bool) -> Result<Vec<MethodSample>, String> {
    let cursor = Cursor::new(input);
    let export = speedscope::export(cursor, include_native).map_err(to_str)?;
    return Ok(export);
}

fn to_str(t: impl Debug) -> String {
    format!("{t:?}")
}
