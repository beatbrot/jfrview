use egui::{vec2, Color32, FontId, Pos2, Rect, Ui, Vec2};

pub fn block(ui: &mut Ui, x: f32, y: f32, width: f32, text: String) {
    assert!(x >= 0.0);
    assert!(y >= 0.0);
    assert!(width > 0.0);
    let greens = vec![
        Color32::from_hex("#53e453").unwrap(),
        Color32::from_hex("#6afb6a").unwrap(),
    ];
    let hover = Color32::from_hex("#FFFFE0").unwrap();

    let idx = (y / 20.0) as usize % 2;
    let green = greens.get(idx).unwrap().to_owned();

    let height = 20.0;
    let rect =
        Rect::from_min_size(Pos2::new(x, y), vec2(width, height)).expand2(Vec2::new(0.0, -1.0));

    let hover_pos: Pos2 = ui.input(|i| i.pointer.hover_pos().unwrap_or_default());
    let hovered = rect.contains(hover_pos);
    ui.painter()
        .rect_filled(rect, 0.0, if hovered { hover } else { green });

    ui.painter().text(
        egui::Pos2::new(x + 2.0, y + 4.0),
        egui::Align2::LEFT_TOP,
        text,
        FontId::proportional(12.0),
        egui::Color32::BLACK,
    );
}
