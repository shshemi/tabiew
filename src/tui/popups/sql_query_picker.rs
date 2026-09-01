use crossterm::event::{KeyCode, KeyModifiers};
use polars::frame::DataFrame;

use crate::{
    handler::message::Message,
    misc::{history::History, sql::sql},
    sql_completion::{self, SqlSuggestion},
    tui::{
        component::Component,
        pickers::text_picker_with_suggestion::{Provider, Suggestion, TextPickerWithSuggestion},
    },
};

static HISTORY: History<HistoryOrSqlSuggestion> = History::<HistoryOrSqlSuggestion>::new(24);

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
            history: HISTORY.to_vec(),
        };
        Self {
            picker: TextPickerWithSuggestion::new(provider).with_title("SQL"),
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
                        match sql().execute(value, self.dataframe.clone()) {
                            Ok(df) => {
                                if df.columns().is_empty() {
                                    Message::AppShowToast(
                                        "The query results in an empty data frame".to_owned(),
                                    )
                                    .enqueue()
                                } else {
                                    Message::AppDismissOverlay.enqueue();
                                    Message::TabsAddQueryPane(df, value.to_owned()).enqueue();
                                    HISTORY.push(HistoryOrSqlSuggestion::History {
                                        text: value.to_owned(),
                                    });
                                }
                            }
                            Err(error) => Message::AppShowToast(error.to_string()).enqueue(),
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
    history: Vec<HistoryOrSqlSuggestion>,
}

impl Provider for SqlQueryProvider {
    type Suggestion = HistoryOrSqlSuggestion;

    fn suggestions(&self, value: &str, cursor: usize) -> Vec<HistoryOrSqlSuggestion> {
        if value.is_empty() {
            self.history.clone()
        } else {
            sql_completion::suggestions(
                value,
                cursor,
                "",
                &self.all_columns,
                self.dataframe.as_ref(),
            )
            .into_iter()
            .map(|suggestion| HistoryOrSqlSuggestion::SqlSuggestion { suggestion })
            .collect()
        }
    }
}

#[derive(Debug, Clone)]
enum HistoryOrSqlSuggestion {
    SqlSuggestion { suggestion: SqlSuggestion },
    History { text: String },
}

impl Suggestion for HistoryOrSqlSuggestion {
    fn title(&self) -> &str {
        match self {
            HistoryOrSqlSuggestion::SqlSuggestion { suggestion } => suggestion.title(),
            HistoryOrSqlSuggestion::History { text } => text,
        }
    }

    fn apply_to(&self, input: &mut crate::tui::widgets::input::Input) {
        match self {
            HistoryOrSqlSuggestion::SqlSuggestion { suggestion } => suggestion.apply_to(input),
            HistoryOrSqlSuggestion::History { text } => {
                for c in text.chars() {
                    input.insert(c);
                }
            }
        }
    }
}
