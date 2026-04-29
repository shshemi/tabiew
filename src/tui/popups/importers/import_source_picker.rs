use std::fmt::Display;

use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};

use crate::{
    io::DataSource,
    tui::{component::Component, pickers::list_picker::ListPicker},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoStaticStr, EnumIter)]
pub enum ImportSource {
    File,
    Stdin,
}

impl Display for ImportSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(self))
    }
}

impl From<&DataSource> for ImportSource {
    fn from(r: &DataSource) -> Self {
        match r {
            DataSource::Stdin => ImportSource::Stdin,
            DataSource::File(_) | DataSource::Url(_) => ImportSource::File,
        }
    }
}

#[derive(Debug)]
pub struct ImportSourcePicker {
    list_picker: ListPicker<ImportSource>,
}

impl ImportSourcePicker {
    pub fn value(&self) -> Option<&ImportSource> {
        self.list_picker.selected_item()
    }
}

impl Component for ImportSourcePicker {
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

impl Default for ImportSourcePicker {
    fn default() -> Self {
        Self {
            list_picker: ListPicker::new(ImportSource::iter().collect())
                .with_title("Import Source"),
        }
    }
}
