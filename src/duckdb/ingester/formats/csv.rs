use crate::{
    AppResult,
    args::{Args, Type},
    duckdb::{
        ingester::{FileIngester, IngestProgress, progress::StepProgress},
        session::SessionConn,
        sql::quote,
    },
};
use duckdb::params;
use serde::Serialize;
use std::{path::PathBuf, sync::Arc};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CsvOptions {
    pub delimiter: char,
    pub quote: char,
    pub header: bool,
    pub ignore_errors: bool,
    pub null_padding: bool,
    pub all_varchar: bool,
    pub type_candidates: Option<Vec<String>>,
}

impl Default for CsvOptions {
    fn default() -> Self {
        Self {
            delimiter: ',',
            quote: '"',
            header: true,
            ignore_errors: false,
            null_padding: false,
            all_varchar: false,
            type_candidates: Some(vec!["BIGINT".to_string(), "DOUBLE".to_string()]),
        }
    }
}

impl CsvOptions {
    pub fn from_args(args: &Args) -> Self {
        Self {
            delimiter: args.separator,
            quote: args.quote_char,
            header: !args.no_header,
            ignore_errors: args.ignore_errors,
            null_padding: args.truncate_ragged_lines,
            all_varchar: args.no_type_inference,
            type_candidates: type_candidates(args.infer_types.inner()),
        }
    }
}

pub struct CsvIngester {
    path: PathBuf,
    options: CsvOptions,
    progress: Arc<StepProgress>,
}

impl CsvIngester {
    pub fn new(path: PathBuf, options: CsvOptions) -> Self {
        Self {
            path,
            options,
            progress: StepProgress::new(1),
        }
    }
    fn sql(&self, alias: &str, varchar: &[String]) -> String {
        let mut options = String::new();
        if let Some(types) = &self.options.type_candidates {
            let types = types
                .iter()
                .map(String::as_str)
                .chain(["VARCHAR"])
                .map(literal)
                .collect::<Vec<_>>();
            options.push_str(&format!(", auto_type_candidates = [{}]", types.join(", ")));
        }
        if !varchar.is_empty() {
            let types = varchar
                .iter()
                .map(|column| format!("{}: 'VARCHAR'", literal(column)))
                .collect::<Vec<_>>();
            options.push_str(&format!(", types = {{{}}}", types.join(", ")));
        }
        format!(
            "CREATE OR REPLACE TABLE {}.data AS SELECT * FROM read_csv(?, delim = ?, quote = ?, header = ?, ignore_errors = ?, null_padding = ?, all_varchar = ?{options})",
            quote(alias)
        )
    }
}

impl FileIngester for CsvIngester {
    fn progress(&self) -> Arc<dyn IngestProgress> {
        self.progress.clone()
    }
    fn ingest(&mut self, conn: SessionConn, alias: &str) -> AppResult<()> {
        self.progress.start(&conn);
        let path = self.path.to_string_lossy().into_owned();
        let mut varchar = Vec::<String>::new();
        loop {
            let result = conn.execute(
                &self.sql(alias, &varchar),
                params![
                    path,
                    self.options.delimiter.to_string(),
                    self.options.quote.to_string(),
                    self.options.header,
                    self.options.ignore_errors,
                    self.options.null_padding,
                    self.options.all_varchar,
                ],
            );
            match result {
                Ok(_) => return Ok(()),
                Err(e) => match failed_column(&e.to_string()) {
                    Some(column) if !varchar.contains(&column) => varchar.push(column),
                    _ => return Err(e.into()),
                },
            }
        }
    }
}

pub(in crate::duckdb::ingester) fn type_candidates(types: &[Type]) -> Option<Vec<String>> {
    types
        .iter()
        .map(|t| {
            match t {
                Type::All => None,
                Type::Int => Some("BIGINT"),
                Type::Float => Some("DOUBLE"),
                Type::Boolean => Some("BOOLEAN"),
                Type::Date => Some("DATE"),
                Type::Datetime => Some("TIMESTAMP"),
            }
            .map(String::from)
        })
        .collect()
}

fn literal(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}

fn failed_column(error: &str) -> Option<String> {
    let rest = error.split_once("Error when converting column \"")?.1;
    Some(rest.split_once('"')?.0.to_string())
}
