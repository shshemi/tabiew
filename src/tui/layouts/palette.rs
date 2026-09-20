use ratatui::layout::{Constraint, Flex, Layout, Rect};

#[derive(Debug, Clone, Copy)]
pub struct PaletteLayout {
    height: u16,
    width: u16,
    vertical_margin: u16,
}

impl PaletteLayout {
    pub fn new(height: u16) -> Self {
        Self {
            height,
            width: 80,
            vertical_margin: 3,
        }
    }
    pub fn with_width(self, width: u16) -> Self {
        Self { width, ..self }
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
            .flex(Flex::Start)
            .vertical_margin(self.vertical_margin)
            .areas(area);
        area
    }
}
