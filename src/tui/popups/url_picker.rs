use crossterm::event::KeyEvent;
use url::Url;

use crate::tui::{component::Component, pickers::text_picker::TextPicker};

#[derive(Debug)]
pub struct UrlPicker {
    text_picker: TextPicker,
}

impl UrlPicker {
    pub fn with_title(self, title: impl Into<String>) -> Self {
        Self {
            text_picker: self.text_picker.with_title(title),
        }
    }
    pub fn url(&self) -> Url {
        self.text_picker.input().value().parse().unwrap()
    }
}

impl Component for UrlPicker {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
    ) {
        self.text_picker.render(area, buf, focus_state);
    }

    fn handle(&mut self, event: KeyEvent) -> bool {
        self.text_picker.handle(event)
    }
}

impl Default for UrlPicker {
    fn default() -> Self {
        Self {
            text_picker: TextPicker::default().with_title("URL"),
        }
    }
}
