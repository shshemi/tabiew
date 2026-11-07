mod cache;
mod config;
mod file_db;
mod helper;
mod ingester;
pub mod reader;
mod session;

use cache::CacheManager;
use config::Config;
use session::Session;

pub use config::Config as DuckDBConfig;

pub struct DuckDB {
    cache_manager: CacheManager,
    config: Config,
    session: Session,
}

impl DuckDB {
    pub fn new() {}
    pub fn flush() {}
}
