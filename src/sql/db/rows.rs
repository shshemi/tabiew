use anyhow::anyhow;

use crate::{
    AppResult,
    sql::db::{Row, Type},
};

pub struct Rows<'stmt> {
    pub(super) rows: duckdb::Rows<'stmt>,
}

impl<'stmt> Rows<'stmt> {
    pub fn column_count(&self) -> usize {
        self.rows.as_ref().map_or(0, |stmt| stmt.column_count())
    }

    pub fn column_names(&self) -> Vec<String> {
        self.rows
            .as_ref()
            .map_or_else(Vec::new, |stmt| stmt.column_names())
    }

    pub fn column_type(&self, idx: usize) -> AppResult<Type> {
        let stmt = self
            .rows
            .as_ref()
            .ok_or_else(|| anyhow!("rows have no statement"))?;
        if idx >= stmt.column_count() {
            return Err(anyhow!("column {idx} is out of range"));
        }
        Type::from_column(&stmt.column_logical_type(idx), &stmt.column_type(idx))
    }

    pub fn next<'a>(&mut self) -> AppResult<Option<Row<'a, '_>>> {
        Ok(self.rows.next()?.map(|row| Row { row }))
    }
}
