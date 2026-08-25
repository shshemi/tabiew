use std::borrow::Cow;

use ratatui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};

use crate::sw::{
    pickers::text_picker::{TextPicker, TextPickerState},
    widgets::input::InputState,
};

#[derive(Debug, Default)]
pub struct TableRegistererState {
    text_picker: TextPickerState,
}

impl TableRegistererState {
    pub fn input(&self) -> &InputState {
        self.text_picker.input()
    }

    pub fn input_mut(&mut self) -> &mut InputState {
        self.text_picker.input_mut()
    }

    pub fn name(&self) -> &str {
        self.text_picker.value().trim()
    }
}

#[derive(Debug)]
pub struct TableRegisterer<'a> {
    title: Cow<'a, str>,
}

impl<'a> TableRegisterer<'a> {
    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self {
        self.title = title.into();
        self
    }
}

impl Default for TableRegisterer<'_> {
    fn default() -> Self {
        Self {
            title: Cow::Borrowed("Name"),
        }
    }
}

impl StatefulWidget for TableRegisterer<'_> {
    type State = TableRegistererState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        TextPicker::default()
            .title(self.title)
            .render(area, buf, &mut state.text_picker);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn typed(state: &mut TableRegistererState, text: &str) {
        for c in text.chars() {
            state.input_mut().insert(c);
        }
    }

    fn render(state: &mut TableRegistererState, registerer: TableRegisterer) -> Buffer {
        let area = Rect::new(0, 0, 100, 30);
        let mut buf = Buffer::empty(area);
        registerer.render(area, &mut buf, state);
        buf
    }

    fn content(buf: &Buffer) -> String {
        buf.content().iter().map(|c| c.symbol()).collect()
    }

    mod state {
        use super::*;

        #[test]
        fn a_fresh_registerer_has_no_name() {
            assert_eq!(TableRegistererState::default().name(), "");
        }

        #[test]
        fn the_name_follows_the_typed_input() {
            let mut state = TableRegistererState::default();
            typed(&mut state, "sales");

            assert_eq!(state.name(), "sales");
        }

        #[test]
        fn surrounding_whitespace_is_trimmed() {
            let mut state = TableRegistererState::default();
            typed(&mut state, "  sales  ");

            assert_eq!(state.name(), "sales");
        }

        #[test]
        fn a_whitespace_only_name_is_empty() {
            let mut state = TableRegistererState::default();
            typed(&mut state, "   ");

            assert_eq!(state.name(), "");
        }

        #[test]
        fn inner_whitespace_is_kept() {
            let mut state = TableRegistererState::default();
            typed(&mut state, " my table ");

            assert_eq!(state.name(), "my table");
        }

        #[test]
        fn editing_the_input_updates_the_name() {
            let mut state = TableRegistererState::default();
            typed(&mut state, "sales");
            state.input_mut().delete_prev();

            assert_eq!(state.name(), "sale");
        }
    }

    mod widget {
        use super::*;

        #[test]
        fn defaults_to_a_name_title() {
            let buf = render(
                &mut TableRegistererState::default(),
                TableRegisterer::default(),
            );

            assert!(content(&buf).contains("Name"));
        }

        #[test]
        fn the_title_is_overridable() {
            let buf = render(
                &mut TableRegistererState::default(),
                TableRegisterer::default().title("Register As"),
            );

            let content = content(&buf);
            assert!(content.contains("Register As"));
            assert!(!content.contains("Name"));
        }

        #[test]
        fn renders_the_typed_name() {
            let mut state = TableRegistererState::default();
            typed(&mut state, "sales");
            let buf = render(&mut state, TableRegisterer::default());

            assert!(content(&buf).contains("sales"));
        }
    }
}
