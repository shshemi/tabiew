use crate::{
    reader::{ArrowIpcToDataFrame, Source},
    tui::popups::{
        importers::dismiss_overlay_and_load_data_frame, multi_step_overlay::OverlayStep,
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
