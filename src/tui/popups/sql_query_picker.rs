use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use polars::frame::DataFrame;

use crate::{
    handler::message::Message,
    misc::globals::sql,
    tui::{component::Component, pickers::text_picker::TextPicker},
};

#[derive(Debug)]
pub struct SqlQueryPicker {
    df: Option<DataFrame>,
    text_picker: TextPicker,
}

impl SqlQueryPicker {
    pub fn new(df: Option<DataFrame>) -> Self {
        Self {
            df,
            text_picker: TextPicker::default().with_title("Sql"),
        }
    }
}

impl Component for SqlQueryPicker {
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
            || match (event.code, event.modifiers) {
                (KeyCode::Enter, KeyModifiers::NONE) => {
                    Message::AppDismissOverlay.enqueue();
                    match sql().execute(self.text_picker.value(), self.df.clone()) {
                        Ok(df) => {
                            Message::TabsAddQueryPane(df, self.text_picker.value().to_owned())
                                .enqueue();
                        }
                        Err(err) => Message::AppShowError(err.to_string()).enqueue(),
                    }
                    true
                }
                (KeyCode::Esc, KeyModifiers::NONE) => {
                    Message::AppDismissOverlay.enqueue();
                    true
                }
                _ => false,
            }
    }
}
