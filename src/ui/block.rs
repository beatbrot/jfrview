use egui::{pos2, Align2, Color32, CursorIcon, Pos2, Rect, Response, Sense, Ui, Widget};

use crate::ui::theme::FONT;

pub const HEIGHT: f32 = 15.0;

pub struct Block<T: FnOnce(bool) -> Color32> {
    pos: Pos2,
    width: f32,
    text: String,
    color: T,
}

impl<T: FnOnce(bool) -> Color32> Block<T> {
    pub fn new(pos: Pos2, width: f32, text: String, color: T) -> Self {
        assert!(pos.x >= 0.0);
        assert!(pos.y >= 0.0);
        assert!(width > 0.0);
        Self {
            pos,
            width,
            text,
            color,
        }
    }
}

impl<T: FnOnce(bool) -> Color32> Widget for Block<T> {
    fn ui(self, ui: &mut Ui) -> Response {
        puffin::profile_scope!("block_render");
        let pos = self.pos;

        // White vertical border of 1px
        let rect = Rect::from_two_pos(pos, pos2(pos.x + self.width, pos.y - HEIGHT + 1.0));
        let res = ui.allocate_rect(rect, Sense::hover());

        if res.hovered() {
            ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
        }

        ui.painter().rect_filled(rect, 0.0, (self.color)(res.hovered()));

        let trimmed_text = trim_text(self.width, self.text);
        if !trimmed_text.is_empty() {
            ui.painter().text(
                pos2(pos.x + 2.0, pos.y + 1.0),
                Align2::LEFT_BOTTOM,
                trimmed_text,
                FONT,
                Color32::BLACK,
            );
        }
        res
    }
}

fn trim_text(width: f32, text: String) -> String {
    // 6 is just a guess here
    let chars = width as usize / 6;
    if chars >= text.len() {
        text
    } else if chars > 0 {
        let mut t2 = String::with_capacity(chars + 2);
        t2.push_str(&text[0..chars]);
        t2.push_str("..");
        t2
    } else {
        String::new()
    }
}
