use crate::{
    AppResult,
    duckdb::{
        ingester::{FileIngester, IngestProgress, progress::CountProgress},
        session::SessionConn,
        skim,
        sql::quote,
    },
};
use duckdb::{Appender, appender_params_from_iter, types::Value};
use rusqlite::{
    OpenFlags,
    types::{Value as SqliteValue, ValueRef},
};
use std::{collections::HashMap, path::PathBuf, sync::Arc};

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
            crate::duckdb::cancel::check()?;
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
    let declarations = sqlite
        .prepare("SELECT name, type FROM pragma_table_xinfo(?)")?
        .query_map([table], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?
        .collect::<Result<HashMap<_, _>, _>>()?;
    let mut select = sqlite.prepare(&format!("SELECT * FROM {}", quote(table)))?;
    let columns = select
        .column_names()
        .into_iter()
        .map(|name| {
            (
                name.to_string(),
                declared(declarations.get(name).map(String::as_str)),
            )
        })
        .collect::<Vec<_>>();
    if columns.is_empty() {
        return Ok(());
    }
    let mut kinds = columns.iter().map(|(_, kind)| *kind).collect::<Vec<_>>();
    let mut rows = select.query([])?;
    let mut sample = Vec::new();
    while kinds.iter().any(Option::is_none) && sample.len() < SAMPLE {
        let Some(row) = rows.next()? else {
            break;
        };
        let values = (0..columns.len())
            .map(|i| row.get::<_, SqliteValue>(i))
            .collect::<Result<Vec<_>, _>>()?;
        for (kind, value) in kinds.iter_mut().zip(&values) {
            if kind.is_none() {
                *kind = first(value.into());
            }
        }
        sample.push(values);
    }
    let mut copy = Copy {
        conn,
        alias,
        table,
        columns: columns.iter().map(|(name, _)| name.as_str()).collect(),
        kinds: kinds
            .into_iter()
            .map(|kind| kind.unwrap_or("VARCHAR"))
            .collect(),
        appender: None,
    };
    let definition = copy
        .columns
        .iter()
        .zip(&copy.kinds)
        .map(|(column, kind)| format!("{} {kind}", quote(column)))
        .collect::<Vec<_>>();
    conn.execute_batch(&format!(
        "CREATE OR REPLACE TABLE {} ({})",
        copy.target(),
        definition.join(", ")
    ))?;
    for values in &sample {
        copy.append(&values.iter().map(ValueRef::from).collect::<Vec<_>>())?;
    }
    let mut copied = 0usize;
    while let Some(row) = rows.next()? {
        copied += 1;
        if copied.is_multiple_of(4096) {
            crate::duckdb::cancel::check()?;
        }
        let values = (0..copy.columns.len())
            .map(|i| row.get_ref(i))
            .collect::<Result<Vec<_>, _>>()?;
        copy.append(&values)?;
    }
    if let Some(mut appender) = copy.appender {
        appender.flush()?;
    }
    Ok(())
}

const SAMPLE: usize = 2048;

struct Copy<'a> {
    conn: &'a SessionConn,
    alias: &'a str,
    table: &'a str,
    columns: Vec<&'a str>,
    kinds: Vec<&'static str>,
    appender: Option<Appender<'a>>,
}

impl<'a> Copy<'a> {
    fn target(&self) -> String {
        format!("{}.{}", quote(self.alias), quote(self.table))
    }
    fn append(&mut self, values: &[ValueRef]) -> AppResult<()> {
        for (index, value) in values.iter().enumerate() {
            let Some(wider) = widen(self.kinds[index], *value) else {
                continue;
            };
            if let Some(mut appender) = self.appender.take() {
                appender.flush()?;
            }
            let column = quote(self.columns[index]);
            let using = match self.kinds[index] {
                "DOUBLE" => format!(" USING {}({column})", skim::FLOAT_TEXT),
                _ => String::new(),
            };
            self.conn.execute_batch(&format!(
                "ALTER TABLE {} ALTER {column} TYPE {wider}{using}",
                self.target()
            ))?;
            self.kinds[index] = wider;
        }
        let appender = match &mut self.appender {
            Some(appender) => appender,
            None => self.appender.insert(
                self.conn
                    .appender_to_catalog_and_db(self.table, self.alias, "main")?,
            ),
        };
        appender.append_row(appender_params_from_iter(
            values
                .iter()
                .zip(&self.kinds)
                .map(|(value, kind)| self::value(*value, kind)),
        ))?;
        Ok(())
    }
}

fn declared(decl: Option<&str>) -> Option<&'static str> {
    let decl = decl?.to_ascii_uppercase();
    let has = |parts: &[&str]| parts.iter().any(|part| decl.contains(part));
    if has(&["INT"]) {
        Some("BIGINT")
    } else if has(&["CHAR", "CLOB", "TEXT"]) {
        Some("VARCHAR")
    } else if has(&["BLOB"]) {
        Some("BLOB")
    } else if has(&["REAL", "FLOA", "DOUB"]) {
        Some("DOUBLE")
    } else {
        None
    }
}

fn first(value: ValueRef) -> Option<&'static str> {
    match value {
        ValueRef::Null => None,
        ValueRef::Integer(_) => Some("BIGINT"),
        ValueRef::Real(_) => Some("DOUBLE"),
        ValueRef::Text(_) => Some("VARCHAR"),
        ValueRef::Blob(_) => Some("BLOB"),
    }
}

fn widen(kind: &str, value: ValueRef) -> Option<&'static str> {
    match (kind, value) {
        (_, ValueRef::Null)
        | ("BIGINT", ValueRef::Integer(_))
        | ("DOUBLE", ValueRef::Integer(_) | ValueRef::Real(_))
        | ("VARCHAR", _)
        | ("BLOB", ValueRef::Blob(_)) => None,
        ("BIGINT", ValueRef::Real(_)) => Some("DOUBLE"),
        _ => Some("VARCHAR"),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::duckdb::session::Session;
    use tempfile::TempDir;

    #[test]
    fn declared_types_are_widened_when_a_value_does_not_fit() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("mixed.sqlite");
        let sqlite = rusqlite::Connection::open(&path).unwrap();
        sqlite
            .execute_batch(
                "CREATE TABLE t (id INTEGER, n INTEGER, r REAL, u, d DATE, b BLOB, empty INTEGER);
                 WITH RECURSIVE s(i) AS (SELECT 0 UNION ALL SELECT i + 1 FROM s WHERE i < 4999)
                 INSERT INTO t SELECT i, i, i + 0.5, i, '2024-01-01', x'00ff', NULL FROM s;
                 UPDATE t SET n = 2.5 WHERE id = 3000;
                 UPDATE t SET n = 'x' WHERE id = 4000;
                 UPDATE t SET r = 77.0 WHERE id = 10;
                 UPDATE t SET r = 'y' WHERE id = 4500;
                 CREATE TABLE s (id INTEGER, v REAL) STRICT;
                 INSERT INTO s VALUES (1, 1.5), (2, NULL);",
            )
            .unwrap();
        drop(sqlite);
        let mut session = Session::new(None).unwrap();
        let alias = session.attach(None).unwrap();
        SqliteIngester::new(path, None)
            .ingest(session.session().unwrap(), &alias)
            .unwrap();
        let conn = session.session().unwrap();
        let types = |table: &str| {
            conn.prepare(&format!("DESCRIBE {}.{}", quote(&alias), quote(table)))
                .unwrap()
                .query_map([], |r| r.get::<_, String>(1))
                .unwrap()
                .collect::<Result<Vec<_>, _>>()
                .unwrap()
        };
        assert_eq!(
            types("t"),
            [
                "BIGINT", "VARCHAR", "VARCHAR", "BIGINT", "VARCHAR", "BLOB", "BIGINT"
            ]
        );
        assert_eq!(types("s"), ["BIGINT", "DOUBLE"]);
        let cells = conn
            .prepare(&format!(
                "SELECT id, n, r FROM {}.t WHERE id IN (0, 10, 2999, 3000, 4000, 4500, 4999)",
                quote(&alias)
            ))
            .unwrap()
            .query_map([], |r| {
                Ok((
                    r.get::<_, i64>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, String>(2)?,
                ))
            })
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let cell = |id: i64, n: &str, r: &str| (id, n.to_string(), r.to_string());
        assert_eq!(
            cells,
            [
                cell(0, "0", "0.5"),
                cell(10, "10", "77"),
                cell(2999, "2999", "2999.5"),
                cell(3000, "2.5", "3000.5"),
                cell(4000, "x", "4000.5"),
                cell(4500, "4500", "y"),
                cell(4999, "4999", "4999.5"),
            ]
        );
        let in_order: bool = conn
            .query_row(
                &format!(
                    "SELECT bool_and(id = rn) FROM (SELECT id, row_number() OVER () - 1 AS rn FROM {}.t)",
                    quote(&alias)
                ),
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert!(in_order);
    }
}
