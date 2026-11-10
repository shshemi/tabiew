use crate::AppResult;
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

pub struct Spool {
    dir: PathBuf,
    lock: File,
    count: usize,
}

impl Spool {
    pub fn new(cache_dir: &Path) -> AppResult<Self> {
        let root = cache_dir.join("spool");
        std::fs::create_dir_all(&root)?;
        for entry in std::fs::read_dir(&root)? {
            let dir = entry?.path();
            if let Ok(lock) = File::open(dir.join(".lock"))
                && lock.try_lock().is_ok()
            {
                drop(lock);
                let _ = std::fs::remove_dir_all(&dir);
            }
        }
        let dir = root.join(std::process::id().to_string());
        std::fs::create_dir_all(&dir)?;
        let lock = File::create(dir.join(".lock"))?;
        lock.try_lock()?;
        Ok(Self {
            dir,
            lock,
            count: 0,
        })
    }
    pub fn path(&mut self, name: &str) -> PathBuf {
        self.count += 1;
        self.dir.join(format!("{}-{name}", self.count))
    }
    pub fn write(&mut self, name: &str, mut reader: impl Read) -> AppResult<PathBuf> {
        let path = self.path(name);
        std::io::copy(&mut reader, &mut File::create(&path)?)?;
        Ok(path)
    }
}

impl Drop for Spool {
    fn drop(&mut self) {
        let _ = self.lock.unlock();
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}
