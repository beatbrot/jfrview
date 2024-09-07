use std::sync::LazyLock;

use egui::{Color32, FontId};

pub const GREENS: LazyLock<[Color32; 3]> = LazyLock::new(||   {
        [
            Color32::from_hex("#b3e2cd").unwrap(),
            Color32::from_hex("#fdcdac").unwrap(),
            Color32::from_hex("#cbd5e8").unwrap()
    ]
});

pub const HOVER: LazyLock<Color32> = LazyLock::new(|| Color32::from_hex("#FFFFE0").unwrap());

pub const FONT: FontId = FontId::proportional(12.0);

pub fn pick_green(index: usize) -> Color32 {
    let i = index % GREENS.len();
    *GREENS.get(i).unwrap()
}


#[cfg(test)]
mod test {
    use std::ops::Deref;
    use crate::ui::theme::{pick_green, GREENS, HOVER};
    
    #[test]
    fn hover_can_be_initialized() {
        let _ = HOVER.deref();
    }

    #[test]
    fn no_panic_with_high_indices() {
        let max = GREENS.len() * 2;
        for i in 0..max {
            pick_green(i);
        }
    }
}
