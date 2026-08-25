use std::ops::{Add, Div};

use itertools::Itertools;
use polars::{frame::DataFrame, prelude::SchemaRef};
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

#[derive(Debug, Default, Clone)]
pub struct TableState {
    schema: Option<SchemaRef>,
    col_space: Option<u16>,
    col_widths: Vec<Constraint>,
    col_offsets: Vec<usize>,
    selected: Option<usize>,
    offset: usize,
    col_offset: usize,
    df_height: usize,
    rendered_rows: usize,
    rendered_width: u16,
    min_compact_width: u16,
}

impl TableState {
    pub fn with_selected(self, selected: impl Into<Option<usize>>) -> Self {
        Self {
            selected: selected.into(),
            ..self
        }
    }

    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn offset(&mut self, idx: impl Into<usize>) {
        self.offset = idx.into().min(self.df_height);
    }

    pub fn select(&mut self, idx: impl Into<Option<usize>>) {
        if self.df_height > 0 {
            self.selected = idx.into().map(|idx| idx.min(self.df_height - 1));
        }
    }

    pub fn fits_in_page(&self) -> bool {
        self.min_compact_width <= self.rendered_width
    }

    pub fn select_up(&mut self) {
        if let Some(selected) = self.selected {
            self.select(selected.saturating_sub(1));
        } else {
            self.select(self.df_height.saturating_sub(1));
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
        self.select(self.df_height.saturating_sub(1));
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

    pub fn col_offset(&self) -> usize {
        self.col_offset
    }

    pub fn scroll_left(&mut self) {
        self.col_offset = self.col_offset.saturating_sub(1);
    }

    pub fn scroll_right(&mut self) {
        self.col_offset = self.col_offset.saturating_add(1);
    }

    pub fn scroll_to_left_column(&mut self) {
        self.col_offset = prev_column_offset(&self.col_offsets, &self.col_offset);
    }

    pub fn scroll_to_right_column(&mut self) {
        self.col_offset = next_column_offset(&self.col_offsets, &self.col_offset);
    }

    pub fn scroll_to_first_column(&mut self) {
        self.col_offset = 0;
    }

    pub fn scroll_to_last_column(&mut self) {
        self.col_offset = self.col_offsets.last().copied().unwrap_or(0);
    }

    fn sync(&mut self, df: &DataFrame, col_space: u16) {
        let new_schema = self.schema.as_ref() != Some(df.schema());
        if new_schema {
            self.schema = Some(df.schema().clone());
            self.col_widths = column_widths(df);
        }
        if new_schema || self.col_space != Some(col_space) {
            self.col_space = Some(col_space);
            self.col_offsets = col_offsets(&self.col_widths, col_space);
            self.min_compact_width = minimum_compact_width(df.width(), col_space);
        }
        self.df_height = df.height();
    }
}

#[derive(Debug)]
pub struct Table<'a> {
    df: &'a DataFrame,
    col_space: u16,
    striped: bool,
    show_header: bool,
    selection: bool,
    gutter: bool,
    expanded: bool,
}

impl<'a> Table<'a> {
    pub fn new(df: &'a DataFrame) -> Self {
        Self {
            df,
            col_space: 1,
            striped: false,
            show_header: false,
            selection: true,
            gutter: true,
            expanded: false,
        }
    }

    pub fn col_space(mut self, col_space: u16) -> Self {
        self.col_space = col_space;
        self
    }

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

    pub fn gutter(mut self, gutter: bool) -> Self {
        self.gutter = gutter;
        self
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }
}

impl StatefulWidget for Table<'_> {
    type State = TableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let df = self.df;
        state.sync(df, self.col_space);

        let height = if self.show_header {
            area.height.saturating_sub(1)
        } else {
            area.height
        } as usize;
        state.rendered_rows = height;
        state.rendered_width = area.width;

        state.selected = state
            .selected
            .map(|selected| selected.min(df.height().saturating_sub(1)));

        if let Some(selected) = state.selected {
            state.offset = state
                .offset
                .clamp(selected.saturating_sub(height.saturating_sub(1)), selected)
                .min(df.height().saturating_sub(height));
        } else {
            state.offset = state.offset.min(df.height().saturating_sub(height))
        }

        let gutter_width = self.gutter.then(|| df.height().to_string().len() as u16);
        let (gutter_area, table_area) = gutter_table_area(area, gutter_width, self.show_header);
        let highlighted = |state: &TableState| {
            self.selection
                .then(|| state.selected.map(|s| s.saturating_sub(state.offset)))
                .flatten()
        };

        if let (Some(gutter_area), Some(gutter_width)) = (gutter_area, gutter_width) {
            List::default()
                .items(
                    (state.offset..(state.offset + height).min(df.height()))
                        .map(|idx| gutter_item(idx, gutter_width)),
                )
                .highlight_style(theme().row_highlighted())
                .render(
                    gutter_area,
                    buf,
                    &mut ListState::default().with_selected(highlighted(state)),
                );
        }

        let expanded = self.expanded || table_area.width < state.min_compact_width;

        let selected = highlighted(state);
        if !expanded {
            let sliced = df.slice(state.offset as i64, height);
            build_table(
                &sliced,
                &state.col_widths,
                self.col_space,
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
        } else {
            if df.columns().is_empty() {
                return;
            }
            let total_width = state
                .col_offsets
                .last()
                .copied()
                .unwrap_or(0)
                .max(table_area.width as usize);
            state.col_offset = state
                .col_offset
                .min(total_width.saturating_sub(table_area.width as usize));
            let x = &state.col_offset;
            let col_start = column_index(&state.col_offsets, x);
            let col_end = column_index(&state.col_offsets, &x.add(table_area.width as usize));
            let sliced = df
                .select(&df.get_column_names()[col_start..=col_end])
                .unwrap()
                .slice(state.offset as i64, height);
            let table = build_table(
                &sliced,
                &state.col_widths[col_start..=col_end],
                self.col_space,
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
                        state
                            .col_offsets
                            .get(col_start)
                            .copied()
                            .unwrap_or_default(),
                    ) as u16,
                    y: 0,
                }),
            );
        }
    }
}

fn minimum_compact_width(cols: usize, col_space: u16) -> u16 {
    let cols = cols as u16;
    cols + (cols.saturating_sub(1) * col_space)
}

fn column_widths(df: &DataFrame) -> Vec<Constraint> {
    df.widths()
        .into_iter()
        .map(|u| Constraint::Length(u as u16))
        .collect_vec()
}

fn gutter_item<'a>(idx: usize, width: u16) -> ListItem<'a> {
    ListItem::new(Text::raw(format!(
        "  {:>w$}  ",
        idx + 1,
        w = width as usize
    )))
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
            Column::new("id".into(), (0..rows).map(|i| i as i64).collect::<Vec<_>>()),
            Column::new(
                "name".into(),
                (0..rows).map(|i| format!("row{i}")).collect::<Vec<_>>(),
            ),
        ])
        .unwrap()
    }

    fn wide_frame(rows: usize, cols: usize) -> DataFrame {
        DataFrame::new_infer_height(
            (0..cols)
                .map(|c| {
                    Column::new(
                        format!("column{c}").into(),
                        (0..rows).map(|r| format!("v{c}x{r}")).collect::<Vec<_>>(),
                    )
                })
                .collect(),
        )
        .unwrap()
    }

    fn render(state: &mut TableState, table: Table, width: u16, height: u16) -> Buffer {
        let area = Rect::new(0, 0, width, height);
        let mut buf = Buffer::empty(area);
        table.render(area, &mut buf, state);
        buf
    }

    fn synced(df: &DataFrame) -> TableState {
        let mut state = TableState::default();
        render(&mut state, Table::new(df), 40, 10);
        state
    }

    fn content(buf: &Buffer) -> String {
        buf.content().iter().map(|c| c.symbol()).collect()
    }

    mod sync {
        use super::*;

        #[test]
        fn a_default_state_knows_nothing_until_it_renders() {
            let state = TableState::default();
            assert_eq!(state.df_height, 0);
            assert!(state.col_widths.is_empty());
            assert!(state.schema.is_none());
            assert!(state.col_space.is_none());
        }

        #[test]
        fn rendering_adopts_the_frame_schema_and_widths() {
            let df = frame(5);
            let state = synced(&df);

            assert_eq!(state.schema.as_ref(), Some(df.schema()));
            assert_eq!(state.col_widths.len(), 2);
            assert_eq!(state.df_height, 5);
        }

        #[test]
        fn a_new_schema_recomputes_the_widths() {
            let narrow = frame(5);
            let mut state = synced(&narrow);
            let before = state.col_widths.clone();

            let wide = wide_frame(5, 4);
            render(&mut state, Table::new(&wide), 40, 10);

            assert_ne!(state.col_widths, before);
            assert_eq!(state.col_widths.len(), 4);
            assert_eq!(state.schema.as_ref(), Some(wide.schema()));
        }

        #[test]
        fn the_same_schema_keeps_the_cached_widths() {
            let df = frame(5);
            let mut state = synced(&df);
            state.col_widths = vec![Constraint::Length(99), Constraint::Length(99)];

            render(&mut state, Table::new(&frame(7)), 40, 10);

            assert_eq!(
                state.col_widths,
                vec![Constraint::Length(99), Constraint::Length(99)]
            );
            assert_eq!(state.df_height, 7);
        }

        #[test]
        fn a_new_col_space_widens_the_offsets() {
            let df = frame(5);
            let mut state = synced(&df);
            let tight = state.col_offsets.clone();

            render(&mut state, Table::new(&df).col_space(5), 40, 10);

            assert_ne!(state.col_offsets, tight);
            assert!(state.col_offsets[1] > tight[1]);
        }

        #[test]
        fn a_new_col_space_widens_the_minimum_compact_width() {
            let df = frame(5);
            let mut state = synced(&df);
            let tight = state.min_compact_width;

            render(&mut state, Table::new(&df).col_space(5), 40, 10);

            assert!(state.min_compact_width > tight);
        }

        #[test]
        fn an_unchanged_col_space_keeps_the_cached_offsets() {
            let df = frame(5);
            let mut state = synced(&df);
            state.col_offsets = vec![7, 7, 7];
            state.min_compact_width = 77;

            render(&mut state, Table::new(&frame(9)), 40, 10);

            assert_eq!(state.col_offsets, vec![7, 7, 7]);
            assert_eq!(state.min_compact_width, 77);
            assert_eq!(state.df_height, 9);
        }

        #[test]
        fn a_new_schema_recomputes_the_offsets_even_at_the_same_col_space() {
            let narrow = frame(5);
            let mut state = synced(&narrow);
            state.col_offsets = vec![7, 7, 7];

            render(&mut state, Table::new(&wide_frame(5, 4)), 40, 10);

            assert_ne!(state.col_offsets, vec![7, 7, 7]);
            assert_eq!(state.col_offsets.len(), 5);
        }
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

        #[test]
        fn minimum_compact_width_counts_columns_and_spacing() {
            assert_eq!(minimum_compact_width(3, 1), 5);
            assert_eq!(minimum_compact_width(3, 0), 3);
            assert_eq!(minimum_compact_width(0, 1), 0);
        }
    }

    mod selection {
        use super::*;

        #[test]
        fn select_clamps_to_the_last_row() {
            let mut state = synced(&frame(5));
            state.select(99);
            assert_eq!(state.selected(), Some(4));
        }

        #[test]
        fn select_is_a_no_op_on_an_empty_frame() {
            let mut state = synced(&frame(0));
            state.select(3);
            assert_eq!(state.selected(), None);
        }

        #[test]
        fn select_up_from_nothing_selects_the_last_row() {
            let mut state = synced(&frame(5));
            state.select_up();
            assert_eq!(state.selected(), Some(4));
        }

        #[test]
        fn select_down_from_nothing_selects_the_first_row() {
            let mut state = synced(&frame(5));
            state.select_down();
            assert_eq!(state.selected(), Some(0));
        }

        #[test]
        fn select_up_and_down_saturate_at_the_edges() {
            let mut state = synced(&frame(5));
            state.select(0);
            state.select_up();
            assert_eq!(state.selected(), Some(0));

            state.select(4);
            state.select_down();
            assert_eq!(state.selected(), Some(4));
        }

        #[test]
        fn select_first_and_last() {
            let mut state = synced(&frame(5));
            state.select_last();
            assert_eq!(state.selected(), Some(4));
            state.select_first();
            assert_eq!(state.selected(), Some(0));
        }

        #[test]
        fn paging_moves_selection_by_the_rendered_row_count() {
            let df = frame(100);
            let mut state = synced(&df);
            state.select(0);

            state.page_down();
            assert_eq!(state.selected(), Some(10));
            state.page_up();
            assert_eq!(state.selected(), Some(0));
        }

        #[test]
        fn half_paging_moves_selection_by_half_the_rendered_row_count() {
            let df = frame(100);
            let mut state = synced(&df);
            state.select(0);

            state.half_page_down();
            assert_eq!(state.selected(), Some(5));
            state.half_page_up();
            assert_eq!(state.selected(), Some(0));
        }

        #[test]
        fn paging_does_nothing_without_a_selection() {
            let df = frame(100);
            let mut state = synced(&df);

            state.page_down();
            assert_eq!(state.selected(), None);
        }
    }

    mod columns {
        use super::*;

        #[test]
        fn a_wide_area_fits_the_table_in_compact_mode() {
            let state = synced(&frame(5));
            assert!(state.fits_in_page());
        }

        #[test]
        fn a_narrow_area_does_not_fit_the_table() {
            let df = wide_frame(5, 30);
            let mut state = TableState::default();
            render(&mut state, Table::new(&df), 10, 10);
            assert!(!state.fits_in_page());
        }

        #[test]
        fn horizontal_scroll_moves_the_column_offset() {
            let mut state = synced(&frame(5));
            state.scroll_right();
            assert_eq!(state.col_offset(), 1);
            state.scroll_left();
            assert_eq!(state.col_offset(), 0);
        }

        #[test]
        fn horizontal_scroll_saturates_at_zero() {
            let mut state = synced(&frame(5));
            state.scroll_left();
            assert_eq!(state.col_offset(), 0);
        }

        #[test]
        fn scroll_to_last_and_first_column_jump_to_the_edge_offsets() {
            let mut state = synced(&frame(5));
            let last = *state.col_offsets.last().unwrap();

            state.scroll_to_last_column();
            assert_eq!(state.col_offset(), last);

            state.scroll_to_first_column();
            assert_eq!(state.col_offset(), 0);
        }

        #[test]
        fn scroll_to_next_and_prev_column_step_between_boundaries() {
            let mut state = synced(&frame(5));
            let boundary = state.col_offsets[1];

            state.scroll_to_right_column();
            assert_eq!(state.col_offset(), boundary);

            state.scroll_to_left_column();
            assert_eq!(state.col_offset(), 0);
        }

        #[test]
        fn column_offset_is_clamped_to_the_scrollable_width_during_render() {
            let df = frame(5);
            let mut state = synced(&df);
            state.scroll_to_last_column();
            render(&mut state, Table::new(&df).expanded(true), 40, 10);

            assert!(state.col_offset() < *state.col_offsets.last().unwrap());
        }
    }

    mod widget {
        use super::*;

        #[test]
        fn renders_cell_values() {
            let df = frame(3);
            let mut state = TableState::default();
            let buf = render(&mut state, Table::new(&df), 40, 10);

            let content = content(&buf);
            assert!(content.contains("row0"));
            assert!(content.contains("row2"));
        }

        #[test]
        fn renders_the_header_when_asked() {
            let df = frame(3);
            let mut state = TableState::default();
            let buf = render(&mut state, Table::new(&df).show_header(true), 40, 10);

            assert!(content(&buf).contains("name"));
        }

        #[test]
        fn hides_the_header_by_default() {
            let df = frame(3);
            let mut state = TableState::default();
            let buf = render(&mut state, Table::new(&df), 40, 10);

            assert!(!content(&buf).contains("name"));
        }

        #[test]
        fn renders_gutter_row_numbers() {
            let df = frame(3);
            let mut state = TableState::default();
            let buf = render(&mut state, Table::new(&df), 40, 10);

            let content = content(&buf);
            assert!(content.contains("1"));
            assert!(content.contains("3"));
        }

        #[test]
        fn hidden_gutter_gives_the_table_the_full_width() {
            let (gutter, table) = gutter_table_area(Rect::new(0, 0, 40, 10), None, false);
            assert_eq!(gutter, None);
            assert_eq!(table, Rect::new(0, 0, 40, 10));
        }

        #[test]
        fn hidden_gutter_does_not_render_row_numbers() {
            let df = frame(3);
            let mut state = TableState::default();
            let buf = render(&mut state, Table::new(&df).gutter(false), 40, 10);

            let [gutter_area, _] = Layout::horizontal([Constraint::Length(5), Constraint::Fill(1)])
                .areas(Rect::new(0, 0, 40, 10));
            let gutter_cells: String = (gutter_area.x..gutter_area.right())
                .flat_map(|x| (0..10).map(move |y| (x, y)))
                .map(|(x, y)| buf[(x, y)].symbol())
                .collect();
            assert!(!gutter_cells.contains("3"));
        }

        #[test]
        fn rendered_rows_excludes_the_header_row() {
            let df = frame(50);
            let mut state = TableState::default();
            render(&mut state, Table::new(&df), 40, 10);
            assert_eq!(state.rendered_rows, 10);

            render(&mut state, Table::new(&df).show_header(true), 40, 10);
            assert_eq!(state.rendered_rows, 9);
        }

        #[test]
        fn scroll_offset_follows_the_selection_downward() {
            let df = frame(50);
            let mut state = synced(&df);
            state.select(30);
            render(&mut state, Table::new(&df), 40, 10);

            assert_eq!(state.offset, 21);
        }

        #[test]
        fn scroll_offset_follows_the_selection_upward() {
            let df = frame(50);
            let mut state = synced(&df);
            state.select(40);
            render(&mut state, Table::new(&df), 40, 10);
            state.select(5);
            render(&mut state, Table::new(&df), 40, 10);

            assert_eq!(state.offset, 5);
        }

        #[test]
        fn selection_out_of_range_is_clamped_during_render() {
            let df = frame(50);
            let mut state = synced(&df);
            state.select(40);
            render(&mut state, Table::new(&frame(10)), 40, 10);

            assert_eq!(state.selected(), Some(9));
        }

        #[test]
        fn gutter_widens_with_the_row_count() {
            let nine = frame(9);
            let hundred = frame(100);
            let narrow = render(&mut TableState::default(), Table::new(&nine), 40, 12);
            let wide = render(&mut TableState::default(), Table::new(&hundred), 40, 12);

            assert_eq!(content(&narrow).find('1'), Some(2));
            assert_eq!(content(&wide).find('1'), Some(4));
        }

        #[test]
        fn empty_frame_renders_without_panicking() {
            let df = frame(0);
            let mut state = TableState::default();
            render(&mut state, Table::new(&df).show_header(true), 40, 10);
            assert_eq!(state.selected(), None);
        }

        #[test]
        fn no_column_frame_renders_without_panicking() {
            let df = DataFrame::empty();
            let mut state = TableState::default();
            render(&mut state, Table::new(&df).expanded(true), 40, 10);
            assert_eq!(state.selected(), None);
        }
    }
}
