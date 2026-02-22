use polars::frame::DataFrame;

use crate::{
    handler::message::Message,
    misc::sql::sql,
    sql_completion,
    tui::{
        component::Component,
        pickers::text_picker_with_suggestion::{SuggestionProvider, TextPickerWithSuggestion},
    },
};

#[derive(Debug)]
pub struct SqlQueryPicker {
    picker: TextPickerWithSuggestion<SqlQueryProvider>,
}

impl SqlQueryPicker {
    pub fn new(dataframe: Option<DataFrame>) -> Self {
        let all_columns = sql_completion::collect_all_columns(dataframe.as_ref());
        let provider = SqlQueryProvider {
            dataframe,
            all_columns,
        };
        Self {
            picker: TextPickerWithSuggestion::new("SQL", provider),
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
    }
}

#[derive(Debug)]
struct SqlQueryProvider {
    dataframe: Option<DataFrame>,
    all_columns: Vec<String>,
}

impl SuggestionProvider for SqlQueryProvider {
    fn suggestions(&self, value: &str, cursor: usize) -> Vec<String> {
        sql_completion::suggestions(value, cursor, "", &self.all_columns, self.dataframe.as_ref())
    }

    fn is_separator(&self, character: char) -> bool {
        sql_completion::is_separator(character)
    }

    fn on_submit(&self, value: &str) {
        Message::AppDismissOverlay.enqueue();
        match sql().execute(value, self.dataframe.clone()) {
            Ok(result) => {
                Message::TabsAddQueryPane(result, value.to_owned()).enqueue();
            }
            Err(error) => Message::AppShowError(error.to_string()).enqueue(),
        }
    }

    fn on_dismiss(&self) {
        Message::AppDismissOverlay.enqueue();
    }
}
