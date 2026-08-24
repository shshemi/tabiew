use ratatui::widgets::{Block, BorderType, List};

use crate::misc::config::theme;

pub trait AppDefault {
    fn app_default() -> Self;
}

impl AppDefault for Block<'_> {
    fn app_default() -> Self {
        Self::bordered()
            .border_type(BorderType::Rounded)
            .style(theme().block())
    }
}

impl AppDefault for List<'_> {
    fn app_default() -> Self {
        List::default()
            .style(theme().text())
            .highlight_style(theme().row_highlighted())
    }
}
