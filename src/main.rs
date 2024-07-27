use std::{env, error::Error, fs::File};

use flame_graph::FlameGraph;

use crate::ui::app::JfrViewApp;

mod data;
mod flame_graph;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_BACKTRACE", "1");
    let fg = create_flame_graph("C:\\Users\\loych\\Development\\jfrview\\cfg6_validate_small.jfr");

    eframe::run_native("JfrView", Default::default(), Box::new(|_| Ok(Box::new(JfrViewApp::new(fg)))))?;
    Ok(())
}

fn create_flame_graph(path: &str) -> FlameGraph {
    let file = File::open(path).unwrap();
    FlameGraph::from(file)
}
