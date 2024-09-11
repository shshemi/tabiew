use itertools::{izip, Itertools};
use polars::frame::DataFrame;
use rand::Rng;
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::symbols;
use ratatui::widgets::{Axis, Chart, Dataset, GraphType};
use ratatui::{
    layout::{Alignment, Constraint, Margin, Rect},
    text::{Line, Span},
    widgets::{
        Block, Borders, Cell, List, ListDirection, ListState, Paragraph, Row, Table, TableState,
        Wrap,
    },
    Frame,
};

use crate::{
    theme::Styler,
    utils::{data_frame_widths, line_count, Scroll, TableValues},
};

use super::{AppResult, ChartNav};

#[derive(Debug)]
pub enum TabularState {
    Table,
    Sheet(Scroll),
    Chart,
}

#[derive(Debug)]
pub enum TabularType {
    Help,
    Schema,
    Name(String),
    Query(String),
}

#[derive(Debug)]
pub struct Tabular {
    offset: usize,
    select: usize,
    rendered_rows: u16,
    widths: Vec<usize>,
    headers: Vec<String>,
    table_values: TableValues,
    data_frame: DataFrame,
    state: TabularState,
    tabular_type: TabularType,
    chart_state: ChartState,
}

#[derive(Debug)]
struct ListControl {
    val: List<'static>,
    list_option: Vec<String>,
    selected: usize,
}

impl ListControl {
    fn new(title: &str, val: Vec<String>) -> Self {
        Self {
            val: List::new(val.clone())
                .block(
                    Block::default()
                        .title(title.to_owned())
                        .borders(Borders::ALL),
                )
                .highlight_symbol(">>")
                .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                .repeat_highlight_symbol(true)
                .direction(ListDirection::TopToBottom),
            list_option: val,
            selected: 0,
        }
    }
    fn next(&mut self) {
        if self.selected < self.val.len() - 1 {
            self.selected += 1;
        } else {
            self.selected = 0;
        }
    }
    fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        } else {
            self.selected = self.val.len() - 1;
        }
    }
    fn get_selected(&self) -> &str {
        self.list_option[self.selected].as_str()
    }
}

#[derive(Debug)]
enum ListNav {
    X,
    Y,
    ChartType,
}

#[derive(Debug)]
pub struct ChartState {
    x: ListControl,
    y: ListControl,
    chart_type: ListControl,
    current: ListNav,
}

impl ChartState {
    fn new(columns: Vec<String>) -> Self {
        Self {
            x: ListControl::new("Select x-axis:", columns.clone()),
            y: ListControl::new("Select y-axis:", columns.clone()),
            chart_type: ListControl::new(
                "Select chart type:",
                vec!["Line".to_string(), "Bar".to_string(), "Scatter".to_string()],
            ),
            current: ListNav::X,
        }
    }
    fn reload_columns(&mut self, columns: Vec<String>) {
        self.x.val = List::new(columns.clone())
            .block(
                Block::default()
                    .title("Select x-axis:")
                    .borders(Borders::ALL),
            )
            .highlight_symbol(">>")
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);
        self.y.val = List::new(columns.clone())
            .block(
                Block::default()
                    .title("Select y-axis:")
                    .borders(Borders::ALL),
            )
            .highlight_symbol(">>")
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);
    }

    fn nav(&mut self, nav: ChartNav) {
        match nav {
            ChartNav::Down => match self.current {
                ListNav::X => self.x.next(),
                ListNav::Y => self.y.next(),
                ListNav::ChartType => self.chart_type.next(),
            },
            ChartNav::Up => match self.current {
                ListNav::X => self.x.previous(),
                ListNav::Y => self.y.previous(),
                ListNav::ChartType => self.chart_type.previous(),
            },
            ChartNav::Right => {
                self.current = match self.current {
                    ListNav::X => ListNav::Y,
                    ListNav::Y => ListNav::ChartType,
                    ListNav::ChartType => ListNav::X,
                }
            }
            ChartNav::Left => {
                self.current = match self.current {
                    ListNav::ChartType => ListNav::Y,
                    ListNav::Y => ListNav::X,
                    ListNav::X => ListNav::ChartType,
                }
            }
            ChartNav::Init => {
                self.current = ListNav::X;
            }
        }
    }
}

impl Tabular {
    /// Constructs a new instance of [`App`].
    pub fn new(data_frame: DataFrame, reset: TabularType) -> Self {
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
            table_values: TableValues::from_dataframe(&data_frame),
            data_frame: data_frame.clone(),
            state: TabularState::Table,
            tabular_type: reset,
            chart_state: ChartState::new(
                data_frame
                    .get_column_names()
                    .into_iter()
                    .map(ToOwned::to_owned)
                    .collect(),
            ),
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
        self.select(rng.gen_range(0..self.table_values.height()))
    }

    pub fn select(&mut self, select: usize) -> AppResult<()> {
        self.select = select.min(self.table_values.height().saturating_sub(1));
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
            TabularState::Chart => self.show_chart(),
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

    pub fn show_chart(&mut self) -> AppResult<()> {
        self.state = TabularState::Chart;
        Ok(())
    }
    pub fn chart_update(&mut self, nav: ChartNav) {
        self.chart_state.nav(nav);
        self.chart_state.reload_columns(
            self.data_frame
                .get_column_names()
                .into_iter()
                .map(ToOwned::to_owned)
                .collect(),
        );
    }
    pub fn update_chart_state(&mut self) {
        self.chart_state = ChartState::new(
            self.data_frame
                .get_column_names()
                .into_iter()
                .map(ToOwned::to_owned)
                .collect(),
        )
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
        self.table_values.replace_dataframe(&data_frame);
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

    pub fn table_values(&self) -> &TableValues {
        &self.table_values
    }

    pub fn tabular_type(&self) -> &TabularType {
        &self.tabular_type
    }

    pub fn render<Theme: Styler>(
        &mut self,
        frame: &mut Frame,
        layout: Rect,
        selection: bool,
    ) -> AppResult<()> {
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
                            &self.table_values,
                            &self.widths,
                            &self.headers,
                            self.offset,
                            self.rendered_rows as usize,
                        ),
                        layout,
                        &mut local_st,
                    );
                } else {
                    frame.render_widget(
                        tabulate::<Theme>(
                            &self.table_values,
                            &self.widths,
                            &self.headers,
                            self.offset,
                            self.rendered_rows as usize,
                        ),
                        layout,
                    );
                }
            }
            TabularState::Sheet(scroll) => {
                self.rendered_rows = 0;
                let space = layout.inner(Margin::new(1, 1));
                let title = format!(" {} ", self.select + 1);

                let values = self.table_values.get_row(self.select);

                let (paragraph, line_count) = paragraph_from_headers_values::<Theme>(
                    &title,
                    &self.headers,
                    &values,
                    space.width,
                );

                scroll.adjust(line_count, space.height as usize);
                frame.render_widget(paragraph.scroll((scroll.to_u16(), 0)), layout);
            }
            TabularState::Chart => {
                self.rendered_rows = 0;

                let mut state1 =
                    ListState::default().with_selected(Some(self.chart_state.x.selected));
                let mut state2 =
                    ListState::default().with_selected(Some(self.chart_state.y.selected));

                let mut state_3 =
                    ListState::default().with_selected(Some(self.chart_state.chart_type.selected));

                let l1_area = Rect::new(0, 0, 20, 20);
                let l2_area = Rect::new(21, 0, 20, 20);
                let l3_area = Rect::new(42, 0, 20, 20);
                let chart_area = Rect::new(0, 20, 60, 20);

                let data = self.data_frame();
                let x_vec = data
                    .column(self.chart_state.x.get_selected())
                    .unwrap()
                    .i64()
                    .unwrap();
                let y_vec = data
                    .column(self.chart_state.y.get_selected())
                    .unwrap()
                    .i64()
                    .unwrap();
                let dataset = x_vec
                    .into_iter()
                    .zip(y_vec.into_iter())
                    .map(|(x, y)| (x.unwrap() as f64, y.unwrap() as f64))
                    .collect::<Vec<(f64, f64)>>();

                let x_axis = Axis::default()
                    .title("X Axis".red())
                    .style(Style::default().white())
                    .bounds([0.0, 10.0])
                    .labels(vec!["0.0".bold(), "5".into(), "10".into()]);

                // Create the Y axis and define its properties
                let y_axis = Axis::default()
                    .title("Y Axis".red())
                    .style(Style::default().white())
                    .bounds([0.0, 10.0])
                    .labels(vec!["0.0".bold(), "5".into(), "10".into()]);

                let datasets = vec![
                    // Scatter chart
                    Dataset::default()
                        .name("data1")
                        .marker(symbols::Marker::Dot)
                        .graph_type(GraphType::Scatter)
                        .style(Style::default().white())
                        .data(&dataset),
                ];

                let chart = match self.chart_state.chart_type.get_selected() {
                    "Line" => Chart::new(datasets)
                        .block(Block::default().title("Chart"))
                        .x_axis(x_axis)
                        .y_axis(y_axis),
                    _ => todo!(),
                };
                // scatter: x,y: only numeric
                // line: x,y: only numeric
                // bar: y: categorical, x: numeric

                frame.render_stateful_widget(self.chart_state.x.val.clone(), l1_area, &mut state1);
                frame.render_stateful_widget(self.chart_state.y.val.clone(), l2_area, &mut state2);
                frame.render_stateful_widget(
                    self.chart_state.chart_type.val.clone(),
                    l3_area,
                    &mut state_3,
                );

                frame.render_widget(chart, chart_area)
            }
        }
        Ok(())
    }
}

fn paragraph_from_headers_values<'a, Theme: Styler>(
    title: &'a str,
    headers: &'a [String],
    values: &'a [&str],
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
    value_pool: &'a TableValues,
    widths: &'a [usize],
    headers: &'a [String],
    offset: usize,
    length: usize,
) -> Table<'a> {
    Table::new(
        (offset..offset + length)
            .map(|row_idx| {
                Row::new(value_pool.get_row(row_idx).into_iter().map(Cell::new))
                    .style(Theme::table_row(row_idx))
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
