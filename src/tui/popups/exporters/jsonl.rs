use crate::writers::ExportData;
use crate::writers::{Destination, WriteToFile};

use crate::{
    handler::message::{Message, UnwrapOrEnqueueError},
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
    writers::{JsonFormat, WriteToJson},
};

pub type JsonlExporter = Wizard<State>;

#[derive(Debug)]
pub enum State {
    PickOutputTarget {
        df: ExportData,
        picker: ExportTargetPicker,
    },
    PickOutputPath {
        df: ExportData,
        picker: FilePicker,
    },
}

impl From<ExportData> for State {
    fn from(value: ExportData) -> Self {
        State::PickOutputTarget {
            df: value,
            picker: Default::default(),
        }
    }
}

impl WizardStep for State {
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
                        .export(Destination::Clipboard, &mut df)
                        .unwrap_or_enqueue_error()
                        .then(|| {
                            Message::PaneDismissModal.enqueue();
                            Message::AppShowToast(
                                "Data frame exported to clipboard in JSON Line format".to_owned(),
                            )
                            .enqueue();
                        });
                    State::PickOutputTarget { df, picker }
                }
                None => State::PickOutputTarget { df, picker },
            },
            State::PickOutputPath { mut df, picker } => {
                WriteToJson::default()
                    .with_format(JsonFormat::JsonLine)
                    .export(Destination::File(picker.path()), &mut df)
                    .unwrap_or_enqueue_error()
                    .then(|| {
                        Message::PaneDismissModal.enqueue();
                        Message::AppShowToast(format!(
                            "Data frame exported to '{}' in JSON Line format",
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
            State::PickOutputTarget { df: _, picker } => picker,
            State::PickOutputPath { df: _, picker } => picker,
        }
    }
}
