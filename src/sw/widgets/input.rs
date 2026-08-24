use std::borrow::Cow;

use crate::{misc::config::theme, tui::widgets::block::Block};
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Paragraph, StatefulWidget, Widget},
};

#[derive(Debug, Default)]
pub struct InputState {
    input: tui_input::Input,
    input_type: InputType,
    max_len: Option<usize>,
}

impl InputState {
    pub fn with_max_len(self, max_len: impl Into<Option<usize>>) -> Self {
        InputState {
            max_len: max_len.into(),
            ..self
        }
    }

    pub fn with_input_type(self, input_type: InputType) -> Self {
        InputState { input_type, ..self }
    }

    pub fn with_value(self, value: String) -> Self {
        Self {
            input: self.input.with_value(value),
            ..self
        }
    }

    pub fn value(&self) -> &str {
        self.input.value()
    }

    pub fn cursor(&self) -> usize {
        self.input.cursor()
    }

    pub fn input_type(&self) -> InputType {
        self.input_type
    }

    pub fn goto_prev(&mut self) {
        self.input.handle(tui_input::InputRequest::GoToPrevChar);
    }

    pub fn goto_next(&mut self) {
        self.input.handle(tui_input::InputRequest::GoToNextChar);
    }

    pub fn goto_start(&mut self) {
        self.input.handle(tui_input::InputRequest::GoToStart);
    }

    pub fn goto_end(&mut self) {
        self.input.handle(tui_input::InputRequest::GoToEnd);
    }

    pub fn goto_prev_word(&mut self) {
        self.input.handle(tui_input::InputRequest::GoToPrevWord);
    }

    pub fn goto_next_word(&mut self) {
        self.input.handle(tui_input::InputRequest::GoToNextWord);
    }

    pub fn set_value(&mut self, value: String) {
        self.input = tui_input::Input::new(value);
    }

    pub fn insert(&mut self, c: char) {
        if let Some(max_len) = self.max_len {
            if self.value().chars().count() < max_len {
                self.input.handle(tui_input::InputRequest::InsertChar(c));
            }
        } else {
            self.input.handle(tui_input::InputRequest::InsertChar(c));
        }
    }

    pub fn delete_prev(&mut self) {
        self.input.handle(tui_input::InputRequest::DeletePrevChar);
    }

    pub fn delete_next(&mut self) {
        self.input.handle(tui_input::InputRequest::DeleteNextChar);
    }

    pub fn delete_prev_word(&mut self) {
        self.input.handle(tui_input::InputRequest::DeletePrevWord);
    }

    pub fn delete_next_word(&mut self) {
        self.input.handle(tui_input::InputRequest::DeleteNextWord);
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum InputType {
    #[default]
    Any,
    Numeric,
    Alphabetic,
    MultiNumeric,
}

#[derive(Debug)]
pub struct Input<'a> {
    block: Option<Block<'a>>,
    hint: Cow<'a, str>,
    style: Style,
    hint_style: Style,
    selection: bool,
}

impl<'a> Input<'a> {
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn hint(mut self, hint: impl Into<Cow<'a, str>>) -> Self {
        self.hint = hint.into();
        self
    }

    pub fn hint_style(mut self, style: impl Into<Style>) -> Self {
        self.hint_style = style.into();
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn with_show_cursor(mut self, selection: bool) -> Self {
        self.selection = selection;
        self
    }
}

impl<'a> Default for Input<'a> {
    fn default() -> Self {
        Self {
            block: None,
            hint: Default::default(),
            style: theme().text(),
            hint_style: theme().subtext(),
            selection: true,
        }
    }
}

impl StatefulWidget for Input<'_> {
    type State = InputState;

    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        // draw block and update area
        let area = if let Some(block) = self.block {
            let new_area = block.inner(area);
            block.render(area, buf);
            new_area
        } else {
            area
        };

        if state.input.value().is_empty() {
            // draw hint
            Paragraph::new(self.hint)
                .style(self.hint_style)
                .render(area, buf);
            // draw cursor
            if self.selection {
                buf.set_style(
                    Rect {
                        x: area.x,
                        y: area.y,
                        width: 1,
                        height: 1,
                    },
                    self.style.add_modifier(Modifier::REVERSED),
                );
            }
        } else {
            // draw text
            let scroll = state
                .input
                .visual_scroll(area.width.saturating_sub(1).into());
            Paragraph::new(state.input.value().chars().skip(scroll).collect::<String>())
                .style(self.style)
                .render(area, buf);
            // draw cursor
            if self.selection {
                buf.set_style(
                    Rect {
                        x: area.x + state.input.visual_cursor().saturating_sub(scroll) as u16,
                        y: area.y,
                        width: 1,
                        height: 1,
                    },
                    self.style.add_modifier(Modifier::REVERSED),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::buffer::Buffer;

    mod state {
        use super::*;

        #[test]
        fn insert_appends_to_value() {
            let mut state = InputState::default();
            state.insert('a');
            state.insert('b');
            state.insert('c');
            assert_eq!(state.value(), "abc");
            assert_eq!(state.cursor(), 3);
        }

        #[test]
        fn insert_respects_max_len() {
            let mut state = InputState::default().with_max_len(2);
            state.insert('a');
            state.insert('b');
            state.insert('c');
            assert_eq!(state.value(), "ab");
        }

        #[test]
        fn with_max_len_none_is_unbounded() {
            let mut state = InputState::default().with_max_len(None);
            for c in "abcdefghij".chars() {
                state.insert(c);
            }
            assert_eq!(state.value(), "abcdefghij");
        }

        #[test]
        fn delete_prev_and_next() {
            let mut state = InputState::default().with_value("abc".to_owned());
            state.goto_start();
            state.goto_next(); // cursor between a|bc
            state.delete_next(); // abc -> ac, cursor stays
            assert_eq!(state.value(), "ac");
            state.delete_prev(); // ac -> c
            assert_eq!(state.value(), "c");
        }

        #[test]
        fn goto_start_and_end() {
            let mut state = InputState::default().with_value("abc".to_owned());
            state.goto_start();
            assert_eq!(state.cursor(), 0);
            state.goto_end();
            assert_eq!(state.cursor(), 3);
        }

        #[test]
        fn goto_prev_and_next_char() {
            let mut state = InputState::default().with_value("abc".to_owned());
            state.goto_start();
            state.goto_next();
            state.goto_next();
            assert_eq!(state.cursor(), 2);
            state.goto_prev();
            assert_eq!(state.cursor(), 1);
        }

        #[test]
        fn word_navigation() {
            let mut state = InputState::default().with_value("foo bar baz".to_owned());
            state.goto_start();
            state.goto_next_word();
            assert_eq!(state.cursor(), 4); // start of "bar"
            state.goto_next_word();
            assert_eq!(state.cursor(), 8); // start of "baz"
            state.goto_prev_word();
            assert_eq!(state.cursor(), 4); // back to start of "bar"
        }

        #[test]
        fn delete_word_forward_and_backward() {
            let mut state = InputState::default().with_value("foo bar baz".to_owned());
            state.goto_start();
            state.delete_next_word();
            assert_eq!(state.value(), "bar baz");

            state.goto_end();
            state.delete_prev_word();
            assert_eq!(state.value(), "bar ");
        }

        #[test]
        fn set_value_replaces_content_and_moves_cursor_to_end() {
            let mut state = InputState::default().with_value("abc".to_owned());
            state.goto_start();
            state.set_value("hello".to_owned());
            assert_eq!(state.value(), "hello");
            assert_eq!(state.cursor(), 5);
        }
    }

    mod widget {
        use super::*;

        #[test]
        fn renders_hint_when_empty() {
            let state = &mut InputState::default();
            let area = Rect::new(0, 0, 10, 1);
            let mut buf = Buffer::empty(area);
            Input::default()
                .hint("search...")
                .render(area, &mut buf, state);

            let mut expected = Buffer::with_lines(["search... "]);
            expected.set_style(Rect::new(0, 0, 10, 1), theme().subtext());
            expected[(0, 0)].set_style(theme().text().add_modifier(Modifier::REVERSED));
            assert_eq!(buf, expected);
        }

        #[test]
        fn renders_value_text_instead_of_hint() {
            let state = &mut InputState::default().with_value("hi".to_owned());
            let area = Rect::new(0, 0, 10, 1);
            let mut buf = Buffer::empty(area);
            Input::default()
                .hint("search...")
                .render(area, &mut buf, state);

            let mut expected = Buffer::with_lines(["hi        "]);
            expected.set_style(Rect::new(0, 0, 10, 1), theme().text());
            // cursor sits right after "hi", at visual column 2
            expected[(2, 0)].set_style(theme().text().add_modifier(Modifier::REVERSED));
            assert_eq!(buf, expected);
        }

        #[test]
        fn cursor_tracks_navigation() {
            let state = &mut InputState::default().with_value("abc".to_owned());
            state.goto_start();
            let area = Rect::new(0, 0, 10, 1);
            let mut buf = Buffer::empty(area);
            Input::default().render(area, &mut buf, state);

            assert!(
                buf[(0, 0)]
                    .style()
                    .add_modifier
                    .contains(Modifier::REVERSED)
            );
            assert!(
                !buf[(1, 0)]
                    .style()
                    .add_modifier
                    .contains(Modifier::REVERSED)
            );
        }

        #[test]
        fn selection_false_hides_cursor() {
            let state = &mut InputState::default().with_value("abc".to_owned());
            let area = Rect::new(0, 0, 10, 1);
            let mut buf = Buffer::empty(area);
            Input::default()
                .with_show_cursor(false)
                .render(area, &mut buf, state);

            for x in 0..10 {
                assert!(
                    !buf[(x, 0)]
                        .style()
                        .add_modifier
                        .contains(Modifier::REVERSED),
                    "cell {x} should not be reversed when selection is disabled"
                );
            }
        }
    }
}
