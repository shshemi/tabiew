mod arrow;
mod avro;
mod csv;
mod excel;
mod fwf;
mod html;
mod json;
mod logfmt;
mod markdown;
mod parquet;
mod progress;
mod sqlite;

use crate::{
    AppResult,
    args::{Args, Format},
    duckdb::{helper::quote, session::SessionConn},
    misc::{available_col_name::available_col_name, format::resolve_format},
};
use arrow::ArrowIngester;
use avro::AvroIngester;
use chrono::{DateTime, NaiveDate, NaiveDateTime};
use clap::ValueEnum;
use csv::CsvIngester;
pub use csv::CsvOptions;
use duckdb::{
    Connection, appender_params_from_iter,
    types::{TimeUnit, Value},
};
use excel::ExcelIngester;
use fwf::{FwfIngester, FwfOptions};
use html::HtmlIngester;
use json::JsonIngester;
use logfmt::LogfmtIngester;
use markdown::MarkdownIngester;
use parquet::ParquetIngester;
pub use progress::IngestProgress;
use serde::{Serialize, Serializer};
use sqlite::SqliteIngester;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::oneshot::{self, Receiver};

pub trait FileIngester: Send {
    fn progress(&self) -> Arc<dyn IngestProgress>;
    fn ingest(&mut self, conn: SessionConn, alias: &str) -> AppResult<()>;
}

const BATCH: usize = 122_880;
const TYPES: [&str; 5] = ["BIGINT", "DOUBLE", "BOOLEAN", "DATE", "TIMESTAMP"];

struct TextTable<'a> {
    conn: &'a Connection,
    alias: &'a str,
    name: String,
    types: &'a [String],
    columns: Vec<String>,
    kinds: Vec<&'static str>,
    created: usize,
    batch: Vec<Vec<(usize, String)>>,
}

impl<'a> TextTable<'a> {
    fn new(conn: &'a Connection, alias: &'a str, name: String, types: &'a [String]) -> Self {
        Self {
            conn,
            alias,
            name,
            types,
            columns: Vec::new(),
            kinds: Vec::new(),
            created: 0,
            batch: Vec::new(),
        }
    }
    fn column(&mut self, name: &str) -> usize {
        let column = available_col_name(name, |name| !self.columns.iter().any(|c| c == name));
        self.columns.push(column);
        self.columns.len() - 1
    }
    fn width(&self) -> usize {
        self.columns.len()
    }
    fn push(&mut self, row: Vec<(usize, String)>) -> AppResult<()> {
        self.batch.push(row);
        if self.batch.len() >= BATCH {
            self.flush()?;
        }
        Ok(())
    }
    fn finish(mut self) -> AppResult<()> {
        self.flush()
    }
    fn flush(&mut self) -> AppResult<()> {
        if self.columns.is_empty() {
            self.batch.clear();
            return Ok(());
        }
        let mut values = vec![Vec::<&str>::new(); self.columns.len()];
        for row in &self.batch {
            for (index, value) in row {
                if !value.is_empty() {
                    values[*index].push(value);
                }
            }
        }
        let mut changed = Vec::new();
        for (index, values) in values.iter().enumerate() {
            if index == self.kinds.len() {
                self.kinds.push(infer(self.types, values));
            } else if values
                .iter()
                .any(|value| typed(self.kinds[index], value).is_none())
            {
                self.kinds[index] = widen(self.types, self.kinds[index], values);
                changed.push(index);
            }
        }
        let table = format!("{}.{}", quote(self.alias), quote(&self.name));
        if self.created == 0 {
            let definition = self
                .columns
                .iter()
                .zip(&self.kinds)
                .map(|(column, kind)| format!("{} {kind}", quote(column)))
                .collect::<Vec<_>>();
            self.conn.execute_batch(&format!(
                "CREATE OR REPLACE TABLE {table} ({})",
                definition.join(", ")
            ))?;
        } else {
            for index in changed.into_iter().filter(|index| *index < self.created) {
                self.conn.execute_batch(&format!(
                    "ALTER TABLE {table} ALTER {} TYPE {}",
                    quote(&self.columns[index]),
                    self.kinds[index]
                ))?;
            }
            for (column, kind) in self.columns.iter().zip(&self.kinds).skip(self.created) {
                self.conn.execute_batch(&format!(
                    "ALTER TABLE {table} ADD COLUMN {} {kind}",
                    quote(column)
                ))?;
            }
        }
        self.created = self.columns.len();
        let mut appender = self
            .conn
            .appender_to_catalog_and_db(&self.name, self.alias, "main")?;
        for row in self.batch.drain(..) {
            let mut values = vec![Value::Null; self.columns.len()];
            for (index, value) in row {
                values[index] = match self.kinds[index] {
                    "VARCHAR" => Value::Text(value),
                    _ if value.is_empty() => Value::Null,
                    kind => typed(kind, &value).unwrap_or(Value::Text(value)),
                };
            }
            appender.append_row(appender_params_from_iter(values))?;
        }
        appender.flush()?;
        Ok(())
    }
}

fn infer(types: &[String], values: &[&str]) -> &'static str {
    TYPES
        .iter()
        .filter(|kind| types.iter().any(|t| t == *kind))
        .find(|kind| !values.is_empty() && values.iter().all(|value| typed(kind, value).is_some()))
        .copied()
        .unwrap_or("VARCHAR")
}

fn widen(types: &[String], kind: &'static str, values: &[&str]) -> &'static str {
    let wider = match kind {
        "BIGINT" => "DOUBLE",
        "DATE" => "TIMESTAMP",
        _ => "VARCHAR",
    };
    if types.iter().any(|t| t == wider) && values.iter().all(|value| typed(wider, value).is_some())
    {
        wider
    } else {
        "VARCHAR"
    }
}

fn typed(kind: &str, value: &str) -> Option<Value> {
    match kind {
        "BIGINT" => value.parse().ok().map(Value::BigInt),
        "DOUBLE" => value.parse().ok().map(Value::Double),
        "BOOLEAN" if value.eq_ignore_ascii_case("true") => Some(Value::Boolean(true)),
        "BOOLEAN" if value.eq_ignore_ascii_case("false") => Some(Value::Boolean(false)),
        "DATE" => NaiveDate::parse_from_str(value, "%Y-%m-%d")
            .ok()
            .map(|date| (date - DateTime::UNIX_EPOCH.date_naive()).num_days() as i32)
            .map(Value::Date32),
        "TIMESTAMP" => DateTime::parse_from_rfc3339(value)
            .map(|datetime| datetime.naive_utc())
            .or_else(|_| NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M:%S%.f"))
            .or_else(|_| NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S%.f"))
            .ok()
            .map(|datetime| {
                Value::Timestamp(TimeUnit::Microsecond, datetime.and_utc().timestamp_micros())
            }),
        "VARCHAR" => Some(Value::Text(value.to_owned())),
        _ => None,
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct IngestOptions {
    #[serde(serialize_with = "format_name")]
    pub format: Option<Format>,
    pub csv: CsvOptions,
    pub fwf: FwfOptions,
    pub types: Vec<String>,
    #[serde(skip)]
    pub sqlite_key: Option<String>,
}

impl IngestOptions {
    pub fn from_args(args: &Args) -> Self {
        Self {
            format: args.format.clone(),
            csv: CsvOptions::from_args(args),
            fwf: FwfOptions::from_args(args),
            types: types_from_args(args),
            sqlite_key: args.sqlite_key.clone(),
        }
    }
}

impl Default for IngestOptions {
    fn default() -> Self {
        Self {
            format: None,
            csv: CsvOptions::default(),
            fwf: FwfOptions::default(),
            types: vec!["BIGINT".to_string(), "DOUBLE".to_string()],
            sqlite_key: None,
        }
    }
}

fn types_from_args(args: &Args) -> Vec<String> {
    if args.no_type_inference {
        return Vec::new();
    }
    let types = csv::type_candidates(args.infer_types.inner())
        .unwrap_or_else(|| TYPES.iter().map(|kind| kind.to_string()).collect());
    TYPES
        .iter()
        .filter(|kind| types.iter().any(|t| t == *kind))
        .map(|kind| kind.to_string())
        .collect()
}

fn format_name<S: Serializer>(format: &Option<Format>, serializer: S) -> Result<S::Ok, S::Error> {
    let name = format
        .as_ref()
        .and_then(ValueEnum::to_possible_value)
        .map(|value| value.get_name().to_string());
    name.serialize(serializer)
}

pub struct Ingester {
    conn: SessionConn,
    alias: String,
    file_ingester: Box<dyn FileIngester>,
}

impl Ingester {
    pub fn new(
        conn: SessionConn,
        alias: String,
        source: PathBuf,
        options: &IngestOptions,
    ) -> AppResult<Self> {
        let file_ingester = Self::file_ingester(source, options)?;
        Ok(Self {
            conn,
            alias,
            file_ingester,
        })
    }
    fn file_ingester(source: PathBuf, options: &IngestOptions) -> AppResult<Box<dyn FileIngester>> {
        Ok(match resolve_format(options.format.as_ref(), &source) {
            Format::Csv | Format::Dsv => Box::new(CsvIngester::new(source, options.csv.clone())),
            Format::Tsv => Box::new(CsvIngester::new(
                source,
                CsvOptions {
                    delimiter: '\t',
                    ..options.csv.clone()
                },
            )),
            Format::Json => Box::new(JsonIngester::new(source, false)),
            Format::Jsonl => Box::new(JsonIngester::new(source, true)),
            Format::Parquet => Box::new(ParquetIngester::new(source)),
            Format::Arrow => Box::new(ArrowIngester::new(source)),
            Format::Fwf => Box::new(FwfIngester::new(
                source,
                options.fwf.clone(),
                options.types.clone(),
            )),
            Format::Sqlite => Box::new(SqliteIngester::new(source, options.sqlite_key.clone())),
            Format::Excel => Box::new(ExcelIngester::new(source, options.csv.header)),
            Format::Logfmt => Box::new(LogfmtIngester::new(source, options.types.clone())),
            Format::Avro => Box::new(AvroIngester::new(source)),
            Format::Html => Box::new(HtmlIngester::new(source, options.types.clone())),
            Format::Markdown => Box::new(MarkdownIngester::new(source, options.types.clone())),
        })
    }
    pub fn ingest(self) -> AppResult<(Arc<dyn IngestProgress>, Receiver<AppResult<()>>)> {
        let Self {
            conn,
            alias,
            mut file_ingester,
        } = self;
        conn.set_query_progress_tracking(true)?;
        let progress = file_ingester.progress();
        let (sender, receiver) = oneshot::channel();
        tokio::task::spawn_blocking(move || sender.send(file_ingester.ingest(conn, &alias)));
        Ok((progress, receiver))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::duckdb::session::Session;
    use anyhow::anyhow;

    struct Fake {
        fail: bool,
    }

    impl FileIngester for Fake {
        fn progress(&self) -> Arc<dyn IngestProgress> {
            progress::CountProgress::new()
        }
        fn ingest(&mut self, conn: SessionConn, alias: &str) -> AppResult<()> {
            if self.fail {
                return Err(anyhow!("broken file"));
            }
            conn.execute_batch(&format!(
                "CREATE TABLE {}.data AS SELECT range AS id FROM range(10)",
                quote(alias)
            ))?;
            Ok(())
        }
    }

    struct Panics;

    impl FileIngester for Panics {
        fn progress(&self) -> Arc<dyn IngestProgress> {
            progress::CountProgress::new()
        }
        fn ingest(&mut self, _conn: SessionConn, _alias: &str) -> AppResult<()> {
            panic!("ingester bug")
        }
    }

    fn ingester(session: &mut Session, file_ingester: Box<dyn FileIngester>) -> Ingester {
        let alias = session.attach(None).unwrap();
        Ingester {
            conn: session.session().unwrap(),
            alias,
            file_ingester,
        }
    }

    #[tokio::test]
    async fn ingest_writes_the_table_then_releases_the_conn() {
        let mut session = Session::new(None).unwrap();
        let (_, result) = ingester(&mut session, Box::new(Fake { fail: false }))
            .ingest()
            .unwrap();
        result.await.unwrap().unwrap();
        session.create_view("mem", "mem", "data", None).unwrap();
        let count: i64 = session
            .session()
            .unwrap()
            .query_row("SELECT count(*) FROM mem", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 10);
        session.detach("mem").unwrap();
        session.unpin("mem");
        assert_eq!(session.flush().unwrap(), ["mem"]);
    }

    #[tokio::test]
    async fn ingest_error_is_sent_back() {
        let mut session = Session::new(None).unwrap();
        let (_, result) = ingester(&mut session, Box::new(Fake { fail: true }))
            .ingest()
            .unwrap();
        let error = result.await.unwrap().unwrap_err();
        assert!(error.to_string().contains("broken file"));
        session.unpin("mem");
        assert_eq!(session.flush().unwrap(), ["mem"]);
    }

    #[tokio::test]
    async fn panicking_ingester_closes_the_channel_and_releases_the_conn() {
        let mut session = Session::new(None).unwrap();
        let (_, result) = ingester(&mut session, Box::new(Panics)).ingest().unwrap();
        assert!(result.await.is_err());
        session.unpin("mem");
        assert_eq!(session.flush().unwrap(), ["mem"]);
    }
}
