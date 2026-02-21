use polars::frame::DataFrame;

use crate::{
    handler::message::Message,
    misc::sql::sql,
    tui::{
        component::Component,
        pane::TableDescription,
        pickers::text_picker_with_suggestion::TextPickerWithSuggestion,
        popups::sql_completion_provider::SqlCompletionProvider,
    },
};

#[derive(Debug)]
pub struct InlineQueryPicker {
    picker: TextPickerWithSuggestion<SqlCompletionProvider>,
}

impl InlineQueryPicker {
    pub fn new(dataframe: DataFrame, query_type: QueryType) -> Self {
        let provider =
            SqlCompletionProvider::new(query_type.sql_prefix(), Some(dataframe.clone()));

        let on_submit = Box::new(move |value: &str| {
            let result = match query_type {
                QueryType::Select => {
                    sql().execute(&format!("SELECT {value} FROM _"), dataframe.clone())
                }
                QueryType::Filter => sql().execute(
                    &format!("SELECT * FROM _ where {value}"),
                    dataframe.clone(),
                ),
                QueryType::Order => sql().execute(
                    &format!("SELECT * FROM _ ORDER BY {value}"),
                    dataframe.clone(),
                ),
            };
            match (result, query_type) {
                (Ok(result_dataframe), QueryType::Select) => {
                    Message::PaneDismissModal.enqueue();
                    Message::PanePushDataFrame(
                        result_dataframe,
                        TableDescription::Select(value.to_owned()),
                    )
                    .enqueue();
                    Message::AppShowToast(format!("Column selection '{value}' occured")).enqueue();
                }
                (Ok(result_dataframe), QueryType::Order) => {
                    Message::PaneDismissModal.enqueue();
                    Message::PanePushDataFrame(
                        result_dataframe,
                        TableDescription::Order(value.to_owned()),
                    )
                    .enqueue();
                    Message::AppShowToast(format!("Data frame ordered by '{value}'")).enqueue();
                }
                (Ok(result_dataframe), QueryType::Filter) => {
                    Message::PaneDismissModal.enqueue();
                    Message::PanePushDataFrame(
                        result_dataframe,
                        TableDescription::Filter(value.to_owned()),
                    )
                    .enqueue();
                    Message::AppShowToast(format!("Filter '{value}' applied")).enqueue();
                }
                (Err(error), _) => {
                    Message::PaneDismissModal.enqueue();
                    Message::AppShowError(error.to_string()).enqueue();
                }
            }
        });

        let on_dismiss = Box::new(|| {
            Message::PaneDismissModal.enqueue();
        });

        Self {
            picker: TextPickerWithSuggestion::new(
                query_type.title(),
                provider,
                on_submit,
                on_dismiss,
            ),
        }
    }
}

impl Component for InlineQueryPicker {
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

#[derive(Debug, Clone, Copy)]
pub enum QueryType {
    Select,
    Filter,
    Order,
}

impl QueryType {
    fn title(&self) -> &'static str {
        match self {
            QueryType::Select => "Select",
            QueryType::Filter => "Filter",
            QueryType::Order => "Order",
        }
    }

    /// SQL fragment prepended to the user's input so that the tokenizer sees
    /// the full clause context (e.g. a comma after `SELECT col1,` is
    /// recognised as being inside a SELECT clause).
    fn sql_prefix(&self) -> &'static str {
        match self {
            QueryType::Select => "SELECT ",
            QueryType::Filter => "SELECT * FROM _ WHERE ",
            QueryType::Order => "SELECT * FROM _ ORDER BY ",
        }
    }
}
