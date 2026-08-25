use std::path::PathBuf;

use ratatui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};
use url::Url;

use crate::sw::{
    pickers::text_picker::{TextPicker, TextPickerState},
    popups::{
        file_picker::{FilePicker, FilePickerState},
        import_source_picker::{ImportSource, ImportSourcePicker, ImportSourcePickerState},
        url_picker::{UrlPicker, UrlPickerState},
        yes_no_picker::{YesNoPicker, YesNoPickerState},
    },
    wizard::PickerValue,
};

#[derive(Debug)]
pub struct CsvImporterState {
    source: PickerValue<ImportSourcePickerState, ImportSource>,
    path: PickerValue<FilePickerState, PathBuf>,
    url: PickerValue<UrlPickerState, Url>,
    has_header: PickerValue<YesNoPickerState, bool>,
    separator: PickerValue<TextPickerState, char>,
    quote: PickerValue<TextPickerState, char>,
}

impl CsvImporterState {
    pub fn source(&self) -> &PickerValue<ImportSourcePickerState, ImportSource> {
        &self.source
    }

    pub fn source_mut(&mut self) -> &mut PickerValue<ImportSourcePickerState, ImportSource> {
        &mut self.source
    }

    pub fn path(&self) -> &PickerValue<FilePickerState, PathBuf> {
        &self.path
    }

    pub fn path_mut(&mut self) -> &mut PickerValue<FilePickerState, PathBuf> {
        &mut self.path
    }

    pub fn url(&self) -> &PickerValue<UrlPickerState, Url> {
        &self.url
    }

    pub fn url_mut(&mut self) -> &mut PickerValue<UrlPickerState, Url> {
        &mut self.url
    }

    pub fn has_header(&self) -> &PickerValue<YesNoPickerState, bool> {
        &self.has_header
    }

    pub fn has_header_mut(&mut self) -> &mut PickerValue<YesNoPickerState, bool> {
        &mut self.has_header
    }

    pub fn separator(&self) -> &PickerValue<TextPickerState, char> {
        &self.separator
    }

    pub fn separator_mut(&mut self) -> &mut PickerValue<TextPickerState, char> {
        &mut self.separator
    }

    pub fn quote(&self) -> &PickerValue<TextPickerState, char> {
        &self.quote
    }

    pub fn quote_mut(&mut self) -> &mut PickerValue<TextPickerState, char> {
        &mut self.quote
    }
}

impl Default for CsvImporterState {
    fn default() -> Self {
        Self {
            source: PickerValue::new(Default::default()),
            path: PickerValue::new(Default::default()),
            url: PickerValue::new(Default::default()),
            has_header: PickerValue::new(Default::default()),
            separator: PickerValue::new(
                TextPickerState::default()
                    .with_max_len(1)
                    .with_value(",".to_owned()),
            ),
            quote: PickerValue::new(
                TextPickerState::default()
                    .with_max_len(1)
                    .with_value("\"".to_owned()),
            ),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CsvImporter;

impl StatefulWidget for CsvImporter {
    type State = CsvImporterState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if let PickerValue::Picker(picker) = &mut state.source {
            ImportSourcePicker::default()
                .title("Source")
                .render(area, buf, picker);
        } else if let Some(ImportSource::File) = state.source.value()
            && let PickerValue::Picker(picker) = &mut state.path
        {
            FilePicker::default().render(area, buf, picker);
        } else if let Some(ImportSource::Url) = state.source.value()
            && let PickerValue::Picker(picker) = &mut state.url
        {
            UrlPicker::default().render(area, buf, picker);
        } else if let PickerValue::Picker(picker) = &mut state.has_header {
            YesNoPicker::default()
                .title("Has Header")
                .render(area, buf, picker);
        } else if let PickerValue::Picker(picker) = &mut state.separator {
            TextPicker::default()
                .title("Separator")
                .render(area, buf, picker);
        } else if let PickerValue::Picker(picker) = &mut state.quote {
            TextPicker::default()
                .title("Quote")
                .render(area, buf, picker);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(state: &mut CsvImporterState) -> Buffer {
        let area = Rect::new(0, 0, 100, 30);
        let mut buf = Buffer::empty(area);
        CsvImporter.render(area, &mut buf, state);
        buf
    }

    fn content(buf: &Buffer) -> String {
        buf.content().iter().map(|c| c.symbol()).collect()
    }

    fn shown(state: &mut CsvImporterState) -> String {
        content(&render(state))
    }

    fn picked_source(source: ImportSource) -> CsvImporterState {
        CsvImporterState {
            source: PickerValue::Value(source),
            ..Default::default()
        }
    }

    mod state {
        use super::*;

        #[test]
        fn nothing_is_picked_to_begin_with() {
            let state = CsvImporterState::default();

            assert!(!state.source().picked());
            assert!(!state.has_header().picked());
            assert!(!state.separator().picked());
            assert!(!state.quote().picked());
        }

        #[test]
        fn the_separator_and_quote_start_on_their_defaults() {
            let state = CsvImporterState::default();

            let PickerValue::Picker(separator) = state.separator() else {
                panic!("separator already picked");
            };
            let PickerValue::Picker(quote) = state.quote() else {
                panic!("quote already picked");
            };

            assert_eq!(separator.value(), ",");
            assert_eq!(quote.value(), "\"");
        }

        #[test]
        fn a_picked_value_is_readable() {
            let state = CsvImporterState {
                separator: PickerValue::Value(';'),
                ..Default::default()
            };

            assert!(state.separator().picked());
            assert_eq!(state.separator().value(), Some(&';'));
        }
    }

    mod widget {
        use super::*;

        #[test]
        fn it_asks_for_the_source_first() {
            let mut state = CsvImporterState::default();

            assert!(shown(&mut state).contains("Source"));
        }

        #[test]
        fn a_file_source_is_asked_for_a_path() {
            let mut state = picked_source(ImportSource::File);

            assert!(shown(&mut state).contains("File Path"));
        }

        #[test]
        fn a_url_source_is_asked_for_a_url() {
            let mut state = picked_source(ImportSource::Url);

            assert!(shown(&mut state).contains("URL"));
        }

        #[test]
        fn a_stdin_source_skips_straight_to_the_header_question() {
            let mut state = picked_source(ImportSource::Stdin);

            assert!(shown(&mut state).contains("Has Header"));
        }

        #[test]
        fn a_picked_path_moves_on_to_the_header_question() {
            let mut state = picked_source(ImportSource::File);
            state.path = PickerValue::Value("/tmp/data.csv".into());

            assert!(shown(&mut state).contains("Has Header"));
        }

        #[test]
        fn a_picked_url_moves_on_to_the_header_question() {
            let mut state = picked_source(ImportSource::Url);
            state.url = PickerValue::Value("https://example.com/data.csv".parse().unwrap());

            assert!(shown(&mut state).contains("Has Header"));
        }

        #[test]
        fn an_unpicked_url_is_ignored_when_the_source_is_a_file() {
            let mut state = picked_source(ImportSource::File);
            state.path = PickerValue::Value("/tmp/data.csv".into());

            let shown = shown(&mut state);
            assert!(!shown.contains("URL"));
        }

        #[test]
        fn the_separator_comes_after_the_header_question() {
            let mut state = picked_source(ImportSource::Stdin);
            state.has_header = PickerValue::Value(true);

            assert!(shown(&mut state).contains("Separator"));
        }

        #[test]
        fn the_quote_comes_last() {
            let mut state = picked_source(ImportSource::Stdin);
            state.has_header = PickerValue::Value(true);
            state.separator = PickerValue::Value(',');

            assert!(shown(&mut state).contains("Quote"));
        }

        #[test]
        fn nothing_is_drawn_once_everything_is_picked() {
            let mut state = picked_source(ImportSource::Stdin);
            state.has_header = PickerValue::Value(true);
            state.separator = PickerValue::Value(',');
            state.quote = PickerValue::Value('"');

            assert_eq!(shown(&mut state).trim(), "");
        }
    }
}
