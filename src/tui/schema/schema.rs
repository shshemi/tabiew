use std::ops::Div;

use ratatui::{
    layout::{Constraint, Flex, Layout},
    widgets::{Clear, Paragraph, Widget, Wrap},
};

use crate::{
    misc::globals::{sql, theme},
    tui::{
        component::Component,
        schema::{data_frame_info::DataFrameInfo, data_frame_names::DataFrameNames},
        widgets::block::Block,
    },
};

#[derive(Debug, Default)]
pub struct SchemaState {
    names: DataFrameNames,
    infos: Vec<DataFrameInfo>,
}

impl SchemaState {
    fn selected_info_mut(&mut self) -> Option<&mut DataFrameInfo> {
        self.names
            .selected()
            .and_then(|idx| self.infos.get_mut(idx))
    }
}

impl Component for SchemaState {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
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
        // 2,3: Info
        //   2: Meta info
        //   3: Fields info

        Widget::render(Clear, area, buf);

        if sql().schema().is_empty() {
            let pg = Paragraph::new(
                        "No data frame found in the backed. Use the 'import' command to import data frames from files.",
                    ).centered().wrap(Wrap { trim: true });
            let width = area.width.saturating_sub(2).div(3).min(64);
            let lines = pg.line_count(width) as u16;
            let [center] = Layout::vertical([Constraint::Length(lines)])
                .flex(Flex::Center)
                .areas(area);
            let [center] = Layout::horizontal([Constraint::Length(width)])
                .flex(Flex::Center)
                .areas(center);
            Block::default().render(area, buf);
            pg.render(center, buf);
        } else {
            let [area1, area23] =
                Layout::horizontal([Constraint::Length(40), Constraint::Fill(1)]).areas(area);

            self.names.render(area1, buf, focus_state);
            if let Some(info) = self
                .names
                .selected()
                .and_then(|idx| self.infos.get_mut(idx))
            {
                info.render(area23, buf, focus_state);
            }

            // if let Some((_table_name, table_info)) = sql()
            //     .schema()
            //     .get_by_index(state.names.table().selected().unwrap_or_default())
            // {
            // Widget::render(TableInfoTable::new(table_info), area2, buf);

            // StatefulWidget::render(
            //     FieldInfoTable::new(table_info.schema()),
            //     area3,
            //     buf,
            //     &mut state.fields,
            // );
            //     DataFrameInfo::new(table_info).render(area23, buf, &mut state.data_frame_info)
            // }
        }
    }
    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        self.names.handle(event)
            || self
                .selected_info_mut()
                .map(|info| info.handle(event))
                .unwrap_or(false)
    }
}

// #[derive(Debug, Default)]
// pub struct Schema {}

// impl StatefulWidget for Schema {
//     type State = SchemaState;

//     fn render(
//         self,
//         area: ratatui::prelude::Rect,
//         buf: &mut ratatui::prelude::Buffer,
//         state: &mut Self::State,
//     ) {
//         buf.set_style(area, theme().text());
//         //
//         // |----------------------------|
//         // |     |          2           |
//         // |     |----------------------|
//         // |  1  |                      |
//         // |     |          3           |
//         // |     |                      |
//         // |     |                      |
//         // |----------------------------|
//         //
//         // 1: Table names
//         // 2,3: Info
//         //   2: Meta info
//         //   3: Fields info

//         Widget::render(Clear, area, buf);

//         if sql().schema().is_empty() {
//             let pg = Paragraph::new(
//                 "No data frame found in the backed. Use the 'import' command to import data frames from files.",
//             ).centered().wrap(Wrap { trim: true });
//             let width = area.width.saturating_sub(2).div(3).min(64);
//             let lines = pg.line_count(width) as u16;
//             let [center] = Layout::vertical([Constraint::Length(lines)])
//                 .flex(Flex::Center)
//                 .areas(area);
//             let [center] = Layout::horizontal([Constraint::Length(width)])
//                 .flex(Flex::Center)
//                 .areas(center);
//             Block::default().render(area, buf);
//             pg.render(center, buf);
//         } else {
//             let [area1, area23] =
//                 Layout::horizontal([Constraint::Length(40), Constraint::Fill(1)]).areas(area);

//             DataFrameNames::new(sql().schema().iter().map(|(name, _)| name)).render(
//                 area1,
//                 buf,
//                 &mut state.names,
//             );

//             if let Some((_table_name, table_info)) = sql()
//                 .schema()
//                 .get_by_index(state.names.table().selected().unwrap_or_default())
//             {
//                 // Widget::render(TableInfoTable::new(table_info), area2, buf);

//                 // StatefulWidget::render(
//                 //     FieldInfoTable::new(table_info.schema()),
//                 //     area3,
//                 //     buf,
//                 //     &mut state.fields,
//                 // );
//                 DataFrameInfo::new(table_info).render(area23, buf, &mut state.data_frame_info)
//             }
//         }
//     }
// }
