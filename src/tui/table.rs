use std::collections::HashMap;
use std::ops::{Add, Div};
use std::time::{Duration, Instant};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use itertools::Itertools;
use polars::frame::DataFrame;
use ratatui::{
    layout::{Constraint, Layout, Position, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Cell, List, ListItem, ListState, Row, StatefulWidget, TableState},
};
use tui_scrollview::{ScrollView, ScrollViewState, ScrollbarVisibility};

use crate::{
    misc::{
        config::theme,
        iter_ext::ZipItersExt,
        polars_ext::{AnyValueExt, DataFrameExt},
        type_ext::ConstraintExt,
    },
    tui::component::Component,
};

/// Whether a cell flash represents an insert or update.
#[derive(Debug, Clone, Copy)]
pub enum FlashKind {
    Insert,
    Update,
}

#[derive(Debug, Clone)]
pub struct Table {
    df: DataFrame,
    col_widths: Vec<Constraint>,
    col_offsets: Vec<u16>,
    col_space: u16,
    striped: bool,
    show_header: bool,
    selected: Option<usize>,
    offset: usize,
    rendered_rows: usize,
    rendered_width: u16,
    column_mode: ColumnMode,
    gutter_mode: GutterMode,
    /// Active cell flashes: (row, col) → (kind, expiry instant).
    flashes: HashMap<(usize, usize), (FlashKind, Instant)>,
    flash_duration: Duration,
    flash_update_color: Color,
}

impl Table {
    pub fn new(df: DataFrame) -> Self {
        let col_space = 1;
        let col_widths = df
            .widths()
            .into_iter()
            .map(|u| Constraint::Length(u as u16))
            .collect_vec();
        let col_offsets = col_offsets(&col_widths, col_space);
        let gutter_width = df.height().to_string().len() as u16;
        Self {
            col_widths,
            selected: None,
            col_offsets,
            offset: 0,
            rendered_rows: 0,
            rendered_width: 0,
            column_mode: ColumnMode::Compact,
            striped: false,
            show_header: false,
            gutter_mode: GutterMode::Visible(gutter_width),
            df,
            col_space,
            flashes: HashMap::new(),
            flash_duration: Duration::from_millis(750),
            flash_update_color: Color::Yellow,
        }
    }

    pub fn clone_with_data_frame(&self, df: DataFrame) -> Self {
        let col_widths = df
            .widths()
            .into_iter()
            .map(|u| Constraint::Length(u as u16))
            .collect_vec();
        let col_offsets = col_offsets(&col_widths, self.col_space);
        let gutter_width = df.height().to_string().len() as u16;
        Self {
            df,
            col_widths,
            col_offsets,
            offset: 0,
            selected: self.selected,
            rendered_rows: self.rendered_rows,
            rendered_width: self.rendered_width,
            column_mode: self.column_mode,
            striped: self.striped,
            show_header: self.show_header,
            gutter_mode: GutterMode::Visible(gutter_width),
            col_space: self.col_space,
            flashes: HashMap::new(),
            flash_duration: self.flash_duration,
            flash_update_color: self.flash_update_color,
        }
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

    pub fn data_frame(&self) -> &DataFrame {
        &self.df
    }

    pub fn data_frame_mut(&mut self) -> &mut DataFrame {
        &mut self.df
    }

    pub fn set_data_frame(&mut self, df: DataFrame) {
        self.df = df;
        self.refresh_layout();
    }

    /// Recalculate column widths and offsets from the current DataFrame.
    /// Call this after mutating the DataFrame in place (e.g. via streaming
    /// upserts) so the rendered table reflects the new schema/content.
    pub fn refresh_layout(&mut self) {
        self.col_widths = self.df
            .widths()
            .into_iter()
            .map(|u| Constraint::Length(u as u16))
            .collect_vec();
        self.col_offsets = col_offsets(&self.col_widths, self.col_space);
        self.gutter_mode = GutterMode::Visible(self.df.height().to_string().len() as u16);
    }

    pub fn set_flash_duration(&mut self, duration: Duration) {
        self.flash_duration = duration;
    }

    pub fn set_flash_update_color(&mut self, color: Color) {
        self.flash_update_color = color;
    }

    /// Register cell flashes for the given coordinates.
    pub fn flash_cells(&mut self, kind: FlashKind, cells: impl Iterator<Item = (usize, usize)>) {
        let expiry = Instant::now() + self.flash_duration;
        for coord in cells {
            self.flashes.insert(coord, (kind, expiry));
        }
    }

    /// Remove expired flashes. Call on each tick.
    pub fn expire_flashes(&mut self) {
        if self.flashes.is_empty() {
            return;
        }
        let now = Instant::now();
        self.flashes.retain(|_, (_, expiry)| *expiry > now);
    }

    /// Snapshot of active flashes keyed by kind for the render pass.
    fn active_flash_kinds(&self) -> HashMap<(usize, usize), FlashKind> {
        let now = Instant::now();
        self.flashes
            .iter()
            .filter(|(_, (_, expiry))| *expiry > now)
            .map(|(&coord, &(kind, _))| (coord, kind))
            .collect()
    }

    pub fn set_gutter_visibility(&mut self, value: bool) {
        if value {
            self.gutter_mode = GutterMode::Visible(self.df.height().to_string().len() as u16)
        } else {
            self.gutter_mode = GutterMode::Hidden
        }
    }

    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn select(&mut self, idx: impl Into<Option<usize>>) {
        let height = self.df.height();
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
            self.select(self.df.height().saturating_sub(1));
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
        self.select(self.df.height().saturating_sub(1));
    }

    fn page_up(&mut self) {
        if let Some(selected) = self.selected {
            self.select(selected.saturating_sub(self.rendered_rows));
        }
    }

    fn page_down(&mut self) {
        if let Some(selected) = self.selected {
            self.select(selected.saturating_add(self.rendered_rows));
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
            self.select(selected.saturating_sub(self.rendered_rows.div(2)));
        }
    }

    fn half_page_down(&mut self) {
        if let Some(selected) = self.selected {
            self.select(selected.saturating_add(self.rendered_rows.div(2)));
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

    fn minimum_compact_width(&self) -> u16 {
        let col_count = self.df.width() as u16;
        col_count + (col_count.saturating_sub(1) * self.col_space)
    }
}

impl Component for Table {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: super::component::FocusState,
    ) {
        // Nothing to render for an empty placeholder (e.g. streaming tab
        // before the first schema event arrives).
        if self.df.width() == 0 {
            return;
        }

        let height = if self.show_header {
            area.height.saturating_sub(1)
        } else {
            area.height
        } as usize;
        self.rendered_rows = height;
        self.rendered_width = area.width;

        self.selected = self.selected.map(|selected| selected.min(self.df.height()));

        if let Some(selected) = self.selected {
            self.offset = self
                .offset
                .clamp(selected.saturating_sub(height.saturating_sub(1)), selected)
                .min(self.df.height().saturating_sub(height));
        } else {
            self.offset = self.offset.min(self.df.height().saturating_sub(height))
        }

        let (gutter_area, table_area) = self.gutter_table_area(area);

        if let Some(gutter_area) = gutter_area {
            List::default()
                .items(
                    (self.offset..(self.offset + height).min(self.df.height()))
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

        let active_flashes = self.active_flash_kinds();

        match &mut self.column_mode {
            ColumnMode::Compact => {
                let df = self.df.slice(self.offset as i64, height);
                let table = build_table(
                    &df,
                    &self.col_widths,
                    self.col_space,
                    self.show_header,
                    self.striped,
                    self.offset,
                    0,
                    &active_flashes,
                    self.flash_update_color,
                );
                table.render(
                    table_area,
                    buf,
                    &mut TableState::default().with_selected(if focus_state.is_focused() {
                        self.selected.map(|s| s.saturating_sub(self.offset))
                    } else {
                        None
                    }),
                );
            }
            ColumnMode::Expanded(x) => {
                let total_width = self
                    .col_offsets
                    .last()
                    .copied()
                    .unwrap_or(0)
                    .max(table_area.width);
                *x = (*x).min(total_width.saturating_sub(table_area.width));
                let col_start = column_index(&self.col_offsets, x);
                let col_end = column_index(&self.col_offsets, &x.add(table_area.width));
                let df = self
                    .df
                    .select(&self.df.get_column_names()[col_start..=col_end])
                    .unwrap()
                    .slice(self.offset as i64, height);
                // Remap flash coordinates: shift column indices for the
                // visible column slice so build_table sees local col indices.
                let remapped_flashes: HashMap<(usize, usize), FlashKind> = active_flashes
                    .iter()
                    .filter(|&(&(_, c), _)| c >= col_start && c <= col_end)
                    .map(|(&(r, c), &kind)| ((r, c - col_start), kind))
                    .collect();
                let table = build_table(
                    &df,
                    &self.col_widths[col_start..=col_end],
                    self.col_space,
                    self.show_header,
                    self.striped,
                    self.offset,
                    col_start,
                    &remapped_flashes,
                    self.flash_update_color,
                );
                let width = (self.col_offsets[col_end + 1] - self.col_offsets[col_start])
                    .max(table_area.width);
                let size = ratatui::layout::Size {
                    width,
                    height: table_area.height,
                };
                let mut scroll_area =
                    ScrollView::new(size).scrollbars_visibility(ScrollbarVisibility::Never);
                scroll_area.render_stateful_widget(
                    table,
                    scroll_area.area(),
                    &mut TableState::default().with_selected(if focus_state.is_focused() {
                        self.selected.map(|s| s.saturating_sub(self.offset))
                    } else {
                        None
                    }),
                );
                scroll_area.render(
                    table_area,
                    buf,
                    &mut ScrollViewState::with_offset(Position {
                        x: x.saturating_sub(
                            self.col_offsets.get(col_start).copied().unwrap_or_default(),
                        ),
                        y: 0,
                    }),
                );
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
            (KeyCode::Char('w'), KeyModifiers::NONE) => {
                self.scroll_to_right_column();
                true
            }
            (KeyCode::Char('b'), KeyModifiers::NONE) => {
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
    Expanded(u16),
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

fn col_offsets(col_widths: &[Constraint], col_space: u16) -> Vec<u16> {
    std::iter::once(0)
        .chain(
            col_widths
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    if i != col_widths.len().saturating_sub(1) {
                        c.value() + col_space
                    } else {
                        c.value()
                    }
                })
                .scan(0, |s, u| {
                    *s += u;
                    Some(*s)
                }),
        )
        .collect_vec()
}

fn column_index(col_offsets: &[u16], offset: &u16) -> usize {
    // col_offsets index: 0    1    2    3    4
    // col_offsets      : 0---10---20---30---40
    // return value     :   0    1    2   (3)-> maximum allowed = col_effsets.len() - 2
    match col_offsets.binary_search(offset) {
        Ok(idx) => idx,
        Err(idx) => idx.saturating_sub(1),
    }
    .min(col_offsets.len().saturating_sub(2))
}
fn prev_column_offset(col_offsets: &[u16], offset: &u16) -> u16 {
    col_offsets
        .get(column_index(col_offsets, &offset.saturating_sub(1)))
        .copied()
        .unwrap_or_default()
}

fn next_column_offset(col_offsets: &[u16], offset: &u16) -> u16 {
    col_offsets
        .get(column_index(col_offsets, offset).saturating_add(1))
        .copied()
        .unwrap_or_default()
}

#[allow(clippy::too_many_arguments)]
fn build_table<'a>(
    df: &'a DataFrame,
    col_widths: &[Constraint],
    col_space: u16,
    show_header: bool,
    striped: bool,
    offset_row: usize,
    offset_col: usize,
    flashes: &HashMap<(usize, usize), FlashKind>,
    update_color: Color,
) -> ratatui::widgets::Table<'a> {
    let mut table = ratatui::widgets::Table::default()
        .widths(col_widths)
        .style(theme().text())
        .row_highlight_style(theme().row_highlighted())
        .column_spacing(col_space)
        .rows(
            df.columns()
                .iter()
                .map(|col| col.as_materialized_series().iter())
                .zip_iters()
                .enumerate()
                .map(|(idx, vals)| {
                    let abs_row = offset_row + idx;
                    let cells: Vec<Cell> = vals
                        .into_iter()
                        .enumerate()
                        .map(|(col_idx, val)| {
                            let mut cell = Cell::new(val.into_single_line());
                            if let Some(kind) = flashes.get(&(abs_row, col_idx)) {
                                cell = cell.style(match kind {
                                    FlashKind::Insert => {
                                        Style::default().bg(Color::Green).fg(Color::Black)
                                    }
                                    FlashKind::Update => {
                                        Style::default().bg(update_color).fg(Color::Black)
                                    }
                                });
                            }
                            cell
                        })
                        .collect();
                    Row::new(cells).style(if striped {
                        theme().row(abs_row)
                    } else {
                        theme().row(0)
                    })
                }),
        );

    if show_header {
        table =
            table.header(
                Row::new(df.columns().iter().enumerate().map(|(i, c)| {
                    Cell::new(c.name().as_str()).style(theme().header(offset_col + i))
                }))
                .style(theme().table_header()),
            )
    }
    table
}
