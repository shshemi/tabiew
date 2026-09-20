use ratatui::layout::{Constraint, Flex, Layout, Margin, Rect};

pub trait RectExt {
    fn goto_line(self, width: u16, height: u16) -> Self;
    fn plot(self) -> Self;
}

impl RectExt for Rect {
    fn goto_line(self, width: u16, height: u16) -> Self {
        const MARGIN: u16 = 1;
        let [area, _] = Layout::horizontal([Constraint::Length(width), Constraint::Length(MARGIN)])
            .flex(Flex::End)
            .areas(self);
        let [_, area] =
            Layout::vertical([Constraint::Length(MARGIN), Constraint::Length(height)]).areas(area);
        area
    }

    fn plot(self) -> Self {
        const MARGIN_HORIZONTAL: u16 = 7;
        const MARGIN_VERTICAL: u16 = 3;
        self.inner(Margin::new(MARGIN_HORIZONTAL, MARGIN_VERTICAL))
    }
}
