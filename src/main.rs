use crate::flame_graph::FlameGraph;
use crate::ui::app::JfrViewApp;
use eframe::AppCreator;
#[cfg(not(target_arch = "wasm32"))]
use std::error::Error;
use std::fs::File;

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
        None => FlameGraph::default(),
    };
    Box::new(|cc| Ok(Box::new(JfrViewApp::new(&cc.egui_ctx, flame_graph))))
}

/*
fn start_puffin_server() {
    puffin::set_scopes_on(true); // tell puffin to collect data

    match puffin_http::Server::new("127.0.0.1:8585") {
        Ok(puffin_server) => {
            eprintln!("Run:  cargo install puffin_viewer && puffin_viewer --url 127.0.0.1:8585");

            std::process::Command::new("puffin_viewer")
                .arg("--url")
                .arg("127.0.0.1:8585")
                .spawn()
                .ok();

            // We can store the server if we want, but in this case we just want
            // it to keep running. Dropping it closes the server, so let's not drop it!
            #[allow(clippy::mem_forget)]
            std::mem::forget(puffin_server);
        }
        Err(err) => {
            eprintln!("Failed to start puffin server: {err}");
        }
    };
}
*/