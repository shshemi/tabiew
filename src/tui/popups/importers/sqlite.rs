use crate::{
    io::{DataSource, reader::SqliteToDataFrames},
    tui::{
        pickers::text_picker::TextPicker,
        popups::{
            importers::{
                dismiss_overlay_and_load_data_frame,
                import_source_picker::{ImportSource, ImportSourcePicker},
            },
            multi_step_overlay::OverlayStep,
            file_picker::FilePicker,
            url_picker::UrlPicker,
        },
    },
};

#[derive(Debug)]
pub enum State {
    PickSource {
        picker: ImportSourcePicker,
    },
    PickPath {
        picker: FilePicker,
    },
    PickUrl {
        picker: UrlPicker,
    },
    PickPassword {
        source: DataSource,
        picker: TextPicker,
    },
}

impl OverlayStep for State {
    fn next(self) -> Self {
        match self {
            State::PickSource { picker } => match picker.value() {
                Some(ImportSource::Stdin) => State::PickPassword {
                    source: DataSource::Stdin,
                    picker: TextPicker::default()
                        .with_title("Password")
                        .with_hint("Leave empty for no password"),
                },
                Some(ImportSource::File) => State::PickPath {
                    picker: FilePicker::default(),
                },
                Some(ImportSource::Url) => State::PickUrl {
                    picker: UrlPicker::default(),
                },
                None => State::PickSource { picker },
            },
            State::PickPath { picker } => State::PickPassword {
                source: DataSource::File(picker.path()),
                picker: TextPicker::default()
                    .with_title("Password")
                    .with_hint("Leave empty for no password"),
            },
            State::PickUrl { picker } => State::PickPassword {
                source: DataSource::Url(picker.url()),
                picker: TextPicker::default()
                    .with_title("Password")
                    .with_hint("Leave empty for no password"),
            },
            State::PickPassword { source, picker } => {
                dismiss_overlay_and_load_data_frame(
                    source,
                    if picker.value().is_empty() {
                        SqliteToDataFrames::default()
                    } else {
                        SqliteToDataFrames::default().key(picker.value().to_owned())
                    },
                );
                Default::default()
            }
        }
    }

    fn responder(&mut self) -> &mut dyn crate::tui::component::Component {
        match self {
            State::PickSource { picker } => picker,
            State::PickPath { picker } => picker,
            State::PickPassword { source: _, picker } => picker,
            State::PickUrl { picker } => picker,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State::PickSource {
            picker: Default::default(),
        }
    }
}
