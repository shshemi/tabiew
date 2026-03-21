use crate::{
    reader::{ParquetToDataFrame, Source},
    tui::popups::{
        component_sequence::ComponentSequence, importers::dismiss_overlay_and_load_data_frame,
        path_picker::PathPicker,
    },
};

#[derive(Debug)]
pub enum State {
    PickPath { picker: PathPicker },
}

impl ComponentSequence for State {
    fn next(self) -> Self {
        match self {
            State::PickPath { picker } => {
                dismiss_overlay_and_load_data_frame(
                    Source::File(picker.path()),
                    ParquetToDataFrame,
                );
                Default::default()
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
        Self::PickPath {
            picker: Default::default(),
        }
    }
}
