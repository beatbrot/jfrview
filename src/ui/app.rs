use std::sync::mpsc::{channel, Receiver, Sender};

use eframe::emath::pos2;
use eframe::epaint::Color32;
use eframe::{App, CreationContext, Frame};
use egui::{Context, Id, ScrollArea, Style};

use crate::flame_graph::FlameGraph;
use crate::ui::block::{Block, HEIGHT};
use crate::ui::fonts::load_fonts;
use crate::ui::theme;
use crate::ui::theme::HOVER;
use crate::ui::ui_frame::UiFrame;

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
                    let uf = UiFrame::new(ui, &vp, self.flame_graph.depth as f32 * HEIGHT);
                    let width = ui.available_width();
                    let parent_ticks = self.flame_graph.ticks(self.include_native);
                    let mut child_x = 0.0;
                    let frames: Vec<_> = self
                        .flame_graph
                        .frames
                        .values()
                        .map(|v| v.to_owned())
                        .collect();
                    for child in frames {
                        let fi: StackFrameInfo = StackFrameInfo {
                            frame: &child,
                            depth: 1,
                            h_offset: child_x,
                            parent_ticks,
                        };
                        child_x += self.draw_node(ui, &fi, width, &uf);
                    }
                });
            });
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
        frame_info: &StackFrameInfo,
        max_width: f32,
        uf: &UiFrame,
    ) -> f32 {
        let node_width = frame_info.ratio(self.include_native) * max_width;
        assert!(node_width > 0.0);
        let y = uf.pos_from_bottom(((frame_info.depth - 1) as f32) * HEIGHT);
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
            child_x += self.draw_node(ui, &fi, node_width, uf);
        }
        assert!(node_width > 0.0);
        node_width
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

/// Information on how to render a [crate::flame_graph::Frame].
struct StackFrameInfo<'a> {
    frame: &'a crate::flame_graph::Frame,
    parent_ticks: usize,
    h_offset: f32,
    depth: usize,
}

impl StackFrameInfo<'_> {
    fn ratio(&self, include_native: bool) -> f32 {
        let res = self.frame.ticks(include_native) as f32 / self.parent_ticks as f32;
        assert!(res <= 1.0);
        res
    }

    fn for_child<'a>(
        &self,
        other: &'a crate::flame_graph::Frame,
        include_native: bool,
        h_offset: f32,
    ) -> StackFrameInfo<'a> {
        StackFrameInfo {
            frame: other,
            parent_ticks: self.frame.ticks(include_native),
            h_offset,
            depth: self.depth + 1,
        }
    }
}
