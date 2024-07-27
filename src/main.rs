use std::{env, error::Error, fs::File};
use eframe::WebRunner;
use flame_graph::FlameGraph;

use crate::ui::app::JfrViewApp;

mod data;
mod flame_graph;
mod ui;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_BACKTRACE", "1");
    let fg = create_flame_graph("C:\\Users\\loych\\Development\\jfrview\\cfg6_validate_small.jfr");

    eframe::run_native("JfrView", Default::default(), Box::new(|_| Ok(Box::new(JfrViewApp::new(fg)))))?;
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();
    let fg = FlameGraph::default();

    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
       let start_result = WebRunner::new()
           .start("canvas", web_options, Box::new(|_| Ok(Box::new(JfrViewApp::new(fg)))))
           .await.unwrap();
    });
}

fn create_flame_graph(path: &str) -> FlameGraph {
    let file = File::open(path).unwrap();
    FlameGraph::from(file)
}
