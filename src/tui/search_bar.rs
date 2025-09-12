use polars::frame::DataFrame;
use ratatui::widgets::StatefulWidget;

use crate::{
    misc::{
        globals::theme,
        search::{self, Contain, Skim},
    },
    tui::widgets::block::Block,
};

use super::widgets::input::{Input, InputState};

#[derive(Debug)]
pub enum Search {
    Fuzzy(search::Search<Skim>),
    Exact(search::Search<Contain>),
}

impl Search {
    pub fn pattern(&self) -> &str {
        match self {
            Search::Fuzzy(search) => search.pattern(),
            Search::Exact(search) => search.pattern(),
        }
    }

    pub fn latest(&self) -> Option<DataFrame> {
        match self {
            Search::Fuzzy(search) => search.latest(),
            Search::Exact(search) => search.latest(),
        }
    }
}

#[derive(Debug)]
pub struct SearchBarState {
    input: InputState,
    search: Search,
    rollback_df: DataFrame,
}

impl SearchBarState {
    pub fn exact(dataframe: DataFrame) -> Self {
        SearchBarState {
            input: Default::default(),
            search: Search::Exact(search::Search::new(dataframe.clone(), Default::default())),
            rollback_df: dataframe,
        }
    }

    pub fn fuzzy(dataframe: DataFrame) -> Self {
        SearchBarState {
            input: Default::default(),
            search: Search::Fuzzy(search::Search::new(dataframe.clone(), Default::default())),
            rollback_df: dataframe,
        }
    }

    pub fn search(&self) -> &Search {
        &self.search
    }

    pub fn insert(&mut self, c: char) {
        self.input.insert(c);
        self.update_search();
    }

    pub fn delete_prev(&mut self) {
        self.input.delete_prev();
        self.update_search();
    }

    pub fn delete_next(&mut self) {
        self.input.delete_next();
        self.update_search();
    }

    pub fn delete_prev_word(&mut self) {
        self.input.delete_prev_word();
        self.update_search();
    }

    pub fn delete_next_word(&mut self) {
        self.input.delete_next_word();
        self.update_search();
    }

    pub fn goto_prev(&mut self) {
        self.input.goto_prev();
    }

    pub fn goto_next(&mut self) {
        self.input.goto_next();
    }

    pub fn goto_prev_word(&mut self) {
        self.input.goto_prev_word();
    }

    pub fn goto_next_word(&mut self) {
        self.input.goto_next_word();
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

    fn update_search(&mut self) {
        if self.input.value() != self.search.pattern() {
            match self.search {
                Search::Fuzzy(_) => {
                    self.search = Search::Fuzzy(search::Search::new(
                        self.rollback_df.clone(),
                        self.input.value().to_owned(),
                    ))
                }
                Search::Exact(_) => {
                    self.search = Search::Exact(search::Search::new(
                        self.rollback_df.clone(),
                        self.input.value().to_owned(),
                    ))
                }
            }
        }
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
        let title = match state.search {
            Search::Fuzzy(_) => "Fuzzy Search",
            Search::Exact(_) => "Search",
        };
        Input::default()
            .style(theme().text())
            .selection(self.selection)
            .block(Block::default().title(title))
            .render(area, buf, &mut state.input);
    }
}
