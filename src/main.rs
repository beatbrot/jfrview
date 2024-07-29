use crate::ui::app::JfrViewApp;
use eframe::AppCreator;
use egui::ViewportBuilder;
use std::{env, error::Error};

mod data;
mod exec;
mod flame_graph;
mod ui;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), Box<dyn Error>> {
    use eframe::NativeOptions;
    env::set_var("RUST_BACKTRACE", "1");
    let opts = NativeOptions {
        viewport: ViewportBuilder::default().with_drag_and_drop(true),
        ..Default::default()
    };
    eframe::run_native("JfrView", opts, create_app())?;
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::WebRunner;

    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    wasm_bindgen_futures::spawn_local(async {
        WebRunner::new()
            .start("canvas", Default::default(), create_app())
            .await
            .unwrap();
    });
}

fn create_app() -> AppCreator {
    Box::new(|_| Ok(Box::new(JfrViewApp::default())))
}
