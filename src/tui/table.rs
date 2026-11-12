use std::{
    ops::{Add, Div},
    sync::{Arc, Mutex},
};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use itertools::Itertools;
use ratatui::{
    layout::{Constraint, Layout, Position, Rect},
    text::Text,
    widgets::{Cell, List, ListItem, ListState, Paragraph, Row, StatefulWidget, TableState},
};
use tui_scrollview::{ScrollView, ScrollViewState, ScrollbarVisibility};

use crate::{
    misc::config::{config, theme},
    reader::{Datum, Rows, is_numeric, single_line},
    tui::component::Component,
};

#[derive(Clone)]
struct Data(Arc<Mutex<dyn Rows>>);

impl std::fmt::Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Reader")
    }
}

#[derive(Debug, Clone)]
pub struct Table {
    data: Data,
    col_widths: Vec<Constraint>,
    col_offsets: Vec<usize>,
    col_space: u16,
    striped: bool,
    show_header: bool,
    selected: Option<usize>,
    offset: usize,
    rendered_rows: usize,
    rendered_width: u16,
    column_mode: ColumnMode,
    gutter_mode: GutterMode,
    fp_precision: Option<usize>,
}

impl Table {
    pub fn reader(rows: impl Rows + 'static) -> Self {
        Self {
            col_widths: Vec::new(),
            selected: None,
            col_offsets: vec![0],
            offset: 0,
            rendered_rows: 0,
            rendered_width: 0,
            column_mode: ColumnMode::Compact,
            striped: false,
            show_header: false,
            gutter_mode: GutterMode::Visible(1),
            fp_precision: config().fp_precision(),
            data: Data(Arc::new(Mutex::new(rows))),
            col_space: 1,
        }
    }

    pub fn set_reader(&mut self, rows: impl Rows + 'static) {
        self.data = Data(Arc::new(Mutex::new(rows)));
        self.col_widths.clear();
        self.col_offsets = vec![0];
    }

    pub fn rows(&self) -> Option<Arc<Mutex<dyn Rows>>> {
        Some(self.data.0.clone())
    }

    pub fn row_count(&self) -> usize {
        self.data.0.lock().unwrap().row_count()
    }

    pub fn complete(&self) -> bool {
        self.data.0.lock().unwrap().complete()
    }

    pub fn col_count(&self) -> usize {
        self.data.0.lock().unwrap().columns().len()
    }

    pub fn clone_with_reader(&self, rows: impl Rows + 'static) -> Self {
        Self {
            selected: self.selected.map(|_| 0),
            rendered_rows: self.rendered_rows,
            rendered_width: self.rendered_width,
            column_mode: self.column_mode,
            striped: self.striped,
            show_header: self.show_header,
            col_space: self.col_space,
            ..Self::reader(rows)
        }
    }

    pub fn view(&self) -> Option<String> {
        self.rows()
            .and_then(|rows| rows.lock().ok().and_then(|rows| rows.view()))
    }

    pub fn striped(self) -> Self {
        Self {
            striped: true,
            ..self
        }
    }

    pub fn with_show_header(self, show_header: bool) -> Self {
        Self {
            show_header,
            ..self
        }
    }

    pub fn with_selected(self, selected: impl Into<Option<usize>>) -> Self {
        Self {
            selected: selected.into(),
            ..self
        }
    }

    pub fn with_col_space(self, col_space: u16) -> Self {
        Self {
            col_space,
            col_offsets: col_offsets(&self.col_widths, col_space),
            ..self
        }
    }

    pub fn with_visible_gutter(mut self) -> Self {
        self.set_gutter_visibility(true);
        self
    }

    pub fn with_hidden_gutter(mut self) -> Self {
        self.set_gutter_visibility(false);
        self
    }

    pub fn with_compaect_column(self) -> Self {
        Self {
            column_mode: ColumnMode::Compact,
            ..self
        }
    }

    pub fn with_extended_column(self) -> Self {
        Self {
            column_mode: ColumnMode::Expanded(0),
            ..self
        }
    }

    pub fn gutter_visible(&self) -> bool {
        matches!(self.gutter_mode, GutterMode::Visible(_))
    }

    pub fn set_gutter_visibility(&mut self, value: bool) {
        if value {
            self.gutter_mode = GutterMode::Visible(self.row_count().to_string().len() as u16)
        } else {
            self.gutter_mode = GutterMode::Hidden
        }
    }

    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn rendered_rows(&self) -> usize {
        self.rendered_rows
    }

    pub fn set_offset(&mut self, idx: impl Into<usize>) {
        self.offset = idx.into().min(self.row_count());
    }

    pub fn select(&mut self, idx: impl Into<Option<usize>>) {
        let height = self.row_count();
        if height > 0 {
            self.selected = idx.into().map(|idx| idx.min(height - 1));
        }
    }

    pub fn fits_in_page(&self) -> bool {
        self.minimum_compact_width() <= self.rendered_width
    }

    pub fn toggle_view_mode(&mut self) {
        match self.column_mode {
            ColumnMode::Compact => {
                self.column_mode = ColumnMode::Expanded(0);
            }
            ColumnMode::Expanded(_) if self.fits_in_page() => {
                self.column_mode = ColumnMode::Compact;
            }
            _ => (),
        }
    }

    pub fn expended_column(&self) -> bool {
        match self.column_mode {
            ColumnMode::Compact => false,
            ColumnMode::Expanded(_) => true,
        }
    }

    fn select_up(&mut self) {
        if let Some(selected) = self.selected {
            self.select(selected.saturating_sub(1));
        } else {
            self.select(self.row_count().saturating_sub(1));
        }
    }

    fn select_down(&mut self) {
        if let Some(selected) = self.selected {
            self.select(selected.saturating_add(1));
        } else {
            self.select(0);
        }
    }

    fn select_first(&mut self) {
        self.select(0);
    }

    fn select_last(&mut self) {
        self.select(self.row_count().saturating_sub(1));
    }

    fn page_up(&mut self) {
        if let Some(selected) = self.selected {
            self.select(selected.saturating_sub(self.rendered_rows));
            self.set_offset(self.offset.saturating_sub(self.rendered_rows));
        }
    }

    fn page_down(&mut self) {
        if let Some(selected) = self.selected {
            self.select(selected.saturating_add(self.rendered_rows));
            self.set_offset(self.offset.saturating_add(self.rendered_rows));
        }
    }

    fn scroll_left(&mut self) {
        if let ColumnMode::Expanded(st) = &mut self.column_mode {
            *st = st.saturating_sub(1)
        }
    }

    fn scroll_right(&mut self) {
        if let ColumnMode::Expanded(st) = &mut self.column_mode {
            *st = st.saturating_add(1)
        }
    }

    fn scroll_to_left_column(&mut self) {
        if let ColumnMode::Expanded(offset) = &mut self.column_mode {
            *offset = prev_column_offset(&self.col_offsets, offset);
        }
    }

    fn scroll_to_right_column(&mut self) {
        if let ColumnMode::Expanded(offset) = &mut self.column_mode {
            *offset = next_column_offset(&self.col_offsets, offset);
        }
    }

    fn scroll_to_first_column(&mut self) {
        if let ColumnMode::Expanded(offset) = &mut self.column_mode {
            *offset = 0;
        }
    }

    fn scroll_to_last_column(&mut self) {
        if let ColumnMode::Expanded(offset) = &mut self.column_mode {
            *offset = self.col_offsets.last().copied().unwrap_or(0);
        }
    }

    fn half_page_up(&mut self) {
        if let Some(selected) = self.selected {
            let len = self.rendered_rows.div(2);
            self.select(selected.saturating_sub(len));
            self.set_offset(self.offset.saturating_sub(len));
        }
    }

    fn half_page_down(&mut self) {
        if let Some(selected) = self.selected {
            let len = self.rendered_rows.div(2);
            self.select(selected.saturating_add(len));
            self.set_offset(self.offset.saturating_add(len));
        }
    }

    fn gutter_item(&self, idx: usize) -> ListItem<'_> {
        ListItem::new(Text::raw(format!(
            "  {:>w$}  ",
            idx + 1,
            w = self.gutter_mode.width().into()
        )))
        .style(theme().gutter(idx))
    }

    fn gutter_table_area(&self, area: Rect) -> (Option<Rect>, Rect) {
        if let GutterMode::Visible(width) = self.gutter_mode {
            let [gutter_area, table_area] =
                Layout::horizontal([Constraint::Length(width + 4), Constraint::Fill(1)])
                    .areas(area);
            if self.show_header {
                let [_, gutter_area] =
                    Layout::vertical([Constraint::Length(1), Constraint::Fill(1)])
                        .areas(gutter_area);
                (Some(gutter_area), table_area)
            } else {
                (Some(gutter_area), table_area)
            }
        } else {
            (None, area)
        }
    }

    /// Column widths depend on the configured floating point precision, so they are recomputed
    /// whenever it changes.
    fn refresh_col_widths(&mut self) {
        self.fp_precision = config().fp_precision();
        self.data.0.lock().unwrap().reset();
        self.col_offsets = col_offsets(&self.col_widths, self.col_space);
    }

    fn minimum_compact_width(&self) -> u16 {
        let col_count = self.col_widths.len();
        let width = col_count + col_count.saturating_sub(1) * self.col_space as usize;
        u16::try_from(width).unwrap_or(u16::MAX)
    }
}

impl Component for Table {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: super::component::FocusState,
    ) {
        let height = if self.show_header {
            area.height.saturating_sub(1)
        } else {
            area.height
        } as usize;
        self.rendered_rows = height;
        self.rendered_width = area.width;

        if self.fp_precision != config().fp_precision() {
            self.refresh_col_widths();
        }

        self.selected = self
            .selected
            .map(|selected| selected.min(self.row_count().saturating_sub(1)));

        if let Some(selected) = self.selected {
            self.offset = self
                .offset
                .clamp(selected.saturating_sub(height.saturating_sub(1)), selected)
                .min(self.row_count().saturating_sub(height));
        } else {
            self.offset = self.offset.min(self.row_count().saturating_sub(height))
        }

        let row_count = self.row_count();
        if let GutterMode::Visible(width) = &mut self.gutter_mode {
            *width = row_count.to_string().len() as u16;
        }

        let reader = Some(self.data.0.clone());
        let mut guard = reader.as_ref().map(|reader| reader.lock().unwrap());
        let range = self.offset..self.offset + height;
        let filled = guard.as_mut().map(|reader| reader.fill(range.clone()));
        let rows = match (guard.as_ref(), filled) {
            (Some(reader), Some(Ok(()))) => {
                let widths = reader.widths();
                if widths.len() != self.col_widths.len()
                    || widths
                        .iter()
                        .zip(&self.col_widths)
                        .any(|(width, constraint)| *width != contraint_value(constraint))
                {
                    self.col_widths = widths.iter().copied().map(Constraint::Length).collect();
                    self.col_offsets = col_offsets(&self.col_widths, self.col_space);
                }
                Some((reader.columns(), reader.cached(range)))
            }
            (_, Some(Err(err))) => {
                ratatui::widgets::Widget::render(
                    Paragraph::new(err.to_string()).style(theme().text()),
                    area,
                    buf,
                );
                return;
            }
            _ => None,
        };

        let (gutter_area, table_area) = self.gutter_table_area(area);

        if let Some(gutter_area) = gutter_area {
            List::default()
                .items(
                    (self.offset..(self.offset + height).min(row_count))
                        .map(|idx| self.gutter_item(idx)),
                )
                .highlight_style(theme().row_highlighted())
                .render(
                    gutter_area,
                    buf,
                    &mut ListState::default().with_selected(if focus_state.is_focused() {
                        self.selected.map(|s| s.saturating_sub(self.offset))
                    } else {
                        None
                    }),
                );
        }

        if table_area.width < self.minimum_compact_width()
            && matches!(self.column_mode, ColumnMode::Compact)
        {
            self.column_mode = ColumnMode::Expanded(0);
        }

        let selected = if focus_state.is_focused() {
            self.selected.map(|s| s.saturating_sub(self.offset))
        } else {
            None
        };

        if let Some((columns, rows)) = rows {
            if columns.is_empty() {
                return;
            }
            let (col_start, col_end) = match &mut self.column_mode {
                ColumnMode::Compact => (0, columns.len() - 1),
                ColumnMode::Expanded(x) => {
                    let total_width = self
                        .col_offsets
                        .last()
                        .copied()
                        .unwrap_or(0)
                        .max(table_area.width as usize);
                    *x = (*x).min(total_width.saturating_sub(table_area.width as usize));
                    (
                        column_index(&self.col_offsets, x),
                        column_index(&self.col_offsets, &x.add(table_area.width as usize)),
                    )
                }
            };
            let col_end = col_end.min(columns.len() - 1);
            let numeric = columns
                .iter()
                .map(|(_, data_type)| is_numeric(data_type))
                .collect_vec();
            let table = build_reader_table(
                &columns[col_start..=col_end],
                &numeric[col_start..=col_end],
                rows.iter().map(|row| &row[col_start..=col_end]),
                &self.col_widths[col_start..=col_end],
                self.col_space,
                self.show_header,
                self.striped,
                self.offset,
                col_start,
            );
            match self.column_mode {
                ColumnMode::Compact => table.render(
                    table_area,
                    buf,
                    &mut TableState::default().with_selected(selected),
                ),
                ColumnMode::Expanded(x) => render_scrolled(
                    table,
                    &self.col_offsets,
                    col_start,
                    col_end,
                    x,
                    table_area,
                    buf,
                    selected,
                ),
            }
        }
    }

    fn handle(&mut self, event: KeyEvent) -> bool {
        match (event.code, event.modifiers) {
            (KeyCode::Up, KeyModifiers::NONE)
            | (KeyCode::Char('k'), KeyModifiers::NONE)
            | (KeyCode::Char('p'), KeyModifiers::CONTROL) => {
                self.select_up();
                true
            }
            (KeyCode::Down, KeyModifiers::NONE)
            | (KeyCode::Char('j'), KeyModifiers::NONE)
            | (KeyCode::Char('n'), KeyModifiers::CONTROL) => {
                self.select_down();
                true
            }
            (KeyCode::Left, KeyModifiers::NONE) | (KeyCode::Char('h'), KeyModifiers::NONE) => {
                self.scroll_left();
                true
            }
            (KeyCode::Right, KeyModifiers::NONE) | (KeyCode::Char('l'), KeyModifiers::NONE) => {
                self.scroll_right();
                true
            }
            (KeyCode::Home, KeyModifiers::NONE) | (KeyCode::Char('g'), KeyModifiers::NONE) => {
                self.select_first();
                true
            }
            (KeyCode::End, KeyModifiers::NONE) | (KeyCode::Char('G'), KeyModifiers::SHIFT) => {
                self.select_last();
                true
            }
            (KeyCode::PageUp, KeyModifiers::NONE) | (KeyCode::Char('b'), KeyModifiers::CONTROL) => {
                self.page_up();
                true
            }
            (KeyCode::PageDown, KeyModifiers::NONE)
            | (KeyCode::Char('f'), KeyModifiers::CONTROL) => {
                self.page_down();
                true
            }
            (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                self.half_page_up();
                true
            }
            (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                self.half_page_down();
                true
            }
            (KeyCode::Right, KeyModifiers::ALT)
            | (KeyCode::Char('f'), KeyModifiers::ALT)
            | (KeyCode::Char('w'), KeyModifiers::NONE) => {
                self.scroll_to_right_column();
                true
            }
            (KeyCode::Left, KeyModifiers::ALT)
            | (KeyCode::Char('b'), KeyModifiers::ALT)
            | (KeyCode::Char('b'), KeyModifiers::NONE) => {
                self.scroll_to_left_column();
                true
            }
            (KeyCode::Char('_'), _) => {
                self.scroll_to_first_column();
                true
            }
            (KeyCode::Char('$'), _) => {
                self.scroll_to_last_column();
                true
            }
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum ColumnMode {
    Compact,
    Expanded(usize),
}

#[derive(Debug, Clone, Copy)]
enum GutterMode {
    Hidden,
    Visible(u16),
}

impl GutterMode {
    fn width(&self) -> u16 {
        match self {
            GutterMode::Hidden => 0,
            GutterMode::Visible(w) => *w,
        }
    }
}

fn col_offsets(col_widths: &[Constraint], col_space: u16) -> Vec<usize> {
    std::iter::once(0)
        .chain(
            col_widths
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    if i != col_widths.len().saturating_sub(1) {
                        contraint_value(c) as usize + col_space as usize
                    } else {
                        contraint_value(c) as usize
                    }
                })
                .scan(0, |s, u| {
                    *s += u;
                    Some(*s)
                }),
        )
        .collect_vec()
}

fn column_index(col_offsets: &[usize], offset: &usize) -> usize {
    // col_offsets index: 0    1    2    3    4
    // col_offsets      : 0---10---20---30---40
    // return value     :   0    1    2   (3)-> maximum allowed = col_effsets.len() - 2
    match col_offsets.binary_search(offset) {
        Ok(idx) => idx,
        Err(idx) => idx.saturating_sub(1),
    }
    .min(col_offsets.len().saturating_sub(2))
}
fn prev_column_offset(col_offsets: &[usize], offset: &usize) -> usize {
    col_offsets
        .get(column_index(col_offsets, &offset.saturating_sub(1)))
        .copied()
        .unwrap_or_default()
}

fn next_column_offset(col_offsets: &[usize], offset: &usize) -> usize {
    col_offsets
        .get(column_index(col_offsets, offset).saturating_add(1))
        .copied()
        .unwrap_or_default()
}

#[allow(clippy::too_many_arguments)]
fn build_reader_table<'a>(
    columns: &'a [(String, String)],
    numeric: &[bool],
    rows: impl Iterator<Item = &'a [Datum]>,
    col_widths: &[Constraint],
    col_space: u16,
    show_header: bool,
    striped: bool,
    offset_row: usize,
    offset_col: usize,
) -> ratatui::widgets::Table<'a> {
    let theme = theme();
    let align = |text: Text<'a>, numeric: bool| match numeric {
        true => text.right_aligned(),
        false => text,
    };
    let mut table = ratatui::widgets::Table::default()
        .widths(col_widths)
        .style(theme.text())
        .row_highlight_style(theme.row_highlighted())
        .column_spacing(col_space)
        .rows(rows.enumerate().map(|(idx, row)| {
            Row::new(row.iter().zip(numeric).map(|(value, numeric)| {
                Cell::new(align(
                    Text::raw(single_line(value.as_deref().unwrap_or_default())),
                    *numeric,
                ))
            }))
            .style(if striped {
                theme.row(offset_row + idx)
            } else {
                theme.row(0)
            })
        }));
    if show_header {
        table =
            table.header(
                Row::new(columns.iter().zip(numeric).enumerate().map(
                    |(i, ((name, _), numeric))| {
                        Cell::new(align(Text::raw(name.as_str()), *numeric))
                            .style(theme.header(offset_col + i))
                    },
                ))
                .style(theme.table_header()),
            )
    }
    table
}

#[allow(clippy::too_many_arguments)]
fn render_scrolled(
    table: ratatui::widgets::Table<'_>,
    col_offsets: &[usize],
    col_start: usize,
    col_end: usize,
    x: usize,
    table_area: Rect,
    buf: &mut ratatui::prelude::Buffer,
    selected: Option<usize>,
) {
    let width = (col_offsets[col_end + 1] - col_offsets[col_start]).max(table_area.width as usize);
    let size = ratatui::layout::Size {
        width: width as u16,
        height: table_area.height,
    };
    let mut scroll_area = ScrollView::new(size).scrollbars_visibility(ScrollbarVisibility::Never);
    scroll_area.render_stateful_widget(
        table,
        scroll_area.area(),
        &mut TableState::default().with_selected(selected),
    );
    scroll_area.render(
        table_area,
        buf,
        &mut ScrollViewState::with_offset(Position {
            x: x.saturating_sub(col_offsets.get(col_start).copied().unwrap_or_default()) as u16,
            y: 0,
        }),
    );
}

fn contraint_value(constraint: &Constraint) -> u16 {
    match constraint {
        Constraint::Min(val)
        | Constraint::Max(val)
        | Constraint::Length(val)
        | Constraint::Fill(val) => *val,
        Constraint::Percentage(_) | Constraint::Ratio(_, _) => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        duckdb::{DuckDB, DuckDBConfig, IngestOptions, Load},
        indexer::Indexer,
        reader::{Reader, ReaderConfig},
        tui::component::FocusState,
    };
    use ratatui::{Terminal, backend::TestBackend};
    use std::{
        sync::atomic::AtomicBool,
        time::{Duration, Instant},
    };
    use tempfile::TempDir;

    const CSV: &str = "id,name,price,note\n1,alpha,1.5,\"two\nlines\"\n2,beta,22.25,\"a\tb\"\n3,gamma,333.125,\n4,delta,4,plain\n";

    fn screen(table: &mut Table) -> String {
        let mut terminal = Terminal::new(TestBackend::new(60, 7)).unwrap();
        terminal
            .draw(|frame| table.render(frame.area(), frame.buffer_mut(), FocusState::Focused))
            .unwrap();
        let buffer = terminal.backend().buffer();
        (0..buffer.area.height)
            .map(|y| {
                (0..buffer.area.width)
                    .map(|x| buffer[(x, y)].symbol())
                    .collect::<String>()
                    .trim_end()
                    .to_string()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn flat(screen: &str) -> String {
        screen
            .lines()
            .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn key(code: KeyCode, modifiers: KeyModifiers) -> KeyEvent {
        KeyEvent::new(code, modifiers)
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn duckdb_and_indexer_readers_render_in_the_table() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("prices.csv");
        std::fs::write(&path, CSV).unwrap();
        let options = IngestOptions::default();
        let indexer =
            Indexer::new(path.clone(), &options, Arc::new(AtomicBool::new(true))).unwrap();
        let start = Instant::now();
        while indexer.row_count().is_none() {
            assert!(start.elapsed() < Duration::from_secs(5));
            std::thread::sleep(Duration::from_millis(10));
        }
        let mut table = Table::reader(Reader::new(
            indexer,
            ReaderConfig {
                segment_rows: 2,
                segments: 2,
                segment_bytes: usize::MAX,
            },
        ))
        .with_show_header(true)
        .with_selected(0);
        let indexed = screen(&mut table);
        println!("indexer:\n{indexed}");
        let indexed = flat(&indexed);
        assert!(indexed.contains("id name price note"));
        assert!(indexed.contains("1 1 alpha 1.5 two"));
        assert!(indexed.contains("2 2 beta 22.25 a b"));
        let mut db = DuckDB::new(DuckDBConfig {
            cache_dir: dir.path().join("cache"),
            ..DuckDBConfig::default()
        })
        .unwrap();
        assert!(matches!(
            db.load(&path, None, &options, None).unwrap(),
            Load::Ingesting(_)
        ));
        let start = Instant::now();
        let views = loop {
            assert!(start.elapsed() < Duration::from_secs(10));
            if let Some((_, views)) = db.poll().unwrap().tables.pop() {
                break views.unwrap();
            }
            std::thread::sleep(Duration::from_millis(10));
        };
        let mut table = Table::reader(db.reader(&views[0]).unwrap())
            .with_show_header(true)
            .with_selected(0);
        let ingested = screen(&mut table);
        println!("duckdb:\n{ingested}");
        let ingested = flat(&ingested);
        assert!(ingested.contains("id name price note"));
        assert!(ingested.contains("1 1 alpha 1.5 two"));
        assert!(ingested.contains("3 3 gamma 333.125"));
        table.handle(key(KeyCode::Char('G'), KeyModifiers::SHIFT));
        let last = screen(&mut table);
        println!("duckdb after G:\n{last}");
        let last = flat(&last);
        assert_eq!(table.selected(), Some(3));
        assert!(last.contains("4 4 delta 4 plain"));
    }
}
