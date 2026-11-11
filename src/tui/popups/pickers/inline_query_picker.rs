use crossterm::event::{KeyCode, KeyModifiers};
use polars::frame::DataFrame;

use crate::{
    handler::message::Message,
    misc::sql::sql,
    sql_completion::{self, SqlSuggestion},
    tui::{
        component::Component,
        pane::TableDescription,
        popups::pickers::text_picker_with_suggestion::{Provider, TextPickerWithSuggestion},
    },
};

#[derive(Debug)]
pub struct InlineQueryPicker {
    picker: TextPickerWithSuggestion<InlineQueryProvider>,
    dataframe: Option<DataFrame>,
    query_type: QueryType,
}

impl InlineQueryPicker {
    pub fn new(dataframe: DataFrame, query_type: QueryType) -> Self {
        let columns = dataframe
            .get_column_names()
            .into_iter()
            .map(|name| name.to_string())
            .collect();
        Self::with_columns(Some(dataframe), columns, query_type)
    }

    pub fn duckdb(columns: Vec<String>, query_type: QueryType) -> Self {
        Self::with_columns(None, columns, query_type)
    }

    fn with_columns(
        dataframe: Option<DataFrame>,
        columns: Vec<String>,
        query_type: QueryType,
    ) -> Self {
        let all_columns = sql_completion::collect_all_columns(Some(&columns));
        let provider = InlineQueryProvider {
            columns,
            query_type,
            all_columns,
        };
        Self {
            picker: TextPickerWithSuggestion::new(provider).with_title(query_type.title()),
            dataframe,
            query_type,
        }
    }

    fn submit(&self) {
        let value = self.picker.value();
        let Some(dataframe) = self.dataframe.clone() else {
            Message::PaneDismissModal.enqueue();
            Message::AppInlineQuery(self.query_type, value.to_owned()).enqueue();
            return;
        };
        let result = match self.query_type {
            QueryType::Select => {
                sql().execute_with_default(&format!("SELECT {value} FROM _"), dataframe)
            }
            QueryType::Filter => {
                sql().execute_with_default(&format!("SELECT * FROM _ where {value}"), dataframe)
            }
            QueryType::Order => {
                sql().execute_with_default(&format!("SELECT * FROM _ ORDER BY {value}"), dataframe)
            }
        };
        match (result, self.query_type) {
            (Ok(result_dataframe), QueryType::Select) => {
                Message::PaneDismissModal.enqueue();
                Message::PanePushDataFrame(
                    result_dataframe,
                    TableDescription::Select(value.to_owned()),
                )
                .enqueue();
                Message::AppShowToast(format!("Column selection '{value}' occurred")).enqueue();
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
            || match (event.code, event.modifiers) {
                (KeyCode::Tab, KeyModifiers::NONE) => {
                    self.picker.apply_selected_suggestion();
                    true
                }
                (KeyCode::Enter, KeyModifiers::NONE) => {
                    if self.picker.has_suggestions() {
                        self.picker.apply_selected_suggestion();
                    } else {
                        self.submit();
                    }
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

#[derive(Debug)]
struct InlineQueryProvider {
    columns: Vec<String>,
    query_type: QueryType,
    all_columns: Vec<String>,
}

impl Provider for InlineQueryProvider {
    type Suggestion = SqlSuggestion;

    fn suggestions(&self, value: &str, cursor: usize) -> Vec<SqlSuggestion> {
        sql_completion::suggestions(
            value,
            cursor,
            self.query_type.sql_prefix(),
            &self.all_columns,
            Some(&self.columns),
        )
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
