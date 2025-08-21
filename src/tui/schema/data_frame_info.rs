use ratatui::{
    layout::{Constraint, Layout},
    widgets::{StatefulWidget, Widget},
};

use crate::{
    misc::sql::TableInfo,
    tui::schema::{
        data_frame_field_info::{DataFrameFieldInfo, DataFrameFieldInfoState},
        data_frame_meta_info::DataFrameMetaInfo,
    },
};

#[derive(Debug, Default)]
pub struct DataFrameInfoState {
    field_info: DataFrameFieldInfoState,
}

impl DataFrameInfoState {
    pub fn field_info_mut(&mut self) -> &mut DataFrameFieldInfoState {
        &mut self.field_info
    }

    pub fn field_info(&self) -> &DataFrameFieldInfoState {
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
        Widget::render(DataFrameMetaInfo::new(self.table_info), area2, buf);
        StatefulWidget::render(
            DataFrameFieldInfo::new(self.table_info.schema()),
            area3,
            buf,
            &mut state.field_info,
        );
    }
}
