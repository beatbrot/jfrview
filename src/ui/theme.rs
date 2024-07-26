use egui::{Color32, FontId};
use once_cell::sync::Lazy;

pub const GREENS: Lazy<[Color32; 3]> = Lazy::new(|| {
    [
        Color32::from_hex("#5CFF5C").unwrap(),
        Color32::from_hex("#49CC49").unwrap(),
        Color32::from_hex("#40B340").unwrap(),
    ]
});

pub const HOVER: Lazy<Color32> = Lazy::new(|| Color32::from_hex("#FFFFE0").unwrap());

pub const FONT: FontId = FontId::proportional(12.0);

pub fn pick_green(indx: usize) -> Color32 {
    let i = indx % GREENS.len();
    return GREENS.get(i).unwrap().to_owned();
}
