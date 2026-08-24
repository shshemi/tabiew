use ratatui::widgets::{Block, BorderType};

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
