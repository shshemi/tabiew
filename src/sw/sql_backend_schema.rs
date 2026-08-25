use std::ops::Div;

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    text::{Span, Text},
    widgets::{Block, Paragraph, Row, StatefulWidget, Table, TableState, Widget, Wrap},
};

use crate::{
    misc::{
        config::theme,
        sql::{BackendSchema, TableInfo},
        type_ext::human_readable_size,
    },
    sw::{
        app_default::AppDefault,
        buffer_ext::BufferExt,
        widgets::{
            split::Split,
            tag_line::{Tag, TagLine},
        },
    },
};

const NAMES_WIDTH: u16 = 40;
const META_HEIGHT: u16 = 4;
const EMPTY_MESSAGE: &str =
    "No data frame found in the backed. Use the 'import' command to import data frames from files.";

#[derive(Debug)]
pub struct SqlBackendSchemaState {
    names: TableState,
    fields: TableState,
}

impl SqlBackendSchemaState {
    pub fn selected(&self) -> Option<usize> {
        self.names.selected()
    }

    pub fn select(&mut self, idx: impl Into<Option<usize>>) {
        self.names.select(idx.into());
    }

    pub fn select_up(&mut self) {
        self.names.select_previous();
    }

    pub fn select_down(&mut self) {
        self.names.select_next();
    }

    pub fn select_first(&mut self) {
        self.names.select_first();
    }

    pub fn select_last(&mut self) {
        self.names.select_last();
    }

    pub fn field_offset(&self) -> usize {
        self.fields.offset()
    }

    pub fn scroll_up(&mut self) {
        *self.fields.offset_mut() = self.fields.offset().saturating_sub(1);
    }

    pub fn scroll_down(&mut self) {
        *self.fields.offset_mut() = self.fields.offset().saturating_add(1);
    }
}

impl Default for SqlBackendSchemaState {
    fn default() -> Self {
        Self {
            names: TableState::default().with_selected(0),
            fields: TableState::default(),
        }
    }
}

#[derive(Debug)]
pub struct SqlBackendSchema<'a> {
    schema: &'a BackendSchema,
}

impl<'a> SqlBackendSchema<'a> {
    pub fn new(schema: &'a BackendSchema) -> Self {
        Self { schema }
    }
}

impl StatefulWidget for SqlBackendSchema<'_> {
    type State = SqlBackendSchemaState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        buf.set_style(area, theme().text());
        buf.clear(area);

        if self.schema.is_empty() {
            render_empty(area, buf);
            return;
        }

        let [names_area, info_area] =
            Layout::horizontal([Constraint::Length(NAMES_WIDTH), Constraint::Fill(1)]).areas(area);

        render_names(self.schema, names_area, buf, &mut state.names);

        if let Some((_, info)) = state
            .names
            .selected()
            .and_then(|idx| self.schema.get_by_index(idx))
        {
            render_info(info, info_area, buf, &mut state.fields);
        }
    }
}

fn render_empty(area: Rect, buf: &mut Buffer) {
    let paragraph = Paragraph::new(EMPTY_MESSAGE)
        .centered()
        .wrap(Wrap { trim: true });
    let width = area.width.saturating_sub(2).div(3).min(64);
    let lines = paragraph.line_count(width) as u16;
    let [center] = Layout::vertical([Constraint::Length(lines)])
        .flex(Flex::Center)
        .areas(area);
    let [center] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(center);

    Block::app_default().render(area, buf);
    paragraph.render(center, buf);
}

fn render_names(schema: &BackendSchema, area: Rect, buf: &mut Buffer, state: &mut TableState) {
    let num_width = schema.len().to_string().len();

    let table = Table::default()
        .rows(schema.iter().enumerate().map(|(idx, (name, _))| {
            Row::new([
                Span::raw(format!(" {:>width$}", idx + 1, width = num_width))
                    .style(theme().subtext()),
                Span::raw(name.to_owned()).style(theme().text()),
            ])
        }))
        .row_highlight_style(theme().row_highlighted())
        .widths([
            Constraint::Length(num_width as u16 + 1),
            Constraint::Fill(1),
        ])
        .column_spacing(1)
        .block(
            Block::app_default()
                .title("Tables")
                .title_alignment(Alignment::Center)
                .title_bottom(
                    TagLine::mono_color()
                        .centered()
                        .tag(Tag::new(" Open ", " Enter"))
                        .tag(Tag::new(" Unload ", " Delete ")),
                ),
        );

    StatefulWidget::render(table, area, buf, state);
}

fn render_info(info: &TableInfo, area: Rect, buf: &mut Buffer, state: &mut TableState) {
    let [meta_area, fields_area] =
        Split::vertical([Constraint::Length(META_HEIGHT), Constraint::Fill(1)])
            .block(
                Block::app_default()
                    .title("Info")
                    .title_alignment(Alignment::Center)
                    .title_bottom(
                        TagLine::mono_color()
                            .centered()
                            .tag(Tag::new(" Scroll Up ", " Shift+K | Shift+\u{2191} "))
                            .tag(Tag::new(" Scroll Down ", " Shift+J | Shift+\u{2193} ")),
                    ),
            )
            .split(buf, area);

    Widget::render(meta_table(info), meta_area, buf);

    *state.offset_mut() = state.offset().min(
        info.schema()
            .len()
            .saturating_sub(fields_area.height.saturating_sub(1).into()),
    );
    StatefulWidget::render(fields_table(info), fields_area, buf, state);
}

fn meta_table(info: &TableInfo) -> Table<'_> {
    Table::default()
        .rows([
            Row::new([
                Span::styled("Path", theme().header(0)),
                Span::styled(info.source().display_path(), theme().text()),
            ]),
            Row::new([
                Span::styled("Shape", theme().header(1)),
                Span::styled(
                    format!("{} x {}", info.height(), info.width()),
                    theme().text(),
                ),
            ]),
            Row::new([
                Span::styled("Total Estimated Memory", theme().header(2)),
                Span::styled(
                    human_readable_size(info.total_est_size() as u64),
                    theme().text(),
                ),
            ]),
            Row::new([
                Span::styled("Total Null Count", theme().header(3)),
                Span::styled(info.total_null().to_string(), theme().text()),
            ]),
        ])
        .widths([Constraint::Max(23), Constraint::Fill(1)])
}

fn fields_table(info: &TableInfo) -> Table<'_> {
    Table::default()
        .header(
            Row::new(
                ["Name", "Type", "Estimated Size", "Null Count", "Min", "Max"]
                    .into_iter()
                    .enumerate()
                    .map(|(idx, name)| Text::styled(name, theme().header(idx))),
            )
            .style(theme().table_header()),
        )
        .rows(
            info.schema()
                .iter()
                .enumerate()
                .map(|(idx, (name, field))| {
                    Row::new([
                        name.to_owned(),
                        format!("{}", field.dtype()),
                        human_readable_size(field.estimated_size() as u64),
                        format!("{}", field.null_count()),
                        field.min().to_string(),
                        field.max().to_string(),
                    ])
                    .style(theme().row(idx))
                }),
        )
        .widths([Constraint::Fill(1); 6])
        .style(theme().text())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::misc::sql::TableSource;
    use polars::{frame::DataFrame, prelude::Column};

    fn frame(cols: usize) -> DataFrame {
        DataFrame::new_infer_height(
            (0..cols)
                .map(|idx| Column::new(format!("column{idx}").into(), vec![1i64, 2, 3]))
                .collect(),
        )
        .unwrap()
    }

    fn schema(tables: usize) -> BackendSchema {
        let mut schema = BackendSchema::default();
        for idx in 0..tables {
            schema.insert(
                format!("table{idx}"),
                TableInfo::new(TableSource::Stdin, &frame(idx + 1)),
            );
        }
        schema
    }

    fn render(state: &mut SqlBackendSchemaState, schema: &BackendSchema) -> Buffer {
        let area = Rect::new(0, 0, 120, 30);
        let mut buf = Buffer::empty(area);
        SqlBackendSchema::new(schema).render(area, &mut buf, state);
        buf
    }

    fn content(buf: &Buffer) -> String {
        buf.content().iter().map(|c| c.symbol()).collect()
    }

    mod state {
        use super::*;

        #[test]
        fn it_opens_on_the_first_table() {
            assert_eq!(SqlBackendSchemaState::default().selected(), Some(0));
        }

        #[test]
        fn selection_moves_through_the_tables() {
            let mut state = SqlBackendSchemaState::default();
            state.select_down();
            assert_eq!(state.selected(), Some(1));
            state.select_up();
            assert_eq!(state.selected(), Some(0));
        }

        #[test]
        fn selection_saturates_at_the_top() {
            let mut state = SqlBackendSchemaState::default();
            state.select_up();

            assert_eq!(state.selected(), Some(0));
        }

        #[test]
        fn the_field_list_starts_at_the_top() {
            assert_eq!(SqlBackendSchemaState::default().field_offset(), 0);
        }

        #[test]
        fn scrolling_up_saturates_at_the_top() {
            let mut state = SqlBackendSchemaState::default();
            state.scroll_up();

            assert_eq!(state.field_offset(), 0);
        }

        #[test]
        fn scrolling_down_advances_until_a_render_clamps_it() {
            let mut state = SqlBackendSchemaState::default();
            state.scroll_down();
            state.scroll_down();

            assert_eq!(state.field_offset(), 2);
        }
    }

    mod widget {
        use super::*;

        #[test]
        fn an_empty_backend_explains_itself() {
            let buf = render(&mut SqlBackendSchemaState::default(), &schema(0));

            assert!(content(&buf).contains("No data frame found"));
        }

        #[test]
        fn every_table_name_is_listed() {
            let buf = render(&mut SqlBackendSchemaState::default(), &schema(3));

            let content = content(&buf);
            assert!(content.contains("table0"));
            assert!(content.contains("table2"));
        }

        #[test]
        fn the_names_pane_is_titled_tables() {
            let buf = render(&mut SqlBackendSchemaState::default(), &schema(2));

            assert!(content(&buf).contains("Tables"));
        }

        #[test]
        fn the_selected_tables_meta_info_is_shown() {
            let buf = render(&mut SqlBackendSchemaState::default(), &schema(2));

            let content = content(&buf);
            assert!(content.contains("Info"));
            assert!(content.contains("Shape"));
            assert!(content.contains("Stdin"));
        }

        #[test]
        fn the_selected_tables_fields_are_shown() {
            let mut state = SqlBackendSchemaState::default();
            state.select(Some(1));
            let buf = render(&mut state, &schema(2));

            let content = content(&buf);
            assert!(content.contains("column0"));
            assert!(content.contains("column1"));
        }

        #[test]
        fn moving_the_selection_switches_the_info_pane() {
            let schema = schema(3);
            let mut state = SqlBackendSchemaState::default();
            let first = content(&render(&mut state, &schema));

            state.select(Some(2));
            let third = content(&render(&mut state, &schema));

            assert_ne!(first, third);
            assert!(!first.contains("column2"));
            assert!(third.contains("column2"));
        }

        #[test]
        fn the_field_hints_are_shown() {
            let buf = render(&mut SqlBackendSchemaState::default(), &schema(2));

            let content = content(&buf);
            assert!(content.contains("Scroll Up"));
            assert!(content.contains("Open"));
        }

        #[test]
        fn content_that_fits_is_never_scrolled() {
            let mut state = SqlBackendSchemaState::default();
            for _ in 0..10 {
                state.scroll_down();
            }
            render(&mut state, &schema(2));

            assert_eq!(state.field_offset(), 0);
        }

        #[test]
        fn a_selection_past_the_end_is_clamped_to_the_last_table() {
            let mut state = SqlBackendSchemaState::default();
            state.select(Some(99));
            let buf = render(&mut state, &schema(2));

            assert_eq!(state.selected(), Some(1));
            assert!(content(&buf).contains("column1"));
        }

        #[test]
        fn no_selection_shows_no_info() {
            let mut state = SqlBackendSchemaState::default();
            state.select(None);
            let buf = render(&mut state, &schema(2));

            let content = content(&buf);
            assert!(content.contains("table0"));
            assert!(!content.contains("Shape"));
        }

        #[test]
        fn a_tiny_area_renders_without_panicking() {
            let area = Rect::new(0, 0, 10, 4);
            let mut buf = Buffer::empty(area);
            SqlBackendSchema::new(&schema(2)).render(
                area,
                &mut buf,
                &mut SqlBackendSchemaState::default(),
            );
        }
    }
}
