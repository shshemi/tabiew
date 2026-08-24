use ratatui::layout::{Constraint, Flex, Layout, Rect};

pub trait RectExt {
    fn palette(self, height: u16) -> Self;
}

impl RectExt for Rect {
    fn palette(self, height: u16) -> Self {
        const WIDTH: u16 = 80;
        const MARGIN_TOP: u16 = 3;
        let [area] = Layout::horizontal([Constraint::Length(WIDTH)])
            .flex(Flex::Center)
            .areas(self);
        let [_, area] =
            Layout::vertical([Constraint::Length(MARGIN_TOP), Constraint::Length(height)])
                .areas(area);
        area
    }
}
