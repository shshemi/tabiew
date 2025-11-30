use std::path::PathBuf;

use crate::{
    handler::action::Action,
    tui::{
        component::Component,
        popups::{
            exporters::exporter::{Exporter, State},
            path_picker::PathPicker,
        },
    },
    writer::Destination,
};

pub type ArrowExporter = Exporter<InnerState>;

#[derive(Debug)]
pub enum InnerState {
    PickOutputPath { picker: PathPicker },
    ExportToFile { path: PathBuf },
}

impl State for InnerState {
    fn next(self) -> Self {
        if let InnerState::PickOutputPath { picker } = self {
            InnerState::ExportToFile {
                path: picker.path(),
            }
        } else {
            self
        }
    }

    fn responder(&mut self) -> Option<&mut dyn Component> {
        match self {
            InnerState::PickOutputPath { picker } => Some(picker),
            _ => None,
        }
    }

    fn export_action(&self) -> Option<Action> {
        if let InnerState::ExportToFile { path } = self {
            Some(Action::ExportArrow(Destination::File(path.to_owned())))
        } else {
            None
        }
    }
}

// impl Component for ArrowExporter {
//     fn render(
//         &mut self,
//         area: ratatui::prelude::Rect,
//         buf: &mut ratatui::prelude::Buffer,
//         focus_state: crate::tui::component::FocusState,
//     ) {
//         match self {
//             ArrowExporter::PickOutputPath { picker } => {
//                 picker.render(area, buf, focus_state);
//             }
//             ArrowExporter::ExportToFile { path: _ } => (),
//         }
//     }

//     fn handle(&mut self, event: KeyEvent) -> bool {
//         if let ArrowExporter::PickOutputPath { picker } = self {
//             picker.handle(event)
//                 || match (event.code, event.modifiers) {
//                     (KeyCode::Enter, KeyModifiers::NONE) => {
//                         self.step();
//                         true
//                     }
//                     _ => false,
//                 }
//         } else {
//             false
//         }
//     }
// }

impl Default for InnerState {
    fn default() -> Self {
        Self::PickOutputPath {
            picker: Default::default(),
        }
    }
}
