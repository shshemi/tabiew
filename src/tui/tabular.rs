use std::marker::PhantomData;

use anyhow::anyhow;
use itertools::Itertools;
use polars::frame::DataFrame;
use rand::Rng;
use ratatui::{layout::Rect, widgets::StatefulWidget};

use super::{
    data_frame_table::{DataFrameTable, DataFrameTableState},
    sheet::{Sheet, SheetBlock, SheetState},
};
use crate::{tui::theme::Styler, utils::polars_ext::IntoString};

use crate::AppResult;

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
}

impl TabularState {
    /// Constructs a new instance of [`App`].
    pub fn new(data_frame: DataFrame, tabular_type: TabularType) -> Self {
        Self {
            table_state: DataFrameTableState::new(data_frame),
            view: TabularView::Table,
            tabular_type,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) -> AppResult<()> {
        Ok(())
    }

    pub fn select_up(&mut self, len: usize) -> AppResult<()> {
        self.table_state.select_up(len);
        Ok(())
    }

    pub fn select_down(&mut self, len: usize) -> AppResult<()> {
        self.table_state.select_down(len);
        Ok(())
    }

    pub fn select_first(&mut self) -> AppResult<()> {
        self.table_state.select_first();
        Ok(())
    }

    pub fn select_last(&mut self) -> AppResult<()> {
        self.table_state.select_last();
        Ok(())
    }

    pub fn select_random(&mut self) -> AppResult<()> {
        let mut rng = rand::thread_rng();
        self.table_state
            .select(rng.gen_range(0..self.table_state.height()));
        Ok(())
    }

    pub fn select(&mut self, select: usize) -> AppResult<()> {
        self.table_state.select(select);
        Ok(())
    }

    pub fn scroll_up(&mut self) -> AppResult<()> {
        if let TabularView::Sheet(scroll) = &mut self.view {
            scroll.scroll_up();
            Ok(())
        } else {
            Err(anyhow!("Not in table view"))
        }
    }

    pub fn scroll_down(&mut self) -> AppResult<()> {
        if let TabularView::Sheet(scroll) = &mut self.view {
            scroll.scroll_down();
            Ok(())
        } else {
            Err(anyhow!("Not in table view"))
        }
    }

    pub fn page_len(&self) -> usize {
        self.table_state.rendered_rows().into()
    }

    pub fn switch_view(&mut self) -> AppResult<()> {
        match self.view {
            TabularView::Table => self.show_sheet(),
            TabularView::Sheet(_) => self.show_table(),
        }
    }

    pub fn show_sheet(&mut self) -> AppResult<()> {
        self.view = TabularView::Sheet(Default::default());
        Ok(())
    }

    pub fn show_table(&mut self) -> AppResult<()> {
        self.view = TabularView::Table;
        Ok(())
    }

    pub fn set_data_frame(&mut self, data_frame: DataFrame) -> AppResult<()> {
        self.table_state.set_data_frame(data_frame);
        Ok(())
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
