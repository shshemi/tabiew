use crate::{
    misc::type_ext::UnwrapOrEnqueueError,
    writer::{Destination, WriteToFile},
};
use polars::frame::DataFrame;

use crate::{
    handler::message::Message,
    tui::{
        component::Component,
        popups::{
            export_target_picker::{ExportTargetPicker, Target},
            path_picker::PathPicker,
            wizard::WizardState,
        },
    },
    writer::{JsonFormat, WriteToJson},
};

#[derive(Debug)]
pub enum State {
    PickOutputTarget {
        df: DataFrame,
        picker: ExportTargetPicker,
    },
    PickOutputPath {
        df: DataFrame,
        picker: PathPicker,
    },
}

impl From<DataFrame> for State {
    fn from(value: DataFrame) -> Self {
        State::PickOutputTarget {
            df: value,
            picker: Default::default(),
        }
    }
}

impl WizardState for State {
    fn next(self) -> Self {
        match self {
            State::PickOutputTarget { mut df, picker } => match picker.selected() {
                Some(Target::File) => State::PickOutputPath {
                    df,
                    picker: Default::default(),
                },
                Some(Target::Clipboard) => {
                    WriteToJson::default()
                        .with_format(JsonFormat::JsonLine)
                        .write_to_file(Destination::Clipboard, &mut df)
                        .unwrap_or_enqueue_error();
                    Message::PaneDismissModal.enqueue();
                    State::PickOutputTarget { df, picker }
                }
                None => State::PickOutputTarget { df, picker },
            },
            State::PickOutputPath { mut df, picker } => {
                WriteToJson::default()
                    .with_format(JsonFormat::JsonLine)
                    .write_to_file(Destination::File(picker.path()), &mut df)
                    .unwrap_or_enqueue_error();
                Message::PaneDismissModal.enqueue();
                State::PickOutputPath { df, picker }
            }
        }
    }

    fn responder(&mut self) -> &mut dyn Component {
        match self {
            State::PickOutputTarget { df: _, picker } => picker,
            State::PickOutputPath { df: _, picker } => picker,
        }
    }
}
