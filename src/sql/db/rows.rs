use crate::{AppResult, sql::db::Row};

pub struct Rows<'stmt> {
    pub(super) rows: duckdb::Rows<'stmt>,
}

impl<'stmt> Rows<'stmt> {
    pub fn column_count(&self) -> usize {
        self.rows.as_ref().map_or(0, |stmt| stmt.column_count())
    }

    pub fn next<'a>(&mut self) -> AppResult<Option<Row<'a, '_>>> {
        Ok(self.rows.next()?.map(|row| Row { row }))
    }
}
