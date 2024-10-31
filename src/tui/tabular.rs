use std::marker::PhantomData;

use itertools::{izip, Itertools};
use polars::frame::DataFrame;
use rand::Rng;
use ratatui::{
    layout::{Alignment, Margin, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use super::{
    utils::{line_count, Scroll},
    widget::data_frame_table::{DataFrameTable, DataFrameTableState},
};
use crate::tui::{theme::Styler, utils::any_value_into_string};

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
        self.table_state.rendered_rows().into()
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
            TabularState::Sheet(scroll) => {
                let space = layout.inner(Margin::new(1, 1));
                let title = format!(" {} ", self.table_state.selected() + 1);
                let values = self
                    .table_state
                    .data_frame()
                    .get(self.table_state.selected())
                    .map(|row| row.into_iter().map(any_value_into_string).collect_vec())
                    .unwrap_or_default();

                let (paragraph, line_count) = paragraph_from_headers_values::<Theme>(
                    &title,
                    self.table_state.headers(),
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
