use crate::misc::{buffer_ext::BufferExt, config::theme, rect_ext::RectExt};
use crossterm::event::KeyCode;
use ratatui::{
    layout::Alignment,
    widgets::{Block, Paragraph, Widget, Wrap},
};

use crate::{
    handler::message::Message,
    tui::{app_default::AppDefault, component::Component},
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
                Block::app_default()
                    .title(" Error ")
                    .title_alignment(Alignment::Center)
                    .style(theme().error()),
            )
            .wrap(Wrap { trim: true });
        let area = buf.area.popup(&pg);
        buf.clear(area);
        pg.render(area, buf);
    }
    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        match event.code {
            KeyCode::Char(':') => Message::AppShowCommandPicker.enqueue(),
            _ => Message::AppDismissOverlay.enqueue(),
        };
        true
    }
}
