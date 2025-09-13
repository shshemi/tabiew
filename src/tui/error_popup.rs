use ratatui::{
    layout::{Alignment, Constraint, Flex, Layout},
    text::Text,
    widgets::{Clear, Paragraph, Widget, Wrap},
};

use crate::{misc::globals::theme, tui::widgets::block::Block};

#[derive(Debug, Default)]
pub struct ErrorPopup<'a> {
    pg: Paragraph<'a>,
}

impl<'a> ErrorPopup<'a> {
    pub fn new() -> Self {
        Self {
            pg: Default::default(),
        }
    }
    pub fn with_message(mut self, message: impl Into<Text<'a>>) -> Self {
        self.pg = Paragraph::new(message)
            .left_aligned()
            .block(
                Block::default()
                    .title(" Error ")
                    .title_alignment(Alignment::Center)
                    .style(theme().error())
                    .into_widget(),
            )
            .wrap(Wrap { trim: true });
        self
    }
}

impl Widget for ErrorPopup<'_> {
    fn render(self, _: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let text_width = self.pg.line_width().min(64) as u16;
        let [area] = Layout::horizontal([Constraint::Length(text_width)])
            .flex(Flex::Center)
            .areas(buf.area);
        let [area] =
            Layout::vertical([Constraint::Length((self.pg.line_count(text_width)) as u16)])
                .flex(Flex::Center)
                .areas(area);
        Clear.render(area, buf);
        self.pg.render(area, buf);
    }
}
