use std::borrow::Cow;

use ratatui::widgets::{Block, StatefulWidget};

use crate::sw::{
    app_default::AppDefault,
    buffer_ext::BufferExt,
    rect_ext::RectExt,
    widgets::input::{Input, InputState, InputType},
};

#[derive(Debug, Default)]
pub struct TextPickerState {
    input: InputState,
}

impl TextPickerState {
    pub fn with_max_len(self, max_len: usize) -> Self {
        Self {
            input: self.input.with_max_len(max_len),
        }
    }

    pub fn with_value(self, value: String) -> Self {
        Self {
            input: self.input.with_value(value),
        }
    }

    pub fn with_input_type(self, input_type: InputType) -> Self {
        Self {
            input: self.input.with_input_type(input_type),
        }
    }

    pub fn value(&self) -> &str {
        self.input.value()
    }

    pub fn input(&self) -> &InputState {
        &self.input
    }

    pub fn input_mut(&mut self) -> &mut InputState {
        &mut self.input
    }
}

#[derive(Debug)]
pub struct TextPicker<'a> {
    title: Cow<'a, str>,
    hint: Cow<'a, str>,
    darken_bg: bool,
}

impl<'a> TextPicker<'a> {
    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self {
        self.title = title.into();
        self
    }

    pub fn hint(mut self, hint: impl Into<Cow<'a, str>>) -> Self {
        self.hint = hint.into();
        self
    }

    pub fn no_darken_bg(mut self) -> Self {
        self.darken_bg = false;
        self
    }
}

impl<'a> Default for TextPicker<'a> {
    fn default() -> Self {
        Self {
            title: Cow::Borrowed(""),
            hint: Cow::Borrowed(""),
            darken_bg: true,
        }
    }
}

impl StatefulWidget for TextPicker<'_> {
    type State = TextPickerState;

    fn render(
        self,
        _area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        if self.darken_bg {
            buf.darken();
        }

        let area = buf.area.palette(3);
        buf.clear(area);

        Input::default()
            .block(Block::app_default().title(self.title))
            .hint(self.hint)
            .render(area, buf, &mut state.input);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{buffer::Buffer, layout::Rect, style::Color};

    mod state {
        use super::*;

        #[test]
        fn value_reflects_with_value() {
            let state = TextPickerState::default().with_value("hello".to_owned());
            assert_eq!(state.value(), "hello");
        }

        #[test]
        fn with_max_len_limits_input() {
            let mut state = TextPickerState::default().with_max_len(2);
            state.input_mut().insert('a');
            state.input_mut().insert('b');
            state.input_mut().insert('c');
            assert_eq!(state.value(), "ab");
        }

        #[test]
        fn with_input_type_propagates_to_input() {
            let state = TextPickerState::default().with_input_type(InputType::Numeric);
            assert!(matches!(state.input().input_type(), InputType::Numeric));
        }

        #[test]
        fn input_mut_edits_are_visible_through_value() {
            let mut state = TextPickerState::default();
            state.input_mut().insert('x');
            state.input_mut().insert('y');
            assert_eq!(state.value(), "xy");
            state.input_mut().delete_prev();
            assert_eq!(state.value(), "x");
        }
    }

    mod widget {
        use super::*;

        fn content(buf: &Buffer) -> String {
            buf.content().iter().map(|c| c.symbol()).collect()
        }

        #[test]
        fn renders_title_and_value() {
            let area = Rect::new(0, 0, 100, 30);
            let mut buf = Buffer::empty(area);
            let mut state = TextPickerState::default().with_value("polars".to_owned());
            TextPicker::default()
                .title("Separator")
                .render(area, &mut buf, &mut state);

            let content = content(&buf);
            assert!(content.contains("Separator"));
            assert!(content.contains("polars"));
        }

        #[test]
        fn renders_hint_when_value_is_empty() {
            let area = Rect::new(0, 0, 100, 30);
            let mut buf = Buffer::empty(area);
            let mut state = TextPickerState::default();
            TextPicker::default()
                .hint("type here")
                .render(area, &mut buf, &mut state);

            assert!(content(&buf).contains("type here"));
        }

        #[test]
        fn hint_is_hidden_once_value_is_present() {
            let area = Rect::new(0, 0, 100, 30);
            let mut buf = Buffer::empty(area);
            let mut state = TextPickerState::default().with_value("abc".to_owned());
            TextPicker::default()
                .hint("type here")
                .render(area, &mut buf, &mut state);

            let content = content(&buf);
            assert!(content.contains("abc"));
            assert!(!content.contains("type here"));
        }

        #[test]
        fn occupies_single_line_palette_area() {
            let area = Rect::new(0, 0, 100, 30);
            let mut buf = Buffer::empty(area);
            let mut state = TextPickerState::default();
            TextPicker::default()
                .title("T")
                .render(area, &mut buf, &mut state);

            assert_eq!(area.palette(3), Rect::new(10, 3, 80, 3));
        }

        #[test]
        fn darken_bg_scales_colors_outside_popup() {
            let area = Rect::new(0, 0, 100, 30);
            let mut buf = Buffer::empty(area);
            buf[(0, 0)].set_bg(Color::Rgb(100, 150, 200));
            buf[(0, 0)].set_fg(Color::Rgb(100, 150, 200));

            let mut state = TextPickerState::default();
            TextPicker::default().render(area, &mut buf, &mut state);

            assert_eq!(buf[(0, 0)].bg, Color::Rgb(20, 30, 40));
            assert_eq!(buf[(0, 0)].fg, Color::Rgb(20, 30, 40));
        }

        #[test]
        fn no_darken_bg_leaves_colors_untouched() {
            let area = Rect::new(0, 0, 100, 30);
            let mut buf = Buffer::empty(area);
            buf[(0, 0)].set_bg(Color::Rgb(100, 150, 200));

            let mut state = TextPickerState::default();
            TextPicker::default()
                .no_darken_bg()
                .render(area, &mut buf, &mut state);

            assert_eq!(buf[(0, 0)].bg, Color::Rgb(100, 150, 200));
        }
    }
}
