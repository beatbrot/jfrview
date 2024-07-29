use egui::{Color32, FontId};

pub const GREENS: [ThemeColor; 3] = [
    ThemeColor::new("#5CFF5C"),
    ThemeColor::new("#49CC49"),
    ThemeColor::new("#40B340"),
];

pub const HOVER: ThemeColor = ThemeColor::new("#FFFFE0");

pub const FONT: FontId = FontId::proportional(12.0);

pub fn pick_green(index: usize) -> Color32 {
    let i = index % GREENS.len();
    return GREENS.get(i).unwrap().into();
}

pub struct ThemeColor {
    value: &'static str,
}

impl ThemeColor {
    const fn new(color: &'static str) -> Self {
        Self { value: color }
    }
}

impl Into<Color32> for &ThemeColor {
    fn into(self) -> Color32 {
        return Color32::from_hex(&self.value).unwrap();
    }
}