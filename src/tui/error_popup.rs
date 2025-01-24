use std::{borrow::Cow, marker::PhantomData};

use ratatui::{
    layout::Alignment,
    widgets::{Block, Clear, Paragraph, Widget, Wrap},
};

use super::{utils::line_count, Styler};

#[derive(Debug, Default)]
pub struct ErrorPopup<'a, Theme> {
    message: Cow<'a, str>,
    _theme: PhantomData<Theme>,
}

impl<'a, Theme> ErrorPopup<'a, Theme> {
    pub fn new() -> Self {
        Self {
            message: Default::default(),
            _theme: Default::default(),
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

impl<Theme: Styler> Widget for ErrorPopup<'_, Theme> {
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
                    .border_style(Theme::status_bar_error()),
            )
            .style(Theme::status_bar_error())
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }
}
