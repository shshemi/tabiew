use ratatui::layout::{Constraint, Flex, Layout, Rect};

pub trait RectExt {
    fn palette(self, items: usize) -> Self;
}

impl RectExt for Rect {
    fn palette(self, items: usize) -> Self {
        const WIDTH: u16 = 80;
        let height = items.saturating_add(2).min(25) as u16;
        let [area] = Layout::horizontal([Constraint::Length(WIDTH)])
            .flex(Flex::Center)
            .areas(self);
        let [_, area] =
            Layout::vertical([Constraint::Length(3), Constraint::Length(height)]).areas(area);
        area
    }
}
