use crate::{
    reader::{ArrowIpcToDataFrame, Source},
    tui::popups::{
        component_sequence::OverlayStep, importers::dismiss_overlay_and_load_data_frame,
        path_picker::PathPicker,
    },
};

#[derive(Debug)]
pub enum State {
    PickImportPath { picker: PathPicker },
}

impl OverlayStep for State {
    fn next(self) -> Self {
        match self {
            State::PickImportPath { picker } => {
                dismiss_overlay_and_load_data_frame(
                    Source::File(picker.path()),
                    ArrowIpcToDataFrame,
                );
                Default::default()
            }
        }
    }

    fn responder(&mut self) -> &mut dyn crate::tui::component::Component {
        match self {
            State::PickImportPath { picker } => picker,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State::PickImportPath {
            picker: Default::default(),
        }
    }
}
