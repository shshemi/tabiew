use std::borrow::Cow;

use ratatui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};
use url::Url;

use crate::{
    AppResult,
    sw::{
        pickers::text_picker::{TextPicker, TextPickerState},
        widgets::input::InputState,
    },
};

#[derive(Debug, Default)]
pub struct UrlPickerState {
    text_picker: TextPickerState,
}

impl UrlPickerState {
    pub fn input(&self) -> &InputState {
        self.text_picker.input()
    }

    pub fn input_mut(&mut self) -> &mut InputState {
        self.text_picker.input_mut()
    }

    pub fn value(&self) -> &str {
        self.text_picker.value()
    }

    pub fn url(&self) -> AppResult<Url> {
        Ok(self.value().parse()?)
    }
}

#[derive(Debug)]
pub struct UrlPicker<'a> {
    title: Cow<'a, str>,
}

impl<'a> UrlPicker<'a> {
    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self {
        self.title = title.into();
        self
    }
}

impl Default for UrlPicker<'_> {
    fn default() -> Self {
        Self {
            title: Cow::Borrowed("URL"),
        }
    }
}

impl StatefulWidget for UrlPicker<'_> {
    type State = UrlPickerState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        TextPicker::default()
            .title(self.title)
            .render(area, buf, &mut state.text_picker);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn typed(state: &mut UrlPickerState, text: &str) {
        for c in text.chars() {
            state.input_mut().insert(c);
        }
    }

    fn render(state: &mut UrlPickerState, picker: UrlPicker) -> Buffer {
        let area = Rect::new(0, 0, 100, 30);
        let mut buf = Buffer::empty(area);
        picker.render(area, &mut buf, state);
        buf
    }

    fn content(buf: &Buffer) -> String {
        buf.content().iter().map(|c| c.symbol()).collect()
    }

    mod state {
        use super::*;

        #[test]
        fn a_fresh_picker_is_empty() {
            assert_eq!(UrlPickerState::default().value(), "");
        }

        #[test]
        fn an_empty_value_is_not_a_url() {
            assert!(UrlPickerState::default().url().is_err());
        }

        #[test]
        fn a_well_formed_url_parses() {
            let mut state = UrlPickerState::default();
            typed(&mut state, "https://example.com/data.csv");

            let url = state.url().unwrap();
            assert_eq!(url.scheme(), "https");
            assert_eq!(url.host_str(), Some("example.com"));
            assert_eq!(url.path(), "/data.csv");
        }

        #[test]
        fn a_url_without_a_scheme_is_rejected() {
            let mut state = UrlPickerState::default();
            typed(&mut state, "example.com/data.csv");

            assert!(state.url().is_err());
        }

        #[test]
        fn value_tracks_the_input_as_it_is_edited() {
            let mut state = UrlPickerState::default();
            typed(&mut state, "ftp://a");
            assert_eq!(state.value(), "ftp://a");

            state.input_mut().delete_prev();
            assert_eq!(state.value(), "ftp://");
        }
    }

    mod widget {
        use super::*;

        #[test]
        fn defaults_to_a_url_title() {
            let buf = render(&mut UrlPickerState::default(), UrlPicker::default());

            assert!(content(&buf).contains("URL"));
        }

        #[test]
        fn the_title_is_overridable() {
            let buf = render(
                &mut UrlPickerState::default(),
                UrlPicker::default().title("Source"),
            );

            let content = content(&buf);
            assert!(content.contains("Source"));
            assert!(!content.contains("URL"));
        }

        #[test]
        fn renders_the_typed_value() {
            let mut state = UrlPickerState::default();
            typed(&mut state, "https://example.com");
            let buf = render(&mut state, UrlPicker::default());

            assert!(content(&buf).contains("https://example.com"));
        }
    }
}
