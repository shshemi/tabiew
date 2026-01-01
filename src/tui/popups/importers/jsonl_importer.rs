use crate::{
    reader::{JsonLineToDataFrame, Source},
    tui::popups::{
        import_source_picker::{self, ImportSourcePicker},
        importers::final_step,
        path_picker::PathPicker,
        wizard::WizardState,
    },
};

#[derive(Debug)]
pub enum State {
    PickSource { picker: ImportSourcePicker },
    PickPath { picker: PathPicker },
}

impl WizardState for State {
    fn next(self) -> Self {
        match self {
            State::PickSource { picker } => match picker.value() {
                Some(import_source_picker::Source::Stdin) => {
                    final_step(Source::Stdin, JsonLineToDataFrame::default());
                    State::PickSource { picker }
                }
                Some(import_source_picker::Source::File) => State::PickPath {
                    picker: Default::default(),
                },
                None => State::PickSource { picker },
            },
            State::PickPath { picker } => {
                final_step(Source::File(picker.path()), JsonLineToDataFrame::default());
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
