use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Clear, Widget},
};

pub trait BufferExt {
    fn clear(&mut self, area: Rect);
}

impl BufferExt for Buffer {
    fn clear(&mut self, area: Rect) {
        Clear.render(area, self);
    }
}
