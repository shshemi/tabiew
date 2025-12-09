use std::ops::Div;

use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Flex, Layout},
    widgets::{Clear, Paragraph, Widget, Wrap},
};

use crate::{
    handler::message::Message,
    misc::{
        globals::{sql, theme},
        sql::TableInfo,
    },
    tui::{
        component::Component,
        schema::{data_frame_info::DataFrameInfo, data_frame_names::DataFrameNames},
        widgets::block::Block,
    },
};

#[derive(Debug, Default)]
pub struct Schema {
    names: DataFrameNames,
    info: Option<DataFrameInfo>,
}

impl Schema {
    // fn selected_info_mut(&mut self) -> Option<&mut DataFrameInfo> {
    //     self.names.selected().and_then(|idx| self.info.get_mut(idx))
    // }

    // fn remove_selected(&mut self) {
    //     if let Some((idx, name)) = self.names.remove_selected() {
    //         sql().unregister(name.as_str());
    //         self.info.remove(idx);
    //     }
    // }

    // fn push(&mut self, name: String, table_info: TableInfo) {
    //     self.names.push(name);
    //     self.info.push(DataFrameInfo::new(table_info));
    // }
}

impl Component for Schema {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
    ) {
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

        buf.set_style(area, theme().text());
        Widget::render(Clear, area, buf);

        if let Some(selected) = self.names.selected()
            && let Some((_, new_info)) = sql().schema().get_by_index(selected)
            && self
                .info
                .as_ref()
                .map(|df_info| df_info.table_info())
                .map(|cur_info| cur_info != new_info)
                .unwrap_or(true)
        {
            self.info = Some(DataFrameInfo::new(new_info.clone()))
        }

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
            if let Some(info) = self.info.as_mut() {
                info.render(area23, buf, focus_state);
            }
        }
    }
    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        self.names.handle(event)
            || self
                .info
                .as_mut()
                .map(|info| info.handle(event))
                .unwrap_or_default()
            || match (event.code, event.modifiers) {
                (KeyCode::Esc, KeyModifiers::NONE) | (KeyCode::Char('q'), KeyModifiers::NONE) => {
                    Message::AppDismissSchema.enqueue();
                    true
                }
                _ => false,
            }
    }
}
