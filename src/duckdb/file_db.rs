use crate::AppResult;
use anyhow::anyhow;
use std::{
    fs::{File, TryLockError},
    path::{Path, PathBuf},
};

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

    pub fn try_exclusive_lock(file_path: &Path) -> AppResult<Option<File>> {
        let file_lock = open_lock(file_path)?;
        Ok(try_write_lock(&file_lock)?.then_some(file_lock))
    }
}

impl Drop for FileDB {
    fn drop(&mut self) {
        let _ = self.file_lock.unlock();
    }
}

fn open_lock(file_path: &Path) -> AppResult<File> {
    File::create(file_path.with_extension("duckdb.lock"))
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

    fn db(dir: &TempDir) -> PathBuf {
        let path = dir.path().join("abc.duckdb");
        std::fs::write(&path, b"db").unwrap();
        path
    }

    #[test]
    fn read_locks_are_shared_and_released_on_drop() {
        let dir = TempDir::new().unwrap();
        let path = db(&dir);
        let one = FileDB::new(&path).unwrap();
        let two = FileDB::new(&path).unwrap();
        assert_eq!(one.file_path(), path);
        assert!(FileDB::try_exclusive_lock(&path).unwrap().is_none());
        drop(one);
        assert!(FileDB::try_exclusive_lock(&path).unwrap().is_none());
        drop(two);
        assert!(FileDB::try_exclusive_lock(&path).unwrap().is_some());
    }

    #[test]
    fn write_lock_is_released_on_drop() {
        let dir = TempDir::new().unwrap();
        let path = db(&dir);
        let mut file_db = FileDB::new(&path).unwrap();
        file_db.get_write_lock().unwrap();
        drop(file_db);
        assert!(FileDB::try_exclusive_lock(&path).unwrap().is_some());
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
        assert!(FileDB::try_exclusive_lock(&path).unwrap().is_none());
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
