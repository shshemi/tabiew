use ratatui::layout::{Constraint, Flex, Layout, Rect};

#[derive(Debug, Clone, Copy)]
pub struct NotificationLayout {
    width: u16,
    margin: u16,
}

impl Default for NotificationLayout {
    fn default() -> Self {
        Self {
            width: 40,
            margin: 1,
        }
    }
}

impl NotificationLayout {
    pub fn with_width(self, width: u16) -> Self {
        Self { width, ..self }
    }

    pub fn with_margin(self, margin: u16) -> Self {
        Self { margin, ..self }
    }

    pub fn area(self, area: Rect) -> Rect {
        let [area] = Layout::horizontal([Constraint::Length(self.width)])
            .flex(Flex::End)
            .horizontal_margin(self.margin)
            .vertical_margin(self.margin)
            .areas(area);
        area
    }
}
