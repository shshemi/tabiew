use std::borrow::Cow;

use crossterm::event::KeyEvent;
use ratatui::widgets::StatefulWidget;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};

use crate::tui::{
    pickers::search_picker::{SearchPicker, SearchPicker},
    popups::exporters::{
        arrow_exporter::{ArrowExporter, ArrowExporterState},
        csv_exporter::{CsvExporter, CsvExporterState},
        json_exporter::{JsonExporter, JsonExporterState},
        jsonl_exporter::{JsonLExporter, JsonLExporterState},
        parquet_exporter::{ParquetExporter, ParquetExporterState},
        tsv_exporter::{TsvExporter, TsvExporterState},
    },
};

#[derive(Debug)]
pub enum ExportWizardState {
    SelectFormat(SearchPicker),
    Csv(CsvExporterState),
    Tsv(TsvExporterState),
    Json(JsonExporterState),
    JsonL(JsonLExporterState),
    Parquet(ParquetExporterState),
    Arrow(ArrowExporterState),
}

impl ExportWizardState {
    pub fn step(&mut self) {
        *self = match std::mem::take(self) {
            ExportWizardState::SelectFormat(picker) => {
                match picker.selected().and_then(Format::new) {
                    Some(Format::Csv) => ExportWizardState::Csv(Default::default()),
                    Some(Format::Tsv) => ExportWizardState::Tsv(Default::default()),
                    Some(Format::Json) => ExportWizardState::Json(Default::default()),
                    Some(Format::JsonL) => ExportWizardState::JsonL(Default::default()),
                    Some(Format::Parquet) => ExportWizardState::Parquet(Default::default()),
                    Some(Format::Arrow) => ExportWizardState::Arrow(Default::default()),
                    None => ExportWizardState::SelectFormat(picker),
                }
            }
            ExportWizardState::Csv(mut state) => {
                state.step();
                ExportWizardState::Csv(state)
            }
            ExportWizardState::Tsv(mut state) => {
                state.step();
                ExportWizardState::Tsv(state)
            }
            ExportWizardState::Json(mut state) => {
                state.step();
                ExportWizardState::Json(state)
            }
            ExportWizardState::JsonL(mut state) => {
                state.step();
                ExportWizardState::JsonL(state)
            }
            ExportWizardState::Parquet(mut state) => {
                state.step();
                ExportWizardState::Parquet(state)
            }
            ExportWizardState::Arrow(mut state) => {
                state.step();
                ExportWizardState::Arrow(state)
            }
        };
    }

    pub fn select_previous(&mut self) {
        match self {
            ExportWizardState::SelectFormat(picker) => picker.list_mut().select_previous(),
            ExportWizardState::Csv(state) => state.select_previous(),
            ExportWizardState::Tsv(state) => state.select_previous(),
            ExportWizardState::Json(state) => state.select_previous(),
            ExportWizardState::JsonL(state) => state.select_previous(),
            _ => (),
        };
    }

    pub fn select_next(&mut self) {
        match self {
            ExportWizardState::SelectFormat(picker) => picker.list_mut().select_next(),
            ExportWizardState::Csv(state) => state.select_next(),
            ExportWizardState::Tsv(state) => state.select_next(),
            ExportWizardState::Json(state) => state.select_next(),
            ExportWizardState::JsonL(state) => state.select_next(),
            _ => (),
        };
    }

    pub fn handle(&mut self, event: KeyEvent) {
        match self {
            ExportWizardState::SelectFormat(pickers) => pickers.input_mut().handle(event),
            ExportWizardState::Csv(state) => state.handle(event),
            ExportWizardState::Tsv(state) => state.handle(event),
            ExportWizardState::Json(state) => state.handle(event),
            ExportWizardState::JsonL(state) => state.handle(event),
            ExportWizardState::Parquet(state) => state.handle(event),
            ExportWizardState::Arrow(state) => state.handle(event),
        }
    }
}

#[derive(Debug, Default)]
pub struct ExportWizard {}

impl StatefulWidget for ExportWizard {
    type State = ExportWizardState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        match state {
            ExportWizardState::SelectFormat(state) => {
                SearchPicker::default()
                    .title("Format")
                    .items(Format::iter().map(|fmt| Cow::Borrowed(fmt.into())))
                    .render(area, buf, state);
            }
            ExportWizardState::Csv(state) => {
                CsvExporter::default().render(area, buf, state);
            }
            ExportWizardState::Tsv(state) => {
                TsvExporter::default().render(area, buf, state);
            }
            ExportWizardState::Json(state) => {
                JsonExporter::default().render(area, buf, state);
            }
            ExportWizardState::JsonL(state) => {
                JsonLExporter::default().render(area, buf, state);
            }
            ExportWizardState::Parquet(state) => {
                ParquetExporter::default().render(area, buf, state);
            }
            ExportWizardState::Arrow(state) => {
                ArrowExporter::default().render(area, buf, state);
            }
        }
    }
}

// #[derive(Debug)]
// pub enum State {
//     SelectFormat(SearchPickerState),
//     Csv(CsvExporterState),
//     Tsv(TsvExporterState),
//     Json(JsonExporterState),
//     JsonL(JsonLExporterState),
//     Parquet(ParquetExporterState),
//     Arrow(ArrowExporterState),
// }

impl Default for ExportWizardState {
    fn default() -> Self {
        ExportWizardState::SelectFormat(SearchPicker::default())
    }
}

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
