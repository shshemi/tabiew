use crate::{
    AppResult,
    duckdb::{
        ingester::{FileIngester, IngestProgress, progress::CountProgress},
        session::SessionConn,
        sql::quote,
    },
};
use duckdb::{appender_params_from_iter, types::Value};
use rusqlite::{OpenFlags, types::ValueRef};
use std::{path::PathBuf, sync::Arc};

pub struct SqliteIngester {
    path: PathBuf,
    key: Option<String>,
    progress: Arc<CountProgress>,
}

impl SqliteIngester {
    pub fn new(path: PathBuf, key: Option<String>) -> Self {
        Self {
            path,
            key,
            progress: CountProgress::new(),
        }
    }
}

impl FileIngester for SqliteIngester {
    fn progress(&self) -> Arc<dyn IngestProgress> {
        self.progress.clone()
    }
    fn ingest(&mut self, conn: SessionConn, alias: &str) -> AppResult<()> {
        let sqlite =
            rusqlite::Connection::open_with_flags(&self.path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
        if let Some(key) = &self.key {
            sqlite.pragma_update(None, "key", key)?;
        }
        let tables = sqlite
            .prepare(
                "SELECT name FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%' ORDER BY rowid",
            )?
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<String>, _>>()?;
        self.progress.total(tables.len() as u64);
        for table in tables {
            copy_table(&sqlite, &conn, alias, &table)?;
            self.progress.add(1);
        }
        Ok(())
    }
}

fn copy_table(
    sqlite: &rusqlite::Connection,
    conn: &SessionConn,
    alias: &str,
    table: &str,
) -> AppResult<()> {
    let mut select = sqlite.prepare(&format!("SELECT * FROM {}", quote(table)))?;
    let columns = select
        .column_names()
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>();
    if columns.is_empty() {
        return Ok(());
    }
    let kinds = columns
        .iter()
        .map(|column| format!("group_concat(DISTINCT typeof({}))", quote(column)))
        .collect::<Vec<_>>();
    let types = sqlite.query_row(
        &format!("SELECT {} FROM {}", kinds.join(", "), quote(table)),
        [],
        |row| {
            (0..columns.len())
                .map(|i| Ok(column_type(row.get::<_, Option<String>>(i)?.as_deref())))
                .collect::<Result<Vec<_>, _>>()
        },
    )?;
    let definition = columns
        .iter()
        .zip(&types)
        .map(|(column, kind)| format!("{} {kind}", quote(column)))
        .collect::<Vec<_>>();
    conn.execute_batch(&format!(
        "CREATE OR REPLACE TABLE {}.{} ({})",
        quote(alias),
        quote(table),
        definition.join(", ")
    ))?;
    let mut appender = conn.appender_to_catalog_and_db(table, alias, "main")?;
    let mut rows = select.query([])?;
    while let Some(row) = rows.next()? {
        let values = types
            .iter()
            .enumerate()
            .map(|(i, kind)| Ok(value(row.get_ref(i)?, kind)))
            .collect::<Result<Vec<_>, rusqlite::Error>>()?;
        appender.append_row(appender_params_from_iter(values))?;
    }
    appender.flush()?;
    Ok(())
}

fn column_type(kinds: Option<&str>) -> &'static str {
    let kinds = kinds
        .unwrap_or_default()
        .split(',')
        .filter(|kind| *kind != "null")
        .collect::<Vec<_>>();
    if kinds.is_empty() {
        "VARCHAR"
    } else if kinds.iter().all(|kind| *kind == "integer") {
        "BIGINT"
    } else if kinds.iter().all(|kind| matches!(*kind, "integer" | "real")) {
        "DOUBLE"
    } else if kinds == ["blob"] {
        "BLOB"
    } else {
        "VARCHAR"
    }
}

fn value(value: ValueRef, kind: &str) -> Value {
    match (value, kind) {
        (ValueRef::Null, _) => Value::Null,
        (ValueRef::Integer(n), "BIGINT") => Value::BigInt(n),
        (ValueRef::Integer(n), "DOUBLE") => Value::Double(n as f64),
        (ValueRef::Real(f), "DOUBLE") => Value::Double(f),
        (ValueRef::Blob(bytes), "BLOB") => Value::Blob(bytes.to_vec()),
        (ValueRef::Integer(n), _) => Value::Text(n.to_string()),
        (ValueRef::Real(f), _) => Value::Text(f.to_string()),
        (ValueRef::Text(bytes) | ValueRef::Blob(bytes), _) => {
            Value::Text(String::from_utf8_lossy(bytes).into_owned())
        }
    }
}
