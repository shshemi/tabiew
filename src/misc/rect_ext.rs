use ratatui::layout::{Margin, Rect};

pub trait RectExt {
    fn plot(self) -> Self;
}

impl RectExt for Rect {
    fn plot(self) -> Self {
        const MARGIN_HORIZONTAL: u16 = 7;
        const MARGIN_VERTICAL: u16 = 3;
        self.inner(Margin::new(MARGIN_HORIZONTAL, MARGIN_VERTICAL))
    }
}
