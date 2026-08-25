use std::{borrow::Cow, fmt::Display};

use ratatui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};
use strum_macros::IntoStaticStr;

use crate::{
    io::reader::ReaderSource,
    sw::pickers::list_picker::{ListPicker, ListPickerState},
};

const SOURCES: [ImportSource; 3] = [ImportSource::File, ImportSource::Stdin, ImportSource::Url];

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoStaticStr)]
pub enum ImportSource {
    File,
    Stdin,
    Url,
}

impl Display for ImportSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(self))
    }
}

impl From<&ReaderSource> for ImportSource {
    fn from(r: &ReaderSource) -> Self {
        match r {
            ReaderSource::Stdin => ImportSource::Stdin,
            ReaderSource::File(_) => ImportSource::File,
        }
    }
}

#[derive(Debug, Default)]
pub struct ImportSourcePickerState {
    list: ListPickerState,
}

impl ImportSourcePickerState {
    pub fn list(&self) -> &ListPickerState {
        &self.list
    }

    pub fn list_mut(&mut self) -> &mut ListPickerState {
        &mut self.list
    }

    pub fn selected(&self) -> Option<ImportSource> {
        self.list
            .selected()
            .and_then(|idx| SOURCES.get(idx))
            .copied()
    }
}

#[derive(Debug)]
pub struct ImportSourcePicker<'a> {
    title: Cow<'a, str>,
}

impl<'a> ImportSourcePicker<'a> {
    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self {
        self.title = title.into();
        self
    }
}

impl Default for ImportSourcePicker<'_> {
    fn default() -> Self {
        Self {
            title: Cow::Borrowed("Import Source"),
        }
    }
}

impl StatefulWidget for ImportSourcePicker<'_> {
    type State = ImportSourcePickerState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        ListPicker::new(&SOURCES)
            .title(self.title)
            .render(area, buf, &mut state.list);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn render(state: &mut ImportSourcePickerState, picker: ImportSourcePicker) -> Buffer {
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
        fn defaults_to_file() {
            assert_eq!(
                ImportSourcePickerState::default().selected(),
                Some(ImportSource::File)
            );
        }

        #[test]
        fn each_index_maps_to_its_source() {
            let mut state = ImportSourcePickerState::default();

            for (idx, expected) in SOURCES.iter().enumerate() {
                state.list_mut().select(Some(idx));
                assert_eq!(state.selected(), Some(*expected));
            }
        }

        #[test]
        fn no_selection_has_no_source() {
            let mut state = ImportSourcePickerState::default();
            state.list_mut().select(None);

            assert_eq!(state.selected(), None);
        }

        #[test]
        fn an_out_of_range_selection_has_no_source() {
            let mut state = ImportSourcePickerState::default();
            state.list_mut().select(Some(9));

            assert_eq!(state.selected(), None);
        }

        #[test]
        fn cycling_wraps_through_all_three_sources() {
            let mut state = ImportSourcePickerState::default();
            render(&mut state, ImportSourcePicker::default());

            state.list_mut().cycle_down();
            assert_eq!(state.selected(), Some(ImportSource::Stdin));

            state.list_mut().cycle_down();
            assert_eq!(state.selected(), Some(ImportSource::Url));

            state.list_mut().cycle_down();
            assert_eq!(state.selected(), Some(ImportSource::File));
        }

        #[test]
        fn rendering_reports_three_sources() {
            let mut state = ImportSourcePickerState::default();
            render(&mut state, ImportSourcePicker::default());

            assert_eq!(state.list().len(), 3);
        }
    }

    mod conversion {
        use super::*;

        #[test]
        fn a_reader_source_maps_to_the_matching_variant() {
            assert_eq!(
                ImportSource::from(&ReaderSource::Stdin),
                ImportSource::Stdin
            );
            assert_eq!(
                ImportSource::from(&ReaderSource::File(PathBuf::from("data.csv"))),
                ImportSource::File
            );
        }
    }

    mod widget {
        use super::*;

        #[test]
        fn renders_every_source() {
            let buf = render(
                &mut ImportSourcePickerState::default(),
                ImportSourcePicker::default(),
            );

            let content = content(&buf);
            assert!(content.contains("File"));
            assert!(content.contains("Stdin"));
            assert!(content.contains("Url"));
        }

        #[test]
        fn defaults_to_an_import_source_title() {
            let buf = render(
                &mut ImportSourcePickerState::default(),
                ImportSourcePicker::default(),
            );

            assert!(content(&buf).contains("Import Source"));
        }

        #[test]
        fn the_title_is_overridable() {
            let buf = render(
                &mut ImportSourcePickerState::default(),
                ImportSourcePicker::default().title("Read From"),
            );

            let content = content(&buf);
            assert!(content.contains("Read From"));
            assert!(!content.contains("Import Source"));
        }
    }
}
