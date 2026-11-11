use crate::writers::ExportData;

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
    writers::{Destination, WriteToCsv, WriteToFile},
};

pub type TsvExporter = Wizard<State>;

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
        Self::PickOutputTarget {
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
                    picker: FilePicker::default(),
                },
                Some(Target::Clipboard) => {
                    WriteToCsv::default()
                        .with_separator_char('\t')
                        .with_quote_char('"')
                        .with_header(false)
                        .export(Destination::Clipboard, &mut df)
                        .unwrap_or_enqueue_error()
                        .then(|| {
                            Message::PaneDismissModal.enqueue();
                            Message::AppShowToast(
                                "Data frame exported to clipboard in TSV format".to_owned(),
                            )
                            .enqueue();
                        });
                    State::PickOutputTarget { df, picker }
                }
                None => State::PickOutputTarget { df, picker },
            },
            State::PickOutputPath { mut df, picker } => {
                WriteToCsv::default()
                    .with_separator_char('\t')
                    .with_quote_char('"')
                    .with_header(false)
                    .export(Destination::File(picker.path()), &mut df)
                    .unwrap_or_enqueue_error()
                    .then(|| {
                        Message::PaneDismissModal.enqueue();
                        Message::AppShowToast(format!(
                            "Data frame exported to '{}' in TSV format",
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
