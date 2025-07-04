use ratatui::{
    layout::{Alignment, Constraint},
    style::{Modifier, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, Row, StatefulWidget, Table, TableState, Widget},
};

use crate::misc::{globals::theme, sql::TableSchema, type_ext::human_readable_size};

#[derive(Debug, Default)]
pub struct DataFrameFieldInfoState {
    table_state: TableState,
}

impl DataFrameFieldInfoState {
    pub fn table_state(&self) -> &TableState {
        &self.table_state
    }

    pub fn table_state_mut(&mut self) -> &mut TableState {
        &mut self.table_state
    }
}

pub struct DataFrameFieldInfo<'a> {
    table_schema: &'a TableSchema,
}

impl<'a> DataFrameFieldInfo<'a> {
    pub fn new(field_info: &'a TableSchema) -> Self {
        Self {
            table_schema: field_info,
        }
    }
}

impl StatefulWidget for DataFrameFieldInfo<'_> {
    type State = DataFrameFieldInfoState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        *state.table_state.offset_mut() = state.table_state.offset().min(
            self.table_schema
                .len()
                .saturating_sub(area.height.saturating_sub(2).into()),
        );
        Widget::render(Clear, area, buf);
        StatefulWidget::render(
            Table::default()
                .header(
                    Row::new(
                        ["Name", "Type", "Estimated Size", "Null Count", "Min", "Max"]
                            .into_iter()
                            .enumerate()
                            .map(|(i, s)| Text::styled(s, theme().header(i))),
                    )
                    .style(theme().table_header()),
                )
                .rows(
                    self.table_schema
                        .iter()
                        .enumerate()
                        .map(|(idx, (name, info))| {
                            Row::new([
                                name.to_owned(),
                                format!("{}", info.dtype()),
                                human_readable_size(info.estimated_size()),
                                format!("{}", info.null_count()),
                                info.min().to_string(),
                                info.max().to_string(),
                            ])
                            .style(theme().row(idx))
                        }),
                )
                .widths([
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .style(theme().text())
                .block(
                    Block::new()
                        .borders(Borders::BOTTOM | Borders::RIGHT | Borders::LEFT)
                        .border_type(BorderType::Rounded)
                        .border_style(theme().block())
                        .title_bottom(Line::from_iter([
                            Span::raw(" Scroll Up ").style(theme().block_tag()),
                            Span::raw(" Shift+K | Shift+\u{2191} ")
                                .style(theme().block_tag())
                                .add_modifier(Modifier::REVERSED),
                            Span::raw(" "),
                            Span::raw(" Scroll Down ").style(theme().block_tag()),
                            Span::raw(" Shift+J | Shift+\u{2193} ")
                                .style(theme().block_tag())
                                .add_modifier(Modifier::REVERSED),
                        ]))
                        .title_alignment(Alignment::Center),
                ),
            area,
            buf,
            &mut state.table_state,
        );
    }
}
