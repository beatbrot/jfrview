use egui::{vec2, Color32, FontId, Pos2, Rect, Ui};

pub fn block(ui: &mut Ui, x: f32, y: f32, width: f32, height: f32, text: String) {
    let rect = Rect::from_min_size(Pos2::new(x, y), vec2(width, height));
    ui.painter().rect_filled(rect, 0.0, Color32::RED);

    // border
    ui.painter().rect_filled(
        egui::Rect::from_min_size(Pos2::new(x + 2.0, y + 2.0), vec2(width - 4.0, height - 4.0)),
        0.0,
        Color32::YELLOW,
    );

    ui.painter().text(
        egui::Pos2::new(x, y),
        egui::Align2::LEFT_TOP,
        text,
        FontId::monospace(12.0),
        egui::Color32::BLACK,
    );
}
