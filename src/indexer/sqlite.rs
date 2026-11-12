use std::{
    path::Path,
    sync::{
        Arc, Mutex, RwLock,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
};

use rusqlite::{Connection, OpenFlags, params, types::ValueRef};

use crate::{
    AppResult,
    indexer::BLOCK,
    reader::{Columns, Datum, RowSource},
};

#[derive(Default)]
struct Scan {
    rowids: RwLock<Vec<i64>>,
    rows: AtomicUsize,
    done: AtomicBool,
}

pub struct SqliteSource {
    conn: Mutex<Connection>,
    table: String,
    columns: Columns,
    scan: Arc<Scan>,
}

impl SqliteSource {
    pub fn open(path: &Path, key: Option<&str>) -> AppResult<Self> {
        let conn = connect(path, key)?;
        let name: String = conn.query_row(
            "SELECT name FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%' ORDER BY name LIMIT 1",
            [],
            |row| row.get(0),
        )?;
        let columns = conn
            .prepare("SELECT name, type FROM pragma_table_xinfo(?) WHERE hidden != 1")?
            .query_map([&name], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    duckdb_type(&row.get::<_, String>(1)?).to_string(),
                ))
            })?
            .collect::<Result<Columns, _>>()?;
        let table = quote(&name);
        let scan = Arc::new(Scan::default());
        let task = scan.clone();
        let (path, key) = (path.to_path_buf(), key.map(str::to_owned));
        let rowids = format!("SELECT rowid FROM {table} ORDER BY rowid");
        let count = format!("SELECT count(*) FROM {table}");
        std::thread::spawn(move || {
            let Ok(conn) = connect(&path, key.as_deref()) else {
                task.done.store(true, Ordering::Release);
                return;
            };
            match conn.prepare(&rowids) {
                Ok(mut statement) => {
                    if let Ok(mut rows) = statement.query([]) {
                        while let Ok(Some(row)) = rows.next() {
                            if Arc::strong_count(&task) == 1 {
                                return;
                            }
                            let seen = task.rows.load(Ordering::Relaxed);
                            if seen.is_multiple_of(BLOCK) {
                                task.rowids.write().unwrap().push(row.get(0).unwrap_or(0));
                            }
                            task.rows.store(seen + 1, Ordering::Release);
                        }
                    }
                }
                Err(_) => {
                    if let Ok(rows) = conn.query_row(&count, [], |row| row.get::<_, i64>(0)) {
                        task.rows.store(rows as usize, Ordering::Release);
                    }
                }
            }
            task.done.store(true, Ordering::Release);
        });
        Ok(Self {
            conn: Mutex::new(conn),
            table,
            columns,
            scan,
        })
    }
}

impl RowSource for SqliteSource {
    fn columns(&self) -> Columns {
        self.columns.clone()
    }
    fn row_count(&self) -> usize {
        self.scan.rows.load(Ordering::Acquire)
    }
    fn complete(&self) -> bool {
        self.scan.done.load(Ordering::Acquire)
    }
    fn fetch(&self, offset: usize, len: usize) -> AppResult<Vec<Vec<Datum>>> {
        let conn = self.conn.lock().unwrap();
        let block = offset / BLOCK;
        let start = self.scan.rowids.read().unwrap().get(block).copied();
        let width = self.columns.len();
        let (sql, skip) = match start {
            Some(start) => (
                format!(
                    "SELECT * FROM {} WHERE rowid >= {start} ORDER BY rowid LIMIT ?1 OFFSET ?2",
                    self.table
                ),
                offset - block * BLOCK,
            ),
            None => (
                format!("SELECT * FROM {} LIMIT ?1 OFFSET ?2", self.table),
                offset,
            ),
        };
        let mut statement = conn.prepare(&sql)?;
        let rows = statement
            .query_map(params![len as i64, skip as i64], |row| {
                (0..width)
                    .map(|idx| row.get_ref(idx).map(datum))
                    .collect::<Result<Vec<_>, _>>()
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }
}

fn connect(path: &Path, key: Option<&str>) -> rusqlite::Result<Connection> {
    let conn = Connection::open_with_flags(
        path,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )?;
    if let Some(key) = key {
        conn.pragma_update(None, "key", key)?;
    }
    Ok(conn)
}

fn datum(value: ValueRef) -> Datum {
    match value {
        ValueRef::Null => None,
        ValueRef::Integer(int) => Some(int.to_string()),
        ValueRef::Real(real) => Some(real.to_string()),
        ValueRef::Text(text) => Some(String::from_utf8_lossy(text).into_owned()),
        ValueRef::Blob(blob) => Some(
            blob.iter()
                .map(|byte| match byte.is_ascii_graphic() || *byte == b' ' {
                    true => (*byte as char).to_string(),
                    false => format!("\\x{byte:02X}"),
                })
                .collect(),
        ),
    }
}

fn duckdb_type(declared: &str) -> &'static str {
    let declared = declared.to_ascii_uppercase();
    if declared.contains("INT") {
        "BIGINT"
    } else if declared.contains("REAL") || declared.contains("FLOA") || declared.contains("DOUB") {
        "DOUBLE"
    } else if declared.contains("BLOB") {
        "BLOB"
    } else {
        "VARCHAR"
    }
}

fn quote(name: &str) -> String {
    format!("\"{}\"", name.replace('"', "\"\""))
}
