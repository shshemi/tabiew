use polars::datatypes::DataType;
use serde::{Deserialize, Serialize};

use crate::tui;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tabs {
    tabs: Vec<Pane>,
    selected: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pane {
    table_stack: Vec<Table>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    schema: Vec<Field>,
    offset: usize,
    rendered_rows: usize,
    description: Description,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    name: String,
    dtype: DataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Description {
    key: String,
    val: String,
}

impl From<&tui::tabs::Tabs> for Tabs {
    fn from(value: &tui::tabs::Tabs) -> Self {
        Self {
            tabs: value.iter().map(Pane::from).collect(),
            selected: value.idx(),
        }
    }
}

impl From<&tui::Pane> for Pane {
    fn from(value: &tui::Pane) -> Self {
        Self {
            table_stack: value
                .iter_tables()
                .zip(value.iter_descriptions())
                .map(|(table, description)| Table::new(table, description))
                .collect(),
        }
    }
}

impl Table {
    fn new(table: &tui::table::Table, description: &tui::pane::TableDescription) -> Self {
        Self {
            schema: table
                .data_frame()
                .schema()
                .iter()
                .map(|(name, dtype)| Field {
                    name: name.to_string(),
                    dtype: dtype.clone(),
                })
                .collect(),
            offset: table.offset(),
            rendered_rows: table.rendered_rows(),
            description: description.into(),
        }
    }
}

impl From<&tui::pane::TableDescription> for Description {
    fn from(value: &tui::pane::TableDescription) -> Self {
        Self {
            key: value.variant().to_owned(),
            val: value.description().to_owned(),
        }
    }
}
