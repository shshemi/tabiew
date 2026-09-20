use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    widgets::Paragraph,
};

const MAX_WIDTH: u16 = 64;

#[derive(Debug, Clone, Copy)]
pub struct ToastLayout {
    width: u16,
    height: u16,
    vertical_margin: u16,
}

impl ToastLayout {
    pub fn new(paragraph: &Paragraph) -> Self {
        let width = paragraph.line_width().min(MAX_WIDTH as usize) as u16;
        let height = paragraph.line_count(width) as u16;
        Self {
            width,
            height,
            vertical_margin: 3,
        }
    }

    pub fn with_vertical_margin(self, vertical_margin: u16) -> Self {
        Self {
            vertical_margin,
            ..self
        }
    }

    pub fn area(self, area: Rect) -> Rect {
        let [area] = Layout::horizontal([Constraint::Length(self.width)])
            .flex(Flex::Center)
            .areas(area);
        let [area] = Layout::vertical([Constraint::Length(self.height)])
            .flex(Flex::End)
            .vertical_margin(self.vertical_margin)
            .areas(area);
        area
    }
}
