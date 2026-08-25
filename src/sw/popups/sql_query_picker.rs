use std::borrow::Cow;

use polars::frame::DataFrame;
use ratatui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};

use crate::{
    sql_completion::{self, SqlSuggestion},
    sw::{
        pickers::text_picker_with_suggestion::{
            Provider, Suggestion, TextPickerWithSuggestion, TextPickerWithSuggestionState,
        },
        widgets::input::InputState,
    },
};

#[derive(Debug, Clone)]
pub enum SqlQuerySuggestion {
    Completion(SqlSuggestion),
    History(String),
}

impl Suggestion for SqlQuerySuggestion {
    fn title(&self) -> &str {
        match self {
            SqlQuerySuggestion::Completion(suggestion) => suggestion.title(),
            SqlQuerySuggestion::History(text) => text,
        }
    }

    fn apply_to(&self, input: &mut InputState) {
        match self {
            SqlQuerySuggestion::Completion(suggestion) => suggestion.apply_to(input),
            SqlQuerySuggestion::History(text) => {
                for character in text.chars() {
                    input.insert(character);
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct SqlQueryProvider {
    dataframe: Option<DataFrame>,
    all_columns: Vec<String>,
    history: Vec<String>,
}

impl Provider for SqlQueryProvider {
    type Suggestion = SqlQuerySuggestion;

    fn suggestions(&self, value: &str, cursor: usize) -> Vec<SqlQuerySuggestion> {
        if value.is_empty() {
            self.history
                .iter()
                .cloned()
                .map(SqlQuerySuggestion::History)
                .collect()
        } else {
            sql_completion::suggestions(
                value,
                cursor,
                "",
                &self.all_columns,
                self.dataframe.as_ref(),
            )
            .into_iter()
            .map(SqlQuerySuggestion::Completion)
            .collect()
        }
    }
}

#[derive(Debug)]
pub struct SqlQueryPickerState {
    picker: TextPickerWithSuggestionState<SqlQueryProvider>,
}

impl SqlQueryPickerState {
    pub fn new(dataframe: Option<DataFrame>, history: Vec<String>) -> Self {
        let all_columns = sql_completion::collect_all_columns(dataframe.as_ref());

        Self {
            picker: TextPickerWithSuggestionState::new(SqlQueryProvider {
                dataframe,
                all_columns,
                history,
            }),
        }
    }

    pub fn picker(&self) -> &TextPickerWithSuggestionState<SqlQueryProvider> {
        &self.picker
    }

    pub fn picker_mut(&mut self) -> &mut TextPickerWithSuggestionState<SqlQueryProvider> {
        &mut self.picker
    }

    pub fn input(&self) -> &InputState {
        self.picker.input()
    }

    pub fn input_mut(&mut self) -> &mut InputState {
        self.picker.input_mut()
    }

    pub fn value(&self) -> &str {
        self.picker.value()
    }
}

#[derive(Debug)]
pub struct SqlQueryPicker<'a> {
    title: Cow<'a, str>,
}

impl<'a> SqlQueryPicker<'a> {
    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self {
        self.title = title.into();
        self
    }
}

impl Default for SqlQueryPicker<'_> {
    fn default() -> Self {
        Self {
            title: Cow::Borrowed("SQL"),
        }
    }
}

impl StatefulWidget for SqlQueryPicker<'_> {
    type State = SqlQueryPickerState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        TextPickerWithSuggestion::default()
            .title(self.title)
            .render(area, buf, &mut state.picker);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use polars::prelude::Column;

    fn frame() -> DataFrame {
        DataFrame::new_infer_height(vec![
            Column::new("alpha".into(), vec![1i64, 2, 3]),
            Column::new("beta".into(), vec![4i64, 5, 6]),
        ])
        .unwrap()
    }

    fn state(history: Vec<String>) -> SqlQueryPickerState {
        SqlQueryPickerState::new(Some(frame()), history)
    }

    fn typed(state: &mut SqlQueryPickerState, text: &str) {
        for character in text.chars() {
            state.input_mut().insert(character);
        }
    }

    fn titles(state: &SqlQueryPickerState) -> Vec<&str> {
        state
            .picker()
            .suggestions()
            .iter()
            .map(Suggestion::title)
            .collect_vec()
    }

    fn render(state: &mut SqlQueryPickerState, picker: SqlQueryPicker) -> Buffer {
        let area = Rect::new(0, 0, 100, 30);
        let mut buf = Buffer::empty(area);
        picker.render(area, &mut buf, state);
        buf
    }

    fn content(buf: &Buffer) -> String {
        buf.content().iter().map(|c| c.symbol()).collect()
    }

    mod state {
        use super::*;

        #[test]
        fn an_empty_query_offers_the_history() {
            let state = state(vec!["SELECT 1".to_owned(), "SELECT 2".to_owned()]);

            assert_eq!(titles(&state), vec!["SELECT 1", "SELECT 2"]);
        }

        #[test]
        fn an_empty_query_with_no_history_offers_nothing() {
            let state = state(Vec::new());

            assert!(!state.picker().has_suggestions());
        }

        #[test]
        fn typing_switches_from_history_to_completions() {
            let mut state = state(vec!["SELECT 1".to_owned()]);
            typed(&mut state, "SEL");
            state.picker_mut().refresh_suggestions();

            assert!(!titles(&state).contains(&"SELECT 1"));
            assert!(titles(&state).iter().any(|title| title.contains("SELECT")));
        }

        #[test]
        fn column_names_are_completed() {
            let mut state = state(Vec::new());
            typed(&mut state, "SELECT al");
            state.picker_mut().refresh_suggestions();

            assert!(titles(&state).contains(&"alpha"));
        }

        #[test]
        fn applying_a_history_entry_types_it_out() {
            let mut state = state(vec!["SELECT 42".to_owned()]);
            state.picker_mut().select(Some(0));
            state.picker_mut().apply_selected_suggestion();

            assert_eq!(state.value(), "SELECT 42");
        }

        #[test]
        fn applying_a_completion_replaces_the_partial_token() {
            let mut state = state(Vec::new());
            typed(&mut state, "SELECT al");
            state.picker_mut().refresh_suggestions();
            let idx = titles(&state)
                .iter()
                .position(|title| *title == "alpha")
                .unwrap();
            state.picker_mut().select(Some(idx));
            state.picker_mut().apply_selected_suggestion();

            assert_eq!(state.value(), "SELECT alpha ");
        }

        #[test]
        fn a_picker_without_a_dataframe_still_completes_keywords() {
            let mut state = SqlQueryPickerState::new(None, Vec::new());
            typed(&mut state, "SEL");
            state.picker_mut().refresh_suggestions();

            assert!(titles(&state).iter().any(|title| title.contains("SELECT")));
        }
    }

    mod widget {
        use super::*;

        #[test]
        fn defaults_to_a_sql_title() {
            let buf = render(&mut state(Vec::new()), SqlQueryPicker::default());

            assert!(content(&buf).contains("SQL"));
        }

        #[test]
        fn the_title_is_overridable() {
            let buf = render(
                &mut state(Vec::new()),
                SqlQueryPicker::default().title("Query"),
            );

            assert!(content(&buf).contains("Query"));
        }

        #[test]
        fn renders_the_typed_query() {
            let mut state = state(Vec::new());
            typed(&mut state, "SELECT alpha");
            let buf = render(&mut state, SqlQueryPicker::default());

            assert!(content(&buf).contains("SELECT alpha"));
        }

        #[test]
        fn renders_the_history_when_empty() {
            let mut state = state(vec!["SELECT 99".to_owned()]);
            let buf = render(&mut state, SqlQueryPicker::default());

            assert!(content(&buf).contains("SELECT 99"));
        }
    }
}
