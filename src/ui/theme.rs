use egui::{Color32, FontId};

pub const GREENS: [ThemeColor; 3] = [
    ThemeColor::new("#b3e2cd"),
    ThemeColor::new("#fdcdac"),
    ThemeColor::new("#cbd5e8"),
];

pub const HOVER: ThemeColor = ThemeColor::new("#FFFFE0");

pub const FONT: FontId = FontId::proportional(12.0);

pub fn pick_green(index: usize) -> Color32 {
    let i = index % GREENS.len();
    GREENS.get(i).unwrap().into()
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
        Color32::from_hex(&self.value).unwrap()
    }
}

#[cfg(test)]
mod test {
    use egui::Color32;
    use egui::ecolor::HexColor;
    use crate::ui::theme::{pick_green, ThemeColor, GREENS};

    #[test]
    fn no_panic_with_high_indices() {
        let max = GREENS.len() * 2;
        for i in 0..max {
            pick_green(i);
        }
    }
    
    #[test]
    fn converting_to_color32_works() {
        let tc = ThemeColor::new("#ff00ff");
        let c32: Color32 = (&tc).into();
        assert_eq!(HexColor::Hex6(c32).to_string(), tc.value)
    }
}
