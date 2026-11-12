use crate::{
    tui::{
        component::Component,
        popups::{
            pickers::file_picker::FilePicker,
            wizard::{Wizard, WizardStep},
        },
    },
    writers::{Destination, ExportFormat, export},
};

pub type ArrowExporter = Wizard<State>;

#[derive(Debug)]
pub enum State {
    PickOutputPath { view: String, picker: FilePicker },
}

impl From<String> for State {
    fn from(value: String) -> Self {
        Self::PickOutputPath {
            view: value,
            picker: Default::default(),
        }
    }
}

impl WizardStep for State {
    fn next(self) -> Self {
        match self {
            State::PickOutputPath { view, picker } => {
                export(&view, ExportFormat::Arrow, Destination::File(picker.path()));
                Self::PickOutputPath { view, picker }
            }
        }
    }

    fn responder(&mut self) -> &mut dyn Component {
        match self {
            State::PickOutputPath { view: _, picker } => picker,
        }
    }
}
