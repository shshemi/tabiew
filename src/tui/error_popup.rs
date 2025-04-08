use std::borrow::Cow;

use ratatui::{
    layout::Alignment,
    widgets::{Block, Clear, Paragraph, Widget, Wrap},
};

use crate::misc::globals::theme;

use super::utils::line_count;

#[derive(Debug, Default)]
pub struct ErrorPopup<'a> {
    message: Cow<'a, str>,
}

impl<'a> ErrorPopup<'a> {
    pub fn new() -> Self {
        Self {
            message: Default::default(),
        }
    }
    pub fn with_message(mut self, message: impl Into<Cow<'a, str>>) -> Self {
        self.message = message.into();
        self
    }

    pub fn line_count(&self, width: usize) -> u16 {
        line_count(self.message.as_ref(), width) as u16 + 2
    }
}

impl Widget for ErrorPopup<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Clear.render(area, buf);
        Paragraph::new(self.message)
            .left_aligned()
            .block(
                Block::bordered()
                    .title(" Error ")
                    .title_alignment(Alignment::Center)
                    .border_style(theme().error()),
            )
            .style(theme().error())
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }
}
