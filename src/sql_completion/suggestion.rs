use crate::tui::{pickers::text_picker_with_suggestion::Suggestion, widgets::input::Input};

use super::extraction::{cursor_byte_offset, is_separator};

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

    pub fn edit(&self, value: &str, cursor: usize) -> (usize, String) {
        let cursor = cursor_byte_offset(value, cursor);
        let before_cursor = &value[..cursor];
        let at_cursor = value[cursor..].chars().next();

        // Find the start of the current token by scanning backwards for a separator.
        let token_start = before_cursor
            .char_indices()
            .rev()
            .find(|(_, character)| is_separator(*character))
            .map(|(index, character)| index + character.len_utf8())
            .unwrap_or(0);

        let delete = before_cursor[token_start..].chars().count();

        let mut insert = if self.text.contains(' ') {
            format!("\"{}\"", self.text)
        } else {
            self.text.clone()
        };

        // Add a trailing space unless the character at the old cursor is already whitespace.
        if !at_cursor.is_some_and(|character| character.is_whitespace()) {
            insert.push(' ');
        }

        (delete, insert)
    }
}

impl Suggestion for SqlSuggestion {
    fn title(&self) -> &str {
        &self.text
    }

    fn apply_to(&self, input: &mut Input) {
        let (delete, insert) = self.edit(input.value(), input.cursor());
        for _ in 0..delete {
            input.delete_prev();
        }
        for character in insert.chars() {
            input.insert(character);
        }
    }
}

impl crate::sw::pickers::text_picker_with_suggestion::Suggestion for SqlSuggestion {
    fn title(&self) -> &str {
        &self.text
    }

    fn apply_to(&self, input: &mut crate::sw::widgets::input::InputState) {
        let (delete, insert) = self.edit(input.value(), input.cursor());
        for _ in 0..delete {
            input.delete_prev();
        }
        for character in insert.chars() {
            input.insert(character);
        }
    }
}
