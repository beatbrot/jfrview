use std::sync::mpsc::{channel, Receiver, Sender};

use eframe::emath::pos2;
use eframe::epaint::Color32;
use eframe::{App, CreationContext, Frame};
use egui::{Context, Id, Rect, ScrollArea, Style};

use crate::flame_graph::FlameGraph;
use crate::ui::block::{Block, HEIGHT};
use crate::ui::fonts::load_fonts;
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
        if let Ok(fg) = self.file_channel.1.try_recv() {
            self.flame_graph = fg;
        }

        self.create_menubar(ctx);
        egui::TopBottomPanel::bottom(Id::new("bottom")).show(&ctx, |ui| {
            self.draw_hover_info(ui);
        });
        self.hovered = None;

        egui::CentralPanel::default()
            .frame(Self::central_frame(&ctx.style()))
            .show(ctx, |ui| {
                ScrollArea::vertical().show_viewport(ui, |ui, vp| {
                    let width = ui.available_width();
                    let height = f32_max(vp.height() + 20.0, self.flame_graph.depth as f32 * HEIGHT);
                    let parent_ticks: usize = self
                        .flame_graph
                        .frames
                        .values()
                        .map(|v| v.ticks(self.include_native))
                        .sum();
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
                        child_x += self.draw_node(ui, &fi, width, height, &vp);
                    }
                });
            });
    }
}

fn f32_max(a: f32, b: f32) -> f32 {
    if a < b {
        b
    } else {
        a
    }
}

impl JfrViewApp {
    pub fn new(cc: &CreationContext, flame_graph: FlameGraph) -> Self {
        cc.egui_ctx.set_fonts(load_fonts());
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
        viewport: &Rect,
    ) -> f32 {
        let offset = viewport.min.y;
        let ratio =
            frame_info.frame.ticks(self.include_native) as f32 / frame_info.parent_ticks as f32;
        assert!(ratio <= 1.0);
        let node_width = ratio * max_width;
        assert!(node_width > 0.0);
        let y = (max_height - (frame_info.depth as f32 * HEIGHT)) - offset;
        if y < 0.0 {
            return 0.0;
        }
        let response = ui.add(Block::new(
            pos2(frame_info.h_offset, y),
            node_width,
            format!("{:?}", frame_info.frame.method),
            |h| Self::get_hover_color(frame_info.depth, h),
        ));
        if response.hovered() {
            self.hovered = Some(format!(
                "{:?} ({} samples)",
                frame_info.frame.method,
                frame_info.frame.ticks(self.include_native)
            ));
        }

        let mut child_x: f32 = frame_info.h_offset;
        for ele in frame_info.frame.children.values() {
            if ele.has_no_samples(self.include_native) {
                continue;
            }
            assert!(ele.ticks(self.include_native) <= frame_info.frame.ticks(self.include_native));
            let fi = frame_info.for_child(ele, self.include_native, child_x);
            child_x += self.draw_node(ui, &fi, node_width, max_height, viewport);
        }
        assert!(node_width > 0.0);
        return node_width;
    }

    fn draw_hover_info(&self, ui: &mut egui::Ui) {
        if let Some(method) = &self.hovered {
            ui.colored_label(Color32::GRAY, method);
        }
    }

    fn get_hover_color(index: usize, hover: bool) -> Color32 {
        if hover {
            (&HOVER).into()
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
    fn for_child<'a>(
        &self,
        other: &'a crate::flame_graph::Frame,
        include_native: bool,
        h_offset: f32,
    ) -> FrameInfo<'a> {
        FrameInfo {
            frame: other,
            parent_ticks: self.frame.ticks(include_native),
            h_offset,
            depth: self.depth + 1,
        }
    }
}
