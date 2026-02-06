use crate::{
    reader::{ParquetToDataFrame, Source},
    tui::popups::{importers::final_step, path_picker::PathPicker, step_by_step::StepByStepState},
};

#[derive(Debug)]
pub enum State {
    PickPath { picker: PathPicker },
}

impl StepByStepState for State {
    fn next(self) -> Self {
        match self {
            State::PickPath { picker } => {
                final_step(Source::File(picker.path()), ParquetToDataFrame);
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
