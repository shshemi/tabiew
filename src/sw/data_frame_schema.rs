use std::borrow::Cow;

use polars::frame::DataFrame;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Rect},
    text::{Span, Text},
    widgets::{Block, Row, StatefulWidget, Table, TableState, Widget},
};

use crate::{
    misc::{
        config::theme,
        sql::{TableInfo, TableSource},
        type_ext::human_readable_size,
    },
    sw::{
        app_default::AppDefault,
        buffer_ext::BufferExt,
        rect_ext::RectExt,
        widgets::{
            split::Split,
            tag_line::{Tag, TagLine},
        },
    },
};

const META_HEIGHT: u16 = 4;

#[derive(Debug)]
pub struct DataFrameSchemaState {
    info: TableInfo,
    fields: TableState,
}

impl DataFrameSchemaState {
    pub fn new(df: &DataFrame, source: TableSource) -> Self {
        Self {
            info: TableInfo::new(source, df),
            fields: TableState::default(),
        }
    }

    pub fn info(&self) -> &TableInfo {
        &self.info
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

#[derive(Debug)]
pub struct DataFrameSchema<'a> {
    title: Cow<'a, str>,
}

impl<'a> DataFrameSchema<'a> {
    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self {
        self.title = title.into();
        self
    }
}

impl Default for DataFrameSchema<'_> {
    fn default() -> Self {
        Self {
            title: Cow::Borrowed("Info"),
        }
    }
}

impl StatefulWidget for DataFrameSchema<'_> {
    type State = DataFrameSchemaState;

    fn render(self, _area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let area = buf.area.plot();
        buf.clear(area);

        let [meta_area, fields_area] =
            Split::vertical([Constraint::Length(META_HEIGHT), Constraint::Fill(1)])
                .block(
                    Block::app_default()
                        .title(self.title)
                        .title_alignment(Alignment::Center)
                        .title_bottom(
                            TagLine::mono_color()
                                .centered()
                                .tag(Tag::new(" Scroll Up ", " Shift+K | Shift+\u{2191} "))
                                .tag(Tag::new(" Scroll Down ", " Shift+J | Shift+\u{2193} ")),
                        ),
                )
                .split(buf, area);

        Widget::render(meta_table(&state.info), meta_area, buf);

        *state.fields.offset_mut() = state.fields.offset().min(
            state
                .info
                .schema()
                .len()
                .saturating_sub(fields_area.height.saturating_sub(1).into()),
        );
        StatefulWidget::render(
            fields_table(&state.info),
            fields_area,
            buf,
            &mut state.fields,
        );
    }
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
    use polars::prelude::Column;

    fn frame(cols: usize) -> DataFrame {
        DataFrame::new_infer_height(
            (0..cols)
                .map(|idx| Column::new(format!("column{idx}").into(), vec![1i64, 2, 3]))
                .collect(),
        )
        .unwrap()
    }

    fn state(cols: usize) -> DataFrameSchemaState {
        DataFrameSchemaState::new(&frame(cols), TableSource::Stdin)
    }

    fn render(state: &mut DataFrameSchemaState, schema: DataFrameSchema) -> Buffer {
        let area = Rect::new(0, 0, 120, 30);
        let mut buf = Buffer::empty(area);
        schema.render(area, &mut buf, state);
        buf
    }

    fn content(buf: &Buffer) -> String {
        buf.content().iter().map(|c| c.symbol()).collect()
    }

    mod state {
        use super::*;

        #[test]
        fn the_frame_shape_is_captured() {
            let state = state(3);

            assert_eq!(state.info().width(), 3);
            assert_eq!(state.info().height(), 3);
        }

        #[test]
        fn every_column_becomes_a_field() {
            let state = state(4);

            assert_eq!(state.info().schema().len(), 4);
        }

        #[test]
        fn the_source_is_kept() {
            let state = DataFrameSchemaState::new(&frame(1), TableSource::User);

            assert_eq!(state.info().source().display_path(), "User");
        }

        #[test]
        fn it_starts_at_the_top() {
            assert_eq!(state(3).field_offset(), 0);
        }

        #[test]
        fn scrolling_up_saturates_at_the_top() {
            let mut state = state(3);
            state.scroll_up();

            assert_eq!(state.field_offset(), 0);
        }

        #[test]
        fn scrolling_down_advances_until_a_render_clamps_it() {
            let mut state = state(3);
            state.scroll_down();
            state.scroll_down();

            assert_eq!(state.field_offset(), 2);
        }
    }

    mod widget {
        use super::*;

        #[test]
        fn the_meta_info_is_shown() {
            let buf = render(&mut state(3), DataFrameSchema::default());

            let content = content(&buf);
            assert!(content.contains("Path"));
            assert!(content.contains("Shape"));
            assert!(content.contains("3 x 3"));
            assert!(content.contains("Stdin"));
        }

        #[test]
        fn the_field_columns_are_shown() {
            let buf = render(&mut state(2), DataFrameSchema::default());

            let content = content(&buf);
            assert!(content.contains("column0"));
            assert!(content.contains("column1"));
            assert!(content.contains("Estimated Size"));
        }

        #[test]
        fn defaults_to_an_info_title() {
            let buf = render(&mut state(2), DataFrameSchema::default());

            assert!(content(&buf).contains("Info"));
        }

        #[test]
        fn the_title_is_overridable() {
            let buf = render(&mut state(2), DataFrameSchema::default().title("Columns"));

            assert!(content(&buf).contains("Columns"));
        }

        #[test]
        fn the_scroll_hints_are_shown() {
            let buf = render(&mut state(2), DataFrameSchema::default());

            let content = content(&buf);
            assert!(content.contains("Scroll Up"));
            assert!(content.contains("Scroll Down"));
        }

        #[test]
        fn it_leaves_a_margin_around_the_overlay() {
            let buf = render(&mut state(2), DataFrameSchema::default());

            assert_eq!(buf[(0, 0)].symbol(), " ");
            assert_eq!(buf[(119, 29)].symbol(), " ");
        }

        #[test]
        fn content_that_fits_is_never_scrolled() {
            let mut state = state(2);
            for _ in 0..10 {
                state.scroll_down();
            }
            render(&mut state, DataFrameSchema::default());

            assert_eq!(state.field_offset(), 0);
        }

        #[test]
        fn scrolling_stops_at_the_last_page() {
            let mut state = state(60);
            for _ in 0..200 {
                state.scroll_down();
            }
            render(&mut state, DataFrameSchema::default());
            let bottom = state.field_offset();
            assert!(bottom > 0);

            state.scroll_down();
            render(&mut state, DataFrameSchema::default());

            assert_eq!(state.field_offset(), bottom);
        }

        #[test]
        fn a_frame_with_no_columns_renders_without_panicking() {
            let mut state = DataFrameSchemaState::new(&DataFrame::empty(), TableSource::Stdin);
            render(&mut state, DataFrameSchema::default());

            assert_eq!(state.info().width(), 0);
        }

        #[test]
        fn a_tiny_area_renders_without_panicking() {
            let area = Rect::new(0, 0, 10, 4);
            let mut buf = Buffer::empty(area);
            DataFrameSchema::default().render(area, &mut buf, &mut state(2));
        }
    }
}
