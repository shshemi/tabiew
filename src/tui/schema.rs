use ratatui::{
    layout::{Constraint, Layout},
    widgets::{StatefulWidget, Widget},
};

use crate::misc::globals::{sql, theme};

use super::{
    field_info_table::{FieldInfoTable, FieldInfoTableState},
    table_info_table::TableInfoTable,
    table_names_table::{TableNamesTable, TableNamesTableState},
};

#[derive(Debug, Default)]
pub struct SchemaState {
    names: TableNamesTableState,
    fields: FieldInfoTableState,
}

impl SchemaState {
    pub fn select_table(&mut self, idx: usize) {
        self.names.table_mut().select(Some(idx));
        *self.fields.table_state_mut().offset_mut() = 0;
    }

    pub fn names(&self) -> &TableNamesTableState {
        &self.names
    }

    pub fn names_mut(&mut self) -> &mut TableNamesTableState {
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

        let [area1, area23] =
            Layout::horizontal([Constraint::Length(40), Constraint::Fill(1)]).areas(area);
        let [area2, area3] =
            Layout::vertical([Constraint::Length(6), Constraint::Fill(1)]).areas(area23);

        TableNamesTable::new(sql().schema().iter().map(|(name, _)| name)).render(
            area1,
            buf,
            &mut state.names,
        );

        if let Some((_table_name, table_info)) = sql()
            .schema()
            .get_by_index(state.names.table().selected().unwrap_or_default())
        {
            Widget::render(TableInfoTable::new(table_info), area2, buf);

            StatefulWidget::render(
                FieldInfoTable::new(table_info.schema()),
                area3,
                buf,
                &mut state.fields,
            );
        }
    }
}
