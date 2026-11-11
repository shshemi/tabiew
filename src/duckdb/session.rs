use crate::{
    AppResult,
    duckdb::{
        cache::file_db::{ColumnStats, FileDB, FileDBMetadata},
        sql::quote,
    },
    iters::enumerate_names,
};
use anyhow::anyhow;
use bytesize::ByteSize;
use duckdb::{Connection, params, vtab::arrow::ArrowVTab};
use indexmap::IndexMap;
use std::{
    collections::HashMap,
    ops::Deref,
    path::{Path, PathBuf},
    sync::Arc,
};

enum Table {
    View(Vec<String>),
    Query,
}

struct Attached {
    file_db: Option<FileDB>,
    pinned: bool,
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
    attached: IndexMap<String, Attached>,
    name_map: IndexMap<String, Table>,
    whole: HashMap<String, (String, String)>,
    pending: Vec<String>,
}

impl Session {
    pub fn new(memory_limit: Option<ByteSize>) -> AppResult<Self> {
        let conn = Connection::open_in_memory()
            .map_err(|e| anyhow!("Couldn't establish connection to duckdb instance: {}", e))?;
        conn.execute_batch(
            "SET autoinstall_known_extensions = false; SET allow_community_extensions = false;",
        )?;
        conn.register_table_function::<ArrowVTab>("arrow_batch")?;
        conn.register_scalar_function::<super::skim::SkimScore>(super::skim::NAME)?;
        conn.register_scalar_function::<super::skim::FloatText>(super::skim::FLOAT_TEXT)?;
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
        Ok(Self {
            conn,
            active: Arc::new(()),
            attached: IndexMap::new(),
            name_map: IndexMap::new(),
            whole: HashMap::new(),
            pending: Vec::new(),
        })
    }
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.name_map
            .keys()
            .filter(|name| !self.pending.contains(name))
            .map(String::as_str)
    }
    fn whole_table(&self, name: &str) -> Option<(&FileDB, &str)> {
        let (alias, table) = self.whole.get(name)?;
        Some((self.attached.get(alias)?.file_db.as_ref()?, table))
    }
    pub fn stored_stats(&self, name: &str) -> Option<Vec<ColumnStats>> {
        let (file_db, table) = self.whole_table(name)?;
        file_db.stats(table)
    }
    pub fn store_stats(&self, name: &str, stats: &[ColumnStats]) {
        if let Some((file_db, table)) = self.whole_table(name) {
            let _ = file_db.store_stats(table, stats);
        }
    }
    pub fn stored_size(&self, name: &str) -> Option<u64> {
        let (alias, table) = self.whole.get(name)?;
        let blocks = self
            .conn
            .query_row(
                "SELECT count(DISTINCT block_id) * (SELECT block_size FROM pragma_database_size() WHERE database_name = ?) FROM pragma_storage_info(?) WHERE persistent",
                params![alias, format!("{}.{}", quote(alias), quote(table))],
                |r| r.get::<_, Option<i64>>(0),
            )
            .ok()
            .flatten()
            .filter(|bytes| *bytes > 0);
        match blocks {
            Some(bytes) => Some(bytes as u64),
            None => {
                let path = self.whole_table(name)?.0.file_path().to_path_buf();
                Some(
                    [path.clone(), path.with_extension("duckdb.wal")]
                        .iter()
                        .filter_map(|path| std::fs::metadata(path).ok())
                        .map(|metadata| metadata.len())
                        .sum(),
                )
            }
        }
    }
    pub fn source_path(&self, name: &str) -> Option<PathBuf> {
        let Table::View(aliases) = self.name_map.get(name)? else {
            return None;
        };
        self.attached
            .get(aliases.first()?)?
            .file_db
            .as_ref()?
            .source_path()
    }
    pub fn execute_batch(&self, sql: &str) -> AppResult<()> {
        Ok(self.conn.execute_batch(sql)?)
    }
    pub fn set_temp_dir(&self, dir: &Path) -> AppResult<()> {
        self.conn
            .execute("SET temp_directory = ?", params![dir.to_string_lossy()])
            .map_err(|e| anyhow!("Failed to set the temp directory. {}", e))?;
        Ok(())
    }
    pub fn attach(&mut self, file_db: Option<FileDB>) -> AppResult<String> {
        let alias = match &file_db {
            Some(file_db) => file_db
                .file_path()
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned(),
            None => enumerate_names("mem")
                .find(|alias| !self.attached.contains_key(alias))
                .expect("Unable to find an alias"),
        };
        if let Some(attached) = self.attached.get_mut(&alias) {
            attached.pinned = true;
            return Ok(alias);
        }
        let (source, read_only) = match &file_db {
            Some(file_db) => (
                file_db.file_path().to_string_lossy().into_owned(),
                FileDBMetadata::is_complete(file_db.file_path()),
            ),
            None => (String::from(":memory:"), false),
        };
        let options = if read_only { " (READ_ONLY)" } else { "" };
        self.conn.execute(
            &format!("ATTACH IF NOT EXISTS ? AS {}{options}", quote(&alias)),
            params![source],
        )?;
        self.attached.insert(
            alias.clone(),
            Attached {
                file_db,
                pinned: true,
            },
        );
        Ok(alias)
    }
    pub fn unpin(&mut self, alias: &str) {
        if let Some(attached) = self.attached.get_mut(alias) {
            attached.pinned = false;
        }
    }
    pub fn tables(&self, alias: &str) -> AppResult<Vec<String>> {
        let tables = self
            .conn
            .prepare(
                "SELECT table_name FROM duckdb_tables() WHERE database_name = ? ORDER BY table_name",
            )?
            .query_map(params![alias], |r| r.get(0))?
            .collect::<Result<Vec<String>, _>>()?;
        Ok(tables)
    }
    fn available_name(&self, preferred: &str) -> String {
        enumerate_names(preferred)
            .find(|name| !self.name_map.contains_key(name) && name != "_")
            .expect("Unable to find a name")
    }
    pub fn create_view(
        &mut self,
        name: &str,
        alias: &str,
        table: &str,
        limit: Option<usize>,
    ) -> AppResult<String> {
        if !self.attached.contains_key(alias) {
            return Err(anyhow!("Unknown alias {alias}"));
        }
        let name = self.available_name(name);
        self.conn.execute_batch(&format!(
            "CREATE VIEW {} AS SELECT * FROM {}.{}{}",
            quote(&name),
            quote(alias),
            quote(table),
            limit.map(|n| format!(" LIMIT {n}")).unwrap_or_default()
        ))?;
        self.name_map
            .insert(name.clone(), Table::View(vec![alias.to_string()]));
        if limit.is_none() {
            self.whole
                .insert(name.clone(), (alias.to_string(), table.to_string()));
        }
        Ok(name)
    }
    pub fn create_union(
        &mut self,
        name: &str,
        members: &[(String, String)],
        limit: Option<usize>,
    ) -> AppResult<String> {
        if let Some((alias, _)) = members
            .iter()
            .find(|(alias, _)| !self.attached.contains_key(alias))
        {
            return Err(anyhow!("Unknown alias {alias}"));
        }
        let name = self.available_name(name);
        let union = members
            .iter()
            .map(|(alias, table)| format!("SELECT * FROM {}.{}", quote(alias), quote(table)))
            .collect::<Vec<_>>();
        self.conn.execute_batch(&format!(
            "CREATE VIEW {} AS SELECT * FROM ({}){}",
            quote(&name),
            union.join(" UNION ALL "),
            limit.map(|n| format!(" LIMIT {n}")).unwrap_or_default()
        ))?;
        let mut aliases = members
            .iter()
            .map(|(alias, _)| alias.clone())
            .collect::<Vec<_>>();
        aliases.dedup();
        self.name_map.insert(name.clone(), Table::View(aliases));
        Ok(name)
    }
    pub fn columns(&self, alias: &str, table: &str) -> AppResult<Vec<(String, String)>> {
        Ok(self
            .conn
            .prepare(
                "SELECT column_name, data_type FROM duckdb_columns() WHERE database_name = ? AND table_name = ? ORDER BY column_index",
            )?
            .query_map(params![alias, table], |r| Ok((r.get(0)?, r.get(1)?)))?
            .collect::<Result<Vec<_>, _>>()?)
    }
    #[cfg(test)]
    pub fn aliases(&self, name: &str) -> Option<&[String]> {
        match self.name_map.get(name)? {
            Table::View(aliases) => Some(aliases),
            Table::Query => None,
        }
    }
    pub fn create_query(&mut self, name: &str, setup: &[String], sql: &str) -> AppResult<String> {
        let name = self.available_name(name);
        if !setup.is_empty() {
            self.conn.execute_batch(&setup.join(";\n"))?;
        }
        self.conn.execute_batch(&format!(
            "CREATE VIEW {} AS {}",
            quote(&name),
            sql.trim().trim_end_matches(';')
        ))?;
        self.name_map.insert(name.clone(), Table::Query);
        Ok(name)
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

    pub fn file_db_mut(&mut self, alias: &str) -> Option<&mut FileDB> {
        self.attached.get_mut(alias)?.file_db.as_mut()
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
        if Arc::strong_count(&self.active) > 1 {
            return Ok(Vec::new());
        }
        while let Some(name) = self.pending.last().cloned() {
            self.conn
                .execute(&format!("DROP VIEW IF EXISTS {}", quote(&name)), [])?;
            self.pending.pop();
            self.name_map.shift_remove(&name);
            self.whole.remove(&name);
        }
        let unused = self
            .attached
            .iter()
            .filter(|(alias, attached)| {
                !attached.pinned
                    && !self
                        .name_map
                        .values()
                        .any(|t| matches!(t, Table::View(aliases) if aliases.contains(alias)))
            })
            .map(|(alias, _)| alias.clone())
            .collect::<Vec<_>>();
        for alias in &unused {
            self.conn.execute(&format!("DETACH {}", quote(alias)), [])?;
            self.attached.shift_remove(alias);
        }
        Ok(unused)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::duckdb::cache::file_db::FileDBStatus;
    use tempfile::TempDir;

    fn attach_file(session: &mut Session, db: &Path) -> String {
        session.attach(Some(FileDB::new(db).unwrap())).unwrap()
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

    fn fill(session: &Session, alias: &str, table: &str) {
        session
            .session()
            .unwrap()
            .execute_batch(&format!(
                "CREATE TABLE {}.{} AS SELECT range AS id FROM range(10)",
                quote(alias),
                quote(table)
            ))
            .unwrap();
    }

    fn complete(db: &Path) {
        let mut meta = FileDBMetadata::new(Path::new("/data/sales.csv"));
        meta.status = FileDBStatus::Complete;
        meta.write(db).unwrap();
    }

    #[test]
    fn views_over_file_and_memory_tables_join() {
        let dir = TempDir::new().unwrap();
        let mut session = Session::new(None).unwrap();
        let file = attach_file(&mut session, &dir.path().join("abc.duckdb"));
        fill(&session, &file, "data");
        session.create_view("file", &file, "data", None).unwrap();
        let memory = session.attach(None).unwrap();
        assert_eq!(memory, "mem");
        assert_eq!(session.attach(None).unwrap(), "mem_2");
        fill(&session, &memory, "data");
        session
            .create_view("mem", &memory, "data", Some(4))
            .unwrap();
        assert_eq!(count(&session, "mem"), 4);
        let joined: i64 = session
            .session()
            .unwrap()
            .query_row("SELECT count(*) FROM file JOIN mem USING (id)", [], |r| {
                r.get(0)
            })
            .unwrap();
        assert_eq!(joined, 4);
    }

    #[test]
    fn create_view_needs_the_alias_and_the_table() {
        let dir = TempDir::new().unwrap();
        let mut session = Session::new(None).unwrap();
        let alias = attach_file(&mut session, &dir.path().join("abc.duckdb"));
        assert!(session.create_view("one", &alias, "data", None).is_err());
        assert!(session.create_view("one", "nope", "data", None).is_err());
        assert!(session.aliases("one").is_none());
    }

    #[test]
    fn tables_are_listed_by_name() {
        let mut session = Session::new(None).unwrap();
        let alias = session.attach(None).unwrap();
        fill(&session, &alias, "users");
        fill(&session, &alias, "orders");
        assert_eq!(session.tables(&alias).unwrap(), ["orders", "users"]);
        assert!(session.tables("nope").unwrap().is_empty());
    }

    #[test]
    fn view_names_get_a_free_one() {
        let mut session = Session::new(None).unwrap();
        let alias = session.attach(None).unwrap();
        fill(&session, &alias, "data");
        let view =
            |session: &mut Session, name| session.create_view(name, &alias, "data", None).unwrap();
        assert_eq!(view(&mut session, "sales"), "sales");
        assert_eq!(view(&mut session, "sales"), "sales_2");
        assert_eq!(view(&mut session, "_"), "__2");
        assert_eq!(session.aliases("sales_2"), Some([alias.clone()].as_slice()));
    }

    #[test]
    fn file_lock_lives_until_the_last_view_is_flushed() {
        let dir = TempDir::new().unwrap();
        let db = dir.path().join("abc.duckdb");
        let mut session = Session::new(None).unwrap();
        let alias = attach_file(&mut session, &db);
        fill(&session, &alias, "data");
        let one = session.create_view("t", &alias, "data", None).unwrap();
        let two = session.create_view("t", &alias, "data", None).unwrap();
        session.unpin(&alias);
        assert!(is_locked(&db));
        session.detach(&one).unwrap();
        assert!(session.flush().unwrap().is_empty());
        assert!(is_locked(&db));
        let active = session.session().unwrap();
        session.detach(&two).unwrap();
        assert!(session.flush().unwrap().is_empty());
        assert!(is_locked(&db));
        drop(active);
        assert_eq!(session.flush().unwrap(), ["abc"]);
        assert!(!is_locked(&db));
        assert!(session.file_db_mut(&alias).is_none());
    }

    #[test]
    fn pinned_attach_without_views_survives_until_unpinned() {
        let dir = TempDir::new().unwrap();
        let db = dir.path().join("abc.duckdb");
        let mut session = Session::new(None).unwrap();
        let alias = attach_file(&mut session, &db);
        assert!(session.flush().unwrap().is_empty());
        assert!(is_locked(&db));
        session.unpin(&alias);
        assert_eq!(session.flush().unwrap(), ["abc"]);
        assert!(!is_locked(&db));
    }

    #[test]
    fn attaching_a_file_again_reuses_the_attach_and_its_lock() {
        let dir = TempDir::new().unwrap();
        let db = dir.path().join("abc.duckdb");
        let mut session = Session::new(None).unwrap();
        let alias = attach_file(&mut session, &db);
        fill(&session, &alias, "data");
        session.create_view("one", &alias, "data", None).unwrap();
        session.unpin(&alias);
        complete(&db);
        assert_eq!(attach_file(&mut session, &db), alias);
        session.create_view("two", &alias, "data", None).unwrap();
        assert_eq!(count(&session, "two"), 10);
        session
            .file_db_mut(&alias)
            .unwrap()
            .get_write_lock()
            .unwrap();
        assert!(FileDB::new(&db).is_err());
        session
            .file_db_mut(&alias)
            .unwrap()
            .get_read_lock()
            .unwrap();
        assert!(FileDB::new(&db).is_ok());
    }

    #[test]
    fn complete_db_attaches_read_only() {
        let dir = TempDir::new().unwrap();
        let db = dir.path().join("abc.duckdb");
        let conn = Connection::open(&db).unwrap();
        conn.execute_batch("CREATE TABLE data (id INT)").unwrap();
        drop(conn);
        complete(&db);
        let mut session = Session::new(None).unwrap();
        attach_file(&mut session, &db);
        let write = session
            .session()
            .unwrap()
            .execute_batch("DROP TABLE abc.data");
        assert!(write.is_err());
    }

    #[test]
    fn query_views_run_setup_take_free_names_and_detach_without_detach() {
        let mut session = Session::new(None).unwrap();
        let setup = ["CREATE TABLE t AS SELECT range AS id FROM range(3)".to_string()];
        let name = session
            .create_query("top", &setup, "SELECT * FROM t;")
            .unwrap();
        assert_eq!(name, "top");
        assert_eq!(count(&session, "top"), 3);
        let name = session.create_query("top", &[], "SELECT 1").unwrap();
        assert_eq!(name, "top_2");
        assert!(session.aliases("top").is_none());
        assert!(session.create_query("bad", &[], "SELEC 1").is_err());
        session.detach("top").unwrap();
        assert_eq!(session.flush().unwrap(), Vec::<String>::new());
        let gone = session
            .session()
            .unwrap()
            .execute_batch("SELECT * FROM top");
        assert!(gone.is_err());
        assert_eq!(count(&session, "top_2"), 1);
    }
}
