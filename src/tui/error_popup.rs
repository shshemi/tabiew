use ratatui::{
    layout::{Alignment, Constraint, Flex, Layout},
    widgets::{Clear, Paragraph, Widget, Wrap},
};

use crate::{
    misc::globals::theme,
    tui::{component::Component, widgets::block::Block},
};

#[derive(Debug, Default)]
pub struct ErrorPopup {
    message: String,
}

impl ErrorPopup {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Component for ErrorPopup {
    fn render(
        &mut self,
        _area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        _focus_state: super::component::FocusState,
    ) {
        let pg = Paragraph::new(self.message.as_str())
            .left_aligned()
            .block(
                Block::default()
                    .title(" Error ")
                    .title_alignment(Alignment::Center)
                    .style(theme().error())
                    .into_widget(),
            )
            .wrap(Wrap { trim: true });
        let text_width = pg.line_width().min(64) as u16;
        let [area] = Layout::horizontal([Constraint::Length(text_width)])
            .flex(Flex::Center)
            .areas(buf.area);
        let [area] = Layout::vertical([Constraint::Length((pg.line_count(text_width)) as u16)])
            .flex(Flex::Center)
            .areas(area);
        Clear.render(area, buf);
        pg.render(area, buf);
    }
}
