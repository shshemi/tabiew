use std::marker::PhantomData;

use itertools::Itertools;
use polars::{frame::DataFrame, prelude::PlSmallStr, series::Series};
use ratatui::{
    layout::Constraint,
    widgets::{Cell, Row, StatefulWidget, Table, TableState, Widget},
};

use crate::{
    tui::{
        utils::{any_value_into_string, data_frame_widths},
        Styler,
    },
    utils::ZipItersExt,
};

#[derive(Debug)]
pub struct DataFrameTableState {
    offset: usize,
    select: usize,
    rendered_rows: u16,
    widths: Vec<usize>,
    headers: Vec<String>,
    data_frame: DataFrame,
}

impl DataFrameTableState {
    pub fn new(data_frame: DataFrame) -> Self {
        Self {
            offset: 0,
            select: 0,
            rendered_rows: 0,
            widths: data_frame_widths(&data_frame),
            headers: data_frame
                .get_column_names()
                .into_iter()
                .map(PlSmallStr::to_string)
                .collect(),
            data_frame,
        }
    }

    pub fn data_frame(&self) -> &DataFrame {
        &self.data_frame
    }

    pub fn data_frame_mut(&mut self) -> &mut DataFrame {
        &mut self.data_frame
    }

    pub fn set_data_frame(&mut self, data_frame: DataFrame) {
        self.offset = 0;
        self.select = 0;
        self.widths = data_frame_widths(&data_frame);
        self.headers = data_frame
            .get_column_names()
            .into_iter()
            .map(PlSmallStr::to_string)
            .collect();
        self.data_frame = data_frame;
    }

    pub fn headers(&self) -> &[String] {
        &self.headers
    }

    pub fn selected(&self) -> usize {
        self.select
    }

    pub fn select(&mut self, select: usize) {
        self.select = select.min(self.data_frame.height().saturating_sub(1));
    }

    pub fn select_up(&mut self, len: usize) {
        self.select(self.select.saturating_sub(len))
    }

    pub fn select_down(&mut self, len: usize) {
        self.select(self.select + len)
    }

    pub fn select_first(&mut self) {
        self.select(0)
    }

    pub fn select_last(&mut self) {
        self.select(self.height());
    }

    pub fn height(&self) -> usize {
        self.data_frame.height()
    }
    pub fn rendered_rows(&self) -> u16 {
        self.rendered_rows
    }

    fn adjust(&mut self, rendered_rows: u16) {
        self.rendered_rows = rendered_rows;
        self.offset = self.offset.clamp(
            self.select
                .saturating_sub(rendered_rows.saturating_sub(1).into()),
            self.select,
        );
    }
}

pub struct DataFrameTable<Theme> {
    selection: bool,
    column_space: u16,
    _theme: PhantomData<Theme>,
}

impl<Theme> DataFrameTable<Theme> {
    pub fn new() -> Self {
        Self {
            selection: false,
            column_space: 1,
            _theme: Default::default(),
        }
    }

    pub fn with_selection(mut self, selection: bool) -> Self {
        self.selection = selection;
        self
    }

    pub fn with_column_space(mut self, space: u16) -> Self {
        self.column_space = space;
        self
    }
}

impl<Theme> Default for DataFrameTable<Theme> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Theme: Styler> StatefulWidget for DataFrameTable<Theme> {
    type State = DataFrameTableState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        state.adjust(area.height.saturating_sub(1));
        let df = state
            .data_frame
            .slice(state.offset as i64, state.rendered_rows as usize);

        let header = Row::new(
            state
                .headers
                .iter()
                .enumerate()
                .map(|(col_idx, name)| {
                    Cell::new(name.as_str()).style(Theme::table_header_cell(col_idx))
                })
                .collect::<Vec<_>>(),
        )
        .style(Theme::table_header());

        let table = Table::new(
            df.iter()
                .map(Series::iter)
                .zip_iters()
                .enumerate()
                .map(|(ridx, vals)| {
                    Row::new(vals.into_iter().map(any_value_into_string).map(Cell::new))
                        .style(Theme::table_row(ridx + state.offset))
                })
                .collect_vec(),
            state
                .widths
                .iter()
                .copied()
                .map(|w| Constraint::Length(w as u16))
                .collect::<Vec<_>>(),
        )
        .header(header)
        .highlight_style(Theme::table_highlight())
        .column_spacing(2);

        if self.selection {
            StatefulWidget::render(
                table,
                area,
                buf,
                &mut TableState::new()
                    .with_offset(0)
                    .with_selected(state.select.saturating_sub(state.offset)),
            );
        } else {
            Widget::render(table, area, buf);
        }
    }
}
