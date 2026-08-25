use std::borrow::Cow;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, StatefulWidget},
};

use crate::sw::{
    app_default::AppDefault,
    buffer_ext::BufferExt,
    rect_ext::RectExt,
    widgets::input::{Input, InputState, InputType},
};

const WIDTH: u16 = 32;
const HEIGHT: u16 = 3;

#[derive(Debug)]
pub struct GoToLineState {
    input: InputState,
    rollback: usize,
}

impl GoToLineState {
    pub fn new(rollback: usize) -> Self {
        Self {
            input: InputState::default().with_input_type(InputType::Numeric),
            rollback,
        }
    }

    pub fn with_value(self, value: usize) -> Self {
        Self {
            input: self.input.with_value(value.to_string()),
            ..self
        }
    }

    pub fn input(&self) -> &InputState {
        &self.input
    }

    pub fn input_mut(&mut self) -> &mut InputState {
        &mut self.input
    }

    pub fn rollback(&self) -> usize {
        self.rollback
    }

    pub fn row(&self) -> usize {
        self.input
            .value()
            .parse::<usize>()
            .unwrap_or(1)
            .saturating_sub(1)
    }
}

#[derive(Debug)]
pub struct GoToLine<'a> {
    title: Cow<'a, str>,
}

impl<'a> GoToLine<'a> {
    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self {
        self.title = title.into();
        self
    }
}

impl Default for GoToLine<'_> {
    fn default() -> Self {
        Self {
            title: Cow::Borrowed("Go to Line"),
        }
    }
}

impl StatefulWidget for GoToLine<'_> {
    type State = GoToLineState;

    fn render(self, _area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let area = buf.area.goto_line(WIDTH, HEIGHT);
        buf.clear(area);

        Input::default()
            .block(Block::app_default().title(self.title))
            .render(area, buf, &mut state.input);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn typed(state: &mut GoToLineState, text: &str) {
        for c in text.chars() {
            state.input_mut().insert(c);
        }
    }

    fn render(state: &mut GoToLineState, go_to_line: GoToLine) -> Buffer {
        let area = Rect::new(0, 0, 100, 30);
        let mut buf = Buffer::empty(area);
        go_to_line.render(area, &mut buf, state);
        buf
    }

    fn content(buf: &Buffer) -> String {
        buf.content().iter().map(|c| c.symbol()).collect()
    }

    mod state {
        use super::*;

        #[test]
        fn an_empty_input_is_the_first_row() {
            assert_eq!(GoToLineState::new(7).row(), 0);
        }

        #[test]
        fn a_line_number_is_one_based() {
            let mut state = GoToLineState::new(0);
            typed(&mut state, "5");

            assert_eq!(state.row(), 4);
        }

        #[test]
        fn line_zero_is_still_the_first_row() {
            let mut state = GoToLineState::new(0);
            typed(&mut state, "0");

            assert_eq!(state.row(), 0);
        }

        #[test]
        fn with_value_seeds_the_input() {
            let state = GoToLineState::new(0).with_value(12);

            assert_eq!(state.input().value(), "12");
            assert_eq!(state.row(), 11);
        }

        #[test]
        fn the_rollback_row_is_remembered() {
            let mut state = GoToLineState::new(42);
            typed(&mut state, "9");

            assert_eq!(state.rollback(), 42);
            assert_eq!(state.row(), 8);
        }

        #[test]
        fn the_input_is_marked_numeric_for_the_dispatcher_to_enforce() {
            let state = GoToLineState::new(0);

            assert!(matches!(state.input().input_type(), InputType::Numeric));
        }

        #[test]
        fn a_non_numeric_value_falls_back_to_the_first_row() {
            let mut state = GoToLineState::new(0);
            typed(&mut state, "abc");

            assert_eq!(state.row(), 0);
        }

        #[test]
        fn editing_the_input_updates_the_row() {
            let mut state = GoToLineState::new(0);
            typed(&mut state, "25");
            assert_eq!(state.row(), 24);

            state.input_mut().delete_prev();
            assert_eq!(state.row(), 1);
        }
    }

    mod widget {
        use super::*;

        #[test]
        fn defaults_to_a_go_to_line_title() {
            let buf = render(&mut GoToLineState::new(0), GoToLine::default());

            assert!(content(&buf).contains("Go to Line"));
        }

        #[test]
        fn the_title_is_overridable() {
            let buf = render(&mut GoToLineState::new(0), GoToLine::default().title("Jump"));

            let content = content(&buf);
            assert!(content.contains("Jump"));
            assert!(!content.contains("Go to Line"));
        }

        #[test]
        fn renders_the_typed_line_number() {
            let mut state = GoToLineState::new(0);
            typed(&mut state, "123");
            let buf = render(&mut state, GoToLine::default());

            assert!(content(&buf).contains("123"));
        }

        #[test]
        fn is_placed_in_the_top_right_corner() {
            let mut state = GoToLineState::new(0);
            let buf = render(&mut state, GoToLine::default());
            let expected = Rect::new(0, 0, 100, 30).goto_line(WIDTH, HEIGHT);

            assert_eq!(buf[(expected.x, expected.y)].symbol(), "╭");
            assert_eq!(buf[(expected.right() - 1, expected.y)].symbol(), "╮");
        }

        #[test]
        fn leaves_the_rest_of_the_buffer_alone() {
            let mut state = GoToLineState::new(0);
            let buf = render(&mut state, GoToLine::default());

            assert_eq!(buf[(0, 0)].symbol(), " ");
            assert_eq!(buf[(0, 29)].symbol(), " ");
        }
    }
}
