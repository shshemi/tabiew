use polars::frame::DataFrame;
use ratatui::widgets::{Block, StatefulWidget};

use crate::misc::{globals::theme, search::Search};

use super::input::{Input, InputState};

#[derive(Debug)]
pub struct SearchBarState {
    input: InputState,
    search: Search,
    rollback_df: DataFrame,
}

impl SearchBarState {
    pub fn new(dataframe: DataFrame) -> Self {
        SearchBarState {
            input: Default::default(),
            search: Search::new(dataframe.clone(), Default::default()),
            rollback_df: dataframe,
        }
    }

    pub fn search(&self) -> &Search {
        &self.search
    }

    pub fn insert(&mut self, c: char) {
        self.input.insert(c);
        if self.input.value() != self.search.pattern() {
            self.search = Search::new(self.rollback_df.clone(), self.input.value().to_owned())
        }
    }

    pub fn delete_prev(&mut self) {
        self.input.delete_prev();
        if self.input.value() != self.search.pattern() {
            self.search = Search::new(self.rollback_df.clone(), self.input.value().to_owned())
        }
    }

    pub fn delete_next(&mut self) {
        self.input.delete_next();
        if self.input.value() != self.search.pattern() {
            self.search = Search::new(self.rollback_df.clone(), self.input.value().to_owned())
        }
    }

    pub fn goto_prev(&mut self) {
        self.input.goto_prev();
    }

    pub fn goto_next(&mut self) {
        self.input.goto_next();
    }

    pub fn goto_start(&mut self) {
        self.input.goto_start();
    }

    pub fn goto_end(&mut self) {
        self.input.goto_end();
    }

    pub fn into_rollback_df(self) -> DataFrame {
        self.rollback_df
    }
}

#[derive(Debug)]
pub struct SearchBar {
    selection: bool,
}

impl SearchBar {
    pub fn new() -> Self {
        Self { selection: false }
    }

    pub fn with_selection(self, selection: bool) -> Self {
        Self { selection }
    }
}

impl Default for SearchBar {
    fn default() -> Self {
        Self::new()
    }
}

impl StatefulWidget for SearchBar {
    type State = SearchBarState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        Input::new()
            .style(theme().text())
            .selection(self.selection)
            .block(
                Block::bordered()
                    .title_top("Fuzzy Search")
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .style(theme().block()),
            )
            .render(area, buf, &mut state.input);
    }
}
