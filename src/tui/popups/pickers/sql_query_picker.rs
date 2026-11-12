use crossterm::event::{KeyCode, KeyModifiers};

use crate::{
    collections::capped_vec::CappedVec,
    handler::message::Message,
    sql_completion::{self, SqlSuggestion},
    tui::{
        component::Component,
        icons,
        popups::pickers::text_picker_with_suggestion::{
            Provider, Suggestion, TextPickerWithSuggestion,
        },
    },
};

static HISTORY: CappedVec<HistoryOrSqlSuggestion> = CappedVec::<HistoryOrSqlSuggestion>::new(24);

#[derive(Debug)]
pub struct SqlQueryPicker {
    picker: TextPickerWithSuggestion<SqlQueryProvider>,
}

pub fn remember(query: &str) {
    HISTORY.push(HistoryOrSqlSuggestion::History {
        text: query.to_owned(),
    });
}

impl SqlQueryPicker {
    pub fn duckdb(columns: Option<Vec<String>>) -> Self {
        let all_columns = sql_completion::collect_all_columns(columns.as_deref());
        let provider = SqlQueryProvider {
            columns,
            all_columns,
            history: HISTORY.to_vec(),
        };
        Self {
            picker: TextPickerWithSuggestion::new(provider)
                .with_title(icons::DATABASE.title("SQL")),
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
                        Message::AppQuery(self.picker.value().to_owned()).enqueue();
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
    columns: Option<Vec<String>>,
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
                self.columns.as_deref(),
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
