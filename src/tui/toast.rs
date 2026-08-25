use std::time::{Duration, Instant};

use ratatui::{
    layout::{Constraint, Flex, Layout},
    widgets::{Block, Paragraph, Widget, Wrap},
};

use crate::{
    misc::{buffer_ext::BufferExt, config::theme},
    tui::{app_default::AppDefault, component::Component},
};

#[derive(Debug)]
pub struct Toast {
    message: String,
    start: Instant,
}

impl Toast {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            start: Instant::now(),
        }
    }

    pub fn is_finished(&self) -> bool {
        self.start.elapsed() > Duration::from_secs(3)
    }
}

impl Component for Toast {
    fn render(
        &mut self,
        _area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        _focus_state: super::component::FocusState,
    ) {
        let pg = Paragraph::new(self.message.as_str())
            .style(theme().text())
            .left_aligned()
            .block(Block::app_default().style(theme().block()))
            .wrap(Wrap { trim: true });
        let width = pg.line_width().min(64) as u16;
        let [area] = Layout::horizontal([Constraint::Length(width)])
            .flex(Flex::Center)
            .areas(buf.area);
        let [area, _] = Layout::vertical([
            Constraint::Length((pg.line_count(width)) as u16),
            Constraint::Length(3),
        ])
        .flex(Flex::End)
        .areas(area);
        buf.clear(area);
        pg.render(area, buf);
    }
}
