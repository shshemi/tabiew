use std::path::PathBuf;

use crate::tui::{
    component::Component,
    popups::{
        exporters::exporter::{Export, Exporter, State},
        output_target_picker::{OutputTargetPicker, Target},
        path_picker::PathPicker,
    },
};

pub type JsonLExporter = Exporter<InnerState>;

#[derive(Debug)]
pub enum InnerState {
    PickOutputTarget { picker: OutputTargetPicker },
    PickOutputPath { picker: PathPicker },
    ExportToFile { path: PathBuf },
    ExportToClipboard,
}

impl State for InnerState {
    fn next(self) -> Self {
        match self {
            InnerState::PickOutputTarget { picker } => match picker.selected() {
                Some(Target::File) => InnerState::PickOutputPath {
                    picker: Default::default(),
                },
                Some(Target::Clipboard) => InnerState::ExportToClipboard,
                None => InnerState::PickOutputTarget { picker },
            },
            InnerState::PickOutputPath { picker } => InnerState::ExportToFile {
                path: picker.path(),
            },
            InnerState::ExportToFile { path } => InnerState::ExportToFile { path },
            InnerState::ExportToClipboard => InnerState::ExportToClipboard,
        }
    }

    fn responder(&mut self) -> Option<&mut dyn Component> {
        match self {
            InnerState::PickOutputTarget { picker } => Some(picker),
            InnerState::PickOutputPath { picker } => Some(picker),
            _ => None,
        }
    }

    fn export(&self) -> Export {
        match self {
            InnerState::ExportToFile { path } => {
                //
                Export::JsonLToFile(path.to_owned())
                // WriteToJson::default()
                //     .with_format(JsonFormat::JsonLine)
                //     .write_to_file(&Destination::File(path.to_owned()), df)
                //     .into()
                // Some(Action::ExportJson(
                //     Destination::File(path.to_owned()),
                //     JsonFormat::JsonLine,
                // ))
            }
            InnerState::ExportToClipboard => {
                //
                Export::JsonLToClipboard
                // WriteToJson::default()
                //     .with_format(JsonFormat::JsonLine)
                //     .write_to_file(&Destination::Clipboard, df)
                //     .into()
                // Some(Action::ExportJson(
                //     Destination::Clipboard,
                //     JsonFormat::JsonLine,
                // ))
            }
            _ => Export::WaitingForUserInput,
        }
    }
}

impl Default for InnerState {
    fn default() -> Self {
        InnerState::PickOutputTarget {
            picker: Default::default(),
        }
    }
}

// #[derive(Debug, Default)]
// pub struct JsonLExporter {}

// impl StatefulWidget for JsonLExporter {
//     type State = JsonLExporterState;

//     fn render(
//         self,
//         area: ratatui::prelude::Rect,
//         buf: &mut ratatui::prelude::Buffer,
//         state: &mut Self::State,
//     ) {
//         match state {
//             JsonLExporterState::PickOutputTarget { picker } => {
//                 OutputTargetPicker::default().render(area, buf, picker)
//             }
//             JsonLExporterState::PickOutputPath { picker } => {
//                 PathPicker::default().render(area, buf, picker)
//             }
//             JsonLExporterState::ExportToFile { path: _ } => (),
//             JsonLExporterState::ExportToClipboard => (),
//         }
//     }
// }
