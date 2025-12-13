use std::ops::Div;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use itertools::Itertools;
use polars::{frame::DataFrame, prelude::AnyValue, series::Series};
use ratatui::{
    layout::{Constraint, Layout, Position, Rect, Size},
    text::Text,
    widgets::{Cell, List, ListItem, ListState, Row, StatefulWidget, TableState},
};
use tui_scrollview::{ScrollView, ScrollViewState, ScrollbarVisibility};

use crate::{
    misc::{
        globals::theme,
        iter_ext::ZipItersExt,
        polars_ext::{AnyValueExt, GetSheetSections, TuiWidths},
        type_ext::ConstraintExt,
    },
    tui::{component::Component, sheet::SheetSection},
};

#[derive(Debug, Clone)]
pub struct Table {
    df: DataFrame,
    col_names: Vec<String>,
    col_widths: Vec<Constraint>,
    col_space: u16,
    selected: Option<usize>,
    offset: usize,
    rendered_rows: usize,
    striped: bool,
    show_header: bool,
    column_mode: ColumnMode,
    gutter_mode: GutterMode,
}

impl Table {
    pub fn new(df: DataFrame) -> Self {
        let col_names = df.iter().map(|ser| ser.name().to_string()).collect();
        let col_widths = df
            .widths()
            .into_iter()
            .map(|u| Constraint::Length(u as u16))
            .collect();
        let gutter_width = df.height().to_string().len() as u16;
        Self {
            col_names,
            col_widths,
            selected: None,
            offset: 0,
            rendered_rows: 0,
            column_mode: ColumnMode::Compact,
            striped: false,
            show_header: false,
            gutter_mode: GutterMode::Visible(gutter_width),
            df,
            col_space: 1,
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
        Self { col_space, ..self }
    }

    pub fn with_visible_gutter(self) -> Self {
        Self {
            gutter_mode: GutterMode::Visible(self.df.height().to_string().len() as u16),
            ..self
        }
    }

    pub fn with_hidden_gutter(self) -> Self {
        Self {
            gutter_mode: GutterMode::Hidden,
            ..self
        }
    }

    pub fn with_compaect_column(self) -> Self {
        Self {
            column_mode: ColumnMode::Compact,
            ..self
        }
    }

    pub fn with_extended_column(self) -> Self {
        Self {
            column_mode: ColumnMode::Expanded(Default::default()),
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
    }

    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn selected_sheet_sections(&self) -> Option<Vec<SheetSection>> {
        self.selected.map(|idx| self.df.get_sheet_sections(idx))
    }

    pub fn select(&mut self, idx: impl Into<Option<usize>>) {
        let height = self.df.height();
        if height > 0 {
            self.selected = idx.into().map(|idx| idx.min(height - 1));
        }
    }

    pub fn select_up(&mut self) {
        if let Some(selected) = self.selected {
            self.select(selected.saturating_sub(1));
        } else {
            self.select(self.df.height().saturating_sub(1));
        }
    }

    pub fn select_down(&mut self) {
        if let Some(selected) = self.selected {
            self.select(selected.saturating_add(1));
        } else {
            self.select(0);
        }
    }

    pub fn toggle_view_mode(&mut self) {
        self.column_mode = match self.column_mode {
            ColumnMode::Compact => ColumnMode::Expanded(0),
            ColumnMode::Expanded(_) => ColumnMode::Compact,
        }
    }

    pub fn select_first(&mut self) {
        self.select(0);
    }

    pub fn select_last(&mut self) {
        self.select(self.df.height().saturating_sub(1));
    }

    pub fn page_up(&mut self) {
        if let Some(selected) = self.selected {
            self.select(selected.saturating_sub(self.rendered_rows));
        }
    }

    pub fn page_down(&mut self) {
        if let Some(selected) = self.selected {
            self.select(selected.saturating_add(self.rendered_rows));
        }
    }

    pub fn scroll_left(&mut self) {
        if let ColumnMode::Expanded(st) = &mut self.column_mode {
            *st = st.saturating_sub(1)
        }
    }

    pub fn scroll_right(&mut self) {
        if let ColumnMode::Expanded(st) = &mut self.column_mode {
            *st = st.saturating_add(1)
        }
    }

    pub fn scroll_to_prev_column(&mut self) {
        if let ColumnMode::Expanded(offset) = &mut self.column_mode {
            *offset = self
                .col_widths
                .iter()
                .map(|c| c.value())
                .scan(0, |s, u| {
                    *s += u + self.col_space;
                    Some(*s)
                })
                .take_while(|u| u < offset)
                .last()
                .unwrap_or(0);
        }
    }

    pub fn scroll_to_next_column(&mut self) {
        if let ColumnMode::Expanded(offset) = &mut self.column_mode {
            self.col_widths
                .iter()
                .map(|c| c.value())
                .scan(0, |s, u| {
                    *s += u + self.col_space;
                    Some(*s)
                })
                .collect_vec();
            *offset = self
                .col_widths
                .iter()
                .map(|c| c.value())
                .scan(0, |s, u| {
                    *s += u + self.col_space;
                    Some(*s)
                })
                .find(|u| &*offset < u)
                .unwrap_or(0);
        }
    }

    pub fn half_page_up(&mut self) {
        if let Some(selected) = self.selected {
            self.select(selected.saturating_sub(self.rendered_rows.div(2)));
        }
    }

    pub fn half_page_down(&mut self) {
        if let Some(selected) = self.selected {
            self.select(selected.saturating_add(self.rendered_rows.div(2)));
        }
    }

    pub fn expended_column(&self) -> bool {
        match self.column_mode {
            ColumnMode::Compact => false,
            ColumnMode::Expanded(_) => true,
        }
    }

    fn header_row(&self) -> Row<'static> {
        Row::new(
            self.col_names
                .iter()
                .cloned()
                .enumerate()
                .map(|(i, d)| Cell::new(d).style(theme().header(i))),
        )
        .style(theme().table_header())
    }

    fn row<'a>(&self, idx: usize, vals: Vec<AnyValue<'a>>) -> Row<'a> {
        let cells = vals
            .into_iter()
            .map(|val| Cell::new(val.into_single_line()));
        Row::new(cells).style(if self.striped {
            theme().row(self.offset + idx)
        } else {
            theme().row(0)
        })
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

    fn required_width(&self) -> u16 {
        let spaces = self.col_space * self.col_widths.len().saturating_sub(1) as u16;
        let columns = self.col_widths.iter().map(|c| c.value()).sum::<u16>();
        columns + spaces
    }
}

impl Component for Table {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        _focus_state: super::component::FocusState,
    ) {
        let length = if self.show_header {
            area.height.saturating_sub(1)
        } else {
            area.height
        } as usize;
        self.rendered_rows = length;

        if let Some(selected) = self.selected {
            self.offset = self
                .offset
                .clamp(selected.saturating_sub(length.saturating_sub(1)), selected);
        } else {
            self.offset = self.offset.min(self.df.height().saturating_sub(length))
        }

        let slice = self.df.slice(self.offset as i64, length);

        let (gutter_area, table_area) = self.gutter_table_area(area);

        if let Some(gutter_area) = gutter_area {
            List::default()
                .items((self.offset..(self.offset + length)).map(|idx| self.gutter_item(idx)))
                .highlight_style(theme().row_highlighted())
                .render(
                    gutter_area,
                    buf,
                    &mut ListState::default()
                        .with_selected(self.selected.map(|s| s.saturating_sub(self.offset))),
                );
        }

        let mut table = ratatui::widgets::Table::default()
            .widths(&self.col_widths)
            .style(theme().text())
            .row_highlight_style(theme().row_highlighted())
            .column_spacing(self.col_space)
            .rows(
                slice
                    .iter()
                    .map(Series::iter)
                    .zip_iters()
                    .enumerate()
                    .map(|(idx, vals)| self.row(idx, vals)),
            );

        if self.show_header {
            table = table.header(self.header_row())
        }
        let width = self.required_width().max(table_area.width);
        match &mut self.column_mode {
            ColumnMode::Compact => {
                table.render(
                    table_area,
                    buf,
                    &mut TableState::default()
                        .with_selected(self.selected.map(|s| s.saturating_sub(self.offset))),
                );
            }
            ColumnMode::Expanded(x) => {
                *x = (*x).min(width.saturating_sub(table_area.width));
                let mut scroll_area = ScrollView::new(Size {
                    width,
                    height: table_area.height,
                })
                .scrollbars_visibility(ScrollbarVisibility::Never);
                scroll_area.render_stateful_widget(
                    table,
                    scroll_area.area(),
                    &mut TableState::default()
                        .with_selected(self.selected.map(|s| s.saturating_sub(self.offset))),
                );
                scroll_area.render(
                    table_area,
                    buf,
                    &mut ScrollViewState::with_offset(Position { x: *x, y: 0 }),
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
                self.scroll_to_next_column();
                true
            }
            (KeyCode::Char('b'), KeyModifiers::NONE) => {
                self.scroll_to_prev_column();
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
