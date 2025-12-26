use crate::{
    reader::{ArrowIpcToDataFrame, Source},
    tui::popups::{importers::final_step, path_picker::PathPicker, wizard::WizardState},
};

#[derive(Debug)]
pub enum State {
    PickImportPath { picker: PathPicker },
}

impl WizardState for State {
    fn next(self) -> Self {
        match self {
            State::PickImportPath { picker } => {
                final_step(Source::File(picker.path()), ArrowIpcToDataFrame);
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
