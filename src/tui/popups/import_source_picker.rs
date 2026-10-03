use std::fmt::Display;

use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};

use crate::{
    readers::ReaderSource,
    tui::{component::Component, icons, popups::list_picker::ListPicker},
};

#[derive(Debug)]
pub struct ImportSourcePicker {
    list_picker: ListPicker<SourceType>,
}

impl ImportSourcePicker {
    pub fn value(&self) -> Option<&SourceType> {
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
            list_picker: ListPicker::new(SourceType::iter().collect())
                .with_title(icons::IMPORT.title("Import Source")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoStaticStr, EnumIter)]
pub enum SourceType {
    File,
    Stdin,
    Url,
}

impl SourceType {
    fn icon(&self) -> icons::Icon {
        match self {
            SourceType::File => icons::FILE,
            SourceType::Stdin => icons::TERMINAL,
            SourceType::Url => icons::GLOBE,
        }
    }
}

impl Display for SourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.icon().item(Into::<&str>::into(self)))
    }
}

impl From<&ReaderSource> for SourceType {
    fn from(r: &ReaderSource) -> Self {
        match r {
            ReaderSource::Stdin => SourceType::Stdin,
            ReaderSource::File(_) => SourceType::File,
        }
    }
}
