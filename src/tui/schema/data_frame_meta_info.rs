use ratatui::{
    layout::{Alignment, Constraint},
    symbols::{
        border::{ROUNDED, Set},
        line::{VERTICAL_LEFT, VERTICAL_RIGHT},
    },
    text::Span,
    widgets::{Block, Clear, Row, Table, Widget},
};

use crate::{
    misc::{
        config::theme,
        sql::{self, TableInfo},
    },
    tui::{
        app_default::{AppDefault, AppTitle},
        component::Component,
    },
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
        Clear.render(area, buf);
        let disk = self.info.disk_text().map(|disk| {
            Row::new([
                Span::styled("Size on Disk", theme().header(2)),
                Span::styled(disk, theme().text()),
            ])
        });
        Table::default()
            .rows(
                [
                    Row::new([
                        Span::styled("Path", theme().header(0)),
                        Span::styled(self.info.source().display_path(), theme().text()),
                    ]),
                    Row::new([
                        Span::styled("Shape", theme().header(1)),
                        Span::styled(self.info.shape_text(), theme().text()),
                    ]),
                    Row::new([
                        Span::styled("Total Estimated Memory", theme().header(2)),
                        Span::styled(self.info.total_est_size_text(), theme().text()),
                    ]),
                    Row::new([
                        Span::styled("Total Null Count", theme().header(3)),
                        Span::styled(self.info.total_null_text(), theme().text()),
                    ]),
                ]
                .into_iter()
                .chain(disk),
            )
            .widths([Constraint::Max(23), Constraint::Fill(1)])
            .block(
                Block::app_default()
                    .border_set(Set {
                        bottom_left: VERTICAL_RIGHT,
                        bottom_right: VERTICAL_LEFT,
                        ..ROUNDED
                    })
                    .title_alignment(Alignment::Center)
                    .app_title("Info"),
            )
            .render(area, buf);
    }
}
