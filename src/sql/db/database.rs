use duckdb::{Connection, Params};

use crate::{AppResult, sql::db::Statement};

#[derive(Debug)]
pub struct Database {
    pub(super) conn: Connection,
}

impl Database {
    pub fn autoinstall(&self) -> AppResult<bool> {
        Ok(self.conn.query_row(
            "SELECT current_setting('autoinstall_known_extensions')",
            [],
            |row| row.get(0),
        )?)
    }

    pub fn set_autoinstall(&self, enabled: bool) -> AppResult<()> {
        self.conn
            .execute(&format!("SET autoinstall_known_extensions = {enabled}"), [])?;
        Ok(())
    }

    pub fn execute<P: Params>(&self, sql: &str, params: P) -> AppResult<usize> {
        Ok(self.conn.execute(sql, params)?)
    }

    pub fn execute_batch(&self, sql: &str) -> AppResult<()> {
        Ok(self.conn.execute_batch(sql)?)
    }

    pub fn prepare(&self, sql: &str) -> AppResult<Statement<'_>> {
        Ok(Statement {
            stmt: self.conn.prepare(sql)?,
        })
    }
}
