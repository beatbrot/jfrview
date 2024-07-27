use eframe::{App, Frame};
use eframe::emath::pos2;
use eframe::epaint::Color32;
use egui::{Context};

use crate::flame_graph::FlameGraph;
use crate::ui::block::{block, HEIGHT};
use crate::ui::theme;
use crate::ui::theme::HOVER;

pub struct JfrViewApp {
    flame_graph: FlameGraph,
    hovered: Option<String>,
}

impl App for JfrViewApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.hovered = None;
            let (width, height) = (ui.available_width(), ui.available_height());
            let parent_ticks: usize = self.flame_graph.frames.values().map(|v| v.ticks).sum();
            let mut child_x = 0.0;
            let frames: Vec<_> = self.flame_graph.frames.values().map(|v| v.to_owned()).collect();
            for child in frames {
                child_x += self.draw_node(ui, parent_ticks, &child, child_x, width, height, 1);
            }
            assert_eq!(width, child_x);
            self.draw_hover_info(ui, height);
        });
    }
}

impl JfrViewApp {
    pub fn new(flame_graph: FlameGraph) -> Self {
        Self { flame_graph, hovered: None }
    }

    fn draw_node(
        &mut self,
        ui: &mut egui::Ui,
        parent_ticks: usize,
        frame: &crate::flame_graph::Frame,
        x: f32,
        max_width: f32,
        max_height: f32,
        depth: usize,
    ) -> f32 {
        assert!((frame.ticks as f32 / parent_ticks as f32) <= 1.0);
        let node_width = (frame.ticks as f32 / parent_ticks as f32) * max_width;
        let y = max_height - (depth as f32 * HEIGHT);
        if y < 0.0 {
            return 0.0;
        }
        let hovered = block(
            ui,
            pos2(x, y),
            node_width,
            format!("{:?}", frame.method),
            |h| Self::get_hover_color(depth, h),
        );
        if hovered {
            self.hovered = Some(format!("{:?}", frame.method));
        }

        let mut child_x: f32 = x;
        for ele in frame.children.values() {
            assert!(ele.ticks <= frame.ticks);
            child_x += self.draw_node(
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

    fn draw_hover_info(&self, ui: &mut egui::Ui, max_height: f32) {
        if let Some(method) = &self.hovered {
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
}