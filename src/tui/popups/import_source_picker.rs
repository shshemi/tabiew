use std::fmt::Display;

use strum::IntoEnumIterator;

use crate::{
    io::ResourceType,
    tui::{component::Component, pickers::list_picker::ListPicker},
};

#[derive(Debug)]
pub struct ImportSourcePicker {
    list_picker: ListPicker<ResourceType>,
}

impl ImportSourcePicker {
    pub fn value(&self) -> Option<&ResourceType> {
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
            list_picker: ListPicker::new(ResourceType::iter().to_owned().collect())
                .with_title("Import Source"),
        }
    }
}

impl Display for ResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(self))
    }
}
