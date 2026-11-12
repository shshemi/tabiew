use crossterm::event::{KeyCode, KeyModifiers};

use crate::{
    handler::message::Message,
    tui::{component::Component, icons, popups::pickers::text_picker::TextPicker},
};

#[derive(Debug)]
pub struct TableRegisterer {
    picker: TextPicker,
}

impl Default for TableRegisterer {
    fn default() -> Self {
        Self {
            picker: TextPicker::default().with_title(icons::TAG.title("Name")),
        }
    }
}

impl TableRegisterer {
    fn register(&self) {
        Message::AppRegister(self.picker.value().trim().to_owned()).enqueue();
    }
}

impl Component for TableRegisterer {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
    ) {
        self.picker.render(area, buf, focus_state);
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        self.picker.handle(event)
            || match (event.code, event.modifiers) {
                (KeyCode::Enter, KeyModifiers::NONE) => {
                    self.register();
                    Message::PaneDismissModal.enqueue();
                    true
                }
                (KeyCode::Esc, KeyModifiers::NONE) => {
                    Message::PaneDismissModal.enqueue();
                    true
                }
                _ => false,
            }
    }
}
