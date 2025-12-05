use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use polars::frame::DataFrame;
use ratatui::widgets::Widget;

use crate::{
    handler::message::Message,
    misc::search::{self, Contain, Skim},
    tui::{component::Component, widgets::block::Block},
};

use super::widgets::input::Input;

#[derive(Debug)]
pub enum Searcher {
    Fuzzy(search::Search<Skim>),
    Exact(search::Search<Contain>),
}

impl Searcher {
    pub fn pattern(&self) -> &str {
        match self {
            Searcher::Fuzzy(search) => search.pattern(),
            Searcher::Exact(search) => search.pattern(),
        }
    }

    pub fn latest(&self) -> Option<DataFrame> {
        match self {
            Searcher::Fuzzy(search) => search.latest(),
            Searcher::Exact(search) => search.latest(),
        }
    }
}

#[derive(Debug)]
pub struct SearchBar {
    input: Input,
    searcher: Searcher,
    rollback_df: DataFrame,
}

impl SearchBar {
    pub fn exact(dataframe: DataFrame) -> Self {
        SearchBar {
            input: Default::default(),
            searcher: Searcher::Exact(search::Search::new(dataframe.clone(), Default::default())),
            rollback_df: dataframe,
        }
    }

    pub fn fuzzy(dataframe: DataFrame) -> Self {
        SearchBar {
            input: Default::default(),
            searcher: Searcher::Fuzzy(search::Search::new(dataframe.clone(), Default::default())),
            rollback_df: dataframe,
        }
    }

    pub fn searcher(&self) -> &Searcher {
        &self.searcher
    }

    // pub fn handle_key(&mut self, event: KeyEvent) {
    //     self.input.handle(event);
    //     self.update_search();
    // }

    pub fn into_rollback_df(self) -> DataFrame {
        self.rollback_df
    }

    fn update_search(&mut self) {
        if self.input.value() != self.searcher.pattern() {
            match self.searcher {
                Searcher::Fuzzy(_) => {
                    self.searcher = Searcher::Fuzzy(search::Search::new(
                        self.rollback_df.clone(),
                        self.input.value().to_owned(),
                    ))
                }
                Searcher::Exact(_) => {
                    self.searcher = Searcher::Exact(search::Search::new(
                        self.rollback_df.clone(),
                        self.input.value().to_owned(),
                    ))
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
        let title = match &self.searcher {
            Searcher::Fuzzy(_) => "Fuzzy Search",
            Searcher::Exact(_) => "Search",
        };
        let area = {
            let block = Block::default().title(title);
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

// #[derive(Debug)]
// pub struct SearchBar {
//     selection: bool,
// }

// impl SearchBar {
//     pub fn new() -> Self {
//         Self { selection: false }
//     }

//     pub fn with_selection(self, selection: bool) -> Self {
//         Self { selection }
//     }
// }

// impl Default for SearchBar {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl StatefulWidget for SearchBar {
//     type State = SearchBarState;

//     fn render(
//         self,
//         area: ratatui::prelude::Rect,
//         buf: &mut ratatui::prelude::Buffer,
//         state: &mut Self::State,
//     ) {
//         let title = match state.search {
//             Search::Fuzzy(_) => "Fuzzy Search",
//             Search::Exact(_) => "Search",
//         };
//         let area = {
//             let block = Block::default().title(title);
//             let inner = block.inner(area);
//             block.render(area, buf);
//             inner
//         };
//         state.input.render(
//             area,
//             buf,
//             if self.selection {
//                 FocusState::Focused
//             } else {
//                 FocusState::Unfocused
//             },
//         );
//     }
// }
