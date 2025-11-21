use crossterm::event::KeyEvent;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};

use crate::tui::{
    component::Component,
    pickers::search_picker::SearchPicker,
    popups::exporters::{
        arrow_exporter::ArrowExporter, csv_exporter::CsvExporterState,
        json_exporter::JsonExporterState, jsonl_exporter::JsonLExporterState,
        parquet_exporter::ParquetExporterState, tsv_exporter::TsvExporter,
    },
};

#[derive(Debug)]
pub enum ExportWizard {
    SelectFormat(SearchPicker),
    Csv(CsvExporterState),
    Tsv(TsvExporter),
    Json(JsonExporterState),
    JsonL(JsonLExporterState),
    Parquet(ParquetExporterState),
    Arrow(ArrowExporter),
}

impl ExportWizard {
    pub fn step(&mut self) {
        *self = match std::mem::take(self) {
            ExportWizard::SelectFormat(picker) => match picker.selected().and_then(Format::new) {
                Some(Format::Csv) => ExportWizard::Csv(Default::default()),
                Some(Format::Tsv) => ExportWizard::Tsv(Default::default()),
                Some(Format::Json) => ExportWizard::Json(Default::default()),
                Some(Format::JsonL) => ExportWizard::JsonL(Default::default()),
                Some(Format::Parquet) => ExportWizard::Parquet(Default::default()),
                Some(Format::Arrow) => ExportWizard::Arrow(Default::default()),
                None => ExportWizard::SelectFormat(picker),
            },
            ExportWizard::Csv(mut state) => {
                state.step();
                ExportWizard::Csv(state)
            }
            ExportWizard::Tsv(mut state) => {
                state.step();
                ExportWizard::Tsv(state)
            }
            ExportWizard::Json(mut state) => {
                state.step();
                ExportWizard::Json(state)
            }
            ExportWizard::JsonL(mut state) => {
                state.step();
                ExportWizard::JsonL(state)
            }
            ExportWizard::Parquet(mut state) => {
                state.step();
                ExportWizard::Parquet(state)
            }
            ExportWizard::Arrow(mut state) => {
                state.step();
                ExportWizard::Arrow(state)
            }
        };
    }

    pub fn responder(&mut self) -> &mut dyn Component {
        match self {
            ExportWizard::SelectFormat(pickers) => pickers,
            ExportWizard::Csv(state) => state,
            ExportWizard::Tsv(state) => state,
            ExportWizard::Json(state) => state,
            ExportWizard::JsonL(state) => state,
            ExportWizard::Parquet(state) => state,
            ExportWizard::Arrow(state) => state,
        }
    }
}

impl Component for ExportWizard {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
    ) {
        self.responder().render(area, buf, focus_state);
    }

    fn handle(&mut self, event: KeyEvent) -> bool {
        self.responder().handle(event)
    }
}

impl Default for ExportWizard {
    fn default() -> Self {
        Self::SelectFormat(SearchPicker::new(
            Format::iter()
                .map(Into::<&str>::into)
                .map(str::to_string)
                .collect(),
        ))
    }
}

// #[derive(Debug, Default)]
// pub struct ExportWizard {}

// impl StatefulWidget for ExportWizard {
//     type State = ExportWizardState;

//     fn render(
//         self,
//         area: ratatui::prelude::Rect,
//         buf: &mut ratatui::prelude::Buffer,
//         state: &mut Self::State,
//     ) {
//         match state {
//             ExportWizardState::SelectFormat(state) => {
//                 state.render(area, buf, focus_state);
//                 // SearchPicker::default()
//                 //     .title("Format")
//                 //     .items(Format::iter().map(|fmt| Cow::Borrowed(fmt.into())))
//                 //     .render(area, buf, state);
//             }
//             ExportWizardState::Csv(state) => {
//                 CsvExporter::default().render(area, buf, state);
//             }
//             ExportWizardState::Tsv(state) => {
//                 TsvExporter::default().render(area, buf, state);
//             }
//             ExportWizardState::Json(state) => {
//                 JsonExporter::default().render(area, buf, state);
//             }
//             ExportWizardState::JsonL(state) => {
//                 JsonLExporter::default().render(area, buf, state);
//             }
//             ExportWizardState::Parquet(state) => {
//                 ParquetExporter::default().render(area, buf, state);
//             }
//             ExportWizardState::Arrow(state) => {
//                 ArrowExporter::default().render(area, buf, state);
//             }
//         }
//     }
// }

// // #[derive(Debug)]
// // pub enum State {
// //     SelectFormat(SearchPickerState),
// //     Csv(CsvExporterState),
// //     Tsv(TsvExporterState),
// //     Json(JsonExporterState),
// //     JsonL(JsonLExporterState),
// //     Parquet(ParquetExporterState),
// //     Arrow(ArrowExporterState),
// // }

// impl Default for ExportWizardState {
//     fn default() -> Self {
//         ExportWizardState::SelectFormat(SearchPicker::new(
//             Format::iter()
//                 .map(Into::<&'static str>::into)
//                 .map(|s| s.to_owned())
//                 .collect(),
//         ))
//     }
// }

#[derive(Debug, IntoStaticStr, EnumIter, PartialEq)]
enum Format {
    Csv,
    Tsv,
    Parquet,
    Json,
    JsonL,
    Arrow,
}

impl Format {
    fn new(idx: usize) -> Option<Self> {
        match idx {
            0 => Some(Self::Csv),
            1 => Some(Self::Tsv),
            2 => Some(Self::Parquet),
            3 => Some(Self::Json),
            4 => Some(Self::JsonL),
            5 => Some(Self::Arrow),
            _ => None,
        }
    }
}
