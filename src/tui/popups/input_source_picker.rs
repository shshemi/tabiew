use std::fmt::Display;

use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};

use crate::tui::{component::Component, pickers::list_picker::ListPicker};

#[derive(Debug)]
pub struct InputSourcePicker {
    list_picker: ListPicker<Source>,
}

impl InputSourcePicker {
    pub fn value(&self) -> Option<&Source> {
        self.list_picker.selected_item()
    }
}

impl Component for InputSourcePicker {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
    ) {
        self.list_picker.render(area, buf, focus_state);
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        self.list_picker.handle(event)
    }
}

impl Default for InputSourcePicker {
    fn default() -> Self {
        Self {
            list_picker: ListPicker::new(Source::iter().to_owned().collect())
                .with_title("Import Source"),
        }
    }
}

#[derive(Debug, Clone, Copy, IntoStaticStr, EnumIter)]
pub enum Source {
    File,
    Stdin,
}

impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(self))
    }
}
