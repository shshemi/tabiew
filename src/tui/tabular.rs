use std::marker::PhantomData;

use itertools::{izip, Itertools};
use polars::frame::DataFrame;
use rand::Rng;
use ratatui::{
    layout::{Alignment, Constraint, Margin, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, ListItem, Paragraph, Row, Table, TableState, Wrap},
    Frame,
};

use super::utils::{any_value_into_string, data_frame_widths, line_count, Scroll};
use crate::{tui::theme::Styler, utils::ZipItersExt};

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
pub struct Tabular<'a, Theme> {
    rendered_rows: u16,
    data_frame: DataFrame,
    state: TabularState,
    tabular_type: TabularType,
    table: Table<'a>,
    table_state: TableState,
    _theme: PhantomData<Theme>,
}

impl<'a, Theme: Styler> Tabular<'a, Theme> {
    /// Constructs a new instance of [`App`].
    pub fn new(data_frame: DataFrame, tabular_type: TabularType) -> Self {
        Self {
            rendered_rows: 0,
            state: TabularState::Table,
            tabular_type,
            _theme: PhantomData,
            table: tabulate::<Theme>(&data_frame),
            table_state: TableState::new().with_offset(0).with_selected(0),
            data_frame,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) -> AppResult<()> {
        Ok(())
    }

    pub fn select_up(&mut self, len: usize) -> AppResult<()> {
        self.table_state
            .selected()
            .map(|idx| self.select(idx.saturating_sub(len)));
        Ok(())
    }

    pub fn select_down(&mut self, len: usize) -> AppResult<()> {
        self.table_state
            .selected()
            .map(|idx| self.select(idx.saturating_add(len)));
        Ok(())
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
        self.table_state.select(
            select
                .min(self.data_frame.height().saturating_sub(1))
                .into(),
        );
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

    // pub fn adjust_offset(&mut self) {
    //     self.offset = self.offset.clamp(
    //         self.select
    //             .saturating_sub(self.rendered_rows.saturating_sub(1).into()),
    //         self.select,
    //     );
    // }

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
        self.state = TabularState::Table;
        self.table = tabulate::<Theme>(&data_frame);
        self.table_state.select(0.into());
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
        self.table_state.selected().unwrap_or_default()
    }

    // pub fn table_values(&self) -> &TableValues {
    //     &self.table_values
    // }

    pub fn tabular_type(&self) -> &TabularType {
        &self.tabular_type
    }

    pub fn render(&mut self, frame: &mut Frame, layout: Rect, selection: bool) -> AppResult<()> {
        match &mut self.state {
            TabularState::Table => {
                self.rendered_rows = layout.height.saturating_sub(1);

                if selection {
                    frame.render_stateful_widget(&self.table, layout, &mut self.table_state)
                } else {
                    frame.render_widget(&self.table, layout)
                }
            }
            TabularState::Sheet(scroll) => {
                self.rendered_rows = 0;
                let space = layout.inner(Margin::new(1, 1));
                let title = format!(" {} ", self.table_state.selected().unwrap() + 1);

                let headers = self
                    .data_frame
                    .get_column_names()
                    .into_iter()
                    .map(|col| col.to_owned())
                    .collect_vec();
                let values = self
                    .data_frame
                    .get(self.table_state.selected().unwrap())
                    .map(|values| values.into_iter().map(any_value_into_string).collect_vec())
                    .unwrap();

                let (paragraph, line_count) =
                    paragraph_from_headers_values::<Theme>(title, headers, values, space.width);

                scroll.adjust(line_count, space.height as usize);
                frame.render_widget(paragraph.scroll((scroll.to_u16(), 0)), layout);
            }
        }
        Ok(())
    }
}

fn paragraph_from_headers_values<'a, Theme: Styler>(
    title: String,
    headers: Vec<String>,
    values: Vec<String>,
    width: u16,
) -> (Paragraph<'a>, usize) {
    let lines = izip!(headers, values)
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
    header: String,
    value: String,
) -> Vec<Line<'a>> {
    std::iter::once(Line::from(Span::styled(
        header,
        Theme::table_header_cell(idx),
    )))
    .chain(
        value
            .lines()
            .map(|line| Line::from(Span::styled(line.to_owned(), Theme::sheet_value()))),
    )
    .chain(std::iter::once(Line::default()))
    .collect_vec()
}

pub fn tabulate<'a, Theme: Styler>(data_frame: &DataFrame) -> Table<'a> {
    Table::new(
        data_frame
            .iter()
            .map(|series| series.iter())
            .zip_iters()
            .enumerate()
            .map(|(idx, values)| {
                Row::new(values.into_iter().map(any_value_into_string).map(Cell::new))
                    .style(Theme::table_row(idx))
            }),
        data_frame_widths(data_frame)
            .into_iter()
            .map(|w| Constraint::Length(w as u16)),
    )
    .header(
        Row::new(
            data_frame
                .get_column_names()
                .into_iter()
                .enumerate()
                .map(|(idx, col)| Cell::new(col.to_owned()).style(Theme::table_header_cell(idx))),
        )
        .style(Theme::table_header()),
    )
    .highlight_style(Theme::table_highlight())
    .column_spacing(2)
}
