use std::fmt::Debug;
use std::io::Cursor;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

mod data;
mod speedscope;

#[wasm_bindgen]
pub fn interpret_jfr(input: Vec<u8>, include_native: bool) -> Result<JsValue, String> {
    let cursor = Cursor::new(input);
    let export = speedscope::export(cursor, include_native).map_err(to_str)?;
    serde_wasm_bindgen::to_value(&export).map_err(to_str)
}

fn to_str(t: impl Debug) -> String {
    format!("{t:?}")
}
