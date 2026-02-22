use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Flex, Layout},
    symbols::{
        border::{ROUNDED, Set},
        line::{VERTICAL_LEFT, VERTICAL_RIGHT},
    },
    widgets::{Borders, Clear, List, ListState, StatefulWidget, Widget},
};

use crate::{
    misc::config::theme,
    tui::{
        component::Component,
        widgets::{block::Block, input::Input},
    },
};

const MAX_VISIBLE_SUGGESTIONS: usize = 10;

/// A text input picker that shows a suggestion list below the input.
///
/// All behaviour is configured via the `Provider` trait: completion logic
/// (`suggestions` / `is_separator`) and domain-specific actions (`on_submit` /
/// `on_dismiss`).
#[derive(Debug)]
pub struct TextPickerWithSuggestion<P> {
    title: String,
    input: Input,
    suggestions: Vec<String>,
    selected_suggestion: Option<usize>,
    provider: P,
}

impl<P> TextPickerWithSuggestion<P>
where
    P: SuggestionProvider,
{
    pub fn new(title: impl Into<String>, provider: P) -> Self {
        Self {
            title: title.into(),
            input: Input::default(),
            suggestions: Vec::new(),
            selected_suggestion: None,
            provider,
        }
    }

    pub fn value(&self) -> &str {
        self.input.value()
    }

    fn update_suggestions(&mut self) {
        self.suggestions = self
            .provider
            .suggestions(self.input.value(), self.input.cursor());
        self.selected_suggestion = if self.suggestions.is_empty() {
            None
        } else {
            Some(0)
        };
    }

    fn accept_suggestion(&mut self) {
        if let Some(index) = self.selected_suggestion {
            if let Some(suggestion) = self.suggestions.get(index).cloned() {
                let cursor = self.input.cursor();
                let value = self.input.value().to_owned();
                let before_cursor = &value[..cursor];
                let at_cursor = value[cursor..].chars().next();

                let token_start = before_cursor
                    .char_indices()
                    .rev()
                    .find(|(_, character)| self.provider.is_separator(*character))
                    .map(|(index, character)| index + character.len_utf8())
                    .unwrap_or(0);

                let token_character_length = before_cursor[token_start..].chars().count();

                for _ in 0..token_character_length {
                    self.input.delete_prev();
                }

                for character in suggestion.chars() {
                    self.input.insert(character);
                }
                if !at_cursor.is_some_and(|character| character.is_whitespace()) {
                    self.input.insert(' ');
                }

                self.suggestions.clear();
                self.selected_suggestion = None;
            }
        }
    }
}

impl<P> Component for TextPickerWithSuggestion<P>
where
    P: SuggestionProvider,
{
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
            let [_, area] =
                Layout::vertical([Constraint::Length(3), Constraint::Length(total_height)])
                    .areas(area);

            Widget::render(Clear, area, buf);

            let [input_area, list_area] =
                Layout::vertical([Constraint::Length(2), Constraint::Fill(1)]).areas(area);

            let input_area_inner = {
                let block = Block::default()
                    .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP)
                    .title(self.title.as_str());
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
                let block = Block::default().title(self.title.as_str());
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
            // Accept the highlighted suggestion.
            (KeyCode::Tab | KeyCode::Enter, KeyModifiers::NONE) if has_suggestions => {
                self.accept_suggestion();
                true
            }
            // Next suggestion.
            (KeyCode::Down, KeyModifiers::NONE) | (KeyCode::Char('n'), KeyModifiers::CONTROL)
                if has_suggestions =>
            {
                let max = self.suggestions.len().min(MAX_VISIBLE_SUGGESTIONS);
                self.selected_suggestion = Some(
                    self.selected_suggestion
                        .map(|index| (index + 1) % max)
                        .unwrap_or(0),
                );
                true
            }
            // Previous suggestion.
            (KeyCode::Up, KeyModifiers::NONE) | (KeyCode::Char('p'), KeyModifiers::CONTROL)
                if has_suggestions =>
            {
                let max = self.suggestions.len().min(MAX_VISIBLE_SUGGESTIONS);
                self.selected_suggestion = Some(
                    self.selected_suggestion
                        .map(|index| if index == 0 { max - 1 } else { index - 1 })
                        .unwrap_or(max - 1),
                );
                true
            }
            // Submit.
            (KeyCode::Enter, KeyModifiers::NONE) => {
                self.provider.on_submit(self.input.value());
                true
            }
            // Dismiss.
            (KeyCode::Esc, KeyModifiers::NONE) => {
                self.provider.on_dismiss();
                true
            }
            // Delegate to input, then refresh suggestions.
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

/// Drives the behaviour of a `TextPickerWithSuggestion`.
pub trait SuggestionProvider {
    /// Return completion suggestions for the given input value at the given
    /// cursor position.
    fn suggestions(&self, value: &str, cursor: usize) -> Vec<String>;

    /// Whether the given character is a word separator for suggestion
    /// acceptance.
    fn is_separator(&self, character: char) -> bool;

    /// Called when the user presses Enter with no active suggestion.
    fn on_submit(&self, value: &str);

    /// Called when the user presses Esc.
    fn on_dismiss(&self);
}
