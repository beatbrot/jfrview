use egui::{vec2, Rect, Ui};

#[derive(Debug)]
pub struct UiFrame {
    /// The upmost coordinate. Often > 0.0 if there are other sibling UI elements above the current one
    #[allow(dead_code)]
    min_height: f32,
    pub max_height: f32,
}

impl UiFrame {
    pub fn new(ui: &mut Ui, vp: &Rect, desired_height: f32) -> Self {
        let h_zero = ui.allocate_space(vec2(10.0, 10.0)).1.min.y;
        Self::from_coords(h_zero, desired_height.max(vp.height()))
    }

    fn from_coords(min_height: f32, max_height: f32) -> Self {
        Self {
            min_height,
            max_height: max_height + min_height,
        }
    }

    pub fn pos_from_bottom(&self, distance: f32) -> f32 {
        self.max_height - distance
    }
}

#[cfg(test)]
mod tests {
    use crate::ui::ui_frame::UiFrame;

    #[test]
    fn max_height_considers_min_height() {
        let uframe = UiFrame::from_coords(10.0, 15.0);
        assert_eq!(uframe.min_height, 10.0);
        assert_eq!(uframe.max_height, 25.0);
    }
}
