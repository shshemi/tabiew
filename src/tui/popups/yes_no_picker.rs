use std::fmt::Display;

use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};

use crate::tui::{component::Component, pickers::list_picker::ListPicker};

#[derive(Debug)]
pub struct YesNoPicker {
    list_picker: ListPicker<YesNo>,
}

impl YesNoPicker {
    pub fn with_title(self, title: impl Into<String>) -> Self {
        YesNoPicker {
            list_picker: self.list_picker.with_title(title),
        }
    }

    pub fn value(&self) -> Option<bool> {
        self.list_picker.selected_item().map(|yn| match yn {
            YesNo::Yes => true,
            YesNo::No => false,
        })
    }
}

impl Component for YesNoPicker {
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

impl Default for YesNoPicker {
    fn default() -> Self {
        Self {
            list_picker: ListPicker::new(YesNo::iter().to_owned().collect()),
        }
    }
}

#[derive(Debug, Clone, Copy, IntoStaticStr, EnumIter)]
pub enum YesNo {
    Yes,
    No,
}

impl Display for YesNo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(self))
    }
}
