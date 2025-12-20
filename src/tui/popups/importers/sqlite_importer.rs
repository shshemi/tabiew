use crate::{
    reader::{Source, SqliteToDataFrames},
    tui::popups::{importers::final_step, path_picker::PathPicker, wizard::WizardState},
};

#[derive(Debug)]
pub enum State {
    PickPath { picker: PathPicker },
}

impl WizardState for State {
    fn next(self) -> Self {
        match self {
            State::PickPath { picker } => {
                final_step(Source::File(picker.path()), SqliteToDataFrames::default());
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
        State::PickPath {
            picker: Default::default(),
        }
    }
}
