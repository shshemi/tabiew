use crate::tui::{pickers::text_picker_with_suggestion::Suggestion, widgets::input::Input};

use super::extraction::is_separator;

/// A SQL completion suggestion that replaces the partial token before the cursor
/// with the completed text.
#[derive(Debug, Clone)]
pub struct SqlSuggestion {
    text: String,
}

impl SqlSuggestion {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}

impl Suggestion for SqlSuggestion {
    fn title(&self) -> &str {
        &self.text
    }

    fn apply_to(&self, input: &mut Input) {
        let cursor = input.cursor();
        let value = input.value().to_owned();
        let before_cursor = &value[..cursor];
        let at_cursor = value[cursor..].chars().next();

        // Find the start of the current token by scanning backwards for a separator.
        let token_start = before_cursor
            .char_indices()
            .rev()
            .find(|(_, character)| is_separator(*character))
            .map(|(index, character)| index + character.len_utf8())
            .unwrap_or(0);

        let token_character_length = before_cursor[token_start..].chars().count();

        // Delete the partial token.
        for _ in 0..token_character_length {
            input.delete_prev();
        }

        // Insert the completed text.
        for character in self.text.chars() {
            input.insert(character);
        }

        // Add a trailing space unless the character at the old cursor is already whitespace.
        if !at_cursor.is_some_and(|character| character.is_whitespace()) {
            input.insert(' ');
        }
    }
}
