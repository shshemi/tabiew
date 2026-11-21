use duckdb::RowIndex;

use crate::{AppResult, sql::db::ValueRef};

pub struct Row<'a, 'stmt> {
    pub(super) row: &'a duckdb::Row<'stmt>,
}

impl<'a, 'stmt> Row<'a, 'stmt> {
    pub fn get_ref<I: RowIndex>(&self, idx: I) -> AppResult<ValueRef<'_>> {
        Ok(self.row.get_ref(idx)?.into())
    }
}
