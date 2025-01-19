use std::{marker::PhantomData, os::macos::raw::stat};

use itertools::Itertools;
use polars::frame::DataFrame;
use rand::Rng;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    widgets::StatefulWidget,
};

use super::{
    data_frame_table::{DataFrameTable, DataFrameTableState},
    search_bar::{self, SearchBar, SearchBarState},
    sheet::{Sheet, SheetBlock, SheetState},
};
use crate::{search::Search, tui::theme::Styler, utils::polars_ext::IntoString};

#[derive(Debug)]
pub enum TabularMode {
    Table,
    Sheet(SheetState),
    Search(Search, SearchBarState),
}

#[derive(Debug)]
pub enum TabularSource {
    Help,
    Schema,
    Name(String),
    Query(String),
}

#[derive(Debug)]
pub struct TabContentState {
    table_state: DataFrameTableState,
    mode: TabularMode,
    tabular_source: TabularSource,
    original_frame: DataFrame,
}

impl TabContentState {
    /// Constructs a new instance of [`App`].
    pub fn new(data_frame: DataFrame, tabular_source: TabularSource) -> Self {
        Self {
            table_state: DataFrameTableState::new(data_frame.clone()),
            mode: TabularMode::Table,
            tabular_source,
            original_frame: data_frame,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        if let TabularMode::Search(search, _) = &mut self.mode {
            if let Some(df) = search.latest() {
                self.table_state.set_data_frame(df);
            }
        }
    }

    pub fn select_up(&mut self, len: usize) {
        self.table_state.select_up(len);
    }

    pub fn select_down(&mut self, len: usize) {
        self.table_state.select_down(len);
    }

    pub fn select_first(&mut self) {
        self.table_state.select_first();
    }

    pub fn select_last(&mut self) {
        self.table_state.select_last();
    }

    pub fn select_random(&mut self) {
        let mut rng = rand::thread_rng();
        self.table_state
            .select(rng.gen_range(0..self.table_state.height()));
    }

    pub fn select(&mut self, select: usize) {
        self.table_state.select(select);
    }

    pub fn selected(&self) -> usize {
        self.table_state.selected()
    }

    pub fn scroll_up(&mut self) {
        if let TabularMode::Sheet(scroll) = &mut self.mode {
            scroll.scroll_up();
        }
    }

    pub fn scroll_down(&mut self) {
        if let TabularMode::Sheet(scroll) = &mut self.mode {
            scroll.scroll_down();
        }
    }

    pub fn scroll_left(&mut self) {
        if matches!(self.mode, TabularMode::Table) {
            self.table_state.scroll_left();
        }
    }

    pub fn scroll_right(&mut self) {
        if matches!(self.mode, TabularMode::Table) {
            self.table_state.scroll_right();
        }
    }

    pub fn page_len(&self) -> usize {
        self.table_state.rendered_rows().into()
    }

    pub fn switch_view(&mut self) {
        match self.mode {
            TabularMode::Table => self.sheet_mode(),
            TabularMode::Sheet(_) => self.table_mode(),
            TabularMode::Search(_, _) => (),
        }
    }

    pub fn table_mode(&mut self) {
        self.mode = TabularMode::Table;
    }

    pub fn sheet_mode(&mut self) {
        self.mode = TabularMode::Sheet(Default::default());
    }

    pub fn search_mode(&mut self) {
        match &self.mode {
            TabularMode::Table => {
                self.mode = TabularMode::Search(
                    Search::new(self.original_frame.clone(), Default::default()),
                    SearchBarState::default(),
                );
            }
            _ => (),
        }
    }

    pub fn search_commit(&mut self) {
        match &self.mode {
            TabularMode::Search(search, _) => {
                if let Some(df) = search.latest() {
                    self.set_data_frame(df);
                }
            }
            _ => (),
        }
    }

    pub fn search_delete_prev(&mut self) {
        if let TabularMode::Search(search, search_bar_state) = &mut self.mode {
            search_bar_state.input().delete_prev();
            if search_bar_state.input().value() != search.pattern() {
                *search = Search::new(
                    self.original_frame.clone(),
                    search_bar_state.input().value().to_owned(),
                )
            }
        }
    }

    pub fn search_delete_next(&mut self) {
        if let TabularMode::Search(search, search_bar_state) = &mut self.mode {
            search_bar_state.input().delete_next();
            if search_bar_state.input().value() != search.pattern() {
                *search = Search::new(
                    self.original_frame.clone(),
                    search_bar_state.input().value().to_owned(),
                )
            }
        }
    }

    pub fn search_goto_prev(&mut self) {
        if let TabularMode::Search(search, search_bar_state) = &mut self.mode {
            search_bar_state.input().goto_prev();
            if search_bar_state.input().value() != search.pattern() {
                *search = Search::new(
                    self.original_frame.clone(),
                    search_bar_state.input().value().to_owned(),
                )
            }
        }
    }

    pub fn search_goto_next(&mut self) {
        if let TabularMode::Search(search, search_bar_state) = &mut self.mode {
            search_bar_state.input().goto_next();
            if search_bar_state.input().value() != search.pattern() {
                *search = Search::new(
                    self.original_frame.clone(),
                    search_bar_state.input().value().to_owned(),
                )
            }
        }
    }

    pub fn search_goto_start(&mut self) {
        if let TabularMode::Search(search, search_bar_state) = &mut self.mode {
            search_bar_state.input().goto_start();
            if search_bar_state.input().value() != search.pattern() {
                *search = Search::new(
                    self.original_frame.clone(),
                    search_bar_state.input().value().to_owned(),
                )
            }
        }
    }

    pub fn search_goto_end(&mut self) {
        if let TabularMode::Search(search, search_bar_state) = &mut self.mode {
            search_bar_state.input().goto_end();
            if search_bar_state.input().value() != search.pattern() {
                *search = Search::new(
                    self.original_frame.clone(),
                    search_bar_state.input().value().to_owned(),
                )
            }
        }
    }

    pub fn search_insert(&mut self, c: char) {
        if let TabularMode::Search(search, search_bar_state) = &mut self.mode {
            search_bar_state.input().insert(c);
            if search_bar_state.input().value() != search.pattern() {
                *search = Search::new(
                    self.original_frame.clone(),
                    search_bar_state.input().value().to_owned(),
                )
            }
        }
    }

    pub fn data_frame(&self) -> &DataFrame {
        self.table_state.data_frame()
    }

    pub fn data_frame_mut(&mut self) -> &mut DataFrame {
        self.table_state.data_frame_mut()
    }

    pub fn set_data_frame(&mut self, data_frame: DataFrame) {
        self.table_state.set_data_frame(data_frame);
    }

    pub fn rollback(&mut self) {
        self.table_state.set_data_frame(self.original_frame.clone());
    }

    pub fn mode(&self) -> &TabularMode {
        &self.mode
    }

    pub fn tabular_source(&self) -> &TabularSource {
        &self.tabular_source
    }
}

pub struct TabContent<Theme> {
    selection: bool,
    _theme: PhantomData<Theme>,
}

impl<Theme: Styler> TabContent<Theme> {
    pub fn new() -> Self {
        Self {
            selection: false,
            _theme: Default::default(),
        }
    }

    pub fn with_selection(mut self, selection: bool) -> Self {
        self.selection = selection;
        self
    }
}

impl<Theme: Styler> Default for TabContent<Theme> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Theme: Styler> StatefulWidget for TabContent<Theme> {
    type State = TabContentState;

    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        match &mut state.mode {
            TabularMode::Table => {
                DataFrameTable::<Theme>::new()
                    .with_selection(self.selection)
                    .render(area, buf, &mut state.table_state);
            }

            TabularMode::Sheet(sheet_state) => {
                let title = format!(" {} ", state.table_state.selected() + 1);
                let sheet = Sheet::<Theme>::new(
                    title,
                    state
                        .table_state
                        .headers()
                        .iter()
                        .cloned()
                        .zip(
                            state
                                .table_state
                                .data_frame()
                                .get(state.table_state.selected())
                                .map(|row| {
                                    row.into_iter().map(IntoString::into_string).collect_vec()
                                })
                                .unwrap_or_default(),
                        )
                        .map(|(header, content)| SheetBlock::new(header, content))
                        .collect_vec(),
                );
                sheet.render(area, buf, sheet_state);
            }

            TabularMode::Search(_, search_bar_state) => {
                let [search_bar_area, table_area] =
                    Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);
                SearchBar::<Theme>::new()
                    .with_selection(self.selection)
                    .render(search_bar_area, buf, search_bar_state);
                DataFrameTable::<Theme>::new().with_selection(false).render(
                    table_area,
                    buf,
                    &mut state.table_state,
                );
            }
        }
    }
}
