use std::sync::mpsc::{channel, Receiver, Sender};

use eframe::{App, Frame};
use eframe::emath::pos2;
use eframe::epaint::Color32;
use egui::{Context, Style};

use crate::flame_graph::FlameGraph;
use crate::ui::block::{block, HEIGHT};
use crate::ui::theme;
use crate::ui::theme::HOVER;

pub struct JfrViewApp {
    pub flame_graph: FlameGraph,
    pub file_channel: (Sender<FlameGraph>, Receiver<FlameGraph>),
    pub include_native: bool,
    pub hovered: Option<String>,
}

impl App for JfrViewApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.hovered = None;
        if let Ok(fg) = self.file_channel.1.try_recv() {
            self.flame_graph = fg;
        }

        self.create_menubar(ctx);
        egui::CentralPanel::default()
            .frame(Self::central_frame(&ctx.style()))
            .show(ctx, |ui| {
                let (width, height) = (ui.available_width(), ui.available_height());
                let parent_ticks: usize = self.flame_graph.frames.values().map(|v| v.ticks(self.include_native)).sum();
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

impl Default for JfrViewApp {
    fn default() -> Self {
        Self {
            file_channel: channel(),
            flame_graph: Default::default(),
            include_native: false,
            hovered: None,
        }
    }
}

impl JfrViewApp {
    pub fn new(flame_graph: FlameGraph) -> Self {
        Self {
            file_channel: channel(),
            flame_graph,
            include_native: false,
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
        let ratio = frame_info.frame.ticks(self.include_native) as f32 / frame_info.parent_ticks as f32;
        assert!(ratio <= 1.0);
        let node_width = ratio * max_width;
        assert!(node_width > 0.0);
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
            self.hovered = Some(format!("{:?} ({} samples)", frame_info.frame.method, frame_info.frame.ticks(self.include_native)));
        }

        let mut child_x: f32 = frame_info.h_offset;
        for ele in frame_info.frame.children.values() {
            if ele.has_no_samples(self.include_native) {
                continue;
            }
            assert!(ele.ticks(self.include_native) <= frame_info.frame.ticks(self.include_native));
            let fi = frame_info.for_child(ele, self.include_native, child_x);
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

    fn central_frame(style: &Style) -> egui::containers::Frame {
        egui::containers::Frame::central_panel(style)
            .outer_margin(0.0)
            .inner_margin(0.0)
    }
}

struct FrameInfo<'a> {
    frame: &'a crate::flame_graph::Frame,
    parent_ticks: usize,
    h_offset: f32,
    depth: usize,
}

impl FrameInfo<'_> {
    fn for_child<'a>(&self, other: &'a crate::flame_graph::Frame, include_native: bool, h_offset: f32) -> FrameInfo<'a> {
        FrameInfo {
            frame: other,
            parent_ticks: self.frame.ticks(include_native),
            h_offset,
            depth: self.depth + 1,
        }
    }
}
