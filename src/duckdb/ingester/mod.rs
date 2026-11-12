mod dates;
mod download;
mod formats;
mod options;
pub(super) mod progress;
mod text_table;

use crate::{
    AppResult,
    args::Format,
    duckdb::session::SessionConn,
    misc::{format::resolve_format, workers},
};
use download::Download;
use formats::{
    ArrowIngester, AvroIngester, CsvIngester, ExcelIngester, FwfIngester, HtmlIngester,
    JsonIngester, LogfmtIngester, MarkdownIngester, ParquetIngester, SqliteIngester,
};
pub use formats::{CsvOptions, FwfOptions};
pub(crate) use formats::{cut_text, sample_columns};
pub use options::{Import, IngestOptions};
pub use progress::IngestProgress;
use std::{
    path::PathBuf,
    sync::{Arc, atomic::AtomicBool},
};
use text_table::{TextTable, line_limit, read_file};
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
    types: Vec<String>,
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
            types: options.types.clone(),
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
    pub fn download(mut self, url: Url, path: PathBuf, complete: Arc<AtomicBool>) -> Self {
        self.file_ingester = Box::new(Download::new(url, path, complete, self.file_ingester));
        self
    }
    pub fn ingest(self) -> AppResult<Ingest> {
        let Self {
            conn,
            alias,
            mut file_ingester,
            types,
        } = self;
        conn.set_query_progress_tracking(true)?;
        let progress = file_ingester.progress();
        let refine = conn.try_clone()?;
        let (sender, receiver) = oneshot::channel();
        workers::spawn(move || {
            let result = file_ingester
                .ingest(conn, &alias)
                .and_then(|()| dates::refine(&refine, &alias, &types));
            let _ = sender.send(result);
        });
        Ok((progress, receiver))
    }
}
