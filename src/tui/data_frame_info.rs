use ratatui::{
    layout::{Constraint, Layout},
    widgets::{StatefulWidget, Widget},
};

use crate::{
    misc::sql::TableInfo,
    tui::{
        field_info_table::{FieldInfoTable, FieldInfoTableState},
        table_info_table::TableInfoTable,
    },
};

#[derive(Debug, Default)]
pub struct DataFrameInfoState {
    field_info: FieldInfoTableState,
}

impl DataFrameInfoState {
    pub fn field_info_mut(&mut self) -> &mut FieldInfoTableState {
        &mut self.field_info
    }

    pub fn field_info(&self) -> &FieldInfoTableState {
        &self.field_info
    }
}

pub struct DataFrameInfo<'a> {
    table_info: &'a TableInfo,
}

impl<'a> DataFrameInfo<'a> {
    pub fn new(table_info: &'a TableInfo) -> Self {
        Self { table_info }
    }
}

impl<'a> StatefulWidget for DataFrameInfo<'a> {
    type State = DataFrameInfoState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let [area2, area3] =
            Layout::vertical([Constraint::Length(6), Constraint::Fill(1)]).areas(area);
        Widget::render(TableInfoTable::new(self.table_info), area2, buf);

        StatefulWidget::render(
            FieldInfoTable::new(self.table_info.schema()),
            area3,
            buf,
            &mut state.field_info,
        );
    }
}
