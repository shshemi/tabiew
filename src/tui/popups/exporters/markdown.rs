use crate::writers::ExportData;
use crate::writers::{Destination, WriteToFile, WriteToMarkdown};

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
};

pub type MarkdownExporter = Wizard<State>;

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
    fn next(self) -> State {
        match self {
            State::PickOutputTarget { mut df, picker } => match picker.selected() {
                Some(Target::File) => State::PickOutputPath {
                    df,
                    picker: Default::default(),
                },
                Some(Target::Clipboard) => {
                    WriteToMarkdown
                        .export(Destination::Clipboard, &mut df)
                        .unwrap_or_enqueue_error()
                        .then(|| {
                            Message::PaneDismissModal.enqueue();
                            Message::AppShowToast(
                                "Data frame exported to clipboard in Markdown format".to_owned(),
                            )
                            .enqueue();
                        });
                    State::PickOutputTarget { df, picker }
                }
                None => State::PickOutputTarget { picker, df },
            },
            State::PickOutputPath { mut df, picker } => {
                WriteToMarkdown
                    .export(Destination::File(picker.path()), &mut df)
                    .unwrap_or_enqueue_error()
                    .then(|| {
                        Message::PaneDismissModal.enqueue();
                        Message::AppShowToast(format!(
                            "Data frame exported to '{}' in Markdown format",
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
            State::PickOutputTarget { picker, df: _ } => picker,
            State::PickOutputPath { picker, df: _ } => picker,
        }
    }
}
