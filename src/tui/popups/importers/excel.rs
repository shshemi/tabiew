use crate::{
    reader::{ExcelToDataFarmes, Source},
    tui::popups::{importers::final_step, path_picker::PathPicker, step_by_step::ComponentSequence},
};

#[derive(Debug)]
pub enum State {
    PickPath { picker: PathPicker },
}

impl ComponentSequence for State {
    fn next(self) -> Self {
        match self {
            State::PickPath { picker } => {
                final_step(Source::File(picker.path()), ExcelToDataFarmes);
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
