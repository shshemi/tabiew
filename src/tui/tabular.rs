use std::marker::PhantomData;

use itertools::Itertools;
use polars::frame::DataFrame;
use rand::Rng;
use ratatui::{layout::Rect, Frame};

use super::widget::{
    data_frame_table::{DataFrameTable, DataFrameTableState},
    sheet::{Sheet, SheetBlock, SheetState},
};
use crate::tui::{theme::Styler, utils::any_value_into_string};

use crate::AppResult;

#[derive(Debug)]
pub enum TabularState {
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
pub struct Tabular<Theme> {
    table_state: DataFrameTableState,
    state: TabularState,
    tabular_type: TabularType,
    _theme: PhantomData<Theme>,
}

impl<Theme: Styler> Tabular<Theme> {
    /// Constructs a new instance of [`App`].
    pub fn new(data_frame: DataFrame, tabular_type: TabularType) -> Self {
        Self {
            table_state: DataFrameTableState::new(data_frame),
            state: TabularState::Table,
            tabular_type,
            _theme: PhantomData,
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
        if let TabularState::Sheet(scroll) = &mut self.state {
            scroll.scroll_up();
            Ok(())
        } else {
            Err("Not in table view".into())
        }
    }

    pub fn scroll_down(&mut self) -> AppResult<()> {
        if let TabularState::Sheet(scroll) = &mut self.state {
            scroll.scroll_down();
            Ok(())
        } else {
            Err("Not in table view".into())
        }
    }

    pub fn page_len(&self) -> usize {
        self.table_state.rendered_rows().into()
    }

    pub fn switch_view(&mut self) -> AppResult<()> {
        match self.state {
            TabularState::Table => self.show_sheet(),
            TabularState::Sheet(_) => self.show_table(),
        }
    }

    pub fn show_sheet(&mut self) -> AppResult<()> {
        self.state = TabularState::Sheet(Default::default());
        Ok(())
    }

    pub fn show_table(&mut self) -> AppResult<()> {
        self.state = TabularState::Table;
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

    pub fn state(&self) -> &TabularState {
        &self.state
    }

    pub fn selected(&self) -> usize {
        self.table_state.selected()
    }

    pub fn tabular_type(&self) -> &TabularType {
        &self.tabular_type
    }

    pub fn render(&mut self, frame: &mut Frame, layout: Rect, selection: bool) -> AppResult<()> {
        match &mut self.state {
            TabularState::Table => {
                frame.render_stateful_widget(
                    DataFrameTable::<Theme>::new()
                        .with_selection(selection)
                        .with_column_space(2),
                    layout,
                    &mut self.table_state,
                );
            }
            TabularState::Sheet(sheet_state) => {
                let title = format!(" {} ", self.table_state.selected() + 1);
                let sheet = Sheet::<Theme>::new(
                    title,
                    self.table_state
                        .headers()
                        .iter()
                        .cloned()
                        .zip(
                            self.table_state
                                .data_frame()
                                .get(self.table_state.selected())
                                .map(|row| row.into_iter().map(any_value_into_string).collect_vec())
                                .unwrap_or_default(),
                        )
                        .map(|(header, content)| SheetBlock::new(header, content))
                        .collect_vec(),
                );
                frame.render_stateful_widget(sheet, layout, sheet_state);
            }
        }
        Ok(())
    }
}
