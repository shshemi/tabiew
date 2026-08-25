use polars::frame::DataFrame;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, StatefulWidget},
};

use crate::{
    misc::search::{self, Contain, Skim},
    sw::{
        app_default::AppDefault,
        widgets::input::{Input, InputState},
    },
};

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

    fn title(&self) -> &'static str {
        match self {
            Searcher::Fuzzy(_) => "Fuzzy Search",
            Searcher::Exact(_) => "Search",
        }
    }

    fn restart(&self, df: DataFrame, pattern: String) -> Self {
        match self {
            Searcher::Fuzzy(_) => Searcher::Fuzzy(search::Search::new(df, pattern)),
            Searcher::Exact(_) => Searcher::Exact(search::Search::new(df, pattern)),
        }
    }
}

#[derive(Debug)]
pub struct SearchBarState {
    input: InputState,
    searcher: Searcher,
    rollback_df: DataFrame,
}

impl SearchBarState {
    pub fn exact(df: DataFrame) -> Self {
        Self {
            input: InputState::default(),
            searcher: Searcher::Exact(search::Search::new(df.clone(), String::default())),
            rollback_df: df,
        }
    }

    pub fn fuzzy(df: DataFrame) -> Self {
        Self {
            input: InputState::default(),
            searcher: Searcher::Fuzzy(search::Search::new(df.clone(), String::default())),
            rollback_df: df,
        }
    }

    pub fn input(&self) -> &InputState {
        &self.input
    }

    pub fn input_mut(&mut self) -> &mut InputState {
        &mut self.input
    }

    pub fn value(&self) -> &str {
        self.input.value()
    }

    pub fn searcher(&self) -> &Searcher {
        &self.searcher
    }

    pub fn latest(&self) -> Option<DataFrame> {
        self.searcher.latest()
    }

    pub fn rollback_df(&self) -> &DataFrame {
        &self.rollback_df
    }

    pub fn into_rollback_df(self) -> DataFrame {
        self.rollback_df
    }

    pub fn sync(&mut self) {
        if self.input.value() != self.searcher.pattern() {
            self.searcher = self
                .searcher
                .restart(self.rollback_df.clone(), self.input.value().to_owned());
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SearchBar;

impl StatefulWidget for SearchBar {
    type State = SearchBarState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        state.sync();

        Input::default()
            .block(Block::app_default().title(state.searcher.title()))
            .render(area, buf, &mut state.input);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polars::prelude::Column;

    fn frame() -> DataFrame {
        DataFrame::new_infer_height(vec![Column::new(
            "name".into(),
            vec!["alpha", "beta", "gamma"],
        )])
        .unwrap()
    }

    fn typed(state: &mut SearchBarState, text: &str) {
        for c in text.chars() {
            state.input_mut().insert(c);
        }
    }

    fn render(state: &mut SearchBarState) -> Buffer {
        let area = Rect::new(0, 0, 40, 3);
        let mut buf = Buffer::empty(area);
        SearchBar.render(area, &mut buf, state);
        buf
    }

    fn content(buf: &Buffer) -> String {
        buf.content().iter().map(|c| c.symbol()).collect()
    }

    mod state {
        use super::*;

        #[test]
        fn an_exact_search_starts_empty() {
            let state = SearchBarState::exact(frame());

            assert_eq!(state.value(), "");
            assert_eq!(state.searcher().pattern(), "");
            assert!(matches!(state.searcher(), Searcher::Exact(_)));
        }

        #[test]
        fn a_fuzzy_search_starts_empty() {
            let state = SearchBarState::fuzzy(frame());

            assert!(matches!(state.searcher(), Searcher::Fuzzy(_)));
        }

        #[test]
        fn an_empty_pattern_yields_the_whole_frame() {
            let state = SearchBarState::exact(frame());

            assert_eq!(state.latest().unwrap().height(), frame().height());
        }

        #[test]
        fn syncing_hands_the_typed_pattern_to_the_searcher() {
            let mut state = SearchBarState::exact(frame());
            typed(&mut state, "bet");
            state.sync();

            assert_eq!(state.searcher().pattern(), "bet");
        }

        #[test]
        fn syncing_keeps_the_search_kind() {
            let mut state = SearchBarState::fuzzy(frame());
            typed(&mut state, "bet");
            state.sync();

            assert!(matches!(state.searcher(), Searcher::Fuzzy(_)));
        }

        #[test]
        fn the_pattern_is_stale_until_synced() {
            let mut state = SearchBarState::exact(frame());
            typed(&mut state, "bet");

            assert_eq!(state.value(), "bet");
            assert_eq!(state.searcher().pattern(), "");
        }

        #[test]
        fn rendering_syncs_the_pattern() {
            let mut state = SearchBarState::exact(frame());
            typed(&mut state, "bet");
            render(&mut state);

            assert_eq!(state.searcher().pattern(), "bet");
        }

        #[test]
        fn the_rollback_frame_survives_searching() {
            let mut state = SearchBarState::exact(frame());
            typed(&mut state, "bet");
            state.sync();

            assert_eq!(state.rollback_df().height(), frame().height());
            assert_eq!(state.into_rollback_df().height(), frame().height());
        }

        #[test]
        fn editing_back_to_empty_restores_the_whole_frame() {
            let mut state = SearchBarState::exact(frame());
            typed(&mut state, "b");
            state.sync();
            state.input_mut().delete_prev();
            state.sync();

            assert_eq!(state.searcher().pattern(), "");
            assert_eq!(state.latest().unwrap().height(), frame().height());
        }
    }

    mod widget {
        use super::*;

        #[test]
        fn an_exact_search_is_titled_search() {
            let buf = render(&mut SearchBarState::exact(frame()));

            let content = content(&buf);
            assert!(content.contains("Search"));
            assert!(!content.contains("Fuzzy"));
        }

        #[test]
        fn a_fuzzy_search_is_titled_fuzzy_search() {
            let buf = render(&mut SearchBarState::fuzzy(frame()));

            assert!(content(&buf).contains("Fuzzy Search"));
        }

        #[test]
        fn renders_the_typed_pattern() {
            let mut state = SearchBarState::exact(frame());
            typed(&mut state, "beta");
            let buf = render(&mut state);

            assert!(content(&buf).contains("beta"));
        }

        #[test]
        fn is_wrapped_in_a_border() {
            let buf = render(&mut SearchBarState::exact(frame()));

            assert!(content(&buf).contains('╭'));
        }

        #[test]
        fn fills_the_area_it_is_given() {
            let area = Rect::new(0, 0, 40, 3);
            let mut buf = Buffer::empty(area);
            SearchBar.render(area, &mut buf, &mut SearchBarState::exact(frame()));

            assert_eq!(buf[(0, 0)].symbol(), "╭");
            assert_eq!(buf[(39, 2)].symbol(), "╯");
        }
    }
}
