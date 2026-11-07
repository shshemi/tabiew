use crate::{AppResult, duckdb::helper::quote};
use duckdb::Connection;

pub struct Ingester {}

impl Ingester {
    pub fn create_placeholder(conn: &Connection, alias: &str) -> AppResult<()> {
        conn.execute_batch(&format!(
            "CREATE TABLE IF NOT EXISTS {}.data (placeholder BOOLEAN)",
            quote(alias)
        ))?;
        Ok(())
    }
}
