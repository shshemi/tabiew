pub(super) mod config;
pub(super) mod file_db;

use crate::AppResult;
use anyhow::anyhow;
use bytesize::ByteSize;
use file_db::{FileDB, FileDBMetadata};
use lru::LruCache;
use serde_json::Value;
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
    pub fn create_new_db(&mut self, file_path: &Path, options: &Value) -> AppResult<FileDB> {
        let size = file_path
            .metadata()
            .map_err(|e| anyhow!("Failed to get metadata of file: {}", e))?
            .len();
        let hashed_file = hash_file_path(&self.cache_dir, file_path)?;
        self.create_db(&hashed_file, file_path, size, options)
    }
    pub fn create_url_db(
        &mut self,
        url: &str,
        validators: &str,
        size: u64,
        options: &Value,
    ) -> AppResult<FileDB> {
        let hashed_file = hash_key(&self.cache_dir, &[url.as_bytes(), validators.as_bytes()]);
        self.create_db(&hashed_file, Path::new(url), size, options)
    }
    pub fn check_existing_cache_db(
        &mut self,
        file_path: &Path,
        options: &Value,
    ) -> AppResult<Option<FileDB>> {
        let hashed_file = hash_file_path(&self.cache_dir, file_path)?;
        self.check_db(&hashed_file, options)
    }
    pub fn check_url_db(
        &mut self,
        url: &str,
        validators: &str,
        options: &Value,
    ) -> AppResult<Option<FileDB>> {
        let hashed_file = hash_key(&self.cache_dir, &[url.as_bytes(), validators.as_bytes()]);
        self.check_db(&hashed_file, options)
    }
    fn create_db(
        &mut self,
        hashed_file: &Path,
        source: &Path,
        size: u64,
        options: &Value,
    ) -> AppResult<FileDB> {
        if self.cache_size.is_some() {
            self.lru_cache_eviction(ByteSize(size))?;
        }
        let file_db = FileDB::create(hashed_file, source, options)?;
        self.track(hashed_file);
        Ok(file_db)
    }
    fn check_db(&mut self, hashed_file: &Path, options: &Value) -> AppResult<Option<FileDB>> {
        let file_db = FileDB::new(hashed_file)?;
        if !std::fs::exists(hashed_file).map_err(|e| anyhow!(e))?
            || !FileDBMetadata::is_complete_for(hashed_file, options)
        {
            return Ok(None);
        }
        self.track(hashed_file);
        Ok(Some(file_db))
    }
    fn lru_cache_eviction(&mut self, file_size: ByteSize) -> AppResult<()> {
        if let Some(cache_size) = self.cache_size {
            if file_size >= cache_size {
                return Err(anyhow!("File size is bigger than the cache size"));
            }
            let mut in_use = Vec::new();
            let mut usage = self.current_cache_usage()?.as_u64();
            let result = loop {
                if cache_size.as_u64().saturating_sub(usage) >= file_size.as_u64() {
                    break Ok(());
                }
                let Some((evicted, ())) = self.lru_file_cache.pop_lru() else {
                    break Err(anyhow!(
                        "Can't allocate {} in the cache, the remaining cached files are in use",
                        file_size
                    ));
                };
                let size = [evicted.clone(), evicted.with_extension("duckdb.wal")]
                    .iter()
                    .filter_map(|path| std::fs::metadata(path).ok())
                    .map(|metadata| metadata.len())
                    .sum::<u64>();
                match delete_file(&evicted) {
                    Ok(()) => usage = usage.saturating_sub(size),
                    Err(_) => in_use.push(evicted),
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
    Ok(hash_key(
        cache_dir,
        &[
            file_path.as_os_str().as_encoded_bytes(),
            &last_modified_time.as_nanos().to_le_bytes(),
        ],
    ))
}

fn hash_key(cache_dir: &Path, parts: &[&[u8]]) -> PathBuf {
    let mut sha_hasher = Sha256::new();
    for part in parts {
        sha_hasher.update(part);
    }
    let hex: String = sha_hasher
        .finalize()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    cache_dir.join(format!("{hex}.duckdb"))
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
    use crate::duckdb::{DuckDBConfig, cache::file_db::FileDBStatus};
    use std::fs;
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
        cm.create_new_db(src, &Value::Null)
            .unwrap()
            .file_path()
            .to_path_buf()
    }

    fn finish_import(db_path: &Path, size: usize) {
        fs::write(db_path, vec![0u8; size]).unwrap();
        let mut meta = FileDBMetadata::read(db_path)
            .unwrap()
            .unwrap_or_else(|| FileDBMetadata::new(Path::new("/data/source.csv")));
        meta.status = FileDBStatus::Complete;
        meta.write(db_path).unwrap();
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

}
