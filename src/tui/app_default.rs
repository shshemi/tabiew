use std::fmt::Display;

use ratatui::{
    style::Modifier,
    text::Span,
    widgets::{Block, BorderType, List},
};

use crate::misc::config::theme;

pub trait AppDefault {
    fn app_default() -> Self;
}

pub trait AppTitle {
    fn app_title(self, title: impl Display) -> Self;
}

impl AppTitle for Block<'_> {
    fn app_title(self, title: impl Display) -> Self {
        self.title(Span::styled(
            format!(" {title} "),
            theme().block().add_modifier(Modifier::BOLD),
        ))
    }
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
