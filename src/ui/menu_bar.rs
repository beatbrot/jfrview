use std::io::Cursor;
use egui::{Context, Id};
use rfd::AsyncFileDialog;
use crate::exec::exec;
use crate::flame_graph::FlameGraph;
use crate::ui::app::JfrViewApp;

impl JfrViewApp {
    pub fn create_menubar(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top(Id::new("top")).show(ctx, |ui| {
            ui.horizontal(|ui| {
                let button = ui.button("Pick file...");
                if button.clicked() {
                    self.pick_jfr_file(ctx);
                }
                ui.checkbox(&mut self.include_native, "Include native");
            });
        });
    }
    
    fn pick_jfr_file(&self, ctx: &Context) {
        let sender = self.file_channel.0.clone();
        let ctx = ctx.clone();
        let native = self.include_native;
        exec(async move {
            if let Some(path) = AsyncFileDialog::new().pick_file().await {
                let bytes = path.read().await;
                let cursor = Cursor::new(bytes);
                sender.send(FlameGraph::new(cursor, native)).unwrap();
                ctx.request_repaint();
            }
        });
    }
}
