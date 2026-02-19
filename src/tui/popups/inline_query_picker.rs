use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use polars::frame::DataFrame;
use ratatui::{
    layout::{Constraint, Flex, Layout},
    symbols::{
        border::{ROUNDED, Set},
        line::{VERTICAL_LEFT, VERTICAL_RIGHT},
    },
    widgets::{Borders, Clear, List, ListState, StatefulWidget, Widget},
};

use crate::{
    AppResult,
    handler::message::Message,
    misc::{config::theme, sql::sql},
    tui::{
        component::Component,
        pane::TableDescription,
        popups::sql_completion::{
            self, CompletionContext, collect_all_columns, filter_by_prefix, get_table_columns,
            get_table_names,
        },
        widgets::{block::Block, input::Input},
    },
};

#[derive(Debug)]
pub struct InlineQueryPicker {
    df: DataFrame,
    input: Input,
    query_type: QueryType,
    all_columns: Vec<String>,
    suggestions: Vec<String>,
    selected_suggestion: Option<usize>,
}

impl InlineQueryPicker {
    pub fn new(df: DataFrame, query_type: QueryType) -> Self {
        let all_columns = collect_all_columns(Some(&df));

        Self {
            input: Input::default(),
            query_type,
            all_columns,
            suggestions: Vec::new(),
            selected_suggestion: None,
            df,
        }
    }

    pub fn value(&self) -> &str {
        self.input.value()
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

    fn update_suggestions(&mut self) {
        let cursor = self.input.cursor();
        let value = self.input.value().to_owned();

        // Prepend a SQL prefix so the tokenizer sees full clause context.
        let prefix = self.query_type.sql_prefix();
        let (token, context) =
            sql_completion::extract_token_and_context(&value, cursor, prefix);

        self.suggestions.clear();
        self.selected_suggestion = None;

        if token.is_empty() {
            return;
        }

        let token_lower = token.to_lowercase();

        match &context {
            CompletionContext::Column => {
                self.suggestions = filter_by_prefix(self.all_columns.iter(), &token_lower);
            }
            CompletionContext::QualifiedColumn(table) => {
                let columns = get_table_columns(table, Some(&self.df));
                self.suggestions = filter_by_prefix(columns.iter(), &token_lower);
            }
            CompletionContext::Table => {
                let tables = get_table_names(true);
                self.suggestions = filter_by_prefix(tables.iter(), &token_lower);
            }
            CompletionContext::None => return,
        };

        if !self.suggestions.is_empty() {
            self.selected_suggestion = Some(0);
        }
    }

    fn accept_suggestion(&mut self) {
        if let Some(idx) = self.selected_suggestion {
            if let Some(suggestion) = self.suggestions.get(idx).cloned() {
                let cursor = self.input.cursor();
                let value = self.input.value().to_owned();
                let before_cursor = &value[..cursor];
                let at_cursor = value[cursor..].chars().next();

                // Find token start
                let token_start = before_cursor
                    .char_indices()
                    .rev()
                    .find(|(_, c)| sql_completion::is_separator(*c))
                    .map(|(i, c)| i + c.len_utf8())
                    .unwrap_or(0);

                let token_char_len = before_cursor[token_start..].chars().count();

                // Delete the current token
                for _ in 0..token_char_len {
                    self.input.delete_prev();
                }

                // Insert the suggestion
                for c in suggestion.chars() {
                    self.input.insert(c);
                }
                if !at_cursor.is_some_and(|c| c.is_whitespace()) {
                    self.input.insert(' ');
                }

                self.suggestions.clear();
                self.selected_suggestion = None;
            }
        }
    }
}

const MAX_VISIBLE_SUGGESTIONS: usize = 10;

impl Component for InlineQueryPicker {
    fn render(
        &mut self,
        _area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
    ) {
        let has_suggestions = !self.suggestions.is_empty();
        let width = 80u16;

        if has_suggestions {
            let visible = self.suggestions.len().min(MAX_VISIBLE_SUGGESTIONS);
            // 3 rows for the input box (top border + input + middle border)
            // + visible items + 1 for bottom border
            let total_height = 3 + visible as u16 + 1;

            let [area] = Layout::horizontal([Constraint::Length(width)])
                .flex(Flex::Center)
                .areas(buf.area);
            let [_, area] = Layout::vertical([
                Constraint::Length(3),
                Constraint::Length(total_height),
            ])
            .areas(area);

            Widget::render(Clear, area, buf);

            let [input_area, list_area] =
                Layout::vertical([Constraint::Length(2), Constraint::Fill(1)]).areas(area);

            // Render input with top + sides border only
            let input_area_inner = {
                let block = Block::default()
                    .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP)
                    .title(self.query_type.title());
                let inner = block.inner(input_area);
                Widget::render(block, input_area, buf);
                inner
            };
            self.input.render(input_area_inner, buf, focus_state);

            // Render suggestion list with connecting border
            let mut list_state = ListState::default();
            list_state.select(self.selected_suggestion);

            let list = List::default()
                .items(
                    self.suggestions
                        .iter()
                        .take(MAX_VISIBLE_SUGGESTIONS)
                        .map(String::as_str),
                )
                .highlight_style(theme().row_highlighted())
                .block(
                    Block::default()
                        .border_set(Set {
                            top_left: VERTICAL_RIGHT,
                            top_right: VERTICAL_LEFT,
                            ..ROUNDED
                        })
                        .into_widget(),
                );

            StatefulWidget::render(list, list_area, buf, &mut list_state);
        } else {
            // No suggestions — render like plain TextPicker
            let [area] = Layout::horizontal([Constraint::Length(width)])
                .flex(Flex::Center)
                .areas(buf.area);
            let [_, area] =
                Layout::vertical([Constraint::Length(3), Constraint::Length(3)]).areas(area);

            Widget::render(Clear, area, buf);

            let area_inner = {
                let block = Block::default().title(self.query_type.title());
                let inner = block.inner(area);
                block.render(area, buf);
                inner
            };

            self.input.render(area_inner, buf, focus_state);
        }
    }

    fn handle(&mut self, event: KeyEvent) -> bool {
        let has_suggestions = !self.suggestions.is_empty();

        match (event.code, event.modifiers) {
            // Tab / Enter: accept the highlighted suggestion
            (KeyCode::Tab | KeyCode::Enter, KeyModifiers::NONE) if has_suggestions => {
                self.accept_suggestion();
                true
            }
            // Down / Ctrl+N: next suggestion
            (KeyCode::Down, KeyModifiers::NONE) | (KeyCode::Char('n'), KeyModifiers::CONTROL)
                if has_suggestions =>
            {
                let max = self.suggestions.len().min(MAX_VISIBLE_SUGGESTIONS);
                self.selected_suggestion =
                    Some(self.selected_suggestion.map(|i| (i + 1) % max).unwrap_or(0));
                true
            }
            // Up / Ctrl+P: previous suggestion
            (KeyCode::Up, KeyModifiers::NONE) | (KeyCode::Char('p'), KeyModifiers::CONTROL)
                if has_suggestions =>
            {
                let max = self.suggestions.len().min(MAX_VISIBLE_SUGGESTIONS);
                self.selected_suggestion = Some(
                    self.selected_suggestion
                        .map(|i| if i == 0 { max - 1 } else { i - 1 })
                        .unwrap_or(max - 1),
                );
                true
            }
            // Enter: submit the query (ignore suggestions)
            (KeyCode::Enter, KeyModifiers::NONE) => {
                let result = match self.query_type {
                    QueryType::Select => self.select(self.value()),
                    QueryType::Filter => self.filter(self.value()),
                    QueryType::Order => self.order(self.value()),
                };
                match (result, self.query_type) {
                    (Ok(df), QueryType::Select) => {
                        Message::PaneDismissModal.enqueue();
                        Message::PanePushDataFrame(
                            df,
                            TableDescription::Select(self.value().to_owned()),
                        )
                        .enqueue();
                        Message::AppShowToast(format!(
                            "Column selection '{}' occured",
                            self.value()
                        ))
                        .enqueue();
                    }
                    (Ok(df), QueryType::Order) => {
                        Message::PaneDismissModal.enqueue();
                        Message::PanePushDataFrame(
                            df,
                            TableDescription::Order(self.value().to_owned()),
                        )
                        .enqueue();
                        Message::AppShowToast(format!(
                            "Data frame ordered by '{}'",
                            self.value()
                        ))
                        .enqueue();
                    }
                    (Ok(df), QueryType::Filter) => {
                        Message::PaneDismissModal.enqueue();
                        Message::PanePushDataFrame(
                            df,
                            TableDescription::Filter(self.value().to_owned()),
                        )
                        .enqueue();
                        Message::AppShowToast(format!("Filter '{}' applied", self.value()))
                            .enqueue();
                    }
                    (Err(err), _) => {
                        Message::PaneDismissModal.enqueue();
                        Message::AppShowError(err.to_string()).enqueue();
                    }
                }
                true
            }
            // Esc: dismiss
            (KeyCode::Esc, KeyModifiers::NONE) => {
                Message::PaneDismissModal.enqueue();
                true
            }
            // Everything else: delegate to input, then update suggestions
            _ => {
                let handled = self.input.handle(event);
                if handled {
                    self.update_suggestions();
                }
                handled
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum QueryType {
    Select,
    Filter,
    Order,
}

impl QueryType {
    fn title(&self) -> String {
        match self {
            QueryType::Select => "Select",
            QueryType::Filter => "Filter",
            QueryType::Order => "Order",
        }
        .to_owned()
    }

    /// SQL fragment prepended to the user's input so that the tokenizer sees
    /// the full clause context (e.g. a comma after `SELECT col1,` is
    /// recognised as being inside a SELECT clause).
    fn sql_prefix(&self) -> &'static str {
        match self {
            QueryType::Select => "SELECT ",
            QueryType::Filter => "SELECT * FROM _ WHERE ",
            QueryType::Order => "SELECT * FROM _ ORDER BY ",
        }
    }
}
