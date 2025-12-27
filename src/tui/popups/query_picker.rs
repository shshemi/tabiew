use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use polars::frame::DataFrame;

use crate::{
    AppResult,
    handler::message::Message,
    misc::globals::sql,
    tui::{component::Component, pane::TableDescription, pickers::text_picker::TextPicker},
};

#[derive(Debug)]
pub struct QueryPicker {
    df: DataFrame,
    text_picker: TextPicker,
    query_type: QueryType,
}

impl QueryPicker {
    pub fn new(df: DataFrame, query_type: QueryType) -> Self {
        Self {
            text_picker: TextPicker::default().with_title(query_type.title()),
            query_type,
            df,
        }
    }

    pub fn value(&self) -> &str {
        self.text_picker.input().value()
    }

    pub fn query_type(&self) -> &QueryType {
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

impl Component for QueryPicker {
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
                        QueryType::Select => self.select(self.value()),
                        QueryType::Filter => self.filter(self.value()),
                        QueryType::Order => self.order(self.value()),
                        QueryType::Sql => self.sql_query(self.value()),
                    };
                    match (result, self.query_type) {
                        (Ok(df), QueryType::Sql) => {
                            Message::PaneDismissModal.enqueue();
                            Message::TabsAddQueryPane(df, self.value().to_owned()).enqueue();
                        }
                        (Ok(df), QueryType::Select) => {
                            Message::PaneDismissModal.enqueue();
                            Message::PanePushDataFrame(
                                df,
                                TableDescription::Select(self.value().to_owned()),
                            )
                            .enqueue();
                        }
                        (Ok(df), QueryType::Order) => {
                            Message::PaneDismissModal.enqueue();
                            Message::PanePushDataFrame(
                                df,
                                TableDescription::Order(self.value().to_owned()),
                            )
                            .enqueue();
                        }
                        (Ok(df), QueryType::Filter) => {
                            Message::PaneDismissModal.enqueue();
                            Message::PanePushDataFrame(
                                df,
                                TableDescription::Filter(self.value().to_owned()),
                            )
                            .enqueue();
                        }
                        (Err(err), _) => {
                            Message::PaneDismissModal.enqueue();
                            Message::AppShowError(err.to_string()).enqueue();
                        }
                    }
                    true
                }
                (KeyCode::Esc, KeyModifiers::NONE) => {
                    Message::PaneDismissModal.enqueue();
                    true
                }
                _ => false,
            }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum QueryType {
    Select,
    Filter,
    Order,
    Sql,
}

impl QueryType {
    fn title(&self) -> String {
        match self {
            QueryType::Select => "Select",
            QueryType::Filter => "Filter",
            QueryType::Order => "Order",
            QueryType::Sql => "Query",
        }
        .to_owned()
    }
}
