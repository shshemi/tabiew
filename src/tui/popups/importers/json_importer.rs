use crate::{
    reader::{JsonToDataFrame, Source},
    tui::popups::{
        importers::final_step,
        input_source_picker::{self, InputSourcePicker},
        path_picker::PathPicker,
        wizard::WizardState,
    },
};

#[derive(Debug)]
pub enum State {
    PickSource { picker: InputSourcePicker },
    PickPath { picker: PathPicker },
}

impl WizardState for State {
    fn next(self) -> Self {
        match self {
            State::PickSource { picker } => match picker.value() {
                Some(input_source_picker::Source::Stdin) => {
                    final_step(Source::Stdin, JsonToDataFrame::default());
                    State::PickSource { picker }
                }
                Some(input_source_picker::Source::File) => State::PickPath {
                    picker: Default::default(),
                },
                None => State::PickSource { picker },
            },
            State::PickPath { picker } => {
                final_step(Source::File(picker.path()), JsonToDataFrame::default());
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
