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
    handler::message::Message,
    misc::{config::theme, sql::sql},
    tui::{
        component::Component,
        popups::sql_completion::{
            self, CompletionContext, collect_all_columns, filter_by_prefix, get_table_columns,
            get_table_names,
        },
        widgets::{block::Block, input::Input},
    },
};

const MAX_VISIBLE_SUGGESTIONS: usize = 10;

#[derive(Debug)]
pub struct SqlQueryPicker {
    df: Option<DataFrame>,
    input: Input,
    all_columns: Vec<String>,
    suggestions: Vec<String>,
    selected_suggestion: Option<usize>,
}

impl SqlQueryPicker {
    pub fn new(df: Option<DataFrame>) -> Self {
        let all_columns = collect_all_columns(df.as_ref());
        Self {
            df,
            input: Input::default(),
            all_columns,
            suggestions: Vec::new(),
            selected_suggestion: None,
        }
    }

    fn update_suggestions(&mut self) {
        let cursor = self.input.cursor();
        let value = self.input.value().to_owned();

        let (token, context) = sql_completion::extract_token_and_context(&value, cursor, "");

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
                let columns = get_table_columns(table, self.df.as_ref());
                self.suggestions = filter_by_prefix(columns.iter(), &token_lower);
            }
            CompletionContext::Table => {
                let tables = get_table_names(self.df.is_some());
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

                let token_start = before_cursor
                    .char_indices()
                    .rev()
                    .find(|(_, c)| sql_completion::is_separator(*c))
                    .map(|(i, c)| i + c.len_utf8())
                    .unwrap_or(0);

                let token_char_len = before_cursor[token_start..].chars().count();

                for _ in 0..token_char_len {
                    self.input.delete_prev();
                }

                for c in suggestion.chars() {
                    self.input.insert(c);
                }

                self.suggestions.clear();
                self.selected_suggestion = None;
            }
        }
    }
}

impl Component for SqlQueryPicker {
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

            let input_area_inner = {
                let block = Block::default()
                    .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP)
                    .title("SQL");
                let inner = block.inner(input_area);
                Widget::render(block, input_area, buf);
                inner
            };
            self.input.render(input_area_inner, buf, focus_state);

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
            let [area] = Layout::horizontal([Constraint::Length(width)])
                .flex(Flex::Center)
                .areas(buf.area);
            let [_, area] =
                Layout::vertical([Constraint::Length(3), Constraint::Length(3)]).areas(area);

            Widget::render(Clear, area, buf);

            let area_inner = {
                let block = Block::default().title("SQL");
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
            (KeyCode::Tab | KeyCode::Enter, KeyModifiers::NONE) if has_suggestions => {
                self.accept_suggestion();
                true
            }
            (KeyCode::Down, KeyModifiers::NONE) | (KeyCode::Char('n'), KeyModifiers::CONTROL)
                if has_suggestions =>
            {
                let max = self.suggestions.len().min(MAX_VISIBLE_SUGGESTIONS);
                self.selected_suggestion =
                    Some(self.selected_suggestion.map(|i| (i + 1) % max).unwrap_or(0));
                true
            }
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
            (KeyCode::Enter, KeyModifiers::NONE) => {
                Message::AppDismissOverlay.enqueue();
                match sql().execute(self.input.value(), self.df.clone()) {
                    Ok(df) => {
                        Message::TabsAddQueryPane(df, self.input.value().to_owned()).enqueue();
                    }
                    Err(err) => Message::AppShowError(err.to_string()).enqueue(),
                }
                true
            }
            (KeyCode::Esc, KeyModifiers::NONE) => {
                Message::AppDismissOverlay.enqueue();
                true
            }
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
