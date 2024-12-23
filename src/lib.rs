use crate::export::Sample;
use crate::flame_graph::FlameGraph;
use std::io::Cursor;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

mod data;
mod export;
mod flame_graph;

#[wasm_bindgen]
pub fn parse(input: Vec<u8>, include_native: bool) -> Result<JsValue, String> {
    let fg = FlameGraph::try_new(Cursor::new(input)).map_err(|e| format!("{e:?}"))?;
    let export = Sample::from(fg, include_native);
    serde_wasm_bindgen::to_value(&export).map_err(|e| format!("{e:?}"))
}
