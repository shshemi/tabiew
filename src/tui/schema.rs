use ratatui::{
    layout::{Constraint, Layout},
    text::Line,
    widgets::{Block, BorderType, List, ListState, StatefulWidget, Widget},
};

use crate::misc::globals::{sql, theme};

use super::{
    field_info_table::{FieldInfoTable, FieldInfoTableState},
    table_info::TableInfo,
};

#[derive(Debug)]
pub struct SchemaState {
    names: ListState,
    fields: FieldInfoTableState,
}

impl Default for SchemaState {
    fn default() -> Self {
        Self {
            names: ListState::default().with_selected(Some(0)),
            fields: FieldInfoTableState::default(),
        }
    }
}

impl SchemaState {
    pub fn names(&self) -> &ListState {
        &self.names
    }

    pub fn names_mut(&mut self) -> &mut ListState {
        &mut self.names
    }

    pub fn fields(&self) -> &FieldInfoTableState {
        &self.fields
    }

    pub fn fields_mut(&mut self) -> &mut FieldInfoTableState {
        &mut self.fields
    }
}

#[derive(Debug, Default)]
pub struct Schema {
    // schema: &'a BackendSchema,
}

impl Schema {}

impl StatefulWidget for Schema {
    type State = SchemaState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        buf.set_style(area, theme().text());
        //
        // |----------------------------|
        // |     |          2           |
        // |     |----------------------|
        // |  1  |                      |
        // |     |          3           |
        // |     |                      |
        // |     |                      |
        // |----------------------------|
        //
        // 1: Table names
        // 2: Table path
        // 3: Table fields

        // let blk = Block::bordered()
        //     .border_type(BorderType::Rounded)
        //     .border_style(theme().block())
        //     .title("Schema");
        // let blk_area = blk.inner(area);
        // blk.render(area, buf);
        // let area = blk_area;

        let [area1, area23] =
            Layout::horizontal([Constraint::Length(32), Constraint::Fill(1)]).areas(area);
        let [area2, area3] =
            Layout::vertical([Constraint::Length(6), Constraint::Fill(1)]).areas(area23);
        // let [area3, area4] =
        //     Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(area34);

        StatefulWidget::render(
            List::new(
                sql()
                    .schema()
                    .iter()
                    .map(|(name, _)| Line::from(name.to_owned()).style(theme().text())),
            )
            .highlight_style(theme().highlight())
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .border_style(theme().block())
                    .title("Tables"),
            ),
            area1,
            buf,
            &mut state.names,
        );

        if let Some((_table_name, table_info)) = sql()
            .schema()
            .get_by_index(state.names.selected().unwrap_or_default())
        {
            Widget::render(TableInfo::new(table_info), area2, buf);

            StatefulWidget::render(
                FieldInfoTable::new(table_info.schema()),
                area3,
                buf,
                &mut state.fields,
            );
        }
    }
}
