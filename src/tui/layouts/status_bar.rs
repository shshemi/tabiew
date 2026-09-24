use ratatui::layout::{Constraint, Flex, Layout, Rect};

#[derive(Debug, Clone, Copy)]
pub struct StatusBarLayout;

impl StatusBarLayout {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self
    }

    pub fn area(self, area: Rect) -> Rect {
        let [area] = Layout::vertical([Constraint::Length(1)])
            .flex(Flex::End)
            .horizontal_margin(1)
            .areas(area);
        area
    }
}
