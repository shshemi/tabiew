use std::fmt::Display;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};

use crate::{
    handler::action::Action,
    tui::{
        component::Component,
        pickers::search_picker::SearchPicker,
        popups::exporters::{
            arrow_exporter::ArrowExporter, csv_exporter::CsvExporter, json_exporter::JsonExporter,
            jsonl_exporter::JsonLExporter, parquet_exporter::ParquetExporter,
            tsv_exporter::TsvExporter,
        },
    },
};

#[derive(Debug)]
pub enum ExportWizard {
    SelectFormat(SearchPicker<Format>),
    Csv(CsvExporter),
    Tsv(TsvExporter),
    Json(JsonExporter),
    JsonL(JsonLExporter),
    Parquet(ParquetExporter),
    Arrow(ArrowExporter),
}

impl ExportWizard {
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
        if let ExportWizard::SelectFormat(search_picker) = self {
            search_picker.handle(event)
                || match (event.code, event.modifiers) {
                    (KeyCode::Esc, KeyModifiers::NONE) => {
                        Action::PaneDismissModal.enqueue();
                        false
                    }
                    (KeyCode::Enter, KeyModifiers::NONE) => {
                        if let Some(fmt) = search_picker.selected_item() {
                            *self = match fmt {
                                Format::Csv => ExportWizard::Csv(Default::default()),
                                Format::Tsv => ExportWizard::Tsv(Default::default()),
                                Format::Json => ExportWizard::Json(Default::default()),
                                Format::JsonL => ExportWizard::JsonL(Default::default()),
                                Format::Parquet => ExportWizard::Parquet(Default::default()),
                                Format::Arrow => ExportWizard::Arrow(Default::default()),
                            };
                        };
                        true
                    }
                    _ => false,
                }
        } else {
            self.responder().handle(event)
        }
    }
}

impl Default for ExportWizard {
    fn default() -> Self {
        Self::SelectFormat(SearchPicker::new(Format::iter().collect()))
    }
}

#[derive(Debug, IntoStaticStr, EnumIter, PartialEq)]
pub enum Format {
    Csv,
    Tsv,
    Parquet,
    Json,
    JsonL,
    Arrow,
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(self))
    }
}
