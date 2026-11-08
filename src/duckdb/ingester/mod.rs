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
mod sqlite;

use crate::{AppResult, args::Format, duckdb::session::SessionConn};
use anyhow::anyhow;
use arrow::ArrowIngester;
use avro::AvroIngester;
use clap::ValueEnum;
use csv::CsvIngester;
use duckdb::ProgressHandle;
use excel::ExcelIngester;
use fwf::FwfIngester;
use html::HtmlIngester;
use json::JsonIngester;
use logfmt::LogfmtIngester;
use markdown::MarkdownIngester;
use parquet::ParquetIngester;
use sqlite::SqliteIngester;
use std::path::PathBuf;
use tokio::sync::oneshot::{self, Receiver};

pub trait FileIngester: Send {
    fn ingest(&mut self, conn: SessionConn, alias: &str) -> AppResult<()>;
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
        format: Option<Format>,
    ) -> AppResult<Self> {
        let file_ingester = Self::file_ingester(source, format)?;
        Ok(Self {
            conn,
            alias,
            file_ingester,
        })
    }
    fn file_ingester(source: PathBuf, format: Option<Format>) -> AppResult<Box<dyn FileIngester>> {
        let format = format.or_else(|| {
            source
                .extension()
                .and_then(|ext| Format::from_str(&ext.to_string_lossy(), true).ok())
        });
        let Some(format) = format else {
            return Err(anyhow!("Unsupported file type {}", source.display()));
        };
        Ok(match format {
            Format::Csv => Box::new(CsvIngester::new(source, Some(','))),
            Format::Tsv => Box::new(CsvIngester::new(source, Some('\t'))),
            Format::Dsv => Box::new(CsvIngester::new(source, None)),
            Format::Json => Box::new(JsonIngester::new(source, false)),
            Format::Jsonl => Box::new(JsonIngester::new(source, true)),
            Format::Parquet => Box::new(ParquetIngester::new(source)),
            Format::Arrow => Box::new(ArrowIngester::new(source)),
            Format::Fwf => Box::new(FwfIngester::new(source)),
            Format::Sqlite => Box::new(SqliteIngester::new(source)),
            Format::Excel => Box::new(ExcelIngester::new(source)),
            Format::Logfmt => Box::new(LogfmtIngester::new(source)),
            Format::Avro => Box::new(AvroIngester::new(source)),
            Format::Html => Box::new(HtmlIngester::new(source)),
            Format::Markdown => Box::new(MarkdownIngester::new(source)),
        })
    }
    pub fn ingest(self) -> AppResult<(ProgressHandle, Receiver<AppResult<()>>)> {
        let Self {
            conn,
            alias,
            mut file_ingester,
        } = self;
        conn.set_query_progress_tracking(true)?;
        let progress = conn.progress_handle();
        let (sender, receiver) = oneshot::channel();
        tokio::task::spawn_blocking(move || sender.send(file_ingester.ingest(conn, &alias)));
        Ok((progress, receiver))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::duckdb::{helper::quote, session::Session};

    struct Fake {
        fail: bool,
    }

    impl FileIngester for Fake {
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
        fn ingest(&mut self, _conn: SessionConn, _alias: &str) -> AppResult<()> {
            panic!("ingester bug")
        }
    }

    fn ingester(session: &mut Session, file_ingester: Box<dyn FileIngester>) -> Ingester {
        session.attach(Some("mem"), None).unwrap();
        Ingester {
            conn: session.session().unwrap(),
            alias: session.alias("mem").unwrap(),
            file_ingester,
        }
    }

    #[tokio::test]
    async fn ingest_writes_the_table_then_releases_the_conn() {
        let mut session = Session::new(None).unwrap();
        let (progress, result) = ingester(&mut session, Box::new(Fake { fail: false }))
            .ingest()
            .unwrap();
        result.await.unwrap().unwrap();
        assert!(!progress.is_connected());
        session.create_view("mem").unwrap();
        let count: i64 = session
            .session()
            .unwrap()
            .query_row("SELECT count(*) FROM mem", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 10);
        session.detach("mem").unwrap();
        assert_eq!(session.flush().unwrap(), ["mem_mem"]);
    }

    #[tokio::test]
    async fn ingest_error_is_sent_back() {
        let mut session = Session::new(None).unwrap();
        let (_, result) = ingester(&mut session, Box::new(Fake { fail: true }))
            .ingest()
            .unwrap();
        let error = result.await.unwrap().unwrap_err();
        assert!(error.to_string().contains("broken file"));
        session.detach("mem").unwrap();
        assert_eq!(session.flush().unwrap(), ["mem_mem"]);
    }

    #[tokio::test]
    async fn panicking_ingester_closes_the_channel_and_releases_the_conn() {
        let mut session = Session::new(None).unwrap();
        let (_, result) = ingester(&mut session, Box::new(Panics)).ingest().unwrap();
        assert!(result.await.is_err());
        session.detach("mem").unwrap();
        assert_eq!(session.flush().unwrap(), ["mem_mem"]);
    }

    #[test]
    fn format_overrides_the_extension() {
        let pick =
            |path: &str, format| Ingester::file_ingester(PathBuf::from(path), format).is_ok();
        assert!(pick("sales.csv", None));
        assert!(pick("sales.CSV", None));
        assert!(pick("sales.txt", Some(Format::Csv)));
        assert!(!pick("sales.txt", None));
        assert!(!pick("sales", None));
        assert!(pick("sales.csv", Some(Format::Parquet)));
    }
}
