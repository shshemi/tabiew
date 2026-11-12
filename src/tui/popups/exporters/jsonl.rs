use crate::{
    tui::{
        component::Component,
        popups::{
            pickers::{
                export_target_picker::{ExportTargetPicker, Target},
                file_picker::FilePicker,
            },
            wizard::{Wizard, WizardStep},
        },
    },
    writers::{Destination, ExportFormat, export},
};

pub type JsonlExporter = Wizard<State>;

#[derive(Debug)]
pub enum State {
    PickOutputTarget {
        view: String,
        picker: ExportTargetPicker,
    },
    PickOutputPath {
        view: String,
        picker: FilePicker,
    },
}

impl From<String> for State {
    fn from(value: String) -> Self {
        State::PickOutputTarget {
            view: value,
            picker: Default::default(),
        }
    }
}

impl WizardStep for State {
    fn next(self) -> Self {
        match self {
            State::PickOutputTarget { view, picker } => match picker.selected() {
                Some(Target::File) => State::PickOutputPath {
                    view,
                    picker: Default::default(),
                },
                Some(Target::Clipboard) => {
                    export(&view, ExportFormat::Jsonl, Destination::Clipboard);
                    State::PickOutputTarget { view, picker }
                }
                None => State::PickOutputTarget { view, picker },
            },
            State::PickOutputPath { view, picker } => {
                export(&view, ExportFormat::Jsonl, Destination::File(picker.path()));
                State::PickOutputPath { view, picker }
            }
        }
    }

    fn responder(&mut self) -> &mut dyn Component {
        match self {
            State::PickOutputTarget { view: _, picker } => picker,
            State::PickOutputPath { view: _, picker } => picker,
        }
    }
}
