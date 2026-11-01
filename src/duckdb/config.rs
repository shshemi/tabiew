use std::path::PathBuf;

use crate::misc::paths::cache_path;
use bytesize::ByteSize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// alloted ram for the duckdb to work with.
    pub allotted_ram: Option<ByteSize>,

    /// the directory in which the cache files will be generated and stored.
    pub cache_dir: PathBuf,

    /// the maximum cache size.
    /// the system will try to do a LRU eviction on the files based on the age.
    /// if the file goes beyond the given limit then it will show an error
    pub max_cache_size: Option<ByteSize>,

    /// the capacity for the lru cache
    /// if all the files can't fit in this the rest of the files will be deleted
    pub lru_capacity: Option<usize>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            allotted_ram: None,
            cache_dir: cache_path().unwrap_or_else(|_| std::env::temp_dir().join("tabiew")),
            max_cache_size: None,
            lru_capacity: None,
        }
    }
}
