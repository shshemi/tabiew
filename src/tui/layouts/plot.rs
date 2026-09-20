use ratatui::layout::{Margin, Rect};

#[derive(Debug, Clone, Copy)]
pub struct PlotLayout {
    horizontal_margin: u16,
    vertical_margin: u16,
}

impl Default for PlotLayout {
    fn default() -> Self {
        Self {
            horizontal_margin: 7,
            vertical_margin: 3,
        }
    }
}

impl PlotLayout {
    pub fn with_horizontal_margin(self, horizontal_margin: u16) -> Self {
        Self {
            horizontal_margin,
            ..self
        }
    }

    pub fn with_vertical_margin(self, vertical_margin: u16) -> Self {
        Self {
            vertical_margin,
            ..self
        }
    }

    pub fn area(self, area: Rect) -> Rect {
        area.inner(Margin::new(self.horizontal_margin, self.vertical_margin))
    }
}
