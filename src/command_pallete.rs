use ratatui::{layout::Rect, style::Style, widgets::StatefulWidget};

#[derive(Debug)]
pub struct CommandPalleteState {
    chars: Vec<Vec<char>>,
    cursor: (usize, usize),
}

impl CommandPalleteState {
    pub fn input_char(&mut self, character: char) -> &mut Self {
        self.chars[self.cursor.0].insert(self.cursor.1, character);
        self.cursor.1 += 1;
        self
    }

    pub fn delete(&mut self) -> &mut Self {
        if self.cursor.1 < self.chars[self.cursor.0].len() {
            self.chars[self.cursor.0].remove(self.cursor.1);
        }
        self
    }

    pub fn delete_backward(&mut self) -> &mut Self {
        if self.cursor.1 > 0 {
            self.chars[self.cursor.0].remove(self.cursor.1 - 1);
            self.cursor.1 -= 1
        }
        self
    }

    pub fn move_up(&mut self) -> &mut Self {
        self.move_cursor(self.cursor.0.saturating_sub(1), self.cursor.1);
        self
    }

    pub fn move_down(&mut self) -> &mut Self {
        self.move_cursor(self.cursor.0.saturating_add(1), self.cursor.1);
        self
    }

    pub fn move_left(&mut self) -> &mut Self {
        self.move_cursor(self.cursor.0, self.cursor.1.saturating_sub(1));
        self
    }

    pub fn move_right(&mut self) -> &mut Self {
        self.move_cursor(self.cursor.0, self.cursor.1.saturating_add(1));
        self
    }

    pub fn move_bol(&mut self) -> &mut Self {
        self.move_cursor(self.cursor.0, 0);
        self
    }

    pub fn move_eol(&mut self) -> &mut Self {
        self.move_cursor(self.cursor.0, usize::MAX);
        self
    }

    pub fn command(&self) -> String {
        self.chars[self.cursor.0].iter().collect()
    }

    pub fn command_len(&self) -> usize {
        self.chars[self.cursor.0].len()
    }

    pub fn cursor(&self) -> (usize, usize) {
        self.cursor
    }

    #[inline]
    fn move_cursor(&mut self, x0: usize, x1: usize) {
        let x0 = x0.min(self.chars.len().saturating_sub(1));
        let x1 = x1.min(self.chars[x0].len());
        self.cursor = (x0, x1);
    }
}

impl From<Vec<String>> for CommandPalleteState {
    fn from(value: Vec<String>) -> Self {
        Self {
            cursor: (
                value.len().saturating_sub(1),
                value
                    .last()
                    .map(|str| str.chars().count())
                    .unwrap_or_default(),
            ),
            chars: value.into_iter().map(|str| str.chars().collect()).collect(),
        }
    }
}

pub struct CommandPallete {
    style: Style,
    cursor_style: Style,
}

impl CommandPallete {
    pub fn new(style: Style, cursor_style: Style) -> Self {
        Self {
            style,
            cursor_style,
        }
    }
}

impl StatefulWidget for CommandPallete {
    type State = CommandPalleteState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        buf.set_string(area.x, area.y, state.command(), self.style);
        buf.set_style(area, self.style);
        buf.set_style(
            Rect {
                x: area.x + state.cursor.1 as u16,
                y: area.y,
                width: 1,
                height: 1,
            },
            self.cursor_style,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_test() {
        let mut state = CommandPalleteState::from(vec!["".to_owned()]);
        println!("{}", state.command());
        state.input_char('c');
        state.input_char('h');
        state.input_char('a');
        state.input_char('r');
        assert_eq!(state.command(), "char")
    }
}
