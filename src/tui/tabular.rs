use std::marker::PhantomData;

use itertools::{izip, Itertools};
use polars::{frame::DataFrame, series::Series};
use rand::Rng;
use ratatui::{
    layout::{Alignment, Constraint, Margin, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState, Wrap},
    Frame,
};

use super::utils::{data_frame_widths, line_count, Scroll};
use crate::{
    tui::{theme::Styler, utils::any_value_into_string},
    utils::ZipItersExt,
};

use crate::AppResult;

#[derive(Debug)]
pub enum TabularState {
    Table,
    Sheet(Scroll),
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
    offset: usize,
    select: usize,
    rendered_rows: u16,
    widths: Vec<usize>,
    headers: Vec<String>,
    data_frame: DataFrame,
    state: TabularState,
    tabular_type: TabularType,
    _theme: PhantomData<Theme>,
}

impl<Theme: Styler> Tabular<Theme> {
    /// Constructs a new instance of [`App`].
    pub fn new(data_frame: DataFrame, tabular_type: TabularType) -> Self {
        Self {
            offset: 0,
            select: 0,
            rendered_rows: 0,
            widths: data_frame_widths(&data_frame),
            headers: data_frame
                .get_column_names()
                .into_iter()
                .map(ToOwned::to_owned)
                .collect(),
            data_frame,
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
        self.select(self.select.saturating_sub(len))
    }

    pub fn select_down(&mut self, len: usize) -> AppResult<()> {
        self.select(self.select + len)
    }

    pub fn select_first(&mut self) -> AppResult<()> {
        self.select(usize::MIN)
    }

    pub fn select_last(&mut self) -> AppResult<()> {
        self.select(usize::MAX)
    }

    pub fn select_random(&mut self) -> AppResult<()> {
        let mut rng = rand::thread_rng();
        self.select(rng.gen_range(0..self.data_frame.height()))
    }

    pub fn select(&mut self, select: usize) -> AppResult<()> {
        self.select = select.min(self.data_frame.height().saturating_sub(1));
        Ok(())
    }

    pub fn scroll_up(&mut self) -> AppResult<()> {
        if let TabularState::Sheet(scroll) = &mut self.state {
            scroll.up();
            Ok(())
        } else {
            Err("Not in table view".into())
        }
    }

    pub fn scroll_down(&mut self) -> AppResult<()> {
        if let TabularState::Sheet(scroll) = &mut self.state {
            scroll.down();
            Ok(())
        } else {
            Err("Not in table view".into())
        }
    }

    pub fn page_len(&self) -> usize {
        self.rendered_rows.into()
    }

    pub fn adjust_offset(&mut self) {
        self.offset = self.offset.clamp(
            self.select
                .saturating_sub(self.rendered_rows.saturating_sub(1).into()),
            self.select,
        );
    }

    pub fn switch_view(&mut self) -> AppResult<()> {
        match self.state {
            TabularState::Table => self.show_sheet(),
            TabularState::Sheet(_) => self.show_table(),
        }
    }

    pub fn show_sheet(&mut self) -> AppResult<()> {
        self.state = TabularState::Sheet(Scroll::default());
        Ok(())
    }

    pub fn show_table(&mut self) -> AppResult<()> {
        self.state = TabularState::Table;
        Ok(())
    }

    pub fn set_data_frame(&mut self, data_frame: DataFrame) -> AppResult<()> {
        self.widths = data_frame_widths(&data_frame);
        self.offset = 0;
        self.select = 0;
        self.headers = data_frame
            .get_column_names()
            .into_iter()
            .map(ToOwned::to_owned)
            .collect();
        self.data_frame = data_frame;
        Ok(())
    }

    pub fn data_frame(&self) -> &DataFrame {
        &self.data_frame
    }

    pub fn state(&self) -> &TabularState {
        &self.state
    }

    pub fn selected(&self) -> usize {
        self.select
    }

    pub fn tabular_type(&self) -> &TabularType {
        &self.tabular_type
    }

    pub fn render(&mut self, frame: &mut Frame, layout: Rect, selection: bool) -> AppResult<()> {
        match &mut self.state {
            TabularState::Table => {
                self.rendered_rows = layout.height.saturating_sub(1);
                self.adjust_offset();

                if selection {
                    let mut local_st = TableState::new()
                        .with_offset(0)
                        .with_selected(self.select.saturating_sub(self.offset));

                    frame.render_stateful_widget(
                        tabulate::<Theme>(
                            self.data_frame
                                .slice(self.offset as i64, self.rendered_rows as usize),
                            &self.widths,
                            &self.headers,
                            self.offset,
                        ),
                        layout,
                        &mut local_st,
                    );
                } else {
                    frame.render_widget(
                        tabulate::<Theme>(
                            self.data_frame
                                .slice(self.offset as i64, self.rendered_rows as usize),
                            &self.widths,
                            &self.headers,
                            self.offset
                        ),
                        layout,
                    );
                }
            }
            TabularState::Sheet(scroll) => {
                self.rendered_rows = 0;
                let space = layout.inner(Margin::new(1, 1));
                let title = format!(" {} ", self.select + 1);

                // let values = self.table_values.get_row(self.select);
                let values = self
                    .data_frame
                    .get(self.select)
                    .map(|row| row.into_iter().map(any_value_into_string).collect_vec())
                    .unwrap_or_default();

                let (paragraph, line_count) = paragraph_from_headers_values::<Theme>(
                    &title,
                    &self.headers,
                    &values,
                    space.width,
                );

                scroll.adjust(line_count, space.height as usize);
                frame.render_widget(paragraph.scroll((scroll.to_u16(), 0)), layout);
            }
        }
        Ok(())
    }
}

fn paragraph_from_headers_values<'a, Theme: Styler>(
    title: &'a str,
    headers: &'a [String],
    values: &'a [String],
    width: u16,
) -> (Paragraph<'a>, usize) {
    let lines = izip!(headers, values.iter())
        .enumerate()
        .flat_map(|(idx, (header, value))| lines_from_header_value::<Theme>(idx, header, value))
        .collect_vec();
    let lc = lines
        .iter()
        .map(|line| line_count(&line.to_string(), width as usize))
        .sum();
    let prgr = Paragraph::new(lines)
        .block(Block::new().title(title).borders(Borders::ALL))
        .style(Theme::sheet_block())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    (prgr, lc)
}

fn lines_from_header_value<'a, Theme: Styler>(
    idx: usize,
    header: &'a str,
    value: &'a str,
) -> Vec<Line<'a>> {
    let header_line = std::iter::once(Line::from(Span::styled(
        header,
        Theme::table_header_cell(idx),
    )));
    let value_lines = value
        .lines()
        .map(|line| Line::from(Span::styled(line, Theme::sheet_value())));
    header_line
        .chain(value_lines)
        .chain(std::iter::once(Line::default()))
        .collect_vec()
}

pub fn tabulate<'a, Theme: Styler>(
    data_frame: DataFrame,
    widths: &'a [usize],
    headers: &'a [String],
    offset: usize,
) -> Table<'a> {
    Table::new(
        data_frame
            .iter()
            .map(Series::iter)
            .zip_iters()
            .enumerate()
            .map(|(ridx, vals)| {
                Row::new(vals.into_iter().map(any_value_into_string).map(Cell::new))
                    .style(Theme::table_row(ridx + offset))
            })
            .collect_vec(),
        widths
            .iter()
            .copied()
            .map(|w| Constraint::Length(w as u16))
            .collect::<Vec<_>>(),
    )
    .header(header_row::<Theme>(headers))
    .highlight_style(Theme::table_highlight())
    .column_spacing(2)
}

fn header_row<Theme: Styler>(df: &[String]) -> Row {
    Row::new(
        df.iter()
            .enumerate()
            .map(|(col_idx, name)| {
                Cell::new(name.as_str()).style(Theme::table_header_cell(col_idx))
            })
            .collect::<Vec<_>>(),
    )
    .style(Theme::table_header())
}
