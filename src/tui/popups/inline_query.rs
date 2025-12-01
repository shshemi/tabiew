use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use polars::frame::DataFrame;

use crate::{
    AppResult,
    handler::action::Action,
    misc::globals::sql,
    tui::{component::Component, pickers::text_picker::TextPicker},
};

#[derive(Debug)]
pub struct InlineQuery {
    df: DataFrame,
    text_picker: TextPicker,
    query_type: InlineQueryType,
}

impl InlineQuery {
    pub fn new(df: DataFrame, query_type: InlineQueryType) -> Self {
        Self {
            text_picker: TextPicker::default().with_title(query_type.title()),
            query_type,
            df,
        }
    }

    pub fn value(&self) -> &str {
        self.text_picker.input().value()
    }

    pub fn query_type(&self) -> &InlineQueryType {
        &self.query_type
    }

    fn sql_query(&self, query: &str) -> AppResult<DataFrame> {
        Ok(sql().execute(query, self.df.clone())?)
    }

    fn select(&self, select: &str) -> AppResult<DataFrame> {
        self.sql_query(&format!("SELECT {select} FROM _"))
    }
    fn order(&self, order: &str) -> AppResult<DataFrame> {
        self.sql_query(&format!("SELECT * FROM _ ORDER BY {order}"))
    }
    fn filter(&self, filter: &str) -> AppResult<DataFrame> {
        self.sql_query(&format!("SELECT * FROM _ where {filter}"))
    }
}

impl Component for InlineQuery {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
    ) {
        self.text_picker.render(area, buf, focus_state);
    }

    fn handle(&mut self, event: KeyEvent) -> bool {
        self.text_picker.handle(event)
            || match (event.code, event.modifiers) {
                (KeyCode::Enter, KeyModifiers::NONE) => {
                    let result = match self.query_type {
                        InlineQueryType::Filter => self.filter(self.value()),
                        InlineQueryType::Order => self.order(self.value()),
                    };
                    match result {
                        Ok(df) => {
                            Action::PaneSetDataFrame(df).enqueue();
                            Action::PaneDismissModal.enqueue();
                        }
                        Err(err) => {
                            Action::AppShowError(err.to_string()).enqueue();
                            Action::PaneDismissModal.enqueue();
                        }
                    }
                    true
                }
                (KeyCode::Esc, KeyModifiers::NONE) => {
                    Action::PaneDismissModal.enqueue();
                    true
                }
                _ => false,
            }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum InlineQueryType {
    Filter,
    Order,
}

impl InlineQueryType {
    fn title(&self) -> String {
        match self {
            InlineQueryType::Filter => "Filter",
            InlineQueryType::Order => "Order",
        }
        .to_owned()
    }
}
