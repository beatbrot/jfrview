use eframe::emath::pos2;
use eframe::epaint::Color32;
use eframe::{App, Frame};
use egui::Context;

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
            let frames: Vec<_> = self
                .flame_graph
                .frames
                .values()
                .map(|v| v.to_owned())
                .collect();
            for child in frames {
                let fi: FrameInfo = FrameInfo {
                    frame: &child,
                    depth: 1,
                    h_offset: child_x,
                    parent_ticks,
                };
                child_x += self.draw_node(ui, &fi, width, height);
            }
            self.draw_hover_info(ui, height);
        });
    }
}

impl JfrViewApp {
    pub fn new(flame_graph: FlameGraph) -> Self {
        Self {
            flame_graph,
            hovered: None,
        }
    }

    fn draw_node(
        &mut self,
        ui: &mut egui::Ui,
        frame_info: &FrameInfo,
        max_width: f32,
        max_height: f32,
    ) -> f32 {
        assert!((frame_info.frame.ticks as f32 / frame_info.parent_ticks as f32) <= 1.0);
        let node_width =
            (frame_info.frame.ticks as f32 / frame_info.parent_ticks as f32) * max_width;
        let y = max_height - (frame_info.depth as f32 * HEIGHT);
        if y < 0.0 {
            return 0.0;
        }
        let hovered = block(
            ui,
            pos2(frame_info.h_offset, y),
            node_width,
            format!("{:?}", frame_info.frame.method),
            |h| Self::get_hover_color(frame_info.depth, h),
        );
        if hovered {
            self.hovered = Some(format!("{:?}", frame_info.frame.method));
        }

        let mut child_x: f32 = frame_info.h_offset;
        for ele in frame_info.frame.children.values() {
            assert!(ele.ticks <= frame_info.frame.ticks);
            let fi = frame_info.for_child(ele, child_x);
            child_x += self.draw_node(ui, &fi, node_width, max_height);
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

struct FrameInfo<'a> {
    frame: &'a crate::flame_graph::Frame,
    parent_ticks: usize,
    h_offset: f32,
    depth: usize,
}

impl FrameInfo<'_> {
    fn for_child<'a>(&self, other: &'a crate::flame_graph::Frame, h_offset: f32) -> FrameInfo<'a> {
        FrameInfo {
            frame: other,
            parent_ticks: self.frame.ticks,
            h_offset,
            depth: self.depth + 1,
        }
    }
}
