use std::time::{Duration, Instant};

use ratatui::widgets::{Block, Paragraph, Widget, Wrap};

use crate::{
    misc::{buffer_ext::BufferExt, config::theme},
    tui::{app_default::AppDefault, component::Component, layouts::toast::ToastLayout},
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
        let area = ToastLayout::new(&pg).area(buf.area);
        buf.clear(area);
        pg.render(area, buf);
    }
}
