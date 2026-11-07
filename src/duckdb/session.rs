use crate::{
    AppResult,
    duckdb::{
        file_db::{FileDB, FileDBMetadata},
        helper::quote,
    },
    iters::enumerate_names,
};
use anyhow::anyhow;
use bytesize::ByteSize;
use duckdb::{Connection, params};
use indexmap::IndexMap;
use std::{ops::Deref, sync::Arc};

enum Table {
    File(FileDB),
    Memory(String),
}

impl Table {
    fn alias(&self) -> String {
        match self {
            Table::File(file_db) => file_db
                .file_path()
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned(),
            Table::Memory(alias) => alias.clone(),
        }
    }
}

pub struct SessionConn {
    conn: Connection,
    _active: Arc<()>,
}

impl Deref for SessionConn {
    type Target = Connection;
    fn deref(&self) -> &Connection {
        &self.conn
    }
}

pub struct Session {
    conn: Connection,
    active: Arc<()>,
    name_map: IndexMap<String, Table>,
    pending: Vec<String>,
}

impl Session {
    pub fn new(memory_limit: Option<ByteSize>) -> AppResult<Self> {
        let conn = Connection::open_in_memory()
            .map_err(|e| anyhow!("Couldn't establish connection to duckdb instance: {}", e))?;
        if let Some(memory_limit) = memory_limit {
            if memory_limit == ByteSize::b(0) {
                return Err(anyhow!("Memory limit cannot be 0"));
            }
            conn.execute(
                "SET memory_limit = ?",
                params![format!("{}B", memory_limit.as_u64())],
            )
            .map_err(|e| anyhow!("Failed to set memory limit. {}", e))?;
        }
        let name_map = IndexMap::<String, Table>::new();
        let pending = Vec::<String>::new();
        Ok(Self {
            conn,
            name_map,
            pending,
            active: Arc::new(()),
        })
    }
    pub fn attach(
        &mut self,
        table_name: Option<&str>,
        file_db: Option<FileDB>,
    ) -> AppResult<Option<String>> {
        let stem = file_db
            .as_ref()
            .and_then(|file_db| file_db.file_path().file_stem())
            .map(|stem| stem.to_string_lossy().into_owned());
        let name = self.available_name(table_name.or(stem.as_deref()).unwrap_or("table"));
        let (table, source, read_only) = match file_db {
            Some(file_db) => {
                let source = file_db.file_path().to_string_lossy().into_owned();
                let table_complete = FileDBMetadata::is_complete(file_db.file_path());
                (Table::File(file_db), source, table_complete)
            }
            None => (
                Table::Memory(format!("mem_{name}")),
                String::from(":memory:"),
                false,
            ),
        };
        let alias = table.alias();
        if !self.name_map.values().any(|t| t.alias() == alias) {
            let options = if read_only { " (READ_ONLY)" } else { "" };
            self.conn.execute(
                &format!("ATTACH IF NOT EXISTS ? AS {}{options}", quote(&alias)),
                params![source],
            )?;
        }
        self.name_map.insert(name.clone(), table);
        Ok(Some(name))
    }
    fn available_name(&self, preferred: &str) -> String {
        enumerate_names(preferred)
            .find(|name| !self.name_map.contains_key(name) && name != "_")
            .expect("Unable to find a name")
    }
    pub fn create_view(&self, name: &str) -> AppResult<()> {
        let alias = self
            .alias(name)
            .ok_or_else(|| anyhow!("Unknown table {name}"))?;
        self.conn.execute_batch(&format!(
            "CREATE VIEW {} AS SELECT * FROM {}.data",
            quote(name),
            quote(&alias)
        ))?;
        Ok(())
    }
    pub fn alias(&self, name: &str) -> Option<String> {
        self.name_map.get(name).map(Table::alias)
    }
    pub fn detach(&mut self, name: &str) -> AppResult<()> {
        if !self.name_map.contains_key(name) {
            return Err(anyhow!("Unknown table {name}"));
        }
        if !self.pending.iter().any(|n| n == name) {
            self.pending.push(name.to_string());
        }
        Ok(())
    }

    pub fn file_db_mut(&mut self, name: &str) -> Option<&mut FileDB> {
        match self.name_map.get_mut(name)? {
            Table::File(file_db) => Some(file_db),
            Table::Memory(_) => None,
        }
    }

    pub fn session(&self) -> AppResult<SessionConn> {
        let conn = self
            .conn
            .try_clone()
            .map_err(|e| anyhow!("Couldn't clone the root connection. {}", e))?;
        Ok(SessionConn {
            conn,
            _active: self.active.clone(),
        })
    }

    pub fn flush(&mut self) -> AppResult<Vec<String>> {
        let mut detached = Vec::<String>::new();
        if Arc::strong_count(&self.active) > 1 {
            return Ok(detached);
        }
        while let Some(name) = self.pending.last().cloned() {
            self.conn
                .execute(&format!("DROP VIEW IF EXISTS {}", quote(&name)), [])?;
            self.pending.pop();
            let table = self
                .name_map
                .shift_remove(&name)
                .expect("pending names are in name_map");
            let alias = table.alias();
            if !self.name_map.values().any(|t| t.alias() == alias) {
                self.conn
                    .execute(&format!("DETACH {}", quote(&alias)), [])?;
                detached.push(alias);
            }
            drop(table);
        }
        Ok(detached)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::duckdb::{file_db::FileDBStatus, ingester::Ingester};
    use std::path::Path;
    use tempfile::TempDir;

    fn attach_file(session: &mut Session, name: &str, db: &Path) {
        let file_db = FileDB::new(db).unwrap();
        session.attach(Some(name), Some(file_db)).unwrap();
    }

    fn is_locked(db: &Path) -> bool {
        FileDB::new(db).is_err() || FileDB::new(db).unwrap().get_write_lock().is_err()
    }

    fn count(session: &Session, table: &str) -> i64 {
        session
            .session()
            .unwrap()
            .query_row(&format!("SELECT count(*) FROM {}", quote(table)), [], |r| {
                r.get(0)
            })
            .unwrap()
    }

    fn ingest(session: &mut Session, name: &str) {
        let alias = session.alias(name).unwrap();
        let conn = session.session().unwrap();
        Ingester::create_placeholder(&conn, &alias).unwrap();
        drop(conn);
        session.create_view(name).unwrap();
        assert_eq!(count(session, name), 0);
        session
            .session()
            .unwrap()
            .execute_batch(&format!(
                "CREATE OR REPLACE TABLE {}.data AS SELECT range AS id FROM range(10)",
                quote(&alias)
            ))
            .unwrap();
        assert_eq!(count(session, name), 10);
    }

    #[test]
    fn view_serves_the_placeholder_then_the_ingested_data() {
        let dir = TempDir::new().unwrap();
        let db = dir.path().join("abc.duckdb");
        let mut session = Session::new(None).unwrap();
        attach_file(&mut session, "file", &db);
        ingest(&mut session, "file");
        session.attach(Some("mem"), None).unwrap();
        ingest(&mut session, "mem");
        let joined: i64 = session
            .session()
            .unwrap()
            .query_row("SELECT count(*) FROM file JOIN mem USING (id)", [], |r| {
                r.get(0)
            })
            .unwrap();
        assert_eq!(joined, 10);
    }

    #[test]
    fn create_view_fails_until_the_ingester_made_the_table() {
        let dir = TempDir::new().unwrap();
        let db = dir.path().join("abc.duckdb");
        let mut session = Session::new(None).unwrap();
        attach_file(&mut session, "one", &db);
        assert!(session.create_view("one").is_err());
        assert!(session.create_view("nope").is_err());
    }

    #[test]
    fn file_lock_lives_until_the_last_table_is_flushed() {
        let dir = TempDir::new().unwrap();
        let db = dir.path().join("abc.duckdb");
        let mut session = Session::new(None).unwrap();
        attach_file(&mut session, "one", &db);
        attach_file(&mut session, "two", &db);
        assert!(is_locked(&db));
        session.detach("one").unwrap();
        session.flush().unwrap();
        assert!(is_locked(&db));
        let active = session.session().unwrap();
        session.detach("two").unwrap();
        assert_eq!(session.flush().unwrap(), Vec::<String>::new());
        assert!(is_locked(&db));
        drop(active);
        assert_eq!(session.flush().unwrap(), ["abc"]);
        assert!(!is_locked(&db));
    }

    #[test]
    fn taken_and_missing_names_get_a_free_one() {
        let dir = TempDir::new().unwrap();
        let db = dir.path().join("abc.duckdb");
        let mut session = Session::new(None).unwrap();
        let memory = |session: &mut Session, name| session.attach(name, None).unwrap().unwrap();
        assert_eq!(memory(&mut session, None), "table");
        assert_eq!(memory(&mut session, None), "table_2");
        assert_eq!(memory(&mut session, Some("sales")), "sales");
        assert_eq!(memory(&mut session, Some("sales")), "sales_2");
        assert_eq!(memory(&mut session, Some("_")), "__2");
        let attach = |session: &mut Session| {
            let file_db = FileDB::new(&db).unwrap();
            session.attach(None, Some(file_db)).unwrap().unwrap()
        };
        assert_eq!(attach(&mut session), "abc");
        assert_eq!(attach(&mut session), "abc_2");
        assert_eq!(session.alias("abc_2").as_deref(), Some("abc"));
        assert!(is_locked(&db));
    }

    #[test]
    fn second_name_on_a_file_blocks_the_write_lock_until_flushed() {
        let dir = TempDir::new().unwrap();
        let db = dir.path().join("abc.duckdb");
        let mut session = Session::new(None).unwrap();
        attach_file(&mut session, "one", &db);
        attach_file(&mut session, "two", &db);
        assert!(
            session
                .file_db_mut("one")
                .unwrap()
                .get_write_lock()
                .is_err()
        );
        session.detach("two").unwrap();
        session.flush().unwrap();
        session
            .file_db_mut("one")
            .unwrap()
            .get_write_lock()
            .unwrap();
    }

    #[test]
    fn complete_db_attaches_read_only() {
        let dir = TempDir::new().unwrap();
        let db = dir.path().join("abc.duckdb");
        let conn = Connection::open(&db).unwrap();
        conn.execute_batch("CREATE TABLE data (id INT)").unwrap();
        drop(conn);
        let mut meta = FileDBMetadata::new(Path::new("/data/sales.csv"));
        meta.status = FileDBStatus::Complete;
        meta.write(&db).unwrap();
        let mut session = Session::new(None).unwrap();
        attach_file(&mut session, "one", &db);
        let write = session.session().unwrap().execute_batch("DROP TABLE abc.data");
        assert!(write.is_err());
    }

    #[test]
    fn reopening_a_file_imported_in_this_session_reuses_the_attach() {
        let dir = TempDir::new().unwrap();
        let db = dir.path().join("abc.duckdb");
        let mut session = Session::new(None).unwrap();
        attach_file(&mut session, "one", &db);
        ingest(&mut session, "one");
        let mut meta = FileDBMetadata::new(Path::new("/data/sales.csv"));
        meta.status = FileDBStatus::Complete;
        meta.write(&db).unwrap();
        attach_file(&mut session, "two", &db);
        session.create_view("two").unwrap();
        assert_eq!(count(&session, "two"), 10);
    }

    #[test]
    fn file_db_mut_finds_the_lock_by_table_name() {
        let dir = TempDir::new().unwrap();
        let db = dir.path().join("abc.duckdb");
        let mut session = Session::new(None).unwrap();
        attach_file(&mut session, "one", &db);
        session
            .file_db_mut("one")
            .unwrap()
            .get_write_lock()
            .unwrap();
        assert!(FileDB::new(&db).is_err());
        session.file_db_mut("one").unwrap().get_read_lock().unwrap();
        assert!(FileDB::new(&db).is_ok());
        assert!(session.file_db_mut("nope").is_none());
    }
}
