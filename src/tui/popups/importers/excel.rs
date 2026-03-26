use crate::{
    reader::{ExcelToDataFarmes, Source},
    tui::popups::{
        importers::dismiss_overlay_and_load_data_frame, multi_step_overlay::OverlayStep,
        path_picker::PathPicker,
    },
};

#[derive(Debug)]
pub enum State {
    PickPath { picker: PathPicker },
}

impl OverlayStep for State {
    fn next(self) -> Self {
        match self {
            State::PickPath { picker } => {
                dismiss_overlay_and_load_data_frame(Source::File(picker.path()), ExcelToDataFarmes);
                State::PickPath { picker }
            }
        }
    }

    fn responder(&mut self) -> &mut dyn crate::tui::component::Component {
        match self {
            State::PickPath { picker } => picker,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State::PickPath {
            picker: Default::default(),
        }
    }
}
