use crate::{
    io::{DataSource, reader::ParquetToDataFrame},
    tui::popups::{
        importers::{
            dismiss_overlay_and_load_data_frame,
            import_source_picker::{ImportSource, ImportSourcePicker},
        },
        multi_step_overlay::OverlayStep,
        file_picker::FilePicker,
        url_picker::UrlPicker,
    },
};

#[derive(Debug)]
pub enum State {
    PickSource { picker: ImportSourcePicker },
    PickPath { picker: FilePicker },
    PickUrl { picker: UrlPicker },
}

impl OverlayStep for State {
    fn next(self) -> Self {
        match self {
            State::PickSource { picker } => match picker.value() {
                Some(ImportSource::Stdin) => {
                    dismiss_overlay_and_load_data_frame(DataSource::Stdin, ParquetToDataFrame);
                    State::PickSource { picker }
                }
                Some(ImportSource::File) => State::PickPath {
                    picker: Default::default(),
                },
                Some(ImportSource::Url) => State::PickUrl {
                    picker: UrlPicker::default(),
                },
                None => State::PickSource { picker },
            },
            State::PickPath { picker } => {
                dismiss_overlay_and_load_data_frame(
                    DataSource::File(picker.path()),
                    ParquetToDataFrame,
                );
                Default::default()
            }
            State::PickUrl { picker } => {
                dismiss_overlay_and_load_data_frame(
                    DataSource::Url(picker.url()),
                    ParquetToDataFrame,
                );
                Default::default()
            }
        }
    }

    fn responder(&mut self) -> &mut dyn crate::tui::component::Component {
        match self {
            State::PickSource { picker } => picker,
            State::PickPath { picker } => picker,
            State::PickUrl { picker } => picker,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::PickSource {
            picker: Default::default(),
        }
    }
}
