use std::fmt::Display;

use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};

use crate::tui::{
    pickers::search_picker::SearchPicker,
    popups::{
        importers::{arrow, csv, excel, fwf, json, jsonl, logfmt, parquet, sqlite, tsv},
        step_by_step::{ComponentSequence, StepByStep},
    },
};

pub type Importer = StepByStep<State>;

#[derive(Debug)]
pub enum State {
    PickFormat { picker: SearchPicker<Formats> },
    Arrow { arrow: arrow::State },
    Csv { csv: csv::State },
    Excel { excel: excel::State },
    Fwf { fwf: fwf::State },
    Json { json: json::State },
    JsonL { jsonl: jsonl::State },
    Parquet { parquet: parquet::State },
    Sqlite { sqlite: sqlite::State },
    Tsv { tsv: tsv::State },
    Logfmt { logfmt: logfmt::State },
}

impl ComponentSequence for State {
    fn next(self) -> Self {
        match self {
            State::PickFormat { picker } => match picker.selected_item() {
                Some(Formats::Arrow) => State::Arrow {
                    arrow: Default::default(),
                },
                Some(Formats::Csv) => Self::Csv {
                    csv: Default::default(),
                },
                Some(Formats::Excel) => Self::Excel {
                    excel: Default::default(),
                },
                Some(Formats::Fwf) => Self::Fwf {
                    fwf: Default::default(),
                },
                Some(Formats::Json) => Self::Json {
                    json: Default::default(),
                },
                Some(Formats::Jsonl) => Self::JsonL {
                    jsonl: Default::default(),
                },
                Some(Formats::Parquet) => Self::Parquet {
                    parquet: Default::default(),
                },
                Some(Formats::Sqlite) => Self::Sqlite {
                    sqlite: Default::default(),
                },
                Some(Formats::Tsv) => Self::Tsv {
                    tsv: Default::default(),
                },
                Some(Formats::Logfmt) => Self::Logfmt {
                    logfmt: Default::default(),
                },
                None => State::PickFormat { picker },
            },
            State::Arrow { arrow } => State::Arrow {
                arrow: arrow.next(),
            },
            State::Csv { csv } => State::Csv { csv: csv.next() },
            State::Excel { excel } => State::Excel {
                excel: excel.next(),
            },
            State::Fwf { fwf } => State::Fwf { fwf: fwf.next() },
            State::Json { json } => State::Json { json: json.next() },
            State::JsonL { jsonl } => State::JsonL {
                jsonl: jsonl.next(),
            },
            State::Parquet { parquet } => State::Parquet {
                parquet: parquet.next(),
            },
            State::Sqlite { sqlite } => State::Sqlite {
                sqlite: sqlite.next(),
            },
            State::Tsv { tsv } => State::Tsv { tsv: tsv.next() },
            State::Logfmt { logfmt } => State::Logfmt {
                logfmt: logfmt.next(),
            },
        }
    }

    fn responder(&mut self) -> &mut dyn crate::tui::component::Component {
        match self {
            State::PickFormat { picker } => picker,
            State::Arrow { arrow } => arrow.responder(),
            State::Csv { csv } => csv.responder(),
            State::Excel { excel } => excel.responder(),
            State::Fwf { fwf } => fwf.responder(),
            State::Json { json } => json.responder(),
            State::JsonL { jsonl } => jsonl.responder(),
            State::Parquet { parquet } => parquet.responder(),
            State::Sqlite { sqlite } => sqlite.responder(),
            State::Tsv { tsv } => tsv.responder(),
            State::Logfmt { logfmt } => logfmt.responder(),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::PickFormat {
            picker: SearchPicker::new(Formats::iter().collect()),
        }
    }
}

#[derive(Debug, Clone, Copy, IntoStaticStr, EnumIter)]
pub enum Formats {
    Csv,
    Tsv,
    Parquet,
    Jsonl,
    Json,
    Arrow,
    Fwf,
    Sqlite,
    Excel,
    Logfmt,
}

impl Display for Formats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(self))
    }
}
