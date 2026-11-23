use std::collections::HashMap;

use anyhow::anyhow;

use crate::{
    AppResult,
    sql::{
        db::{Database, Row, ValueRef},
        ops::reply::{Receiver, channel},
        worker::submit,
    },
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DbSchema {
    tables: Vec<Table>,
}

impl DbSchema {
    pub fn new(tables: Vec<Table>) -> Self {
        Self { tables }
    }

    pub fn get(&self, idx: usize) -> Option<&Table> {
        self.tables.get(idx)
    }

    pub fn find(&self, name: &str) -> Option<&Table> {
        self.tables.iter().find(|table| table.name() == name)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Table> {
        self.tables.iter()
    }

    pub fn len(&self) -> usize {
        self.tables.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tables.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    database: String,
    schema_name: String,
    name: String,
    temporary: bool,
    estimated_rows: u64,
    columns: Vec<Column>,
}

impl Table {
    pub fn database(&self) -> &str {
        &self.database
    }

    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn temporary(&self) -> bool {
        self.temporary
    }

    pub fn estimated_rows(&self) -> u64 {
        self.estimated_rows
    }

    pub fn columns(&self) -> &[Column] {
        &self.columns
    }

    pub fn column(&self, name: &str) -> Option<&Column> {
        self.columns.iter().find(|column| column.name() == name)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Column {
    name: String,
    dtype: String,
    nullable: bool,
    default: Option<String>,
}

impl Column {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn dtype(&self) -> &str {
        &self.dtype
    }

    pub fn nullable(&self) -> bool {
        self.nullable
    }

    pub fn default(&self) -> Option<&str> {
        self.default.as_deref()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Fetcher;

impl Fetcher {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self) -> Receiver<DbSchema> {
        let this = self.clone();
        let (send, recv) = channel();
        submit(move |db| send.send(this.fetch(db)));
        recv
    }

    fn fetch(&self, db: &Database) -> AppResult<DbSchema> {
        let mut tables = self.tables(db)?;
        let positions = tables
            .iter()
            .enumerate()
            .map(|(idx, (oid, _))| (*oid, idx))
            .collect::<HashMap<_, _>>();
        let mut stmt = db.prepare(
            "SELECT table_oid, column_name, data_type, is_nullable, column_default \
             FROM duckdb_columns() WHERE NOT internal \
             ORDER BY table_oid, column_index",
        )?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            if let Some(idx) = positions.get(&int(&row, 0)?) {
                tables[*idx].1.columns.push(Column {
                    name: required_text(&row, 1)?,
                    dtype: required_text(&row, 2)?,
                    nullable: boolean(&row, 3)?,
                    default: text(&row, 4)?,
                });
            }
        }
        Ok(DbSchema::new(
            tables.into_iter().map(|(_, table)| table).collect(),
        ))
    }

    fn tables(&self, db: &Database) -> AppResult<Vec<(i64, Table)>> {
        let mut stmt = db.prepare(
            "SELECT table_oid, database_name, schema_name, table_name, temporary, estimated_size \
             FROM duckdb_tables() WHERE NOT internal \
             ORDER BY database_name, schema_name, table_name",
        )?;
        let mut rows = stmt.query([])?;
        let mut tables = Vec::new();
        while let Some(row) = rows.next()? {
            tables.push((
                int(&row, 0)?,
                Table {
                    database: required_text(&row, 1)?,
                    schema_name: required_text(&row, 2)?,
                    name: required_text(&row, 3)?,
                    temporary: boolean(&row, 4)?,
                    estimated_rows: u64::try_from(int(&row, 5)?)?,
                    columns: Vec::new(),
                },
            ));
        }
        Ok(tables)
    }
}

fn text(row: &Row, idx: usize) -> AppResult<Option<String>> {
    match row.get_ref(idx)? {
        ValueRef::Text(bytes) => Ok(Some(String::from_utf8_lossy(bytes).into_owned())),
        ValueRef::Null => Ok(None),
        other => Err(anyhow!(
            "expected text in catalog column {idx}, got {other:?}"
        )),
    }
}

fn required_text(row: &Row, idx: usize) -> AppResult<String> {
    text(row, idx)?.ok_or_else(|| anyhow!("catalog column {idx} is null"))
}

fn int(row: &Row, idx: usize) -> AppResult<i64> {
    match row.get_ref(idx)? {
        ValueRef::BigInt(value) => Ok(value),
        ValueRef::Int(value) => Ok(value.into()),
        other => Err(anyhow!(
            "expected an integer in catalog column {idx}, got {other:?}"
        )),
    }
}

fn boolean(row: &Row, idx: usize) -> AppResult<bool> {
    match row.get_ref(idx)? {
        ValueRef::Boolean(value) => Ok(value),
        other => Err(anyhow!(
            "expected a boolean in catalog column {idx}, got {other:?}"
        )),
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::*;
    use crate::sql::{db::DatabaseConfig, ops::Next};

    fn db() -> Database {
        let db = DatabaseConfig::default().open_in_memory().unwrap();
        db.execute(
            "CREATE TABLE users (id INTEGER NOT NULL, name VARCHAR DEFAULT 'anon', tags VARCHAR[])",
            [],
        )
        .unwrap();
        db.execute("INSERT INTO users VALUES (1, 'a', []), (2, 'b', [])", [])
            .unwrap();
        db.execute("CREATE TABLE \"a table\" (\"x y\" DOUBLE)", [])
            .unwrap();
        db.execute("CREATE VIEW v AS SELECT id FROM users", [])
            .unwrap();
        db
    }

    fn wait<T>(recv: &Receiver<T>) -> AppResult<T> {
        let deadline = Instant::now() + Duration::from_secs(30);
        loop {
            match recv.recv() {
                Next::Value(value) => return value,
                Next::Done => panic!("sender dropped without sending a value"),
                Next::Empty if Instant::now() > deadline => panic!("timed out waiting for a value"),
                Next::Empty => std::thread::sleep(Duration::from_millis(5)),
            }
        }
    }

    #[test]
    fn tables_are_listed_by_name_without_views() {
        let schema = Fetcher::new().fetch(&db()).unwrap();
        assert_eq!(
            schema.iter().map(Table::name).collect::<Vec<_>>(),
            ["a table", "users"]
        );
        assert!(schema.find("v").is_none());
    }

    #[test]
    fn table_info_comes_from_the_catalog() {
        let schema = Fetcher::new().fetch(&db()).unwrap();
        let users = schema.find("users").unwrap();
        assert_eq!(users.database(), "memory");
        assert_eq!(users.schema_name(), "main");
        assert!(!users.temporary());
        assert_eq!(users.estimated_rows(), 2);
    }

    #[test]
    fn columns_keep_their_order_and_details() {
        let schema = Fetcher::new().fetch(&db()).unwrap();
        let users = schema.find("users").unwrap();
        assert_eq!(
            users.columns().iter().map(Column::name).collect::<Vec<_>>(),
            ["id", "name", "tags"]
        );
        let id = users.column("id").unwrap();
        assert_eq!(id.dtype(), "INTEGER");
        assert!(!id.nullable());
        assert_eq!(id.default(), None);
        let name = users.column("name").unwrap();
        assert!(name.nullable());
        assert_eq!(name.default(), Some("'anon'"));
        assert_eq!(users.column("tags").unwrap().dtype(), "VARCHAR[]");
    }

    #[test]
    fn quoted_names_are_kept() {
        let schema = Fetcher::new().fetch(&db()).unwrap();
        let table = schema.find("a table").unwrap();
        assert_eq!(table.column("x y").unwrap().dtype(), "DOUBLE");
    }

    #[test]
    fn temporary_tables_are_flagged() {
        let db = db();
        db.execute("CREATE TEMP TABLE scratch (n INTEGER)", [])
            .unwrap();
        let schema = Fetcher::new().fetch(&db).unwrap();
        let scratch = schema.find("scratch").unwrap();
        assert!(scratch.temporary());
        assert_eq!(scratch.database(), "temp");
    }

    #[test]
    fn empty_database_has_no_tables() {
        let db = DatabaseConfig::default().open_in_memory().unwrap();
        assert!(Fetcher::new().fetch(&db).unwrap().is_empty());
    }

    #[test]
    fn run_returns_the_schema_through_the_worker() {
        let (send, recv) = channel();
        submit(move |db| {
            send.send(db.execute("CREATE OR REPLACE TABLE db_schema_run (n INTEGER)", []))
        });
        wait(&recv).unwrap();
        let schema = wait(&Fetcher::new().run()).unwrap();
        let table = schema.find("db_schema_run").unwrap();
        assert_eq!(table.column("n").unwrap().dtype(), "INTEGER");
    }
}
