use polars::frame::DataFrame;

use crate::{
    handler::message::Message,
    misc::sql::sql,
    tui::{
        component::Component,
        pickers::text_picker_with_suggestion::TextPickerWithSuggestion,
        popups::sql_completion_provider::SqlCompletionProvider,
    },
};

#[derive(Debug)]
pub struct SqlQueryPicker {
    picker: TextPickerWithSuggestion<SqlCompletionProvider>,
}

impl SqlQueryPicker {
    pub fn new(dataframe: Option<DataFrame>) -> Self {
        let provider = SqlCompletionProvider::new("", dataframe.clone());

        let captured_dataframe = dataframe;
        let on_submit = Box::new(move |value: &str| {
            Message::AppDismissOverlay.enqueue();
            match sql().execute(value, captured_dataframe.clone()) {
                Ok(result) => {
                    Message::TabsAddQueryPane(result, value.to_owned()).enqueue();
                }
                Err(error) => Message::AppShowError(error.to_string()).enqueue(),
            }
        });

        let on_dismiss = Box::new(|| {
            Message::AppDismissOverlay.enqueue();
        });

        Self {
            picker: TextPickerWithSuggestion::new("SQL", provider, on_submit, on_dismiss),
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
