use ratatui::{
    layout::{Alignment, Constraint},
    symbols::{
        border::{ROUNDED, Set},
        line::{VERTICAL_LEFT, VERTICAL_RIGHT},
    },
    text::Span,
    widgets::{Clear, Row, Table, Widget},
};

use crate::{
    misc::{
        globals::theme,
        sql::{self, TableInfo},
        type_ext::human_readable_size,
    },
    tui::{component::Component, widgets::block::Block},
};

#[derive(Debug)]
pub struct DataFrameMetaInfo {
    info: sql::TableInfo,
}

impl DataFrameMetaInfo {
    pub fn new(info: sql::TableInfo) -> Self {
        Self { info }
    }

    pub fn table_info(&self) -> &TableInfo {
        &self.info
    }
}

impl Component for DataFrameMetaInfo {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        _focus_state: crate::tui::component::FocusState,
    ) {
        Widget::render(Clear, area, buf);
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
                    Span::styled(
                        human_readable_size(self.info.total_est_size() as u64),
                        theme().text(),
                    ),
                ]),
                Row::new([
                    Span::styled("Total Null Count", theme().header(3)),
                    Span::styled(self.info.total_null().to_string(), theme().text()),
                ]),
            ])
            .widths([Constraint::Max(23), Constraint::Fill(1)])
            .block(
                Block::default()
                    .border_set(Set {
                        bottom_left: VERTICAL_RIGHT,
                        bottom_right: VERTICAL_LEFT,
                        ..ROUNDED
                    })
                    .title_alignment(Alignment::Center)
                    .title("Info")
                    .into_widget(),
            )
            .render(area, buf);
    }
}
