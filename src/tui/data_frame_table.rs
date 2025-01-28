use std::{marker::PhantomData, ops::Add};

use itertools::Itertools;
use polars::{frame::DataFrame, prelude::PlSmallStr, series::Series};
use ratatui::{
    layout::{Constraint, Rect},
    widgets::{Block, Cell, Row, StatefulWidget, Table, TableState, Widget},
};

use crate::{
    tui::Styler,
    utils::{
        iter_ext::ZipItersExt,
        polars_ext::{IntoString, TuiWidths},
    },
};

#[derive(Debug, Default)]
pub struct DataFrameTableState {
    offset_y: usize,
    offset_x: usize,
    select: usize,
    rendered_rows: u16,
    expanded: bool,
    widths: Vec<usize>,
    headers: Vec<String>,
    data_frame: DataFrame,
}

impl DataFrameTableState {
    pub fn new(data_frame: DataFrame) -> Self {
        Self {
            offset_y: 0,
            offset_x: 0,
            select: 0,
            rendered_rows: 0,
            expanded: true,
            widths: data_frame.tui_widths(),
            headers: data_frame
                .get_column_names()
                .into_iter()
                .map(PlSmallStr::to_string)
                .collect(),
            data_frame: data_frame.clone(),
        }
    }

    pub fn data_frame(&self) -> &DataFrame {
        &self.data_frame
    }

    pub fn data_frame_mut(&mut self) -> &mut DataFrame {
        &mut self.data_frame
    }

    pub fn set_data_frame(&mut self, data_frame: DataFrame) {
        self.offset_y = 0;
        self.select = 0;
        self.widths = data_frame.tui_widths();
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
        self.select(usize::MAX);
    }

    pub fn scroll_left(&mut self) {
        if self.expanded {
            self.offset_x = self.offset_x.saturating_sub(1);
        }
    }

    pub fn scroll_right(&mut self) {
        if self.expanded {
            self.offset_x = self.offset_x.saturating_add(1);
        }
    }

    pub fn scroll_start(&mut self) {
        if self.expanded {
            self.offset_x = 0;
        }
    }

    pub fn scroll_end(&mut self) {
        if self.expanded {
            self.offset_x = usize::MAX;
        }
    }

    pub fn expanded(&self) -> bool {
        self.expanded
    }
    pub fn toggle_expansion(&mut self) {
        self.expanded = !self.expanded;
    }

    pub fn height(&self) -> usize {
        self.data_frame.height()
    }
    pub fn rendered_rows(&self) -> u16 {
        self.rendered_rows
    }
}

pub struct DataFrameTable<'a, Theme> {
    block: Option<Block<'a>>,
    _theme: PhantomData<Theme>,
}

impl<'a, Theme> DataFrameTable<'a, Theme> {
    pub fn new() -> Self {
        Self {
            block: None,
            _theme: Default::default(),
        }
    }

    pub fn with_block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}

impl<Theme> Default for DataFrameTable<'_, Theme> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Theme: Styler> StatefulWidget for DataFrameTable<'_, Theme> {
    type State = DataFrameTableState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let area = if let Some(block) = self.block {
            let new_area = block.inner(area);
            block.render(area, buf);
            new_area
        } else {
            area
        };
        // let block = Block::bordered()
        //     .title_bottom(self.status_bar)
        //     .border_type(ratatui::widgets::BorderType::Rounded)
        //     .border_style(Theme::pallete());
        // let new_area = block.inner(area);
        // block.render(area, buf);
        // let area = new_area;

        // rendered rows = table height - header height
        state.rendered_rows = area.height.saturating_sub(1);

        // 0 <= select < table height
        state.select = state
            .select
            .min(state.data_frame.height().saturating_sub(1));

        // 0 <= offset_y <= select <= offset_y + rendered rows < table height
        state.offset_y = state.offset_y.clamp(
            state
                .select
                .saturating_sub(state.rendered_rows.saturating_sub(1).into()),
            state.select,
        );

        // 0 <= offset_x < sum(widths) - area.width + (paddings between cols)
        state.offset_x = state.offset_x.min(
            state
                .widths
                .iter()
                .sum::<usize>()
                .saturating_sub(area.width as usize)
                .add(state.widths.len().saturating_sub(1))
                .saturating_sub(1),
        );

        if state.expanded {
            // expanded table uses a custom widget table
            // draw background
            buf.set_style(area, Theme::table_header());

            // draw header
            buf.set_string(
                area.x,
                area.y,
                state
                    .headers
                    .iter()
                    .zip(state.widths.iter())
                    .map(|(val, wid)| format!("{:<width$}", val, width = wid))
                    .join(" ")
                    .chars()
                    .skip(state.offset_x)
                    .take(area.width.into())
                    .collect::<String>(),
                Theme::table_header(),
            );
            buf.set_style(
                Rect {
                    x: area.x,
                    y: area.y,
                    width: area.width,
                    height: 1,
                },
                Theme::table_header(),
            );

            // style header
            state
                .headers()
                .iter()
                .zip(state.widths.iter())
                .scan(0_usize, |before, (header, width)| {
                    let start = *before;
                    let end = start + header.len();
                    *before = start + width + 1; // the width +1 for the columns padding
                    Some((start, end))
                })
                .enumerate()
                .filter_map(|(idx, (i, j))| {
                    let min = i.saturating_sub(state.offset_x);
                    let max = j.saturating_sub(state.offset_x);
                    (min < max && min < area.width as usize).then_some((idx, min, max))
                })
                .for_each(|(col, i, j)| {
                    buf.set_style(
                        Rect {
                            x: area.x + i as u16,
                            y: area.y,
                            width: (j - i).min(area.width as usize - i) as u16,
                            height: 1,
                        },
                        Theme::table_header_cell(col),
                    );
                });

            // draw rows
            state
                .data_frame
                .slice(state.offset_y as i64, state.rendered_rows as usize)
                .iter()
                .map(|series| series.iter())
                .zip_iters()
                .enumerate()
                .map(|(idx, vec)| {
                    (
                        idx as u16,
                        vec.into_iter()
                            .zip(state.widths.iter())
                            .map(|(val, width)| {
                                format!(
                                    "{:<width$}",
                                    val.into_string().lines().next().unwrap_or_default(),
                                    width = width
                                )
                            })
                            .join(" ")
                            .chars()
                            .skip(state.offset_x)
                            .take(area.width.into())
                            .collect::<String>(),
                        if state.offset_y + idx == state.select {
                            Theme::table_highlight()
                        } else {
                            Theme::table_row(state.offset_y + idx)
                        },
                    )
                })
                .for_each(|(idx, line, style)| {
                    buf.set_string(area.x, area.y + 1 + idx, line, style);
                    buf.set_style(
                        Rect {
                            x: area.x,
                            y: area.y + 1 + idx,
                            width: area.width,
                            height: 1,
                        },
                        style,
                    );
                });
        } else {
            // Non expanded uses defautl ratatui table
            let header = Row::new(
                state
                    .headers
                    .iter()
                    .enumerate()
                    .map(|(col_idx, name)| {
                        Cell::new(name.as_str()).style(Theme::table_header_cell(col_idx))
                    })
                    .collect_vec(),
            )
            .style(Theme::table_header());

            let rows = state
                .data_frame
                .slice(state.offset_y as i64, state.rendered_rows as usize)
                .iter()
                .map(Series::iter)
                .zip_iters()
                .enumerate()
                .map(|(ridx, vals)| {
                    Row::new(vals.into_iter().map(IntoString::into_string).map(Cell::new))
                        .style(Theme::table_row(ridx + state.offset_y))
                })
                .collect_vec();

            let widths = &state
                .widths
                .iter()
                .copied()
                .map(|w| Constraint::Length(w as u16))
                .collect_vec();

            let table = Table::default()
                .rows(rows)
                .widths(widths)
                .header(header)
                .row_highlight_style(Theme::table_highlight())
                .style(Theme::table_header())
                .column_spacing(1);

            let mut ts = TableState::new()
                .with_offset(0)
                .with_selected(state.select.saturating_sub(state.offset_y));
            StatefulWidget::render(table, area, buf, &mut ts);
        }
    }
}
