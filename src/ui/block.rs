use egui::{vec2, Color32, FontId, Pos2, Rect, Ui, Vec2};
use once_cell::sync::Lazy;

static GREENS: Lazy<Vec<Color32>> = Lazy::new(|| {
    vec![
        Color32::from_hex("#5CFF5C").unwrap(),
        Color32::from_hex("#49CC49").unwrap(),
        Color32::from_hex("#40B340").unwrap(),
        //        Color32::from_hex("#379937").unwrap(),
    ]
});

static HOVER: Lazy<Color32> = Lazy::new(|| Color32::from_hex("#FFFFE0").unwrap());

pub fn block(ui: &mut Ui, x: f32, y: f32, width: f32, text: String) {
    assert!(x >= 0.0);
    assert!(y >= 0.0);
    assert!(width > 0.0);

    let idx = (y / 20.0) as usize % GREENS.len();
    let green = GREENS.get(idx).unwrap().to_owned();

    let height = 20.0;
    let rect =
        // White vertical border of 1px
        Rect::from_min_size(Pos2::new(x, y), vec2(width, height)).expand2(Vec2::new(0.0, -1.0));

    let hover_pos: Option<Pos2> = ui.input(|i| i.pointer.hover_pos());
    let hovered = hover_pos.map(|p| rect.contains(p)).unwrap_or(false);
    if hovered {
        ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
    }
    ui.painter()
        .rect_filled(rect, 0.0, if hovered { HOVER.to_owned() } else { green });

    ui.painter().text(
        egui::Pos2::new(x + 2.0, y + 4.0),
        egui::Align2::LEFT_TOP,
        text,
        FontId::proportional(12.0),
        egui::Color32::BLACK,
    );
}
