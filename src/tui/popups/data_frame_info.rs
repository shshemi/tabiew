use crossterm::event::{KeyCode, KeyModifiers};
use polars::frame::DataFrame;
use ratatui::layout::Margin;

use crate::{
    handler::message::Message,
    misc::sql::TableInfo,
    tui::{component::Component, schema::data_frame_info},
};

#[derive(Debug)]
pub struct DataFrameInfo {
    data_frame_info: data_frame_info::DataFrameInfo,
    view: Option<String>,
}

impl DataFrameInfo {
    pub fn new(df: &DataFrame, input: crate::misc::sql::TableSource) -> Self {
        Self {
            data_frame_info: data_frame_info::DataFrameInfo::new(TableInfo::new(input, df)),
            view: None,
        }
    }
    pub fn duckdb(info: TableInfo, view: String) -> Self {
        Self {
            data_frame_info: data_frame_info::DataFrameInfo::new(info),
            view: Some(view),
        }
    }
    pub fn view(&self) -> Option<&str> {
        self.view.as_deref()
    }
}

impl Component for DataFrameInfo {
    fn render(
        &mut self,
        _area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
    ) {
        let area = buf.area.inner(Margin::new(7, 3));
        self.data_frame_info.render(area, buf, focus_state);
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        self.data_frame_info.handle(event) || {
            match (event.code, event.modifiers) {
                (KeyCode::Enter, KeyModifiers::NONE) => true,
                (KeyCode::Esc, KeyModifiers::NONE)
                | (KeyCode::Char('q'), KeyModifiers::NONE)
                | (KeyCode::Char('i'), KeyModifiers::NONE) => {
                    Message::PaneDismissModal.enqueue();
                    true
                }
                _ => false,
            }
        }
    }
}
