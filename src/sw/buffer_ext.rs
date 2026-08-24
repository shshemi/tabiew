use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Clear, Widget},
};

use crate::misc::color_ext::ColorExt;

pub trait BufferExt {
    fn clear(&mut self, area: Rect);
    fn darken(&mut self);
}

impl BufferExt for Buffer {
    fn clear(&mut self, area: Rect) {
        Clear.render(area, self);
    }

    fn darken(&mut self) {
        for cell in self.content.iter_mut() {
            cell.bg = cell.bg.darken();
            cell.fg = cell.fg.darken();
        }
    }
}
