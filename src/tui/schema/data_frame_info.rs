use ratatui::layout::{Constraint, Layout};

use crate::{
    misc::sql::TableInfo,
    tui::{
        component::Component,
        schema::{
            data_frame_field_info::DataFrameFieldInfo, data_frame_meta_info::DataFrameMetaInfo,
        },
    },
};

#[derive(Debug)]
pub struct DataFrameInfo {
    meta_info: DataFrameMetaInfo,
    field_info: DataFrameFieldInfo,
}

impl DataFrameInfo {
    pub fn new(table_info: TableInfo) -> Self {
        Self {
            field_info: DataFrameFieldInfo::new(table_info.schema().clone()),
            meta_info: DataFrameMetaInfo::new(table_info),
        }
    }

    pub fn table_info(&self) -> &TableInfo {
        self.meta_info.table_info()
    }
}

impl Component for DataFrameInfo {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
    ) {
        let [area2, area3] =
            Layout::vertical([Constraint::Length(6), Constraint::Fill(1)]).areas(area);
        self.meta_info.render(area2, buf, focus_state);
        self.field_info.render(area3, buf, focus_state);
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        self.field_info.handle(event)
    }
}
