use egui::{pos2, vec2, Align2, Color32, Id, Pos2, Rect, Ui};

use crate::ui::theme::FONT;

pub const HEIGHT: f32 = 15.0;

/// Returns a boolean indicating whether the block is hovered
pub fn block(
    ui: &mut Ui,
    pos: Pos2,
    width: f32,
    text: String,
    color: impl FnOnce(bool) -> Color32,
) -> bool {
    assert!(pos.x >= 0.0);
    assert!(pos.y >= 0.0);
    assert!(width > 0.0);

    // White vertical border of 1px
    let rect = Rect::from_min_size(pos, vec2(width, HEIGHT)).expand2(vec2(0.0, -1.0));

    let hover_pos: Option<Pos2> = ui.input(|i| i.pointer.hover_pos());

    let hovered = hover_pos.map(|p| rect.contains(p)).unwrap_or(false);
    if hovered {
        render_hover(ui);
        ui.data_mut(|d| d.insert_temp(Id::new("hovered_method"), text.clone()));
    }

    ui.painter().rect_filled(rect, 0.0, color(hovered));

    ui.painter().text(
        pos2(pos.x + 2.0, pos.y + 1.0),
        Align2::LEFT_TOP,
        text,
        FONT,
        Color32::BLACK,
    );
    hovered
}

fn render_hover(ui: &mut Ui) {
    ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
}
