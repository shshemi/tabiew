use std::path::PathBuf;

use crossterm::event::KeyEvent;
use home::home_dir;

use crate::tui::{component::Component, pickers::text_picker::TextPicker};

#[derive(Debug)]
pub struct PathPicker {
    text_picker: TextPicker,
}

impl PathPicker {
    pub fn path(&self) -> PathBuf {
        self.text_picker.input().value().into()
    }
}

impl Component for PathPicker {
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

impl Default for PathPicker {
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

// #[derive(Debug, Default)]
// pub struct PathPicker<'a> {
//     title: Option<Cow<'a, str>>,
//     hint: Option<Cow<'a, str>>,
// }

// impl<'a> PathPicker<'a> {
//     pub fn with_title(self, title: impl Into<Cow<'a, str>>) -> Self {
//         Self {
//             title: Some(title.into()),
//             ..self
//         }
//     }

//     pub fn with_hint(self, hint: impl Into<Cow<'a, str>>) -> Self {
//         Self {
//             hint: Some(hint.into()),
//             ..self
//         }
//     }
// }

// impl<'a> StatefulWidget for PathPicker<'a> {
//     type State = PathPickerState;

//     fn render(
//         self,
//         area: ratatui::prelude::Rect,
//         buf: &mut ratatui::prelude::Buffer,
//         state: &mut Self::State,
//     ) {
//         TextPicker::default()
//             .title(self.title.unwrap_or(Cow::Borrowed("Output Path")))
//             .hint(self.hint.unwrap_or_default())
//             .render(area, buf, &mut state.text_picker);
//     }
// }
