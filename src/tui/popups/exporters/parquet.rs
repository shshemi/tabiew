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
    writers::{Destination, WriteToFile, WriteToParquet},
};

pub type ParquetExporter = Wizard<State>;

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
                WriteToParquet
                    .export(Destination::File(picker.path()), &mut df)
                    .unwrap_or_enqueue_error()
                    .then(|| {
                        Message::PaneDismissModal.enqueue();
                        Message::AppShowToast(format!(
                            "Data frame exported to '{}' in Parquet format",
                            picker
                                .path()
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                        ))
                        .enqueue();
                    });
                State::PickOutputPath { df, picker }
            }
        }
    }

    fn responder(&mut self) -> &mut dyn Component {
        match self {
            State::PickOutputPath { df: _, picker } => picker,
        }
    }
}
