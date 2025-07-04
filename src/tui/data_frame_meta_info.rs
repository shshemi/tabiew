use ratatui::{
    layout::{Alignment, Constraint},
    symbols::{
        border::{ROUNDED, Set},
        line::{VERTICAL_LEFT, VERTICAL_RIGHT},
    },
    text::Span,
    widgets::{Block, BorderType, Borders, Row, Table, Widget},
};

use crate::misc::{globals::theme, sql};

pub struct DataFrameMetaInfo<'a> {
    info: &'a sql::TableInfo,
}

impl<'a> DataFrameMetaInfo<'a> {
    pub fn new(info: &'a sql::TableInfo) -> Self {
        Self { info }
    }
}

impl Widget for DataFrameMetaInfo<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Table::default()
            .rows([
                Row::new([
                    Span::styled("Path", theme().header(0)),
                    Span::styled(self.info.source().display_path(), theme().text()),
                ]),
                Row::new([
                    Span::styled("Shape", theme().header(1)),
                    Span::styled(
                        format!("{} x {}", self.info.height(), self.info.width()),
                        theme().text(),
                    ),
                ]),
                Row::new([
                    Span::styled("Total Estimated Memory", theme().header(2)),
                    Span::styled(self.info.total_est_size().to_string(), theme().text()),
                ]),
                Row::new([
                    Span::styled("Total Null Count", theme().header(3)),
                    Span::styled(self.info.total_null().to_string(), theme().text()),
                ]),
            ])
            .widths([Constraint::Max(23), Constraint::Fill(1)])
            .block(
                Block::new()
                    .borders(Borders::all())
                    .border_type(BorderType::Rounded)
                    .border_set(Set {
                        bottom_left: VERTICAL_RIGHT,
                        bottom_right: VERTICAL_LEFT,
                        ..ROUNDED
                    })
                    .border_style(theme().block())
                    .title_alignment(Alignment::Center)
                    .title("Info"),
            )
            .render(area, buf);
    }
}
