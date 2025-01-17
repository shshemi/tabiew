use std::marker::PhantomData;

use itertools::Itertools;
use polars::frame::DataFrame;
use rand::Rng;
use ratatui::{layout::Rect, widgets::StatefulWidget};

use super::{
    data_frame_table::{DataFrameTable, DataFrameTableState},
    sheet::{Sheet, SheetBlock, SheetState},
};
use crate::{search::Search, tui::theme::Styler, utils::polars_ext::IntoString};

#[derive(Debug)]
pub enum TabularView {
    Table,
    Sheet(SheetState),
}

#[derive(Debug)]
pub enum TabularType {
    Help,
    Schema,
    Name(String),
    Query(String),
}

#[derive(Debug)]
pub struct TabularState {
    table_state: DataFrameTableState,
    view: TabularView,
    tabular_type: TabularType,
    search: Option<Search>,
    original_frame: DataFrame,
}

impl TabularState {
    /// Constructs a new instance of [`App`].
    pub fn new(data_frame: DataFrame, tabular_type: TabularType) -> Self {
        Self {
            table_state: DataFrameTableState::new(data_frame.clone()),
            view: TabularView::Table,
            tabular_type,
            search: None,
            original_frame: data_frame,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        if let Some(df) = self.search.as_ref().and_then(Search::latest) {
            self.table_state.set_data_frame(df);
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

    pub fn scroll_up(&mut self) {
        if let TabularView::Sheet(scroll) = &mut self.view {
            scroll.scroll_up();
        }
    }

    pub fn scroll_down(&mut self) {
        if let TabularView::Sheet(scroll) = &mut self.view {
            scroll.scroll_down();
        }
    }

    pub fn page_len(&self) -> usize {
        self.table_state.rendered_rows().into()
    }

    pub fn switch_view(&mut self) {
        match self.view {
            TabularView::Table => self.show_sheet(),
            TabularView::Sheet(_) => self.show_table(),
        }
    }

    pub fn show_sheet(&mut self) {
        self.view = TabularView::Sheet(Default::default());
    }

    pub fn show_table(&mut self) {
        self.view = TabularView::Table;
    }

    pub fn set_data_frame(&mut self, data_frame: DataFrame) {
        self.table_state.set_data_frame(data_frame);
    }

    pub fn data_frame(&self) -> &DataFrame {
        self.table_state.data_frame()
    }

    pub fn data_frame_mut(&mut self) -> &mut DataFrame {
        self.table_state.data_frame_mut()
    }

    pub fn view(&self) -> &TabularView {
        &self.view
    }

    pub fn selected(&self) -> usize {
        self.table_state.selected()
    }

    pub fn tabular_type(&self) -> &TabularType {
        &self.tabular_type
    }

    pub fn search_pattern(&mut self, pattern: String) {
        if self
            .search
            .as_ref()
            .map(|search| search.pattern() != &pattern)
            .unwrap_or(true)
        {
            self.search = Some(Search::new(self.original_frame.clone(), pattern))
        }
    }

    pub fn commit_search(&mut self) {
        if let Some(df) = self.search.take().and_then(|ser| ser.latest()) {
            self.set_data_frame(df);
        }
    }

    pub fn cancel_search(&mut self) {
        self.search = None;
    }

    pub fn rollback(&mut self) {
        self.table_state.set_data_frame(self.original_frame.clone());
    }
}

pub struct Tabular<Theme> {
    selection: bool,
    _theme: PhantomData<Theme>,
}

impl<Theme: Styler> Tabular<Theme> {
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

impl<Theme: Styler> Default for Tabular<Theme> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Theme: Styler> StatefulWidget for Tabular<Theme> {
    type State = TabularState;

    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        match &mut state.view {
            TabularView::Table => {
                StatefulWidget::render(
                    DataFrameTable::<Theme>::new()
                        .with_selection(self.selection)
                        .with_column_space(2),
                    area,
                    buf,
                    &mut state.table_state,
                );
            }

            TabularView::Sheet(sheet_state) => {
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
                StatefulWidget::render(sheet, area, buf, sheet_state);
            }
        }
    }
}
