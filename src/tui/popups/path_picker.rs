use std::{borrow::Cow, path::PathBuf};

use ratatui::widgets::StatefulWidget;

use crate::tui::pickers::text_picker::{TextPicker, TextPickerState};

#[derive(Debug, Default)]
pub struct PathPickerState {
    text_picker: TextPickerState,
}

impl PathPickerState {
    pub fn path(&self) -> PathBuf {
        self.text_picker.input().value().into()
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
