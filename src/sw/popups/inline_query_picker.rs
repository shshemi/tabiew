use polars::frame::DataFrame;
use ratatui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};

use crate::{
    sql_completion::{self, SqlSuggestion},
    sw::{
        pickers::text_picker_with_suggestion::{
            Provider, TextPickerWithSuggestion, TextPickerWithSuggestionState,
        },
        widgets::input::InputState,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryType {
    Select,
    Filter,
    Order,
}

impl QueryType {
    pub fn title(&self) -> &'static str {
        match self {
            QueryType::Select => "Select",
            QueryType::Filter => "Filter",
            QueryType::Order => "Order",
        }
    }

    pub fn sql_prefix(&self) -> &'static str {
        match self {
            QueryType::Select => "SELECT ",
            QueryType::Filter => "SELECT * FROM _ WHERE ",
            QueryType::Order => "SELECT * FROM _ ORDER BY ",
        }
    }

    pub fn query(&self, value: &str) -> String {
        match self {
            QueryType::Select => format!("SELECT {value} FROM _"),
            QueryType::Filter => format!("SELECT * FROM _ where {value}"),
            QueryType::Order => format!("SELECT * FROM _ ORDER BY {value}"),
        }
    }
}

#[derive(Debug)]
pub struct InlineQueryProvider {
    dataframe: DataFrame,
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
            Some(&self.dataframe),
        )
    }
}

#[derive(Debug)]
pub struct InlineQueryPickerState {
    picker: TextPickerWithSuggestionState<InlineQueryProvider>,
    dataframe: DataFrame,
    query_type: QueryType,
}

impl InlineQueryPickerState {
    pub fn new(dataframe: DataFrame, query_type: QueryType) -> Self {
        let all_columns = sql_completion::collect_all_columns(Some(&dataframe));

        Self {
            picker: TextPickerWithSuggestionState::new(InlineQueryProvider {
                dataframe: dataframe.clone(),
                query_type,
                all_columns,
            }),
            dataframe,
            query_type,
        }
    }

    pub fn picker(&self) -> &TextPickerWithSuggestionState<InlineQueryProvider> {
        &self.picker
    }

    pub fn picker_mut(&mut self) -> &mut TextPickerWithSuggestionState<InlineQueryProvider> {
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

    pub fn dataframe(&self) -> &DataFrame {
        &self.dataframe
    }

    pub fn query_type(&self) -> QueryType {
        self.query_type
    }

    pub fn query(&self) -> String {
        self.query_type.query(self.value())
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct InlineQueryPicker;

impl StatefulWidget for InlineQueryPicker {
    type State = InlineQueryPickerState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        TextPickerWithSuggestion::default()
            .title(state.query_type.title())
            .render(area, buf, &mut state.picker);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sw::pickers::text_picker_with_suggestion::Suggestion;
    use itertools::Itertools;
    use polars::prelude::Column;

    fn frame() -> DataFrame {
        DataFrame::new_infer_height(vec![
            Column::new("alpha".into(), vec![1i64, 2, 3]),
            Column::new("beta".into(), vec![4i64, 5, 6]),
        ])
        .unwrap()
    }

    fn state(query_type: QueryType) -> InlineQueryPickerState {
        InlineQueryPickerState::new(frame(), query_type)
    }

    fn typed(state: &mut InlineQueryPickerState, text: &str) {
        for character in text.chars() {
            state.input_mut().insert(character);
        }
    }

    fn titles(state: &InlineQueryPickerState) -> Vec<&str> {
        state
            .picker()
            .suggestions()
            .iter()
            .map(Suggestion::title)
            .collect_vec()
    }

    fn render(state: &mut InlineQueryPickerState) -> Buffer {
        let area = Rect::new(0, 0, 100, 30);
        let mut buf = Buffer::empty(area);
        InlineQueryPicker.render(area, &mut buf, state);
        buf
    }

    fn content(buf: &Buffer) -> String {
        buf.content().iter().map(|c| c.symbol()).collect()
    }

    mod query {
        use super::*;

        #[test]
        fn select_projects_the_given_columns() {
            let mut state = state(QueryType::Select);
            typed(&mut state, "alpha, beta");

            assert_eq!(state.query(), "SELECT alpha, beta FROM _");
        }

        #[test]
        fn filter_becomes_a_where_clause() {
            let mut state = state(QueryType::Filter);
            typed(&mut state, "alpha > 1");

            assert_eq!(state.query(), "SELECT * FROM _ where alpha > 1");
        }

        #[test]
        fn order_becomes_an_order_by_clause() {
            let mut state = state(QueryType::Order);
            typed(&mut state, "beta DESC");

            assert_eq!(state.query(), "SELECT * FROM _ ORDER BY beta DESC");
        }

        #[test]
        fn an_empty_input_still_builds_a_query() {
            let state = state(QueryType::Select);

            assert_eq!(state.query(), "SELECT  FROM _");
        }

        #[test]
        fn each_query_type_has_its_own_completion_prefix() {
            assert_eq!(QueryType::Select.sql_prefix(), "SELECT ");
            assert_eq!(QueryType::Filter.sql_prefix(), "SELECT * FROM _ WHERE ");
            assert_eq!(QueryType::Order.sql_prefix(), "SELECT * FROM _ ORDER BY ");
        }
    }

    mod state {
        use super::*;

        #[test]
        fn the_query_type_is_kept() {
            assert_eq!(state(QueryType::Filter).query_type(), QueryType::Filter);
        }

        #[test]
        fn the_dataframe_is_kept_for_execution() {
            assert_eq!(state(QueryType::Select).dataframe().width(), 2);
        }

        #[test]
        fn column_names_are_completed() {
            let mut state = state(QueryType::Select);
            typed(&mut state, "al");
            state.picker_mut().refresh_suggestions();

            assert!(titles(&state).contains(&"alpha"));
        }

        #[test]
        fn completions_are_offered_for_a_filter_too() {
            let mut state = state(QueryType::Filter);
            typed(&mut state, "bet");
            state.picker_mut().refresh_suggestions();

            assert!(titles(&state).contains(&"beta"));
        }

        #[test]
        fn applying_a_completion_replaces_the_partial_token() {
            let mut state = state(QueryType::Select);
            typed(&mut state, "al");
            state.picker_mut().refresh_suggestions();
            let idx = titles(&state)
                .iter()
                .position(|title| *title == "alpha")
                .unwrap();
            state.picker_mut().select(Some(idx));
            state.picker_mut().apply_selected_suggestion();

            assert_eq!(state.value(), "alpha ");
            assert_eq!(state.query(), "SELECT alpha  FROM _");
        }
    }

    mod widget {
        use super::*;

        #[test]
        fn the_title_names_the_query_type() {
            assert!(content(&render(&mut state(QueryType::Select))).contains("Select"));
            assert!(content(&render(&mut state(QueryType::Filter))).contains("Filter"));
            assert!(content(&render(&mut state(QueryType::Order))).contains("Order"));
        }

        #[test]
        fn renders_the_typed_expression() {
            let mut state = state(QueryType::Filter);
            typed(&mut state, "alpha > 1");
            let buf = render(&mut state);

            assert!(content(&buf).contains("alpha > 1"));
        }

        #[test]
        fn renders_the_completions() {
            let mut state = state(QueryType::Select);
            typed(&mut state, "al");
            let buf = render(&mut state);

            assert!(content(&buf).contains("alpha"));
        }
    }
}
