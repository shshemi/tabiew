use polars::frame::DataFrame;

use crate::{
    handler::message::Message,
    io::writer::{Destination, WriteToAvro, WriteToFile},
    misc::type_ext::UnwrapOrEnqueueError,
    tui::{
        component::Component,
        popups::{file_picker::FilePicker, multi_step_overlay::OverlayStep},
    },
};

#[derive(Debug)]
pub enum State {
    PickOutputPath { df: DataFrame, picker: FilePicker },
}

impl From<DataFrame> for State {
    fn from(value: DataFrame) -> Self {
        Self::PickOutputPath {
            df: value,
            picker: Default::default(),
        }
    }
}

impl OverlayStep for State {
    fn next(self) -> Self {
        match self {
            State::PickOutputPath { mut df, picker } => {
                WriteToAvro
                    .write_to_file(Destination::File(picker.path()), &mut df)
                    .unwrap_or_enqueue_error();
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
