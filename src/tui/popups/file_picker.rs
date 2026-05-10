use std::path::PathBuf;

use crossterm::event::KeyEvent;
use home::home_dir;

use crate::tui::{component::Component, pickers::text_picker::TextPicker};

#[derive(Debug)]
pub struct FilePicker {
    text_picker: TextPicker,
}

impl FilePicker {
    pub fn with_title(self, title: impl Into<String>) -> Self {
        Self {
            text_picker: self.text_picker.with_title(title),
        }
    }
    pub fn path(&self) -> PathBuf {
        self.text_picker.input().value().into()
    }
}

impl Component for FilePicker {
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

impl Default for FilePicker {
    fn default() -> Self {
        Self {
            text_picker: TextPicker::default().with_title("File Path").with_value(
                std::env::current_dir()
                    .ok()
                    .or(home_dir())
                    .and_then(|p| p.into_os_string().into_string().ok())
                    .unwrap_or_default(),
            ),
        }
    }
}
