use ratatui::{
    layout::Constraint,
    text::Span,
    widgets::{Block, BorderType, Row, Table, Widget},
};

use crate::misc::{globals::theme, sql};

pub struct TableInfo<'a> {
    info: &'a sql::TableInfo,
}

impl<'a> TableInfo<'a> {
    pub fn new(info: &'a sql::TableInfo) -> Self {
        Self { info }
    }
}

impl Widget for TableInfo<'_> {
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
                    Span::styled("Total Null", theme().header(3)),
                    Span::styled(self.info.total_null().to_string(), theme().text()),
                ]),
            ])
            .widths([Constraint::Max(23), Constraint::Fill(1)])
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .border_style(theme().block())
                    .title("Info"),
            )
            .render(area, buf);
    }
}
