use crate::ui::app::JfrViewApp;
use eframe::AppCreator;
use std::fs::File;
#[cfg(not(target_arch = "wasm32"))]
use std::error::Error;
use crate::flame_graph::FlameGraph;

mod data;
mod exec;
mod flame_graph;
mod ui;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), Box<dyn Error>> {
    use std::env;
    env::set_var("RUST_BACKTRACE", "1");
    let args: Vec<String> = env::args().collect::<Vec<_>>();
    let arg = args.get(1);
    let file = parse_jfr_arg(arg)?;
    eframe::run_native("JfrView", Default::default(), create_app(file))?;
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_jfr_arg(input: Option<&String>) -> std::io::Result<Option<File>> {
    if let Some(path_str) = input {
        let file = File::open(path_str);
        file.map(|f| Some(f))
    } else {
        Ok(None)
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::WebRunner;

    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    wasm_bindgen_futures::spawn_local(async {
        WebRunner::new()
            .start("canvas", Default::default(), create_app(None))
            .await
            .unwrap();
    });
}

fn create_app(jfr_file: Option<File>) -> AppCreator {
    let flame_graph: FlameGraph = match jfr_file {
        Some(v) => FlameGraph::try_new(v).unwrap(),
        None => FlameGraph::default()
    };
    Box::new(|cc| Ok(Box::new(JfrViewApp::new(cc, flame_graph))))
}
