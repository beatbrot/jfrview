use crate::export::Sample;
use data::Stats;
use std::fmt::Debug;
use std::io::Cursor;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

mod data;
mod export;
#[cfg(test)]
mod type_explorer;

#[wasm_bindgen]
pub fn parse(input: Vec<u8>, include_native: bool, threads: bool) -> Result<JsValue, String> {
    let cursor = Cursor::new(input);
    let export = Sample::from_file(cursor, include_native, threads).map_err(to_str)?;
    serde_wasm_bindgen::to_value(&export).map_err(to_str)
}

#[wasm_bindgen]
pub fn jfr_stats(input: Vec<u8>) -> Result<JsValue, String> {
    let cursor = Cursor::new(input);
    let stats = Stats::from(cursor, "").map_err(to_str)?;
    serde_wasm_bindgen::to_value(&stats).map_err(to_str)
}

fn to_str(t: impl Debug) -> String {
    format!("{t:?}")
}
