use duckdb::{Connection, Params, RowIndex};

use crate::AppResult;

#[derive(Debug)]
pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> AppResult<Self> {
        Ok(Self {
            conn: duckdb::Connection::open_in_memory()?,
        })
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

pub struct Statement<'conn> {
    stmt: duckdb::Statement<'conn>,
}

impl<'conn> Statement<'conn> {
    pub fn query<P: Params>(&mut self, params: P) -> AppResult<Rows<'_>> {
        Ok(Rows {
            rows: self.stmt.query(params)?,
        })
    }
}

pub struct Rows<'stmt> {
    rows: duckdb::Rows<'stmt>,
}

impl<'stmt> Rows<'stmt> {
    pub fn next<'a>(&mut self) -> AppResult<Option<Row<'a, '_>>> {
        Ok(self.rows.next()?.map(|row| Row { row: row }))
    }
}

pub struct Row<'a, 'stmt> {
    row: &'a duckdb::Row<'stmt>,
}

impl<'a, 'stmt> Row<'a, 'stmt> {
    pub fn get_ref<I: RowIndex>(&self, idx: I) -> AppResult<ValueRef<'_>> {
        let value_ref = self.row.get_ref(idx)?;
        Ok(ValueRef { value_ref })
    }
}

pub struct ValueRef<'a> {
    value_ref: duckdb::types::ValueRef<'a>,
}

impl<'a> ValueRef<'a> {
    pub fn single_line_string(&self) -> String {
        todo!()
    }

    pub fn multi_line_string(&self) -> String {
        todo!()
    }
}
