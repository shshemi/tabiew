use std::borrow::Cow;

use crossterm::event::KeyEvent;
use ratatui::widgets::StatefulWidget;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};

use crate::tui::{
    pickers::search_picker::{SearchPicker, SearchPickerState},
    popups::exporters::{
        arrow_exporter::{self, ArrowExporter, ArrowExporterState},
        csv_exporter::{self, CsvExporter, CsvExporterState},
        json_exporter::{self, JsonExporter, JsonExporterState},
        jsonl_exporter::{self, JsonLExporter, JsonLExporterState},
        parquet_exporter::{self, ParquetExporter, ParquetExporterState},
        tsv_exporter::{self, TsvExporter, TsvExporterState},
    },
};

#[derive(Debug, Default)]
pub struct ExportWizardState {
    state: State,
}

impl ExportWizardState {
    pub fn step(&mut self) -> &State {
        self.state = match std::mem::take(&mut self.state) {
            State::SelectFormat(picker) => match picker.selected().and_then(Format::new) {
                Some(Format::Csv) => State::Csv(Default::default()),
                Some(Format::Tsv) => State::Tsv(Default::default()),
                Some(Format::Json) => State::Json(Default::default()),
                Some(Format::JsonL) => State::JsonL(Default::default()),
                Some(Format::Parquet) => State::Parquet(Default::default()),
                Some(Format::Arrow) => State::Arrow(Default::default()),
                None => State::SelectFormat(picker),
            },
            State::Csv(mut state) => {
                state.step();
                State::Csv(state)
            }
            State::Tsv(mut state) => {
                state.step();
                State::Tsv(state)
            }
            State::Json(mut state) => {
                state.step();
                State::Json(state)
            }
            State::JsonL(mut state) => {
                state.step();
                State::JsonL(state)
            }
            State::Parquet(mut state) => {
                state.step();
                State::Parquet(state)
            }
            State::Arrow(mut state) => {
                state.step();
                State::Arrow(state)
            }
        };
        &self.state
    }

    pub fn select_previous(&mut self) {
        match &mut self.state {
            State::SelectFormat(picker) => picker.list_mut().select_previous(),
            State::Csv(state) => state.select_previous(),
            State::Tsv(state) => state.select_previous(),
            State::Json(state) => state.select_previous(),
            State::JsonL(state) => state.select_previous(),
            _ => (),
        };
    }

    pub fn select_next(&mut self) {
        match &mut self.state {
            State::SelectFormat(picker) => picker.list_mut().select_next(),
            State::Csv(state) => state.select_next(),
            State::Tsv(state) => state.select_next(),
            State::Json(state) => state.select_next(),
            State::JsonL(state) => state.select_next(),
            _ => (),
        };
    }

    pub fn handle(&mut self, event: KeyEvent) {
        match &mut self.state {
            State::SelectFormat(pickers) => pickers.input_mut().handle(event),
            State::Csv(state) => {
                state.handle(event);
            }
            State::Tsv(state) => {
                state.handle(event);
            }
            State::Json(state) => {
                state.handle(event);
            }
            State::JsonL(state) => {
                state.handle(event);
            }
            State::Parquet(state) => {
                state.handle(event);
            }
            State::Arrow(state) => {
                state.handle(event);
            }
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
        match &mut state.state {
            State::SelectFormat(state) => {
                SearchPicker::default()
                    .title("Format")
                    .items(Format::iter().map(|fmt| Cow::Borrowed(fmt.into())))
                    .render(area, buf, state);
            }
            State::Csv(state) => {
                CsvExporter::default().render(area, buf, state);
            }
            State::Tsv(state) => {
                TsvExporter::default().render(area, buf, state);
            }
            State::Json(state) => {
                JsonExporter::default().render(area, buf, state);
            }
            State::JsonL(state) => {
                JsonLExporter::default().render(area, buf, state);
            }
            State::Parquet(state) => {
                ParquetExporter::default().render(area, buf, state);
            }
            State::Arrow(state) => {
                ArrowExporter::default().render(area, buf, state);
            }
        }
    }
}

#[derive(Debug)]
pub enum State {
    SelectFormat(SearchPickerState),
    Csv(CsvExporterState),
    Tsv(TsvExporterState),
    Json(JsonExporterState),
    JsonL(JsonLExporterState),
    Parquet(ParquetExporterState),
    Arrow(ArrowExporterState),
}

impl Default for State {
    fn default() -> Self {
        State::SelectFormat(SearchPickerState::default())
    }
}

#[derive(Debug)]
pub enum Export<'a> {
    SelectFormat(&'a SearchPickerState),
    Csv(&'a csv_exporter::State),
    Tsv(&'a tsv_exporter::State),
    Json(&'a json_exporter::State),
    JsonL(&'a jsonl_exporter::State),
    Parquet(&'a parquet_exporter::State),
    Arrow(&'a arrow_exporter::State),
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
