use crate::{
    reader::{CsvToDataFrame, Source},
    tui::popups::{
        component_sequence::OverlayStep,
        import_source_picker::{self, ImportSourcePicker},
        importers::dismiss_overlay_and_load_data_frame,
        path_picker::PathPicker,
    },
};

#[derive(Debug)]
pub enum State {
    PickSource { picker: ImportSourcePicker },
    PickPath { picker: PathPicker },
}

impl OverlayStep for State {
    fn next(self) -> Self {
        match self {
            State::PickSource { picker } => match picker.value() {
                Some(import_source_picker::Source::Stdin) => {
                    dismiss_overlay_and_load_data_frame(
                        Source::Stdin,
                        CsvToDataFrame::default()
                            .with_no_header(true)
                            .with_quote_char('"')
                            .with_separator('\t'),
                    );
                    State::PickSource { picker }
                }
                Some(import_source_picker::Source::File) => State::PickPath {
                    picker: Default::default(),
                },
                None => State::PickSource { picker },
            },
            State::PickPath { picker } => {
                dismiss_overlay_and_load_data_frame(
                    Source::File(picker.path()),
                    CsvToDataFrame::default()
                        .with_no_header(true)
                        .with_quote_char('"')
                        .with_separator('\t'),
                );
                Default::default()
            }
        }
    }

    fn responder(&mut self) -> &mut dyn crate::tui::component::Component {
        match self {
            State::PickSource { picker } => picker,
            State::PickPath { picker } => picker,
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
