use std::ops::Div;

use ratatui::{
    layout::{Constraint, Flex, Layout},
    widgets::{Paragraph, StatefulWidget, Widget, Wrap},
};

use crate::{
    misc::globals::{sql, theme},
    tui::{
        schema::{
            data_frame_info::{DataFrameInfo, DataFrameInfoState},
            data_frame_names::{DataFrameNames, DataFrameNamesState},
        },
        widgets::block::Block,
    },
};

#[derive(Debug, Default)]
pub struct SchemaState {
    names: DataFrameNamesState,
    data_frame_info: DataFrameInfoState,
}

impl SchemaState {
    pub fn select_table(&mut self, idx: usize) {
        self.names.table_mut().select(Some(idx));
        *self
            .data_frame_info
            .field_mut()
            .table_state_mut()
            .offset_mut() = 0;
    }

    pub fn names(&self) -> &DataFrameNamesState {
        &self.names
    }

    pub fn names_mut(&mut self) -> &mut DataFrameNamesState {
        &mut self.names
    }

    pub fn info(&self) -> &DataFrameInfoState {
        &self.data_frame_info
    }

    pub fn info_mut(&mut self) -> &mut DataFrameInfoState {
        &mut self.data_frame_info
    }
}

#[derive(Debug, Default)]
pub struct Schema {}

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
        // 2,3: Info
        //   2: Meta info
        //   3: Fields info

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

            DataFrameNames::new(sql().schema().iter().map(|(name, _)| name)).render(
                area1,
                buf,
                &mut state.names,
            );

            if let Some((_table_name, table_info)) = sql()
                .schema()
                .get_by_index(state.names.table().selected().unwrap_or_default())
            {
                // Widget::render(TableInfoTable::new(table_info), area2, buf);

                // StatefulWidget::render(
                //     FieldInfoTable::new(table_info.schema()),
                //     area3,
                //     buf,
                //     &mut state.fields,
                // );
                DataFrameInfo::new(table_info).render(area23, buf, &mut state.data_frame_info)
            }
        }
    }
}
