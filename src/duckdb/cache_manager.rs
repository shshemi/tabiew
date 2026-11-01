use crate::AppResult;
use anyhow::anyhow;
use bytesize::ByteSize;
use lru::LruCache;
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    num::NonZeroUsize,
    path::{Path, PathBuf},
    time::SystemTime,
};

pub struct CacheManager {
    cache_dir: PathBuf,
    cache_size: Option<ByteSize>,
    lru_file_cache: LruCache<PathBuf, Option<File>>,
}

impl CacheManager {
    pub fn new<P: AsRef<Path>>(
        cache_dir: P,
        cache_size: Option<ByteSize>,
        lru_capacity: usize,
    ) -> AppResult<Self> {
        let mut lru_cache: LruCache<PathBuf, Option<File>>;
        if lru_capacity == 0 {
            lru_cache = LruCache::unbounded();
        } else {
            lru_cache = LruCache::new(NonZeroUsize::new(lru_capacity).unwrap());
        }
        build_db_lru(&mut lru_cache, cache_dir.as_ref(), cache_size)?;
        Ok(Self {
            cache_dir: cache_dir.as_ref().to_path_buf(),
            cache_size,
            lru_file_cache: lru_cache,
        })
    }
    pub fn create_new_db(&mut self, file_path: &Path) -> AppResult<PathBuf> {
        if self.cache_size.is_some() {
            self.lru_cache_eviction(file_path)?;
        }
        let duckdb_file_name = hash_file_path(&self.cache_dir, file_path)?;
        let lock = lock_db(&duckdb_file_name)?;
        self.track(&duckdb_file_name, lock);
        Ok(duckdb_file_name)
    }
    pub fn check_existing_cache_db(&mut self, file_path: &Path) -> AppResult<Option<PathBuf>> {
        let hashed_file = hash_file_path(&self.cache_dir, file_path)?;
        if self
            .lru_file_cache
            .peek(&hashed_file)
            .is_some_and(Option::is_some)
        {
            self.lru_file_cache.promote(&hashed_file);
            return Ok(Some(hashed_file));
        }
        let lock = lock_db(&hashed_file)?;
        if !std::fs::exists(&hashed_file).map_err(|e| anyhow!(e))? {
            return Ok(None);
        }
        self.track(&hashed_file, lock);
        Ok(Some(hashed_file))
    }
    pub fn delete_db(&mut self, file_path: &Path) -> AppResult<()> {
        let hashed_file = hash_file_path(&self.cache_dir, file_path)?;
        self.lru_file_cache.pop(&hashed_file);
        delete_file(&hashed_file)
    }
    pub fn release_db(&mut self, file_path: &Path) {
        if let Some(lock) = self.lru_file_cache.peek_mut(file_path) {
            *lock = None;
        }
    }
    fn lru_cache_eviction(&mut self, file_path: &Path) -> AppResult<()> {
        let file_size = ByteSize(
            file_path
                .metadata()
                .map_err(|e| anyhow!("Failed to get metadata of file: {}", e))?
                .len(),
        );
        if let Some(cache_size) = self.cache_size {
            if file_size >= cache_size {
                return Err(anyhow!("File size is bigger than the cache size"));
            }
            let mut in_use = Vec::new();
            let result = loop {
                let free_space = match self.current_cache_usage() {
                    Ok(usage) => cache_size.as_u64().saturating_sub(usage.as_u64()),
                    Err(e) => break Err(e),
                };
                if free_space >= file_size.as_u64() {
                    break Ok(());
                }
                let Some((evicted, lock)) = self.lru_file_cache.pop_lru() else {
                    break Err(anyhow!(
                        "Can't allocate {} in the cache, the remaining cached files are in use",
                        file_size
                    ));
                };
                drop(lock);
                if delete_file(&evicted).is_err() {
                    in_use.push(evicted);
                }
            };
            for path in in_use.into_iter().rev() {
                self.lru_file_cache.push(path.clone(), None);
                self.lru_file_cache.demote(&path);
            }
            return result;
        }
        Ok(())
    }
    fn current_cache_usage(&self) -> AppResult<ByteSize> {
        let mut total = 0;
        for entry in std::fs::read_dir(&self.cache_dir)? {
            let entry = entry?;
            let metadata = entry.metadata()?;
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if metadata.is_file() && (name.ends_with(".duckdb") || name.ends_with(".duckdb.wal")) {
                total += metadata.len();
            }
        }
        Ok(ByteSize(total))
    }
    fn track(&mut self, file_path: &Path, lock: File) {
        if let Some((evicted, evicted_lock)) = self
            .lru_file_cache
            .push(file_path.to_path_buf(), Some(lock))
            && evicted != file_path
        {
            drop(evicted_lock);
            let _ = delete_file(&evicted);
        }
    }
}

fn lock_db(file_path: &Path) -> AppResult<File> {
    let lock = File::create(file_path.with_extension("duckdb.lock"))?;
    lock.lock_shared()?;
    Ok(lock)
}

fn delete_file(file_path: &Path) -> AppResult<()> {
    let file_lock = std::fs::File::create(&file_path.with_extension("duckdb.lock"))?;
    if let Err(std::fs::TryLockError::WouldBlock) = file_lock.try_lock() {
        return Err(anyhow!("File {:?} is in use", file_path));
    }
    std::fs::remove_file(file_path)
        .map_err(|e| anyhow!("Failed to delete file {:?}: {}", file_path, e))?;
    match std::fs::remove_file(file_path.with_extension("duckdb.wal")) {
        Err(e) if e.kind() != std::io::ErrorKind::NotFound => Err(anyhow!(
            "Failed to delete file {:?}: {}",
            file_path.with_extension("duckdb.wal"),
            e
        )),
        _ => Ok(()),
    }
}

fn hash_file_path(cache_dir: &Path, file_path: &Path) -> AppResult<PathBuf> {
    let last_modified_time = std::fs::metadata(file_path)
        .map_err(|e| anyhow!("Failed to get metadata of the file: {}", e))?
        .modified()
        .map_err(|e| anyhow!("Failed to get modified time of the file: {}", e))?
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    let file_path_bytes = file_path.as_os_str().as_encoded_bytes();
    let mut sha_hasher = Sha256::new();
    sha_hasher.update(file_path_bytes);
    sha_hasher.update(last_modified_time.as_nanos().to_le_bytes());
    let hash_res = sha_hasher.finalize();
    let hex: String = hash_res.iter().map(|b| format!("{b:02x}")).collect();
    let duckdb_file_name = format!("{hex}.duckdb");
    Ok(cache_dir.join(duckdb_file_name))
}

struct CacheEntry {
    path: PathBuf,
    size: ByteSize,
    accessed: SystemTime,
}

fn build_db_lru(
    cache: &mut LruCache<PathBuf, Option<File>>,
    cache_dir: &Path,
    cache_size: Option<ByteSize>,
) -> AppResult<()> {
    std::fs::create_dir_all(cache_dir)
        .map_err(|e| anyhow!("Failed to create the cache directory: {}", e))?;
    let mut total_size = ByteSize(0);
    let entries = std::fs::read_dir(cache_dir)
        .map_err(|e| anyhow!("Failed to read the cache directory: {}", e))?;
    let mut sorted_entries = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| anyhow!("Failed to read entry: {}", e))?;
        let path = entry.path();
        if path.extension().is_none_or(|ext| ext != "duckdb") {
            continue;
        }
        let metadata = entry
            .metadata()
            .map_err(|e| anyhow!("Failed to get the metadata of entry: {}", e))?;
        if !metadata.is_file() {
            continue;
        }
        let accessed_time = metadata
            .accessed()
            .map_err(|e| anyhow!("Failed to get accessed time of entry: {}", e))?;
        sorted_entries.push(CacheEntry {
            path,
            size: ByteSize(metadata.len()),
            accessed: accessed_time,
        });
    }
    sorted_entries.sort_by_key(|e| std::cmp::Reverse(e.accessed));
    let mut delete_index = 0;
    let cache_cap = cache.cap().get();
    for entry in &sorted_entries {
        if delete_index >= cache_cap {
            break;
        }
        if let Some(cache_size) = cache_size {
            if entry.size + total_size > cache_size {
                break;
            }
            total_size = entry.size + total_size;
        }
        delete_index += 1;
    }
    for entry in sorted_entries[..delete_index].iter().rev() {
        cache.push(entry.path.clone(), None);
    }
    delete_files(&sorted_entries[delete_index..]);
    Ok(())
}

fn delete_files(files: &[CacheEntry]) {
    for entry in files {
        let _ = delete_file(&entry.path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::duckdb::DuckDBConfig;
    use std::{
        fs::{self, FileTimes},
        time::Duration,
    };
    use tempfile::TempDir;

    fn config(
        dir: &TempDir,
        max_cache_size: Option<ByteSize>,
        lru_capacity: Option<usize>,
    ) -> DuckDBConfig {
        DuckDBConfig {
            cache_dir: dir.path().join("cache"),
            max_cache_size,
            lru_capacity,
            ..DuckDBConfig::default()
        }
    }

    fn manager(config: &DuckDBConfig) -> CacheManager {
        CacheManager::new(
            &config.cache_dir,
            config.max_cache_size,
            config.lru_capacity.unwrap_or(0),
        )
        .unwrap()
    }

    fn source(dir: &TempDir, name: &str, size: usize) -> PathBuf {
        let path = dir.path().join(name);
        fs::write(&path, vec![b'x'; size]).unwrap();
        path
    }

    fn fill_db(db_path: &Path, size: usize) {
        fs::write(db_path, vec![0u8; size]).unwrap();
    }

    fn set_accessed_age(path: &Path, age_secs: u64) {
        File::options()
            .write(true)
            .open(path)
            .unwrap()
            .set_times(
                FileTimes::new().set_accessed(SystemTime::now() - Duration::from_secs(age_secs)),
            )
            .unwrap();
    }

    #[test]
    fn new_creates_missing_cache_dir() {
        let dir = TempDir::new().unwrap();
        let config = config(&dir, None, None);
        assert!(!config.cache_dir.exists());
        manager(&config);
        assert!(config.cache_dir.is_dir());
    }

    #[test]
    fn miss_then_create_then_hit() {
        let dir = TempDir::new().unwrap();
        let config = config(&dir, None, None);
        let mut cm = manager(&config);
        let src = source(&dir, "data.csv", 10);
        assert_eq!(cm.check_existing_cache_db(&src).unwrap(), None);
        let db = cm.create_new_db(&src).unwrap();
        assert_eq!(db.parent().unwrap(), config.cache_dir);
        let name = db.file_name().unwrap().to_str().unwrap();
        assert_eq!(name.len(), 64 + ".duckdb".len());
        assert!(name[..64].chars().all(|c| c.is_ascii_hexdigit()));
        fill_db(&db, 10);
        assert_eq!(cm.check_existing_cache_db(&src).unwrap(), Some(db));
    }

    #[test]
    fn modified_source_misses_cache() {
        let dir = TempDir::new().unwrap();
        let mut cm = manager(&config(&dir, None, None));
        let src = source(&dir, "data.csv", 10);
        fill_db(&cm.create_new_db(&src).unwrap(), 10);
        File::options()
            .write(true)
            .open(&src)
            .unwrap()
            .set_modified(SystemTime::UNIX_EPOCH + Duration::from_secs(1_000_000))
            .unwrap();
        assert_eq!(cm.check_existing_cache_db(&src).unwrap(), None);
    }

    #[test]
    fn delete_db_removes_db_and_wal() {
        let dir = TempDir::new().unwrap();
        let mut cm = manager(&config(&dir, None, None));
        let src = source(&dir, "data.csv", 10);
        let db = cm.create_new_db(&src).unwrap();
        fill_db(&db, 10);
        let wal = db.with_extension("duckdb.wal");
        fs::write(&wal, "wal").unwrap();
        cm.delete_db(&src).unwrap();
        assert!(!db.exists());
        assert!(!wal.exists());
        assert_eq!(cm.check_existing_cache_db(&src).unwrap(), None);
    }

    #[test]
    fn new_keeps_most_recently_accessed_within_capacity() {
        let dir = TempDir::new().unwrap();
        let config = config(&dir, None, Some(2));
        fs::create_dir_all(&config.cache_dir).unwrap();
        let old = config.cache_dir.join("old.duckdb");
        let mid = config.cache_dir.join("mid.duckdb");
        let new = config.cache_dir.join("new.duckdb");
        for (path, age) in [(&old, 300), (&mid, 200), (&new, 100)] {
            fill_db(path, 10);
            set_accessed_age(path, age);
        }
        let notes = config.cache_dir.join("notes.txt");
        fs::write(&notes, "keep me").unwrap();
        let subdir = config.cache_dir.join("sub.duckdb");
        fs::create_dir(&subdir).unwrap();
        manager(&config);
        assert!(!old.exists());
        assert!(mid.exists() && new.exists());
        assert!(notes.exists() && subdir.is_dir());
    }

    #[test]
    fn new_keeps_most_recently_accessed_within_max_cache_size() {
        let dir = TempDir::new().unwrap();
        let config = config(&dir, Some(ByteSize(25)), None);
        fs::create_dir_all(&config.cache_dir).unwrap();
        let old = config.cache_dir.join("old.duckdb");
        let mid = config.cache_dir.join("mid.duckdb");
        let new = config.cache_dir.join("new.duckdb");
        for (path, age) in [(&old, 300), (&mid, 200), (&new, 100)] {
            fill_db(path, 10);
            set_accessed_age(path, age);
        }
        manager(&config);
        assert!(!old.exists());
        assert!(mid.exists() && new.exists());
    }

    #[test]
    fn new_does_not_delete_db_in_use_by_another_instance() {
        let dir = TempDir::new().unwrap();
        let config = config(&dir, None, Some(1));
        let mut other = manager(&config);
        let src = source(&dir, "data.csv", 10);
        let in_use = other.create_new_db(&src).unwrap();
        fill_db(&in_use, 10);
        set_accessed_age(&in_use, 300);
        let newer = config.cache_dir.join("newer.duckdb");
        fill_db(&newer, 10);
        set_accessed_age(&newer, 100);
        manager(&config);
        assert!(in_use.exists());
        assert!(newer.exists());
    }

    #[test]
    fn create_over_capacity_evicts_least_recently_used() {
        let dir = TempDir::new().unwrap();
        let mut cm = manager(&config(&dir, None, Some(2)));
        let (a, b, c) = (
            source(&dir, "a.csv", 10),
            source(&dir, "b.csv", 10),
            source(&dir, "c.csv", 10),
        );
        let db_a = cm.create_new_db(&a).unwrap();
        fill_db(&db_a, 10);
        let db_b = cm.create_new_db(&b).unwrap();
        fill_db(&db_b, 10);
        assert_eq!(cm.check_existing_cache_db(&a).unwrap(), Some(db_a.clone()));
        fill_db(&cm.create_new_db(&c).unwrap(), 10);
        assert!(db_a.exists());
        assert!(!db_b.exists());
    }

    #[test]
    fn create_over_max_cache_size_evicts_least_recently_used() {
        let dir = TempDir::new().unwrap();
        let mut cm = manager(&config(&dir, Some(ByteSize(100)), None));
        let (a, b, c) = (
            source(&dir, "a.csv", 40),
            source(&dir, "b.csv", 40),
            source(&dir, "c.csv", 40),
        );
        let db_a = cm.create_new_db(&a).unwrap();
        fill_db(&db_a, 40);
        cm.release_db(&db_a);
        let db_b = cm.create_new_db(&b).unwrap();
        fill_db(&db_b, 40);
        cm.release_db(&db_b);
        cm.create_new_db(&c).unwrap();
        assert!(!db_a.exists());
        assert!(db_b.exists());
    }

    #[test]
    fn create_rejects_source_bigger_than_max_cache_size() {
        let dir = TempDir::new().unwrap();
        let mut cm = manager(&config(&dir, Some(ByteSize(10)), None));
        let src = source(&dir, "big.csv", 20);
        assert!(cm.create_new_db(&src).is_err());
    }

    #[test]
    fn eviction_skips_db_in_use_by_another_instance() {
        let dir = TempDir::new().unwrap();
        let config = config(&dir, Some(ByteSize(100)), None);
        let mut cm = manager(&config);
        let (a, b, c) = (
            source(&dir, "a.csv", 40),
            source(&dir, "b.csv", 40),
            source(&dir, "c.csv", 40),
        );
        let db_a = cm.create_new_db(&a).unwrap();
        fill_db(&db_a, 40);
        cm.release_db(&db_a);
        let db_b = cm.create_new_db(&b).unwrap();
        fill_db(&db_b, 40);
        cm.release_db(&db_b);
        let mut other = manager(&config);
        assert!(other.check_existing_cache_db(&a).unwrap().is_some());
        cm.create_new_db(&c).unwrap();
        assert!(db_a.exists());
        assert!(!db_b.exists());
    }

    #[test]
    fn eviction_errors_when_every_db_is_in_use_then_recovers_after_release() {
        let dir = TempDir::new().unwrap();
        let config = config(&dir, Some(ByteSize(100)), None);
        let mut cm = manager(&config);
        let (a, b, c) = (
            source(&dir, "a.csv", 40),
            source(&dir, "b.csv", 40),
            source(&dir, "c.csv", 40),
        );
        let db_a = cm.create_new_db(&a).unwrap();
        fill_db(&db_a, 40);
        cm.release_db(&db_a);
        let db_b = cm.create_new_db(&b).unwrap();
        fill_db(&db_b, 40);
        cm.release_db(&db_b);
        let mut other = manager(&config);
        other.check_existing_cache_db(&a).unwrap();
        other.check_existing_cache_db(&b).unwrap();
        let err = cm.create_new_db(&c).unwrap_err();
        assert!(err.to_string().contains("Can't allocate"), "{err}");
        assert!(db_a.exists() && db_b.exists());
        other.release_db(&db_a);
        other.release_db(&db_b);
        cm.create_new_db(&c).unwrap();
        assert!(!db_a.exists());
        assert!(db_b.exists());
    }
}
