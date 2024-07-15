mod data;
mod flame_graph;

use std::{env, fs::File};

use data::ExecutionSample;
use egui::{vec2, Color32, FontId, Pos2};
use flame_graph::{FlameGraph, Frame};
use jfrs::reader::JfrReader;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let fg = create_flame_graph("C:\\Users\\loych\\Development\\jfrview\\cfg6_validate_small.jfr");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_simple_native("JfrView", options, move |ctx, _| {
        egui::CentralPanel::default().show(&ctx, |ui| {
            let (width, _) = (ui.available_width(), ui.available_height());
            let parent_ticks: usize = fg.frames.values().map(|v| v.ticks).sum();
            let mut child_x = 0.0;
            for child in fg.frames.values() {
                child_x += draw_node(ui, parent_ticks, child, child_x, width, 0);
            }
        });
    })
    .unwrap();
}

fn draw_node(
    ui: &mut egui::Ui,
    parent_ticks: usize,
    frame: &Frame,
    x: f32,
    max_width: f32,
    depth: usize,
) -> f32 {
    assert!(
        (frame.ticks as f32 / parent_ticks as f32) <= 1.0,
        "{} / {}",
        frame.ticks,
        parent_ticks
    );
    let node_width = (frame.ticks as f32 / parent_ticks as f32) * max_width;
    let node_height = 20.0;
    let y = depth as f32 * node_height;

    ui.painter().rect_filled(
        egui::Rect::from_min_size(Pos2::new(x, y), vec2(node_width, node_height)),
        0.0,
        Color32::RED,
    );
    ui.painter().rect_filled(
        egui::Rect::from_min_size(
            Pos2::new(x + 2.0, y + 2.0),
            vec2(node_width - 4.0, node_height - 4.0),
        ),
        0.0,
        Color32::YELLOW,
    );

    ui.painter().text(
        egui::Pos2::new(x, y),
        egui::Align2::LEFT_TOP,
        format!("{:?}", frame.method),
        FontId::default(),
        egui::Color32::BLACK,
    );

    let mut child_x: f32 = x;
    for ele in frame.children.values() {
        assert!(
            ele.ticks <= frame.ticks,
            "ele.ticks <= frame.ticks: {} <= {}",
            ele.ticks,
            frame.ticks
        );
        child_x += draw_node(ui, frame.ticks, ele, child_x, node_width, depth + 1);
    }
    return node_width;
}

fn create_flame_graph(path: &str) -> FlameGraph {
    let file = File::open(path).unwrap();
    let mut reader = JfrReader::new(file);

    let mut fg = FlameGraph::default();

    for (mut r, c) in reader.chunks().flatten() {
        for sample in r
            .events(&c)
            .flatten()
            .filter(|e| e.class.name() == "jdk.ExecutionSample")
            .map(|e| ExecutionSample::from(e))
        {
            if !sample.stack_trace.truncated {
                fg.add_sample(sample);
            }
        }
    }
    return fg;
}
