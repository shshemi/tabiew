use crate::AppResult;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::{
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
pub struct FileDBMetadata {
    pub file_path: PathBuf,
    pub file_name: String,
    pub status: FileDBStatus,
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

    pub fn create(file_path: &Path, source_path: &Path) -> AppResult<Self> {
        let mut file_db = Self::new(file_path)?;
        file_db.get_write_lock()?;
        FileDBMetadata::new(source_path).write(file_path)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn is_locked(path: &Path) -> bool {
        FileDB::new(path).is_err() || FileDB::new(path).unwrap().get_write_lock().is_err()
    }

    fn db(dir: &TempDir) -> PathBuf {
        let path = dir.path().join("abc.duckdb");
        std::fs::write(&path, b"db").unwrap();
        path
    }

    #[test]
    fn metadata_round_trips_through_json() {
        let dir = TempDir::new().unwrap();
        let db = dir.path().join("abc.duckdb");
        assert_eq!(FileDBMetadata::read(&db).unwrap(), None);
        let meta = FileDBMetadata::new(Path::new("/data/sales.csv"));
        meta.write(&db).unwrap();
        assert_eq!(
            FileDBMetadata::path(&db),
            dir.path().join("abc.duckdb.meta.json")
        );
        let json = std::fs::read_to_string(FileDBMetadata::path(&db)).unwrap();
        assert!(json.contains("\"status\": \"incomplete\""), "{json}");
        assert_eq!(FileDBMetadata::read(&db).unwrap(), Some(meta));
        assert!(!FileDBMetadata::is_complete(&db));
    }

    #[test]
    fn create_locks_and_writes_incomplete_metadata() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("abc.duckdb");
        let file_db = FileDB::create(&path, Path::new("/data/sales.csv")).unwrap();
        assert!(is_locked(&path));
        let meta = FileDBMetadata::read(&path).unwrap().unwrap();
        assert_eq!(meta.file_name, "sales.csv");
        assert_eq!(meta.status, FileDBStatus::Incomplete);
        drop(file_db);
        assert!(!is_locked(&path));
    }

    #[test]
    fn mark_complete_writes_status_and_downgrades() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("abc.duckdb");
        let mut file_db = FileDB::create(&path, Path::new("/data/sales.csv")).unwrap();
        std::fs::write(&path, b"db").unwrap();
        assert!(FileDB::new(&path).is_err());
        file_db.mark_complete().unwrap();
        assert!(FileDBMetadata::is_complete(&path));
        let _reader = FileDB::new(&path).unwrap();
    }

    #[test]
    fn delete_removes_db_wal_and_metadata() {
        let dir = TempDir::new().unwrap();
        let path = db(&dir);
        let wal = path.with_extension("duckdb.wal");
        std::fs::write(&wal, b"wal").unwrap();
        FileDBMetadata::new(Path::new("/data/sales.csv"))
            .write(&path)
            .unwrap();
        FileDB::new(&path).unwrap().delete().unwrap();
        assert!(!path.exists() && !wal.exists());
        assert!(!FileDBMetadata::path(&path).exists());
    }

    #[test]
    fn delete_removes_metadata_when_the_db_was_never_created() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("abc.duckdb");
        let file_db = FileDB::create(&path, Path::new("/data/sales.csv")).unwrap();
        file_db.delete().unwrap();
        assert!(!FileDBMetadata::path(&path).exists());
    }

    #[test]
    fn delete_fails_while_someone_else_reads() {
        let dir = TempDir::new().unwrap();
        let path = db(&dir);
        let _reader = FileDB::new(&path).unwrap();
        let err = FileDB::new(&path).unwrap().delete().unwrap_err();
        assert!(err.to_string().contains("in use"), "{err}");
        assert!(path.exists());
    }

    #[test]
    fn read_locks_are_shared_and_released_on_drop() {
        let dir = TempDir::new().unwrap();
        let path = db(&dir);
        let one = FileDB::new(&path).unwrap();
        let two = FileDB::new(&path).unwrap();
        assert_eq!(one.file_path(), path);
        assert!(is_locked(&path));
        drop(one);
        assert!(is_locked(&path));
        drop(two);
        assert!(!is_locked(&path));
    }

    #[test]
    fn write_lock_is_released_on_drop() {
        let dir = TempDir::new().unwrap();
        let path = db(&dir);
        let mut file_db = FileDB::new(&path).unwrap();
        file_db.get_write_lock().unwrap();
        drop(file_db);
        assert!(!is_locked(&path));
    }

    #[test]
    fn write_lock_shuts_out_readers_until_downgraded() {
        let dir = TempDir::new().unwrap();
        let path = db(&dir);
        let mut writer = FileDB::new(&path).unwrap();
        writer.get_write_lock().unwrap();
        let err = FileDB::new(&path).unwrap_err();
        assert!(err.to_string().contains("in use"), "{err}");
        writer.get_read_lock().unwrap();
        let _reader = FileDB::new(&path).unwrap();
    }

    #[test]
    fn failed_upgrade_keeps_the_read_lock() {
        let dir = TempDir::new().unwrap();
        let path = db(&dir);
        let mut one = FileDB::new(&path).unwrap();
        let two = FileDB::new(&path).unwrap();
        let err = one.get_write_lock().unwrap_err();
        assert!(err.to_string().contains("in use"), "{err}");
        drop(two);
        assert!(is_locked(&path));
        one.get_write_lock().unwrap();
    }

    #[test]
    fn downgrade_errors_if_the_db_is_gone() {
        let dir = TempDir::new().unwrap();
        let path = db(&dir);
        let mut file_db = FileDB::new(&path).unwrap();
        file_db.get_write_lock().unwrap();
        std::fs::remove_file(&path).unwrap();
        assert!(file_db.get_read_lock().is_err());
    }
}
