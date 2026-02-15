use polars::frame::DataFrame;

use crate::{
    handler::message::Message,
    misc::type_ext::UnwrapOrEnqueueError,
    tui::{
        component::Component,
        popups::{path_picker::PathPicker, step_by_step::ComponentSequence},
    },
    writer::{Destination, WriteToArrow, WriteToFile},
};

#[derive(Debug)]
pub enum State {
    PickOutputPath { df: DataFrame, picker: PathPicker },
}

impl From<DataFrame> for State {
    fn from(value: DataFrame) -> Self {
        Self::PickOutputPath {
            df: value,
            picker: Default::default(),
        }
    }
}

impl ComponentSequence for State {
    fn next(self) -> Self {
        match self {
            State::PickOutputPath { mut df, picker } => {
                WriteToArrow
                    .write_to_file(Destination::File(picker.path()), &mut df)
                    .unwrap_or_enqueue_error();
                Message::PaneDismissModal.enqueue();
                Message::AppShowToast(format!(
                    "Data frame exported to '{}' in Arrow format",
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
