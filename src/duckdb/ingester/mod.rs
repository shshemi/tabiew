mod download;
mod formats;
mod options;
pub(super) mod progress;
mod text_table;

use crate::{AppResult, args::Format, duckdb::session::SessionConn, misc::format::resolve_format};
use download::Download;
pub use formats::CsvOptions;
use formats::{
    ArrowIngester, AvroIngester, CsvIngester, ExcelIngester, FwfIngester, HtmlIngester,
    JsonIngester, LogfmtIngester, MarkdownIngester, ParquetIngester, SqliteIngester,
};
pub use options::IngestOptions;
pub use progress::IngestProgress;
use std::{path::PathBuf, sync::Arc};
use text_table::TextTable;
use tokio::sync::oneshot::{self, Receiver};
use url::Url;

type Ingest = (Arc<dyn IngestProgress>, Receiver<AppResult<()>>);

pub trait FileIngester: Send {
    fn progress(&self) -> Arc<dyn IngestProgress>;
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
    pub fn download(mut self, url: Url, path: PathBuf) -> Self {
        self.file_ingester = Box::new(Download::new(url, path, self.file_ingester));
        self
    }
    pub fn ingest(self) -> AppResult<Ingest> {
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
    use crate::duckdb::{session::Session, sql::quote};
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
