use duckdb::Params;

use crate::{AppResult, sql::db::Rows};

pub struct Statement<'conn> {
    pub(super) stmt: duckdb::Statement<'conn>,
}

impl<'conn> Statement<'conn> {
    pub fn execute<P: Params>(&mut self, params: P) -> AppResult<usize> {
        Ok(self.stmt.execute(params)?)
    }

    pub fn query<P: Params>(&mut self, params: P) -> AppResult<Rows<'_>> {
        Ok(Rows {
            rows: self.stmt.query(params)?,
        })
    }
}
