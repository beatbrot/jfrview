use flame_graph::FlameGraph;
use std::{env, error::Error, fs::File};

use crate::ui::app::JfrViewApp;

mod data;
mod flame_graph;
mod ui;
mod exec;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_BACKTRACE", "1");
    eframe::run_native(
        "JfrView",
        Default::default(),
        Box::new(|_| Ok(Box::new(JfrViewApp::new(Default::default())))),
    )?;
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::WebRunner;

    eframe::WebLogger::init(log::LevelFilter::Debug).ok();
    let fg = FlameGraph::default();

    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        let start_result = WebRunner::new()
            .start(
                "canvas",
                web_options,
                Box::new(|_| Ok(Box::new(JfrViewApp::new(fg)))),
            )
            .await
            .unwrap();
    });
}
