use std::{borrow::Cow, path::PathBuf};

use crossterm::event::KeyEvent;
use home::home_dir;
use ratatui::widgets::StatefulWidget;

use crate::tui::pickers::text_picker::{TextPicker, TextPicker};

#[derive(Debug)]
pub struct PathPickerState {
    text_picker: TextPicker,
}

impl PathPickerState {
    pub fn path(&self) -> PathBuf {
        self.text_picker.input().value().into()
    }

    pub fn handle(&mut self, event: KeyEvent) {
        self.text_picker.input_mut().handle(event);
    }
}

impl Default for PathPickerState {
    fn default() -> Self {
        Self {
            text_picker: TextPicker::default().with_value(
                std::env::current_dir()
                    .ok()
                    .or(home_dir())
                    .and_then(|p| p.into_os_string().into_string().ok())
                    .unwrap_or_default(),
            ),
        }
    }
}

#[derive(Debug, Default)]
pub struct PathPicker<'a> {
    title: Option<Cow<'a, str>>,
    hint: Option<Cow<'a, str>>,
}

impl<'a> PathPicker<'a> {
    pub fn with_title(self, title: impl Into<Cow<'a, str>>) -> Self {
        Self {
            title: Some(title.into()),
            ..self
        }
    }

    pub fn with_hint(self, hint: impl Into<Cow<'a, str>>) -> Self {
        Self {
            hint: Some(hint.into()),
            ..self
        }
    }
}

impl<'a> StatefulWidget for PathPicker<'a> {
    type State = PathPickerState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        TextPicker::default()
            .title(self.title.unwrap_or(Cow::Borrowed("Output Path")))
            .hint(self.hint.unwrap_or_default())
            .render(area, buf, &mut state.text_picker);
    }
}
