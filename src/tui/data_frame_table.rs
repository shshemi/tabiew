use std::{borrow::Cow, ops::Div};

use anyhow::anyhow;
use itertools::Itertools;
use polars::{frame::DataFrame, prelude::PlSmallStr};
use ratatui::{
    layout::Rect,
    widgets::{StatefulWidget, Widget},
};
use unicode_width::UnicodeWidthChar;

use crate::{
    AppResult,
    misc::{
        globals::theme,
        iter_ext::ZipItersExt,
        polars_ext::{AnyValueExt, TuiWidths},
    },
    tui::widgets::block::Block,
};

#[derive(Debug, Default)]
pub struct DataFrameTableState {
    offset_y: usize,
    offset_x: usize,
    select: usize,
    rendered_rows: u16,
    rendered_width: u16,
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
            rendered_width: 0,
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

    pub fn scroll_right_column(&mut self) {
        self.offset_x = self
            .widths
            .iter()
            .copied()
            .scan(0, |a, b| {
                *a += b + 1;
                Some(*a)
            })
            .find(|&s| s > self.offset_x)
            .unwrap_or(self.widths.len());
    }

    pub fn scroll_left_column(&mut self) {
        self.offset_x = self
            .widths
            .iter()
            .copied()
            .scan(0, |a, b| {
                *a += b + 1;
                Some(*a)
            })
            .take_while(|&s| s < self.offset_x)
            .last()
            .unwrap_or(0)
    }

    pub fn expanded(&self) -> bool {
        self.expanded
    }
    pub fn toggle_expansion(&mut self) -> AppResult<()> {
        match TableFitState::with(self.rendered_width.into(), &self.widths) {
            TableFitState::Fitable => {
                self.expanded = !self.expanded;
                if !self.expanded {
                    self.offset_x = 0;
                }
                Ok(())
            }
            TableFitState::LargerThanColumns => {
                self.expanded = !self.expanded;
                if !self.expanded {
                    self.offset_x = 0;
                }
                Ok(())
            }
            TableFitState::TooSmallToFit => Err(anyhow!("Table is too large to fit")),
        }
    }

    pub fn rendered_rows(&self) -> u16 {
        self.rendered_rows
    }
}

pub struct DataFrameTable<'a> {
    block: Option<Block<'a>>,
}

impl<'a> DataFrameTable<'a> {
    pub fn new() -> Self {
        Self { block: None }
    }

    pub fn with_block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}

impl Default for DataFrameTable<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl StatefulWidget for DataFrameTable<'_> {
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

        // rendered rows = table height - header height
        state.rendered_rows = area.height.saturating_sub(1);

        // rendered rows = table height - header height
        state.rendered_width = area.width;

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
                .copied()
                .sum::<usize>()
                .saturating_add(state.widths.len().saturating_sub(1))
                .saturating_sub(area.width.into()),
        );

        // set widths according to expanded (not auto-fit)
        if TableFitState::with(area.width.into(), &state.widths).force_expand() {
            state.expanded = true;
        };
        let widths = if state.expanded {
            Cow::Borrowed(&state.widths)
        } else {
            Cow::Owned(shrink_columns(area.width.into(), &state.widths))
        };

        // draw background
        buf.set_style(area, theme().table_header());

        // draw header
        buf.set_string(
            area.x,
            area.y,
            {
                let full_line = state
                    .headers
                    .iter()
                    .zip(widths.iter())
                    .map(|(val, width)| viewport(val, 0, *width))
                    .join(" ");

                viewport(&full_line, state.offset_x, area.width.into())
            },
            theme().table_header(),
        );
        buf.set_style(
            Rect {
                x: area.x,
                y: area.y,
                width: area.width,
                height: 1,
            },
            theme().table_header(),
        );

        // style header
        state
            .headers()
            .iter()
            .zip(widths.iter())
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
                    theme().header(col),
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
                    {
                        let full_line = vec
                            .into_iter()
                            .zip(widths.iter())
                            .map(|(val, width)| {
                                viewport(
                                    val.into_single_line().lines().next().unwrap_or_default(),
                                    0,
                                    *width,
                                )
                            })
                            .join(" ");

                        viewport(&full_line, state.offset_x, area.width.into())
                    },
                    if state.offset_y + idx == state.select {
                        theme().row_highlighted()
                    } else {
                        theme().row(state.offset_y + idx)
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
    }
}

pub enum TableFitState {
    Fitable,
    LargerThanColumns,
    TooSmallToFit,
}
impl TableFitState {
    pub fn with(table_width: usize, col_widths: &[usize]) -> TableFitState {
        if col_widths.len() * 2 > table_width {
            TableFitState::TooSmallToFit
        } else if col_widths.iter().sum::<usize>() + col_widths.len() - 1 <= table_width {
            TableFitState::LargerThanColumns
        } else {
            TableFitState::Fitable
        }
    }

    fn force_expand(self) -> bool {
        match self {
            TableFitState::Fitable => false,
            TableFitState::LargerThanColumns => false,
            TableFitState::TooSmallToFit => true,
        }
    }
}
fn shrink_columns(table_width: usize, column_widths: &[usize]) -> Vec<usize> {
    let total_width = column_widths.iter().sum::<usize>();
    let available_width = table_width.saturating_sub(column_widths.len().saturating_sub(1));
    let mut new_widths = column_widths
        .iter()
        .map(|w| available_width.div(column_widths.len()).min(*w))
        .collect_vec();
    let mut remainder = total_width
        .min(available_width)
        .saturating_sub(new_widths.iter().sum::<usize>());
    for idx in (0..new_widths.len()).cycle() {
        if remainder == 0 {
            break;
        }
        if new_widths[idx] < column_widths[idx] {
            remainder -= 1;
            new_widths[idx] += 1;
        }
    }
    new_widths
}

/// Take a viewport of single line.
///
/// Both `start_column` and `available_width` are base on displaying grid,
/// not index nor Unicode scalar.
fn viewport(line: &str, start_column: usize, available_width: usize) -> String {
    let end_column = start_column + available_width;

    let mut current_column = 0;

    let mut output = String::new();

    for c in line.chars() {
        // We don't care any characters after end_column.
        if current_column >= end_column {
            break;
        }

        let char_width = c.width().unwrap_or_default();

        match char_width {
            // control character.
            0 => (),

            // half-width character.
            1 => {
                if current_column >= start_column && current_column < end_column {
                    output.push(c)
                }
            }

            // full-width character.
            2 => {
                // this full-width character only have half space (1 column) can use.
                if current_column + 1 == start_column || current_column + 1 == end_column {
                    const HALF_PLACEHOLDER: char = ' ';

                    output.push(HALF_PLACEHOLDER)
                }
                // have full space can use.
                else if current_column >= start_column && current_column + 1 < end_column {
                    output.push(c)
                }
            }

            _ => unreachable!(),
        }

        current_column += char_width;
    }

    // if not run out all available_width (chars not enough), pad it.
    for _ in current_column..end_column {
        output.push(' ');
    }

    output
}
