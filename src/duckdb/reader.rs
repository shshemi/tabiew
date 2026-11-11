use crate::{
    AppResult,
    duckdb::{DuckDB, session::SessionConn, sql::quote},
    misc::config::config,
    reader::{Columns, Datum, Reader, ReaderConfig, RowSource},
};
use std::sync::{
    Arc, OnceLock,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};

pub(super) type Count = Arc<OnceLock<usize>>;

const FETCH_BUDGET_MS: u64 = 150;

pub struct ViewSource {
    conn: SessionConn,
    view: String,
    columns: Columns,
    count: Count,
    seen: AtomicUsize,
    limited: AtomicBool,
    deferred: AtomicBool,
    select: String,
}

impl DuckDB {
    pub fn reader(&self, view: &str) -> AppResult<Reader<ViewSource>> {
        Ok(Reader::new(
            ViewSource::new(self.session.session()?, view)?,
            self.reader,
        ))
    }
    pub(super) fn growing_reader(&self, view: &str) -> AppResult<(Reader<ViewSource>, Count)> {
        let source = ViewSource::growing(self.session.session()?, view)?;
        source
            .conn
            .execute_batch(&format!("SET max_execution_time = {FETCH_BUDGET_MS}"))?;
        source.limited.store(true, Ordering::Relaxed);
        let count = source.count.clone();
        Ok((Reader::new(source, self.reader), count))
    }
    pub fn reader_config(&self) -> ReaderConfig {
        self.reader
    }
}

impl ViewSource {
    pub fn new(conn: SessionConn, view: &str) -> AppResult<Self> {
        let source = Self::growing(conn, view)?;
        let row_count =
            source
                .conn
                .query_row(&format!("SELECT count(*) FROM {}", quote(view)), [], |r| {
                    r.get::<_, i64>(0)
                })? as usize;
        let _ = source.count.set(row_count);
        Ok(source)
    }
    pub(super) fn growing(conn: SessionConn, view: &str) -> AppResult<Self> {
        let columns: Columns = conn
            .prepare(&format!("DESCRIBE {}", quote(view)))?
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
            .collect::<Result<Vec<(String, String)>, _>>()?
            .into();
        let select = columns
            .iter()
            .map(|(name, data_type)| match is_float(data_type) {
                true => quote(name),
                false => format!("CAST({} AS VARCHAR)", quote(name)),
            })
            .collect::<Vec<_>>()
            .join(", ");
        Ok(Self {
            conn,
            view: view.to_string(),
            columns,
            count: Count::default(),
            seen: AtomicUsize::new(1),
            limited: AtomicBool::new(false),
            deferred: AtomicBool::new(false),
            select,
        })
    }
}

impl ViewSource {
    pub fn deferred(&self) -> bool {
        self.deferred.load(Ordering::Relaxed)
    }
}

impl RowSource for ViewSource {
    fn columns(&self) -> Columns {
        self.columns.clone()
    }
    fn row_count(&self) -> usize {
        self.count
            .get()
            .copied()
            .unwrap_or_else(|| self.seen.load(Ordering::Relaxed))
    }
    fn complete(&self) -> bool {
        self.count.get().is_some()
    }
    fn view(&self) -> Option<&str> {
        Some(&self.view)
    }
    fn fetch(&self, offset: usize, len: usize) -> AppResult<Vec<Vec<Datum>>> {
        let limited = !self.complete();
        if limited && self.deferred.load(Ordering::Relaxed) {
            return Ok(Vec::new());
        }
        if !limited && self.limited.swap(false, Ordering::Relaxed) {
            self.conn.execute_batch("RESET max_execution_time")?;
        }
        let rows = match self.query(offset, len) {
            Err(e) if limited && e.to_string().contains("maximum execution time") => {
                self.deferred.store(true, Ordering::Relaxed);
                return Ok(Vec::new());
            }
            rows => rows?,
        };
        match rows.len() < len {
            true => {
                let _ = self.count.set(offset + rows.len());
            }
            false => {
                self.seen.fetch_max(offset + len + 1, Ordering::Relaxed);
            }
        }
        Ok(rows)
    }
}

impl ViewSource {
    fn query(&self, offset: usize, len: usize) -> AppResult<Vec<Vec<Datum>>> {
        let width = self.columns.len();
        let precision = config().fp_precision();
        let floats = self
            .columns
            .iter()
            .map(|(_, data_type)| is_float(data_type))
            .collect::<Vec<_>>();
        Ok(self
            .conn
            .prepare(&format!(
                "SELECT {} FROM {} LIMIT {len} OFFSET {offset}",
                self.select,
                quote(&self.view),
            ))?
            .query_map([], |r| {
                (0..width)
                    .map(|idx| {
                        Ok(match floats[idx] {
                            true => r.get::<_, Option<f64>>(idx)?.map(|value| match precision {
                                Some(precision) => format!("{value:.precision$}"),
                                None => value.to_string(),
                            }),
                            false => r.get::<_, Option<String>>(idx)?,
                        })
                    })
                    .collect()
            })?
            .collect::<Result<_, _>>()?)
    }
}

pub(super) fn is_float(data_type: &str) -> bool {
    matches!(data_type, "DOUBLE" | "FLOAT")
}
