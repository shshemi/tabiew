use std::fmt::Display;

use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};

use crate::tui::{
    pickers::search_picker::SearchPicker,
    popups::{
        importers::{
            arrow_importer, csv_importer, excel_importer, fwf_importer, json_importer,
            jsonl_importer, parquet_importer, sqlite_importer, tsv_importer,
        },
        wizard::{Wizard, WizardState},
    },
};

pub type ImportWizard = Wizard<State>;

#[derive(Debug)]
pub enum State {
    PickFormat { picker: SearchPicker<Formats> },
    Arrow { arrow: arrow_importer::State },
    Csv { csv: csv_importer::State },
    Excel { excel: excel_importer::State },
    Fwf { fwf: fwf_importer::State },
    Json { json: json_importer::State },
    JsonL { jsonl: jsonl_importer::State },
    Parquet { parquet: parquet_importer::State },
    Sqlite { sqlite: sqlite_importer::State },
    Tsv { tsv: tsv_importer::State },
}

impl WizardState for State {
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
                None => todo!(),
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
}

impl Display for Formats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(self))
    }
}
