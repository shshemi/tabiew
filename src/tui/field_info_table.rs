use ratatui::{
    layout::Constraint,
    text::Text,
    widgets::{Block, BorderType, Row, StatefulWidget, Table, TableState},
};

use crate::misc::{globals::theme, sql::TableSchema};

#[derive(Debug, Default)]
pub struct FieldInfoTableState {
    table_state: TableState,
}

pub struct FieldInfoTable<'a> {
    table_schema: &'a TableSchema,
}

impl<'a> FieldInfoTable<'a> {
    pub fn new(field_info: &'a TableSchema) -> Self {
        Self {
            table_schema: field_info,
        }
    }
}

impl StatefulWidget for FieldInfoTable<'_> {
    type State = FieldInfoTableState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        Table::default()
            .header(
                Row::new(
                    ["Name", "Type", "Estimated Size", "Null Count"]
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
                            format!("{}", info.estimated_size()),
                            format!("{}", info.null_count()),
                        ])
                        .style(theme().row(idx))
                    }),
            )
            .widths([
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ])
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .border_style(theme().block())
                    .title("Fields"),
            )
            .render(area, buf, &mut state.table_state);
    }
}
