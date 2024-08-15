use egui::{vec2, Rect, Ui};
use crate::ui::block::HEIGHT;
use crate::ui::ui_frame::CullingVisibility::Visible;

const DOUBLE_HEIGHT: f32 = HEIGHT + HEIGHT;

/// Visibility of a row (including its 'children', which are the rows above it)
pub enum CullingVisibility {
    /// This row is hidden - children may be visible
    Hidden,
    /// This row is hidden - children are definitely not visible
    HiddenWithChildren,
    /// Row is visible and shall be rendered at given offset
    Visible(f32),
}

#[derive(Debug)]
pub struct UiFrame {
    /// The upmost coordinate. Often > 0.0 if there are other sibling UI elements above the current one
    #[allow(dead_code)]
    min_height: f32,
    /// Maximum visible height
    pub max_vis_height: f32,
    pub max_height: f32,
}

impl UiFrame {
    pub fn new(ui: &mut Ui, vp: &Rect, desired_height: f32) -> Self {
        let h_zero = ui.allocate_space(vec2(10.0, desired_height)).1.min.y;
        Self::from_coords(h_zero, vp.max.y, desired_height.max(vp.height()))
    }

    fn from_coords(min_height: f32, max_vis_height: f32, max_height: f32) -> Self {
        Self {
            min_height,
            max_vis_height,
            max_height: max_height + min_height,
        }
    }

    /// Selects row starting from the bottom. The bottommost row is zero.
    pub fn pos_from_bottom(&self, row: usize) -> CullingVisibility {
        let offset = self.max_height - (row as f32 * HEIGHT);
        if offset <= 0.0 {
            CullingVisibility::HiddenWithChildren
        } else if offset > (self.max_vis_height + DOUBLE_HEIGHT) {
            CullingVisibility::Hidden
        } else {
            Visible(offset)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ui::block::HEIGHT;
    use crate::ui::ui_frame::{CullingVisibility, UiFrame};

    #[test]
    fn max_height_considers_min_height() {
        let uframe = UiFrame::from_coords(10.0, 0.0, 15.0);
        assert_eq!(uframe.min_height, 10.0);
        assert_eq!(uframe.max_height, 25.0);
    }

    #[test]
    fn culling_visiblity() {
        let uframe = UiFrame::from_coords(10.0, HEIGHT * 2.0, HEIGHT * 10.0);
        assert!(matches!(uframe.pos_from_bottom(8),CullingVisibility::Visible(_)));
        assert!(matches!(uframe.pos_from_bottom(3),CullingVisibility::Hidden));
        assert!(matches!(uframe.pos_from_bottom(20),CullingVisibility::HiddenWithChildren));
    }
}
