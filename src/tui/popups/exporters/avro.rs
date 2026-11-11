use crate::writers::ExportData;

use crate::{
    handler::message::{Message, UnwrapOrEnqueueError},
    tui::{
        component::Component,
        popups::{
            pickers::file_picker::FilePicker,
            wizard::{Wizard, WizardStep},
        },
    },
    writers::{Destination, WriteToAvro, WriteToFile},
};

pub type AvroExporter = Wizard<State>;

#[derive(Debug)]
pub enum State {
    PickOutputPath { df: ExportData, picker: FilePicker },
}

impl From<ExportData> for State {
    fn from(value: ExportData) -> Self {
        Self::PickOutputPath {
            df: value,
            picker: Default::default(),
        }
    }
}

impl WizardStep for State {
    fn next(self) -> Self {
        match self {
            State::PickOutputPath { mut df, picker } => {
                WriteToAvro
                    .export(Destination::File(picker.path()), &mut df)
                    .unwrap_or_enqueue_error()
                    .then(|| {
                        Message::PaneDismissModal.enqueue();
                        Message::AppShowToast(format!(
                            "Data frame exported to '{}' in Avro format",
                            picker
                                .path()
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                        ))
                        .enqueue();
                    });
                Self::PickOutputPath { df, picker }
            }
        }
    }

    fn responder(&mut self) -> &mut dyn Component {
        match self {
            State::PickOutputPath { df: _, picker } => picker,
        }
    }
}
