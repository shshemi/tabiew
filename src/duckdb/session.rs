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

impl SessionConn {
    pub fn try_clone(&self) -> duckdb::Result<Self> {
        Ok(Self {
            conn: self.conn.try_clone()?,
            _active: self._active.clone(),
        })
    }
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
    pub fn memory_limit(&self) -> Option<u64> {
        self.conn
            .query_row("SELECT current_setting('memory_limit')", [], |r| {
                r.get::<_, String>(0)
            })
            .ok()?
            .parse::<ByteSize>()
            .ok()
            .map(|limit| limit.as_u64())
    }
    pub fn in_memory(&self, name: &str) -> bool {
        matches!(self.name_map.get(name), Some(Table::View(aliases)) if aliases.iter().all(|alias| {
            self.attached
                .get(alias)
                .is_some_and(|attached| attached.file_db.is_none())
        }))
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
    use tempfile::TempDir;

    fn attach_file(session: &mut Session, db: &Path) -> String {
        session.attach(Some(FileDB::new(db).unwrap())).unwrap()
    }

    fn is_locked(db: &Path) -> bool {
        FileDB::new(db).is_err() || FileDB::new(db).unwrap().get_write_lock().is_err()
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
}
