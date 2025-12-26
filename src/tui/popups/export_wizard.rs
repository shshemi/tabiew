use std::fmt::Display;

use polars::frame::DataFrame;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};

use crate::tui::{
    component::Component,
    pickers::search_picker::SearchPicker,
    popups::{
        exporters::{
            arrow_exporter, csv_exporter, json_exporter, jsonl_exporter, parquet_exporter,
            tsv_exporter,
        },
        wizard::{Wizard, WizardState},
    },
};

pub type ExportWizard = Wizard<State>;

#[derive(Debug)]
pub enum State {
    PickFormat {
        df: DataFrame,
        picker: SearchPicker<Format>,
    },
    Arrow {
        state: arrow_exporter::State,
    },
    Csv {
        state: csv_exporter::State,
    },
    Json {
        state: json_exporter::State,
    },
    JsonL {
        state: jsonl_exporter::State,
    },
    Parquet {
        state: parquet_exporter::State,
    },
    Tsv {
        state: tsv_exporter::State,
    },
}

impl From<DataFrame> for State {
    fn from(value: DataFrame) -> Self {
        State::PickFormat {
            df: value,
            picker: SearchPicker::new(Format::iter().collect()).with_title("Format"),
        }
    }
}

impl WizardState for State {
    fn next(self) -> Self {
        match self {
            State::PickFormat { df, picker } => match picker.selected_item() {
                Some(Format::Arrow) => State::Arrow { state: df.into() },
                Some(Format::Csv) => State::Csv { state: df.into() },
                Some(Format::Json) => State::Json { state: df.into() },
                Some(Format::JsonL) => State::JsonL { state: df.into() },
                Some(Format::Parquet) => State::Parquet { state: df.into() },
                Some(Format::Tsv) => State::Tsv { state: df.into() },
                None => State::PickFormat { df, picker },
            },
            State::Arrow { state } => State::Arrow {
                state: state.next(),
            },
            State::Csv { state } => State::Csv {
                state: state.next(),
            },
            State::Json { state } => State::Json {
                state: state.next(),
            },
            State::JsonL { state } => State::JsonL {
                state: state.next(),
            },
            State::Parquet { state } => State::Parquet {
                state: state.next(),
            },
            State::Tsv { state } => State::Tsv {
                state: state.next(),
            },
        }
    }

    fn responder(&mut self) -> &mut dyn Component {
        match self {
            State::PickFormat { df: _, picker } => picker,
            State::Arrow { state } => state.responder(),
            State::Csv { state } => state.responder(),
            State::Json { state } => state.responder(),
            State::JsonL { state } => state.responder(),
            State::Parquet { state } => state.responder(),
            State::Tsv { state } => state.responder(),
        }
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
