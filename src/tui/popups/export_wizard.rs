use std::fmt::Display;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use polars::frame::DataFrame;
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
pub struct ExportWizard {
    //
    state: State,
    df: DataFrame,
}

impl ExportWizard {
    pub fn new(df: DataFrame) -> Self {
        Self {
            state: Default::default(),
            df,
        }
    }
    pub fn responder(&mut self) -> &mut dyn Component {
        match &mut self.state {
            State::SelectFormat(pickers) => pickers,
            State::Csv(state) => state,
            State::Tsv(state) => state,
            State::Json(state) => state,
            State::JsonL(state) => state,
            State::Parquet(state) => state,
            State::Arrow(state) => state,
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
        if let State::SelectFormat(search_picker) = &mut self.state {
            search_picker.handle(event)
                || match (event.code, event.modifiers) {
                    (KeyCode::Esc, KeyModifiers::NONE) => {
                        Action::PaneDismissModal.enqueue();
                        false
                    }
                    (KeyCode::Enter, KeyModifiers::NONE) => {
                        if let Some(fmt) = search_picker.selected_item() {
                            self.state = match fmt {
                                Format::Csv => State::Csv(CsvExporter::new(self.df.clone())),
                                Format::Tsv => State::Tsv(TsvExporter::new(self.df.clone())),
                                Format::Json => State::Json(JsonExporter::new(self.df.clone())),
                                Format::JsonL => State::JsonL(JsonLExporter::new(self.df.clone())),
                                Format::Parquet => {
                                    State::Parquet(ParquetExporter::new(self.df.clone()))
                                }
                                Format::Arrow => State::Arrow(ArrowExporter::new(self.df.clone())),
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

#[derive(Debug)]
pub enum State {
    SelectFormat(SearchPicker<Format>),
    Csv(CsvExporter),
    Tsv(TsvExporter),
    Json(JsonExporter),
    JsonL(JsonLExporter),
    Parquet(ParquetExporter),
    Arrow(ArrowExporter),
}

impl Default for State {
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
