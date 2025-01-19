use std::{marker::PhantomData, ops::Add, os::macos::raw::stat, usize};

use itertools::{enumerate, Itertools};
use polars::{frame::DataFrame, prelude::PlSmallStr};
use ratatui::{layout::Rect, text::Line, widgets::StatefulWidget};

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
        self.select = select;
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
        self.offset_x = self.offset_x.saturating_sub(1);
    }

    pub fn scroll_right(&mut self) {
        self.offset_x = self.offset_x.saturating_add(1);
    }

    pub fn height(&self) -> usize {
        self.data_frame.height()
    }
    pub fn rendered_rows(&self) -> u16 {
        self.rendered_rows
    }
}

pub struct DataFrameTable<Theme> {
    selection: bool,
    _theme: PhantomData<Theme>,
}

impl<Theme> DataFrameTable<Theme> {
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
        // rendered rows = table height - header height
        state.rendered_rows = area.height.saturating_sub(1);

        // 0 <= select < table height
        state.select = state.select.min(state.data_frame.height());

        // 0 <= offset_y <= select <= offset_y + rendered rows < table height
        state.offset_y = state.offset_y.clamp(
            state
                .select
                .saturating_sub(state.rendered_rows.saturating_sub(1).into()),
            state.select,
        );

        // total width
        let total_width = state.widths.iter().sum::<usize>();

        // 0 <= offset_x < sum(widths) - area.width
        state.offset_x = state
            .offset_x
            .min(total_width.saturating_sub(area.width as usize) + (state.widths.len() - 1));

        // draw header
        buf.set_line(
            area.x,
            area.y,
            &Line::styled(
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
            ),
            area.width,
        );

        // style header
        let v = state
            .widths
            .iter()
            .copied()
            .scan(0, |s, w| {
                let ret = (*s, *s + w);
                *s = ret.1 + 1;
                Some(ret)
            })
            .enumerate()
            .map(|(idx, (i, j))| (Theme::table_header_cell(idx), i, j))
            .filter(|(_, i, j)| {
                let min = state.offset_x as usize;
                let max = min + area.width as usize;
                i.clamp(&min, &max) < j.clamp(&min, &max)
            })
            .collect_vec();

        v.into_iter().for_each(|(s, i, j)| {
            buf.set_style(
                Rect {
                    x: (area.x + i as u16).saturating_sub(state.offset_x as u16),
                    y: area.y,
                    width: (j - i) as u16,
                    height: 1,
                },
                s,
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
                    Line::styled(
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
                        if state.offset_y + idx == state.select && self.selection {
                            Theme::table_highlight()
                        } else {
                            Theme::table_row(state.offset_y + idx)
                        },
                    ),
                )
            })
            .for_each(|(idx, line)| {
                buf.set_line(area.x, area.y + 1 + idx, &line, area.width);
            });

        // let header = Row::new(
        //     state
        //         .headers
        //         .iter()
        //         .zip(state.widths.iter())
        //         .enumerate()
        //         .map(|(col_idx, (name, cons))| {
        //             Cell::new(format!("{} ({:?})",name.as_str(), cons)).style(Theme::table_header_cell(col_idx))
        //         })
        //         .collect::<Vec<_>>(),
        // )
        // .style(Theme::table_header());

        // let rows = df
        //     .iter()
        //     .map(Series::iter)
        //     .zip_iters()
        //     .enumerate()
        //     .map(|(ridx, vals)| {
        //         Row::new(vals.into_iter().map(IntoString::into_string).map(Cell::new))
        //             .style(Theme::table_row(ridx + state.offset))
        //     })
        //     .collect_vec();

        // let table = Table::default()
        //     .rows(rows)
        //     .widths(&state.widths)
        //     .header(header)
        //     .flex(Flex::Start)
        //     .row_highlight_style(Theme::table_highlight())
        //     .style(Theme::table_header())
        //     .column_spacing(1);

        // if self.selection {
        //     let mut ts = TableState::new()
        //             .with_offset(0)
        //             .with_selected(state.select.saturating_sub(state.offset));
        //     StatefulWidget::render(
        //         table,
        //         area,
        //         buf,
        //         &mut ts,
        //     );
        // } else {
        //     Widget::render(table, area, buf);
        // }
    }
}
