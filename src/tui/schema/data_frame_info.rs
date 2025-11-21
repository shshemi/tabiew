use polars::frame::DataFrame;
use ratatui::layout::{Constraint, Layout};

use crate::{
    misc::sql::{Source, TableInfo, TableSchema},
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
    pub fn new(df: &DataFrame, input: Source) -> Self {
        //
        Self {
            meta_info: DataFrameMetaInfo::new(TableInfo::new(input, df)),
            field_info: DataFrameFieldInfo::new(TableSchema::new(df)),
        }
    }

    pub fn fields_mut(&mut self) -> &mut DataFrameFieldInfo {
        &mut self.field_info
    }

    pub fn fields(&self) -> &DataFrameFieldInfo {
        &self.field_info
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
        // Widget::render(DataFrameMetaInfo::new(self.meta_info), area2, buf);
        self.meta_info.render(area2, buf, focus_state);
        self.field_info.render(area3, buf, focus_state);

        // StatefulWidget::render(
        //     DataFrameFieldInfo::new(self.meta_info.schema()),
        //     area3,
        //     buf,
        //     &mut state.field_info,
        // );
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        self.field_info.handle(event)
    }
}

// pub struct DataFrameInfo<'a> {
//     table_info: &'a TableInfo,
// }

// impl<'a> DataFrameInfo<'a> {
//     pub fn new(table_info: &'a TableInfo) -> Self {
//         Self { table_info }
//     }
// }

// impl<'a> StatefulWidget for DataFrameInfo<'a> {
//     type State = DataFrameInfoState;

//     fn render(
//         self,
//         area: ratatui::prelude::Rect,
//         buf: &mut ratatui::prelude::Buffer,
//         state: &mut Self::State,
//     ) {
//         let [area2, area3] =
//             Layout::vertical([Constraint::Length(6), Constraint::Fill(1)]).areas(area);
//         Widget::render(DataFrameMetaInfo::new(self.table_info), area2, buf);
//         StatefulWidget::render(
//             DataFrameFieldInfo::new(self.table_info.schema()),
//             area3,
//             buf,
//             &mut state.field_info,
//         );
//     }
// }
