use std::{path::PathBuf, sync::Arc, time::Instant};

use crate::{
    ctl::{Message, types::Shape},
    duckdb::IngestProgress,
};

pub enum Kind {
    Sql(String),
    Filter(String),
    Select(String),
    Order(String),
    Export(PathBuf),
    Import,
}

#[derive(PartialEq)]
pub enum Wait {
    View(String),
    Export(usize),
    Source(PathBuf),
}

pub enum Done {
    Shape(usize, usize),
    Names(Vec<String>),
    Written,
}

pub struct Job {
    pub to: u32,
    pub kind: Kind,
    pub wait: Wait,
    pub progress: Option<Arc<dyn IngestProgress>>,
    pub sent: Instant,
}

impl Kind {
    pub fn reply(&self, to: u32, result: Result<Done, String>) -> Message {
        let shape = |result: Result<Done, String>| {
            result.map(|done| match done {
                Done::Shape(height, width) => Shape { height, width },
                _ => Shape {
                    height: 0,
                    width: 0,
                },
            })
        };
        match self {
            Kind::Sql(query) => Message::sql_replay(
                to,
                shape(result).map_err(|err| format!("Query {query} failed: {err}")),
            ),
            Kind::Filter(filter) => Message::filter_reply(
                to,
                shape(result).map_err(|err| format!("Filter '{filter}' failed: {err}")),
            ),
            Kind::Select(columns) => Message::select_reply(
                to,
                shape(result).map_err(|err| format!("Column selection '{columns}' failed: {err}")),
            ),
            Kind::Order(order_by) => Message::order_reply(
                to,
                shape(result).map_err(|err| format!("Ordering by '{order_by}' failed: {err}")),
            ),
            Kind::Export(path) => Message::export_reply(
                to,
                result
                    .map(|_| path.clone())
                    .map_err(|err| format!("Export to '{}' failed: {err}", path.display())),
            ),
            Kind::Import => Message::import_reply(
                to,
                result
                    .map(|done| match done {
                        Done::Names(names) => names,
                        _ => Vec::new(),
                    })
                    .map_err(|err| format!("Import failed: {err}")),
            ),
        }
    }
}
