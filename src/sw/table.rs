use std::ops::{Add, Div};

use itertools::Itertools;
use polars::frame::DataFrame;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Position, Rect},
    text::Text,
    widgets::{
        Cell, List, ListItem, ListState, Row, StatefulWidget, TableState as RatatuiTableState,
    },
};
use tui_scrollview::{ScrollView, ScrollViewState, ScrollbarVisibility};

use crate::misc::{
    config::theme,
    iter_ext::ZipItersExt,
    polars_ext::{AnyValueExt, DataFrameExt},
    type_ext::ConstraintExt,
};

#[derive(Debug, Clone, Copy)]
enum ColumnMode {
    Compact,
    Expanded(usize),
}

#[derive(Debug, Clone)]
pub struct TableState {
    df: DataFrame,
    col_widths: Vec<Constraint>,
    col_offsets: Vec<usize>,
    col_space: u16,
    selected: Option<usize>,
    offset: usize,
    rendered_rows: usize,
    rendered_width: u16,
    column_mode: ColumnMode,
    gutter: bool,
}

impl TableState {
    pub fn new(df: DataFrame) -> Self {
        let col_space = 1;
        let col_widths = column_widths(&df);
        let col_offsets = col_offsets(&col_widths, col_space);
        Self {
            df,
            col_widths,
            col_offsets,
            col_space,
            selected: None,
            offset: 0,
            rendered_rows: 0,
            rendered_width: 0,
            column_mode: ColumnMode::Compact,
            gutter: true,
        }
    }

    pub fn clone_with_data_frame(&self, df: DataFrame) -> Self {
        let col_widths = column_widths(&df);
        let col_offsets = col_offsets(&col_widths, self.col_space);
        Self {
            df,
            col_widths,
            col_offsets,
            col_space: self.col_space,
            selected: self.selected,
            offset: 0,
            rendered_rows: self.rendered_rows,
            rendered_width: self.rendered_width,
            column_mode: self.column_mode,
            gutter: self.gutter,
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
            col_offsets: col_offsets(&self.col_widths, col_space),
            col_space,
            ..self
        }
    }

    pub fn with_compact_column(self) -> Self {
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
        if self.df.schema_equal(&df).is_ok() {
            self.df = df;
        }
    }

    pub fn set_gutter_visibility(&mut self, value: bool) {
        self.gutter = value;
    }

    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn offset(&mut self, idx: impl Into<usize>) {
        self.offset = idx.into().min(self.df.height());
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

    pub fn select_first(&mut self) {
        self.select(0);
    }

    pub fn select_last(&mut self) {
        self.select(self.df.height().saturating_sub(1));
    }

    pub fn page_up(&mut self) {
        if let Some(selected) = self.selected {
            self.select(selected.saturating_sub(self.rendered_rows));
            self.offset(self.offset.saturating_sub(self.rendered_rows));
        }
    }

    pub fn page_down(&mut self) {
        if let Some(selected) = self.selected {
            self.select(selected.saturating_add(self.rendered_rows));
            self.offset(self.offset.saturating_add(self.rendered_rows));
        }
    }

    pub fn half_page_up(&mut self) {
        if let Some(selected) = self.selected {
            let len = self.rendered_rows.div(2);
            self.select(selected.saturating_sub(len));
            self.offset(self.offset.saturating_sub(len));
        }
    }

    pub fn half_page_down(&mut self) {
        if let Some(selected) = self.selected {
            let len = self.rendered_rows.div(2);
            self.select(selected.saturating_add(len));
            self.offset(self.offset.saturating_add(len));
        }
    }

    pub fn scroll_left(&mut self) {
        if let ColumnMode::Expanded(offset) = &mut self.column_mode {
            *offset = offset.saturating_sub(1)
        }
    }

    pub fn scroll_right(&mut self) {
        if let ColumnMode::Expanded(offset) = &mut self.column_mode {
            *offset = offset.saturating_add(1)
        }
    }

    pub fn scroll_to_left_column(&mut self) {
        if let ColumnMode::Expanded(offset) = &mut self.column_mode {
            *offset = prev_column_offset(&self.col_offsets, offset);
        }
    }

    pub fn scroll_to_right_column(&mut self) {
        if let ColumnMode::Expanded(offset) = &mut self.column_mode {
            *offset = next_column_offset(&self.col_offsets, offset);
        }
    }

    pub fn scroll_to_first_column(&mut self) {
        if let ColumnMode::Expanded(offset) = &mut self.column_mode {
            *offset = 0;
        }
    }

    pub fn scroll_to_last_column(&mut self) {
        if let ColumnMode::Expanded(offset) = &mut self.column_mode {
            *offset = self.col_offsets.last().copied().unwrap_or(0);
        }
    }

    fn gutter_width(&self) -> Option<u16> {
        self.gutter
            .then(|| self.df.height().to_string().len() as u16)
    }

    fn minimum_compact_width(&self) -> u16 {
        let col_count = self.df.width() as u16;
        col_count + (col_count.saturating_sub(1) * self.col_space)
    }
}

#[derive(Debug)]
pub struct Table {
    striped: bool,
    show_header: bool,
    selection: bool,
}

impl Table {
    pub fn striped(mut self) -> Self {
        self.striped = true;
        self
    }

    pub fn show_header(mut self, show_header: bool) -> Self {
        self.show_header = show_header;
        self
    }

    pub fn selection(mut self, selection: bool) -> Self {
        self.selection = selection;
        self
    }
}

impl Default for Table {
    fn default() -> Self {
        Self {
            striped: false,
            show_header: false,
            selection: true,
        }
    }
}

impl StatefulWidget for Table {
    type State = TableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let height = if self.show_header {
            area.height.saturating_sub(1)
        } else {
            area.height
        } as usize;
        state.rendered_rows = height;
        state.rendered_width = area.width;

        state.selected = state
            .selected
            .map(|selected| selected.min(state.df.height().saturating_sub(1)));

        if let Some(selected) = state.selected {
            state.offset = state
                .offset
                .clamp(selected.saturating_sub(height.saturating_sub(1)), selected)
                .min(state.df.height().saturating_sub(height));
        } else {
            state.offset = state.offset.min(state.df.height().saturating_sub(height))
        }

        let gutter_width = state.gutter_width();
        let (gutter_area, table_area) = gutter_table_area(area, gutter_width, self.show_header);
        let highlighted = |state: &TableState| {
            self.selection
                .then(|| state.selected.map(|s| s.saturating_sub(state.offset)))
                .flatten()
        };

        if let (Some(gutter_area), Some(gutter_width)) = (gutter_area, gutter_width) {
            List::default()
                .items(
                    (state.offset..(state.offset + height).min(state.df.height()))
                        .map(|idx| gutter_item(idx, gutter_width)),
                )
                .highlight_style(theme().row_highlighted())
                .render(
                    gutter_area,
                    buf,
                    &mut ListState::default().with_selected(highlighted(state)),
                );
        }

        if table_area.width < state.minimum_compact_width()
            && matches!(state.column_mode, ColumnMode::Compact)
        {
            state.column_mode = ColumnMode::Expanded(0);
        }

        let selected = highlighted(state);
        match &mut state.column_mode {
            ColumnMode::Compact => {
                let df = state.df.slice(state.offset as i64, height);
                build_table(
                    &df,
                    &state.col_widths,
                    state.col_space,
                    self.show_header,
                    self.striped,
                    state.offset,
                    0,
                )
                .render(
                    table_area,
                    buf,
                    &mut RatatuiTableState::default().with_selected(selected),
                );
            }
            ColumnMode::Expanded(x) => {
                if state.df.columns().is_empty() {
                    return;
                }
                let total_width = state
                    .col_offsets
                    .last()
                    .copied()
                    .unwrap_or(0)
                    .max(table_area.width as usize);
                *x = (*x).min(total_width.saturating_sub(table_area.width as usize));
                let col_start = column_index(&state.col_offsets, x);
                let col_end = column_index(&state.col_offsets, &x.add(table_area.width as usize));
                let df = state
                    .df
                    .select(&state.df.get_column_names()[col_start..=col_end])
                    .unwrap()
                    .slice(state.offset as i64, height);
                let table = build_table(
                    &df,
                    &state.col_widths[col_start..=col_end],
                    state.col_space,
                    self.show_header,
                    self.striped,
                    state.offset,
                    col_start,
                );
                let width = (state.col_offsets[col_end + 1] - state.col_offsets[col_start])
                    .max(table_area.width as usize);
                let size = ratatui::layout::Size {
                    width: width as u16,
                    height: table_area.height,
                };
                let mut scroll_area =
                    ScrollView::new(size).scrollbars_visibility(ScrollbarVisibility::Never);
                scroll_area.render_stateful_widget(
                    table,
                    scroll_area.area(),
                    &mut RatatuiTableState::default().with_selected(selected),
                );
                scroll_area.render(
                    table_area,
                    buf,
                    &mut ScrollViewState::with_offset(Position {
                        x: x.saturating_sub(
                            state.col_offsets.get(col_start).copied().unwrap_or_default(),
                        ) as u16,
                        y: 0,
                    }),
                );
            }
        }
    }
}

fn column_widths(df: &DataFrame) -> Vec<Constraint> {
    df.widths()
        .into_iter()
        .map(|u| Constraint::Length(u as u16))
        .collect_vec()
}

fn gutter_item<'a>(idx: usize, width: u16) -> ListItem<'a> {
    ListItem::new(Text::raw(format!("  {:>w$}  ", idx + 1, w = width as usize)))
        .style(theme().gutter(idx))
}

fn gutter_table_area(
    area: Rect,
    gutter_width: Option<u16>,
    show_header: bool,
) -> (Option<Rect>, Rect) {
    let Some(width) = gutter_width else {
        return (None, area);
    };
    let [gutter_area, table_area] =
        Layout::horizontal([Constraint::Length(width + 4), Constraint::Fill(1)]).areas(area);
    if show_header {
        let [_, gutter_area] =
            Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).areas(gutter_area);
        (Some(gutter_area), table_area)
    } else {
        (Some(gutter_area), table_area)
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
                        c.value() + col_space
                    } else {
                        c.value()
                    }
                })
                .scan(0, |s, u| {
                    *s += u as usize;
                    Some(*s)
                }),
        )
        .collect_vec()
}

fn column_index(col_offsets: &[usize], offset: &usize) -> usize {
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

fn build_table<'a>(
    df: &'a DataFrame,
    col_widths: &[Constraint],
    col_space: u16,
    show_header: bool,
    striped: bool,
    offset_row: usize,
    offset_col: usize,
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
                    let cells = vals
                        .into_iter()
                        .map(|val| Cell::new(val.into_single_line()));
                    Row::new(cells).style(if striped {
                        theme().row(offset_row + idx)
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

#[cfg(test)]
mod tests {
    use super::*;
    use polars::prelude::Column;

    fn frame(rows: usize) -> DataFrame {
        DataFrame::new_infer_height(vec![
            Column::new(
                "id".into(),
                (0..rows).map(|i| i as i64).collect::<Vec<_>>(),
            ),
            Column::new(
                "name".into(),
                (0..rows).map(|i| format!("row{i}")).collect::<Vec<_>>(),
            ),
        ])
        .unwrap()
    }

    fn state(rows: usize) -> TableState {
        TableState::new(frame(rows))
    }

    fn render(state: &mut TableState, table: Table, width: u16, height: u16) -> Buffer {
        let area = Rect::new(0, 0, width, height);
        let mut buf = Buffer::empty(area);
        table.render(area, &mut buf, state);
        buf
    }

    fn content(buf: &Buffer) -> String {
        buf.content().iter().map(|c| c.symbol()).collect()
    }

    mod offsets {
        use super::*;

        fn widths(lengths: [u16; 3]) -> Vec<Constraint> {
            lengths.into_iter().map(Constraint::Length).collect()
        }

        #[test]
        fn col_offsets_accumulate_widths_plus_spacing_except_the_last() {
            assert_eq!(col_offsets(&widths([10, 10, 10]), 1), vec![0, 11, 22, 32]);
        }

        #[test]
        fn col_offsets_without_spacing_are_plain_prefix_sums() {
            assert_eq!(col_offsets(&widths([10, 10, 10]), 0), vec![0, 10, 20, 30]);
        }

        #[test]
        fn col_offsets_of_no_columns_is_just_the_origin() {
            assert_eq!(col_offsets(&[], 1), vec![0]);
        }

        #[test]
        fn column_index_maps_an_exact_boundary_to_that_column() {
            let offsets = vec![0, 10, 20, 30];
            assert_eq!(column_index(&offsets, &0), 0);
            assert_eq!(column_index(&offsets, &10), 1);
        }

        #[test]
        fn column_index_maps_a_mid_column_offset_to_the_column_it_lands_in() {
            let offsets = vec![0, 10, 20, 30];
            assert_eq!(column_index(&offsets, &5), 0);
            assert_eq!(column_index(&offsets, &15), 1);
        }

        #[test]
        fn column_index_is_clamped_to_the_last_real_column() {
            let offsets = vec![0, 10, 20, 30];
            assert_eq!(column_index(&offsets, &30), 2);
            assert_eq!(column_index(&offsets, &999), 2);
        }

        #[test]
        fn next_column_offset_steps_to_the_following_boundary() {
            let offsets = vec![0, 10, 20, 30];
            assert_eq!(next_column_offset(&offsets, &0), 10);
            assert_eq!(next_column_offset(&offsets, &5), 10);
            assert_eq!(next_column_offset(&offsets, &10), 20);
        }

        #[test]
        fn prev_column_offset_steps_to_the_preceding_boundary() {
            let offsets = vec![0, 10, 20, 30];
            assert_eq!(prev_column_offset(&offsets, &20), 10);
            assert_eq!(prev_column_offset(&offsets, &15), 10);
            assert_eq!(prev_column_offset(&offsets, &0), 0);
        }
    }

    mod selection {
        use super::*;

        #[test]
        fn select_clamps_to_the_last_row() {
            let mut state = state(5);
            state.select(99);
            assert_eq!(state.selected(), Some(4));
        }

        #[test]
        fn select_is_a_no_op_on_an_empty_frame() {
            let mut state = state(0);
            state.select(3);
            assert_eq!(state.selected(), None);
        }

        #[test]
        fn select_up_from_nothing_selects_the_last_row() {
            let mut state = state(5);
            state.select_up();
            assert_eq!(state.selected(), Some(4));
        }

        #[test]
        fn select_down_from_nothing_selects_the_first_row() {
            let mut state = state(5);
            state.select_down();
            assert_eq!(state.selected(), Some(0));
        }

        #[test]
        fn select_up_and_down_saturate_at_the_edges() {
            let mut state = state(5);
            state.select(0);
            state.select_up();
            assert_eq!(state.selected(), Some(0));

            state.select(4);
            state.select_down();
            assert_eq!(state.selected(), Some(4));
        }

        #[test]
        fn select_first_and_last() {
            let mut state = state(5);
            state.select_last();
            assert_eq!(state.selected(), Some(4));
            state.select_first();
            assert_eq!(state.selected(), Some(0));
        }

        #[test]
        fn offset_is_clamped_to_the_frame_height() {
            let mut state = state(5);
            state.offset(99usize);
            state.select(0);
            render(&mut state, Table::default(), 40, 10);
            assert_eq!(state.selected(), Some(0));
        }

        #[test]
        fn paging_moves_selection_by_the_rendered_row_count() {
            let mut state = state(100);
            state.select(0);
            render(&mut state, Table::default(), 40, 10);

            state.page_down();
            assert_eq!(state.selected(), Some(10));
            state.page_up();
            assert_eq!(state.selected(), Some(0));
        }

        #[test]
        fn half_paging_moves_selection_by_half_the_rendered_row_count() {
            let mut state = state(100);
            state.select(0);
            render(&mut state, Table::default(), 40, 10);

            state.half_page_down();
            assert_eq!(state.selected(), Some(5));
            state.half_page_up();
            assert_eq!(state.selected(), Some(0));
        }

        #[test]
        fn paging_does_nothing_without_a_selection() {
            let mut state = state(100);
            render(&mut state, Table::default(), 40, 10);

            state.page_down();
            assert_eq!(state.selected(), None);
        }
    }

    mod column_mode {
        use super::*;

        #[test]
        fn new_state_starts_compact() {
            assert!(!state(5).expended_column());
        }

        #[test]
        fn with_extended_column_starts_expanded() {
            assert!(state(5).with_extended_column().expended_column());
        }

        #[test]
        fn toggle_switches_compact_to_expanded() {
            let mut state = state(5);
            state.toggle_view_mode();
            assert!(state.expended_column());
        }

        #[test]
        fn toggle_returns_to_compact_only_when_the_table_fits() {
            let mut state = state(5).with_extended_column();
            render(&mut state, Table::default(), 40, 10);
            assert!(state.fits_in_page());

            state.toggle_view_mode();
            assert!(!state.expended_column());
        }

        #[test]
        fn toggle_stays_expanded_when_the_table_does_not_fit() {
            let mut state = state(5).with_extended_column();
            state.rendered_width = 1;

            state.toggle_view_mode();
            assert!(state.expended_column());
        }

        #[test]
        fn horizontal_scroll_is_ignored_in_compact_mode() {
            let mut state = state(5);
            state.scroll_right();
            assert!(!state.expended_column());
        }

        #[test]
        fn scroll_to_last_column_jumps_to_the_final_offset() {
            let mut state = state(5).with_extended_column();
            let last = *state.col_offsets.last().unwrap();

            state.scroll_to_last_column();
            assert!(matches!(state.column_mode, ColumnMode::Expanded(x) if x == last));

            state.scroll_to_first_column();
            assert!(matches!(state.column_mode, ColumnMode::Expanded(0)));
        }

        #[test]
        fn narrow_area_forces_expanded_mode_during_render() {
            let mut state = state(5);
            render(&mut state, Table::default(), 1, 10);
            assert!(state.expended_column());
        }
    }

    mod widget {
        use super::*;

        #[test]
        fn renders_cell_values() {
            let mut state = state(3);
            let buf = render(&mut state, Table::default(), 40, 10);

            let content = content(&buf);
            assert!(content.contains("row0"));
            assert!(content.contains("row2"));
        }

        #[test]
        fn renders_the_header_when_asked() {
            let mut state = state(3);
            let buf = render(&mut state, Table::default().show_header(true), 40, 10);

            assert!(content(&buf).contains("name"));
        }

        #[test]
        fn hides_the_header_by_default() {
            let mut state = state(3);
            let buf = render(&mut state, Table::default(), 40, 10);

            assert!(!content(&buf).contains("name"));
        }

        #[test]
        fn renders_gutter_row_numbers() {
            let mut state = state(3);
            let buf = render(&mut state, Table::default(), 40, 10);

            let content = content(&buf);
            assert!(content.contains("1"));
            assert!(content.contains("3"));
        }

        #[test]
        fn hidden_gutter_gives_the_table_the_full_width() {
            let mut state = state(3);
            state.set_gutter_visibility(false);
            assert_eq!(state.gutter_width(), None);

            let (gutter, table) = gutter_table_area(Rect::new(0, 0, 40, 10), None, false);
            assert_eq!(gutter, None);
            assert_eq!(table, Rect::new(0, 0, 40, 10));
        }

        #[test]
        fn rendered_rows_excludes_the_header_row() {
            let mut state = state(50);
            render(&mut state, Table::default(), 40, 10);
            assert_eq!(state.rendered_rows, 10);

            render(&mut state, Table::default().show_header(true), 40, 10);
            assert_eq!(state.rendered_rows, 9);
        }

        #[test]
        fn scroll_offset_follows_the_selection_downward() {
            let mut state = state(50);
            state.select(30);
            render(&mut state, Table::default(), 40, 10);

            assert_eq!(state.offset, 21);
        }

        #[test]
        fn scroll_offset_follows_the_selection_upward() {
            let mut state = state(50);
            state.select(40);
            render(&mut state, Table::default(), 40, 10);
            state.select(5);
            render(&mut state, Table::default(), 40, 10);

            assert_eq!(state.offset, 5);
        }

        #[test]
        fn selection_out_of_range_is_clamped_during_render() {
            let mut state = state(50);
            state.select(40);
            state.set_data_frame(frame(10));
            render(&mut state, Table::default(), 40, 10);

            assert_eq!(state.selected(), Some(9));
        }

        #[test]
        fn gutter_width_tracks_the_row_count() {
            assert_eq!(state(9).gutter_width(), Some(1));
            assert_eq!(state(10).gutter_width(), Some(2));
            assert_eq!(state(100).gutter_width(), Some(3));
        }

        #[test]
        fn empty_frame_renders_without_panicking() {
            let mut state = state(0);
            render(&mut state, Table::default().show_header(true), 40, 10);
            assert_eq!(state.selected(), None);
        }

        #[test]
        fn no_column_frame_renders_without_panicking() {
            let mut state = TableState::new(DataFrame::empty()).with_extended_column();
            render(&mut state, Table::default(), 40, 10);
            assert_eq!(state.selected(), None);
        }
    }
}
