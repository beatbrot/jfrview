use std::fmt::Debug;
use std::io::Cursor;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

pub mod data;
pub mod export;

pub use crate::export::Sample;

#[wasm_bindgen]
pub fn parse(input: Vec<u8>, include_native: bool, threads: bool) -> Result<JsValue, String> {
    let cursor = Cursor::new(input);
    let export = Sample::from_file(cursor, include_native, threads).map_err(to_str)?;
    serde_wasm_bindgen::to_value(&export).map_err(to_str)
}

fn to_str(t: impl Debug) -> String {
    format!("{t:?}")
}
