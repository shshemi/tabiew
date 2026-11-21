use duckdb::Connection;

use crate::{AppResult, sql::db::Database};

#[derive(Debug, Clone, Default)]
pub struct DatabaseConfig {
    autoinstall: bool,
}

impl DatabaseConfig {
    pub fn with_autoinstall(mut self, autoinstall: bool) -> Self {
        self.autoinstall = autoinstall;
        self
    }

    pub fn open_in_memory(self) -> AppResult<Database> {
        Ok(Database {
            conn: Connection::open_in_memory_with_flags(self.to_duckdb()?)?,
        })
    }

    fn to_duckdb(&self) -> AppResult<duckdb::Config> {
        Ok(duckdb::Config::default()
            .with("autoinstall_known_extensions", self.autoinstall.to_string())?)
    }
}
