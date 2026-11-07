use crate::{
    AppResult,
    duckdb::file_db::{FileDB, FileDBMetadata},
};
use anyhow::anyhow;
use bytesize::ByteSize;
use lru::LruCache;
use sha2::{Digest, Sha256};
use std::{
    num::NonZeroUsize,
    path::{Path, PathBuf},
    time::SystemTime,
};

pub struct CacheManager {
    cache_dir: PathBuf,
    cache_size: Option<ByteSize>,
    lru_file_cache: LruCache<PathBuf, ()>,
}

impl CacheManager {
    pub fn new<P: AsRef<Path>>(
        cache_dir: P,
        cache_size: Option<ByteSize>,
        lru_capacity: usize,
    ) -> AppResult<Self> {
        let mut lru_cache: LruCache<PathBuf, ()>;
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
    pub fn create_new_db(&mut self, file_path: &Path) -> AppResult<FileDB> {
        if self.cache_size.is_some() {
            self.lru_cache_eviction(file_path)?;
        }
        let duckdb_file_name = hash_file_path(&self.cache_dir, file_path)?;
        let file_db = FileDB::create(&duckdb_file_name, file_path)?;
        self.track(&duckdb_file_name);
        Ok(file_db)
    }
    pub fn check_existing_cache_db(&mut self, file_path: &Path) -> AppResult<Option<FileDB>> {
        let hashed_file = hash_file_path(&self.cache_dir, file_path)?;
        let file_db = FileDB::new(&hashed_file)?;
        if !std::fs::exists(&hashed_file).map_err(|e| anyhow!(e))?
            || !FileDBMetadata::is_complete(&hashed_file)
        {
            return Ok(None);
        }
        self.track(&hashed_file);
        Ok(Some(file_db))
    }
    pub fn delete_cache(&mut self, file_db: FileDB) -> AppResult<()> {
        self.lru_file_cache.pop(file_db.file_path());
        file_db.delete()
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
                let Some((evicted, ())) = self.lru_file_cache.pop_lru() else {
                    break Err(anyhow!(
                        "Can't allocate {} in the cache, the remaining cached files are in use",
                        file_size
                    ));
                };
                if delete_file(&evicted).is_err() {
                    in_use.push(evicted);
                }
            };
            for path in in_use.into_iter().rev() {
                self.lru_file_cache.push(path.clone(), ());
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
    fn track(&mut self, file_path: &Path) {
        if let Some((evicted, ())) = self.lru_file_cache.push(file_path.to_path_buf(), ())
            && evicted != file_path
        {
            let _ = delete_file(&evicted);
        }
    }
}

fn delete_file(file_path: &Path) -> AppResult<()> {
    FileDB::new(file_path)?.delete()
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
    cache: &mut LruCache<PathBuf, ()>,
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
        if !FileDBMetadata::is_complete(&path) {
            let _ = delete_file(&path);
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
        cache.push(entry.path.clone(), ());
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
    use crate::duckdb::{DuckDBConfig, FileDBStatus};
    use std::{
        fs::{self, File, FileTimes},
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

    fn create(cm: &mut CacheManager, src: &Path) -> PathBuf {
        cm.create_new_db(src).unwrap().file_path().to_path_buf()
    }

    fn hit(cm: &mut CacheManager, src: &Path) -> Option<PathBuf> {
        cm.check_existing_cache_db(src)
            .unwrap()
            .map(|file_db| file_db.file_path().to_path_buf())
    }

    fn finish_import(db_path: &Path, size: usize) {
        fs::write(db_path, vec![0u8; size]).unwrap();
        let mut meta = FileDBMetadata::read(db_path)
            .unwrap()
            .unwrap_or_else(|| FileDBMetadata::new(Path::new("/data/source.csv")));
        meta.status = FileDBStatus::Complete;
        meta.write(db_path).unwrap();
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
        assert_eq!(hit(&mut cm, &src), None);
        let db = create(&mut cm, &src);
        assert_eq!(db.parent().unwrap(), config.cache_dir);
        let name = db.file_name().unwrap().to_str().unwrap();
        assert_eq!(name.len(), 64 + ".duckdb".len());
        assert!(name[..64].chars().all(|c| c.is_ascii_hexdigit()));
        finish_import(&db, 10);
        assert_eq!(hit(&mut cm, &src), Some(db));
    }

    #[test]
    fn modified_source_misses_cache() {
        let dir = TempDir::new().unwrap();
        let mut cm = manager(&config(&dir, None, None));
        let src = source(&dir, "data.csv", 10);
        finish_import(&create(&mut cm, &src), 10);
        File::options()
            .write(true)
            .open(&src)
            .unwrap()
            .set_modified(SystemTime::UNIX_EPOCH + Duration::from_secs(1_000_000))
            .unwrap();
        assert_eq!(hit(&mut cm, &src), None);
    }

    #[test]
    fn delete_cache_removes_db_wal_and_metadata() {
        let dir = TempDir::new().unwrap();
        let mut cm = manager(&config(&dir, None, None));
        let src = source(&dir, "data.csv", 10);
        let db = create(&mut cm, &src);
        finish_import(&db, 10);
        let wal = db.with_extension("duckdb.wal");
        fs::write(&wal, "wal").unwrap();
        cm.delete_cache(FileDB::new(&db).unwrap()).unwrap();
        assert!(!db.exists());
        assert!(!wal.exists());
        assert!(!FileDBMetadata::path(&db).exists());
        assert_eq!(hit(&mut cm, &src), None);
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
            finish_import(path, 10);
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
            finish_import(path, 10);
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
        let in_use = create(&mut other, &src);
        finish_import(&in_use, 10);
        let _held = FileDB::new(&in_use).unwrap();
        set_accessed_age(&in_use, 300);
        let newer = config.cache_dir.join("newer.duckdb");
        finish_import(&newer, 10);
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
        let db_a = create(&mut cm, &a);
        finish_import(&db_a, 10);
        let db_b = create(&mut cm, &b);
        finish_import(&db_b, 10);
        assert_eq!(hit(&mut cm, &a), Some(db_a.clone()));
        finish_import(&create(&mut cm, &c), 10);
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
        let db_a = create(&mut cm, &a);
        finish_import(&db_a, 40);
        let db_b = create(&mut cm, &b);
        finish_import(&db_b, 40);
        create(&mut cm, &c);
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
        let db_a = create(&mut cm, &a);
        finish_import(&db_a, 40);
        let db_b = create(&mut cm, &b);
        finish_import(&db_b, 40);
        let _held = FileDB::new(&db_a).unwrap();
        create(&mut cm, &c);
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
        let db_a = create(&mut cm, &a);
        finish_import(&db_a, 40);
        let db_b = create(&mut cm, &b);
        finish_import(&db_b, 40);
        let held_a = FileDB::new(&db_a).unwrap();
        let held_b = FileDB::new(&db_b).unwrap();
        let err = cm.create_new_db(&c).unwrap_err();
        assert!(err.to_string().contains("Can't allocate"), "{err}");
        assert!(db_a.exists() && db_b.exists());
        drop((held_a, held_b));
        create(&mut cm, &c);
        assert!(!db_a.exists());
        assert!(db_b.exists());
    }

    #[test]
    fn create_new_db_writes_incomplete_metadata() {
        let dir = TempDir::new().unwrap();
        let mut cm = manager(&config(&dir, None, None));
        let src = source(&dir, "data.csv", 10);
        let db = create(&mut cm, &src);
        let meta = FileDBMetadata::read(&db).unwrap().unwrap();
        assert_eq!(meta.file_path, src);
        assert_eq!(meta.file_name, "data.csv");
        assert_eq!(meta.status, FileDBStatus::Incomplete);
    }

    #[test]
    fn incomplete_db_is_not_a_cache_hit_until_marked_complete() {
        let dir = TempDir::new().unwrap();
        let config = config(&dir, None, None);
        let mut cm = manager(&config);
        let src = source(&dir, "data.csv", 10);
        let db = create(&mut cm, &src);
        fs::write(&db, vec![0u8; 10]).unwrap();
        let _held = FileDB::new(&db).unwrap();

        assert_eq!(hit(&mut cm, &src), None);
        assert_eq!(hit(&mut manager(&config), &src), None);

        let mut meta = FileDBMetadata::read(&db).unwrap().unwrap();
        meta.status = FileDBStatus::Complete;
        meta.write(&db).unwrap();
        assert_eq!(hit(&mut cm, &src), Some(db.clone()));
        assert_eq!(hit(&mut manager(&config), &src), Some(db));
    }

    #[test]
    fn new_deletes_incomplete_dbs_unless_locked() {
        let dir = TempDir::new().unwrap();
        let config = config(&dir, None, None);
        let mut importing_instance = manager(&config);
        let src = source(&dir, "data.csv", 10);
        let importing = create(&mut importing_instance, &src);
        fs::write(&importing, vec![0u8; 10]).unwrap();
        let _held = FileDB::new(&importing).unwrap();

        let done = config.cache_dir.join("done.duckdb");
        finish_import(&done, 10);
        let half = config.cache_dir.join("half.duckdb");
        fs::write(&half, vec![0u8; 10]).unwrap();
        FileDBMetadata::new(Path::new("/data/half.csv"))
            .write(&half)
            .unwrap();
        let no_meta = config.cache_dir.join("no_meta.duckdb");
        fs::write(&no_meta, vec![0u8; 10]).unwrap();
        let broken = config.cache_dir.join("broken.duckdb");
        fs::write(&broken, vec![0u8; 10]).unwrap();
        fs::write(FileDBMetadata::path(&broken), "not json").unwrap();

        manager(&config);
        assert!(done.exists());
        assert!(importing.exists() && FileDBMetadata::path(&importing).exists());
        assert!(!half.exists() && !FileDBMetadata::path(&half).exists());
        assert!(!no_meta.exists());
        assert!(!broken.exists() && !FileDBMetadata::path(&broken).exists());
    }
}
