use crate::AppResult;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::BTreeMap,
    fs::{File, TryLockError},
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileDBStatus {
    Complete,
    Incomplete,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ColumnStats {
    pub nulls: u64,
    pub min: Option<String>,
    pub max: Option<String>,
    #[serde(default)]
    pub bytes: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileDBMetadata {
    pub file_path: PathBuf,
    pub file_name: String,
    pub status: FileDBStatus,
    #[serde(default)]
    pub options: Value,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub stats: BTreeMap<String, Vec<ColumnStats>>,
}

impl FileDBMetadata {
    pub fn new(file_path: &Path) -> Self {
        Self {
            file_path: file_path.to_path_buf(),
            file_name: file_path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_default(),
            status: FileDBStatus::Incomplete,
            options: Value::Null,
            stats: BTreeMap::new(),
        }
    }

    pub fn path(db_path: &Path) -> PathBuf {
        db_path.with_extension("duckdb.meta.json")
    }

    pub fn read(db_path: &Path) -> AppResult<Option<Self>> {
        let path = Self::path(db_path);
        match std::fs::read(&path) {
            Ok(bytes) => Ok(Some(serde_json::from_slice(&bytes).map_err(|e| {
                anyhow!("Failed to parse cache metadata {:?}: {}", path, e)
            })?)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(anyhow!("Failed to read cache metadata {:?}: {}", path, e)),
        }
    }

    pub fn write(&self, db_path: &Path) -> AppResult<()> {
        let path = Self::path(db_path);
        let tmp_path = path.with_extension("json.tmp");
        std::fs::write(&tmp_path, serde_json::to_vec_pretty(self)?)
            .map_err(|e| anyhow!("Failed to write cache metadata {:?}: {}", tmp_path, e))?;
        std::fs::rename(&tmp_path, &path)
            .map_err(|e| anyhow!("Failed to write cache metadata {:?}: {}", path, e))
    }

    pub fn is_complete(db_path: &Path) -> bool {
        matches!(Self::read(db_path), Ok(Some(meta)) if meta.status == FileDBStatus::Complete)
    }

    pub fn is_complete_for(db_path: &Path, options: &Value) -> bool {
        matches!(Self::read(db_path), Ok(Some(meta)) if meta.status == FileDBStatus::Complete && meta.options == *options)
    }
}

#[derive(Debug)]
pub struct FileDB {
    file_path: PathBuf,
    file_lock: File,
}

impl FileDB {
    pub fn new(file_path: &Path) -> AppResult<Self> {
        let file_lock = open_lock(file_path)?;
        if !try_read_lock(&file_lock)? {
            return Err(anyhow!("File {:?} is in use", file_path));
        }
        Ok(Self {
            file_path: file_path.to_path_buf(),
            file_lock,
        })
    }

    pub fn create(file_path: &Path, source_path: &Path, options: &Value) -> AppResult<Self> {
        let mut file_db = Self::new(file_path)?;
        file_db.get_write_lock()?;
        let mut metadata = FileDBMetadata::new(source_path);
        metadata.options = options.clone();
        metadata.write(file_path)?;
        Ok(file_db)
    }

    pub fn file_path(&self) -> &Path {
        &self.file_path
    }

    pub fn get_write_lock(&mut self) -> AppResult<()> {
        self.file_lock.unlock()?;
        if try_write_lock(&self.file_lock)? {
            return Ok(());
        }
        if !try_read_lock(&self.file_lock)? {
            return Err(anyhow!("Lost the lock on {:?}", self.file_path));
        }
        Err(anyhow!("File {:?} is in use", self.file_path))
    }

    pub fn get_read_lock(&mut self) -> AppResult<()> {
        self.file_lock.unlock()?;
        if !try_read_lock(&self.file_lock)? {
            return Err(anyhow!("Lost the lock on {:?}", self.file_path));
        }
        if !std::fs::exists(&self.file_path)? {
            return Err(anyhow!("File {:?} was deleted", self.file_path));
        }
        Ok(())
    }

    pub fn mark_complete(&mut self) -> AppResult<()> {
        let mut metadata = FileDBMetadata::read(&self.file_path)?
            .ok_or_else(|| anyhow!("No metadata for {:?}", self.file_path))?;
        metadata.status = FileDBStatus::Complete;
        metadata.write(&self.file_path)?;
        self.get_read_lock()
    }

    pub fn stats(&self, table: &str) -> Option<Vec<ColumnStats>> {
        FileDBMetadata::read(&self.file_path)
            .ok()??
            .stats
            .remove(table)
    }

    pub fn store_stats(&self, table: &str, stats: &[ColumnStats]) -> AppResult<()> {
        let mut metadata = FileDBMetadata::read(&self.file_path)?
            .ok_or_else(|| anyhow!("No metadata for {:?}", self.file_path))?;
        metadata.stats.insert(table.to_string(), stats.to_vec());
        metadata.write(&self.file_path)
    }

    pub fn source_path(&self) -> Option<PathBuf> {
        Some(FileDBMetadata::read(&self.file_path).ok()??.file_path)
    }

    pub fn delete(mut self) -> AppResult<()> {
        self.get_write_lock()?;
        remove_if_exists(&self.file_path)?;
        remove_if_exists(&self.file_path.with_extension("duckdb.wal"))?;
        remove_if_exists(&FileDBMetadata::path(&self.file_path))
    }
}

impl Drop for FileDB {
    fn drop(&mut self) {
        let _ = self.file_lock.unlock();
    }
}

fn remove_if_exists(path: &Path) -> AppResult<()> {
    match std::fs::remove_file(path) {
        Err(e) if e.kind() != std::io::ErrorKind::NotFound => {
            Err(anyhow!("Failed to delete file {:?}: {}", path, e))
        }
        _ => Ok(()),
    }
}

fn open_lock(file_path: &Path) -> AppResult<File> {
    File::options()
        .write(true)
        .create(true)
        .truncate(false)
        .open(file_path.with_extension("duckdb.lock"))
        .map_err(|e| anyhow!("Failed to open the lock of {:?}: {}", file_path, e))
}

fn try_read_lock(file_lock: &File) -> AppResult<bool> {
    match file_lock.try_lock_shared() {
        Ok(()) => Ok(true),
        Err(TryLockError::WouldBlock) => Ok(false),
        Err(TryLockError::Error(e)) => Err(e.into()),
    }
}

fn try_write_lock(file_lock: &File) -> AppResult<bool> {
    match file_lock.try_lock() {
        Ok(()) => Ok(true),
        Err(TryLockError::WouldBlock) => Ok(false),
        Err(TryLockError::Error(e)) => Err(e.into()),
    }
}
