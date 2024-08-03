use egui::{Align2, Color32, CursorIcon, pos2, Pos2, Rect, Response, Sense, Ui, Widget};

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
        let pos = self.pos;

        // White vertical border of 1px
        let rect = Rect::from_two_pos(pos, pos2(pos.x + self.width, pos.y + HEIGHT - 1.0));
        let res = ui.allocate_rect(rect, Sense::hover());

        let hover_pos: Option<Pos2> = ui.input(|i| i.pointer.hover_pos());

        let hovered = hover_pos.map(|p| rect.contains(p)).unwrap_or(false);
        if res.hovered() {
            ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
        }

        ui.painter().rect_filled(rect, 0.0, (self.color)(hovered));

        ui.painter().text(
            pos2(pos.x + 2.0, pos.y + 1.0),
            Align2::LEFT_TOP,
            trim_text(self.width, self.text),
            FONT,
            Color32::BLACK,
        );
        res
    }
}

fn trim_text(width: f32, text: String) -> String {
    // 6 is just a guess here
    let chars = width as usize / 6;
    if chars >= text.len() {
        text
    } else if chars > 0 {
        let mut t2 = text.clone();
        t2.truncate(chars);
        t2.push_str("..");
        t2
    } else {
        String::new()
    }
}
