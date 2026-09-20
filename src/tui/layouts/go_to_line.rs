use ratatui::layout::{Constraint, Flex, Layout, Rect};

#[derive(Debug, Clone, Copy)]
pub struct GoToLineLayout {
    width: u16,
    height: u16,
    margin: u16,
}

impl GoToLineLayout {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            margin: 1,
        }
    }

    pub fn with_margin(self, margin: u16) -> Self {
        Self { margin, ..self }
    }

    pub fn area(self, area: Rect) -> Rect {
        let [area] = Layout::horizontal([Constraint::Length(self.width)])
            .flex(Flex::End)
            .horizontal_margin(self.margin)
            .areas(area);
        let [area] = Layout::vertical([Constraint::Length(self.height)])
            .flex(Flex::Start)
            .vertical_margin(self.margin)
            .areas(area);
        area
    }
}
