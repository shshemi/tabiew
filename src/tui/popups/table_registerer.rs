use crossterm::event::{KeyCode, KeyModifiers};
use polars::frame::DataFrame;

use crate::{
    handler::message::Message,
    misc::{globals::sql, sql::Source},
    tui::{component::Component, pickers::text_picker::TextPicker},
};

#[derive(Debug)]
pub struct TableRegisterer {
    df: DataFrame,
    picker: TextPicker,
}

impl TableRegisterer {
    pub fn new(df: DataFrame) -> Self {
        Self {
            df,
            picker: TextPicker::default().with_title("Name"),
        }
    }

    fn register(&self) {
        let name = self.picker.value().trim();
        if name.is_empty() {
            Message::AppShowError(format!("'{name}' is not a valid name")).enqueue();
        } else if sql().schema().get(name).is_some() {
            Message::AppShowError(format!("Table name '{name}' already exists in the backed"))
                .enqueue();
        } else {
            sql().register(name, self.df.clone(), Source::User);
        }
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
