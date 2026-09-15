use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use polars::frame::DataFrame;
use ratatui::widgets::{Block, Widget};

use crate::{
    handler::message::Message,
    misc::search::Searcher,
    tui::{
        app_default::{AppDefault, AppTitle},
        component::Component,
        icons,
    },
};

use super::widgets::input::Input;

#[derive(Debug)]
pub enum SearchType {
    Fuzzy,
    Exact,
}

#[derive(Debug)]
pub struct SearchBar {
    input: Input,
    search_type: SearchType,
    searcher: Searcher,
    rollback_df: DataFrame,
}

impl SearchBar {
    pub fn exact(dataframe: DataFrame) -> Self {
        SearchBar {
            input: Default::default(),
            search_type: SearchType::Exact,
            searcher: Searcher::exact(dataframe.clone(), Default::default()),
            rollback_df: dataframe,
        }
    }

    pub fn fuzzy(dataframe: DataFrame) -> Self {
        SearchBar {
            input: Default::default(),
            search_type: SearchType::Fuzzy,
            searcher: Searcher::fuzzy(dataframe.clone(), Default::default()),
            rollback_df: dataframe,
        }
    }

    pub fn searcher(&self) -> &Searcher {
        &self.searcher
    }

    pub fn search_type(&self) -> &SearchType {
        &self.search_type
    }

    pub fn into_rollback_df(self) -> DataFrame {
        self.rollback_df
    }

    pub fn value(&self) -> &str {
        self.input.value()
    }

    fn update_search(&mut self) {
        if self.input.value() != self.searcher.pattern() {
            match self.search_type {
                SearchType::Fuzzy => {
                    self.searcher =
                        Searcher::fuzzy(self.rollback_df.clone(), self.input.value().to_owned())
                }
                SearchType::Exact => {
                    self.searcher =
                        Searcher::exact(self.rollback_df.clone(), self.input.value().to_owned())
                }
            }
        }
    }
}

impl Component for SearchBar {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: super::component::FocusState,
    ) {
        let title = match &self.search_type {
            SearchType::Fuzzy => icons::FUZZY_SEARCH.title("Fuzzy Search"),
            SearchType::Exact => icons::SEARCH.title("Search"),
        };
        let area = {
            let block = Block::app_default().app_title(title);
            let inner = block.inner(area);
            block.render(area, buf);
            inner
        };
        self.input.render(area, buf, focus_state);
    }

    fn handle(&mut self, event: KeyEvent) -> bool {
        if self.input.handle(event) {
            self.update_search();
            true
        } else {
            match (event.code, event.modifiers) {
                (KeyCode::Esc, KeyModifiers::NONE) => {
                    Message::PaneDismissModal.enqueue();
                    Message::PanePopDataFrame.enqueue();
                    true
                }
                (KeyCode::Enter, KeyModifiers::NONE) => {
                    Message::PaneDismissModal.enqueue();
                    true
                }
                _ => false,
            }
        }
    }
}
