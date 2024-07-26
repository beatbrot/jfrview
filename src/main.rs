mod data;
mod flame_graph;
mod ui;

use std::{env, error::Error, fs::File};

use egui::{pos2, Color32, Id};
use flame_graph::{FlameGraph, Frame};
use once_cell::sync::Lazy;
use ui::{
    block::HEIGHT,
    theme::{self, HOVER},
};

use crate::ui::block::block;

static ID: Lazy<Id> = Lazy::new(|| Id::new("hovered_method"));

fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_BACKTRACE", "1");
    let fg = create_flame_graph("C:\\Users\\loych\\Development\\jfrview\\cfg6_validate_small.jfr");

    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_simple_native("JfrView", options, move |ctx, _| {
        egui::CentralPanel::default().show(&ctx, |ui| {
            ui.data_mut(|d| d.remove::<String>(ID.to_owned()));
            let (width, height) = (ui.available_width(), ui.available_height());
            let parent_ticks: usize = fg.frames.values().map(|v| v.ticks).sum();
            let mut child_x = 0.0;
            for child in fg.frames.values() {
                child_x += draw_node(ui, parent_ticks, child, child_x, width, height, 1);
            }
            draw_hover_info(ui, height);
        });
    })?;
    Ok(())
}

fn draw_hover_info(ui: &mut egui::Ui, max_height: f32) {
    let hm: Option<String> = ui.data(|d| d.get_temp(ID.to_owned()));

    if let Some(method) = hm {
        ui.painter().text(
            pos2(0.0, max_height),
            egui::Align2::LEFT_TOP,
            method,
            theme::FONT,
            Color32::GRAY,
        );
    }
}

fn get_hover_color(indx: usize, hover: bool) -> Color32 {
    if hover {
        HOVER.to_owned()
    } else {
        theme::pick_green(indx)
    }
}

fn draw_node(
    ui: &mut egui::Ui,
    parent_ticks: usize,
    frame: &Frame,
    x: f32,
    max_width: f32,
    max_height: f32,
    depth: usize,
) -> f32 {
    assert!(
        (frame.ticks as f32 / parent_ticks as f32) <= 1.0,
        "{} / {}",
        frame.ticks,
        parent_ticks
    );
    let node_width = (frame.ticks as f32 / parent_ticks as f32) * max_width;
    let y = max_height - (depth as f32 * HEIGHT);
    if y < 0.0 {
        return 0.0;
    }
    block(
        ui,
        pos2(x, y),
        node_width,
        format!("{:?}", frame.method),
        |h| get_hover_color(depth, h),
    );

    let mut child_x: f32 = x;
    for ele in frame.children.values() {
        assert!(
            ele.ticks <= frame.ticks,
            "ele.ticks <= frame.ticks: {} <= {}",
            ele.ticks,
            frame.ticks
        );
        child_x += draw_node(
            ui,
            frame.ticks,
            ele,
            child_x,
            node_width,
            max_height,
            depth + 1,
        );
    }
    assert!(node_width > 0.0);
    return node_width;
}

fn create_flame_graph(path: &str) -> FlameGraph {
    let file = File::open(path).unwrap();
    FlameGraph::from(file)
}
