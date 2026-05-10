use crossterm::event::{KeyCode, KeyModifiers};
use polars::frame::DataFrame;

use crate::{
    handler::message::Message,
    misc::sql::sql,
    sql_completion::{self, SqlSuggestion},
    tui::{
        component::Component,
        pickers::text_picker_with_suggestion::{Provider, TextPickerWithSuggestion},
    },
};

#[derive(Debug)]
pub struct SqlQueryPicker {
    picker: TextPickerWithSuggestion<SqlQueryProvider>,
    dataframe: Option<DataFrame>,
}

impl SqlQueryPicker {
    pub fn new(dataframe: Option<DataFrame>) -> Self {
        let all_columns = sql_completion::collect_all_columns(dataframe.as_ref());
        let provider = SqlQueryProvider {
            dataframe: dataframe.clone(),
            all_columns,
        };
        Self {
            picker: TextPickerWithSuggestion::new("SQL", provider),
            dataframe,
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
        self.picker.render(area, buf, focus_state);
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        self.picker.handle(event)
            || match (event.code, event.modifiers) {
                (KeyCode::Tab, KeyModifiers::NONE) => {
                    self.picker.apply_selected_suggestion();
                    true
                }
                (KeyCode::Enter, KeyModifiers::NONE) => {
                    if self.picker.has_suggestions() {
                        self.picker.apply_selected_suggestion();
                    } else {
                        let value = self.picker.value();
                        Message::AppDismissOverlay.enqueue();
                        match sql().execute(value, self.dataframe.clone()) {
                            Ok(result) => {
                                Message::TabsAddQueryPane(result, value.to_owned()).enqueue();
                            }
                            Err(error) => Message::AppShowError(error.to_string()).enqueue(),
                        }
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

#[derive(Debug)]
struct SqlQueryProvider {
    dataframe: Option<DataFrame>,
    all_columns: Vec<String>,
}

impl Provider for SqlQueryProvider {
    type Suggestion = SqlSuggestion;

    fn suggestions(&self, value: &str, cursor: usize) -> Vec<SqlSuggestion> {
        sql_completion::suggestions(
            value,
            cursor,
            "",
            &self.all_columns,
            self.dataframe.as_ref(),
        )
    }
}
