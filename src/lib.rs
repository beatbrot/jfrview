use crate::export::Sample;
use std::fmt::Debug;
use std::io::Cursor;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

mod data;
mod export;

#[wasm_bindgen]
pub fn parse(input: Vec<u8>, include_native: bool) -> Result<JsValue, String> {
    let cursor = Cursor::new(input);
    let export = Sample::from_file(cursor, include_native).map_err(to_str)?;
    serde_wasm_bindgen::to_value(&export).map_err(to_str)
}

fn to_str(t: impl Debug) -> String {
    format!("{t:?}")
}
