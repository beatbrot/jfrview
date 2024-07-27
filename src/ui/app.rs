use eframe::{App, Frame};
use eframe::emath::pos2;
use eframe::epaint::Color32;
use egui::{Context, Id};
use once_cell::sync::Lazy;

use crate::flame_graph::FlameGraph;
use crate::ui::block::{block, HEIGHT};
use crate::ui::theme;
use crate::ui::theme::HOVER;

const ID: Lazy<Id> = Lazy::new(|| Id::new("hovered_method"));

pub struct JfrViewApp {
    flame_graph: FlameGraph,
}

impl JfrViewApp {
    pub fn new(flame_graph: FlameGraph) -> Self {
        Self { flame_graph }
    }
}

impl App for JfrViewApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.data_mut(|d| d.remove::<String>(ID.to_owned()));
            let (width, height) = (ui.available_width(), ui.available_height());
            let parent_ticks: usize = self.flame_graph.frames.values().map(|v| v.ticks).sum();
            let mut child_x = 0.0;
            for child in self.flame_graph.frames.values() {
                child_x += draw_node(ui, parent_ticks, child, child_x, width, height, 1);
            }
            assert_eq!(width, child_x);
            draw_hover_info(ui, height);
        });
    }
}

fn draw_node(
    ui: &mut egui::Ui,
    parent_ticks: usize,
    frame: &crate::flame_graph::Frame,
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

fn get_hover_color(index: usize, hover: bool) -> Color32 {
    if hover {
        HOVER.to_owned()
    } else {
        theme::pick_green(index)
    }
}