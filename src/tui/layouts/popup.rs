use ratatui::{
    layout::{Constraint, Rect},
    widgets::Paragraph,
};

const MAX_WIDTH: u16 = 64;

#[derive(Debug, Clone, Copy)]
pub struct PopupLayout {
    width: u16,
    height: u16,
}

impl PopupLayout {
    pub fn new(paragraph: &Paragraph) -> Self {
        let width = paragraph.line_width().min(MAX_WIDTH as usize) as u16;
        let height = paragraph.line_count(width) as u16;
        Self { width, height }
    }

    pub fn area(self, area: Rect) -> Rect {
        area.centered(
            Constraint::Length(self.width),
            Constraint::Length(self.height),
        )
    }
}
