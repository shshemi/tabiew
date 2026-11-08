use duckdb::{
    Connection, Params, RowIndex,
    arrow::array::{ArrayRef, FixedSizeListArray, MapArray, StructArray},
    types::{Decimal, EnumType, ListType, TimeUnit},
};

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
        Ok(self.row.get_ref(idx)?.into())
    }
}

/// A borrowed cell value; mirrors [`duckdb::types::ValueRef`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValueRef<'a> {
    Null,
    Boolean(bool),
    TinyInt(i8),
    SmallInt(i16),
    Int(i32),
    BigInt(i64),
    HugeInt(i128),
    UHugeInt(u128),
    UTinyInt(u8),
    USmallInt(u16),
    UInt(u32),
    UBigInt(u64),
    Float(f32),
    Double(f64),
    Decimal(Decimal),
    Timestamp(TimeUnit, i64),
    Text(&'a [u8]),
    Blob(&'a [u8]),
    Geometry(&'a [u8]),
    Date32(i32),
    Time64(TimeUnit, i64),
    Interval { months: i32, days: i32, nanos: i64 },
    List(ListType<'a>, usize),
    Enum(EnumType<'a>, usize),
    Struct(&'a StructArray, usize),
    Array(&'a FixedSizeListArray, usize),
    Map(&'a MapArray, usize),
    Union(&'a ArrayRef, usize),
}

impl<'a> From<duckdb::types::ValueRef<'a>> for ValueRef<'a> {
    fn from(value: duckdb::types::ValueRef<'a>) -> Self {
        use duckdb::types::ValueRef as D;
        match value {
            D::Null => Self::Null,
            D::Boolean(b) => Self::Boolean(b),
            D::TinyInt(i) => Self::TinyInt(i),
            D::SmallInt(i) => Self::SmallInt(i),
            D::Int(i) => Self::Int(i),
            D::BigInt(i) => Self::BigInt(i),
            D::HugeInt(i) => Self::HugeInt(i),
            D::UHugeInt(u) => Self::UHugeInt(u),
            D::UTinyInt(u) => Self::UTinyInt(u),
            D::USmallInt(u) => Self::USmallInt(u),
            D::UInt(u) => Self::UInt(u),
            D::UBigInt(u) => Self::UBigInt(u),
            D::Float(f) => Self::Float(f),
            D::Double(f) => Self::Double(f),
            D::Decimal(d) => Self::Decimal(d),
            D::Timestamp(unit, t) => Self::Timestamp(unit, t),
            D::Text(s) => Self::Text(s),
            D::Blob(b) => Self::Blob(b),
            D::Geometry(wkb) => Self::Geometry(wkb),
            D::Date32(d) => Self::Date32(d),
            D::Time64(unit, t) => Self::Time64(unit, t),
            D::Interval {
                months,
                days,
                nanos,
            } => Self::Interval {
                months,
                days,
                nanos,
            },
            D::List(list, idx) => Self::List(list, idx),
            D::Enum(dict, idx) => Self::Enum(dict, idx),
            D::Struct(arr, idx) => Self::Struct(arr, idx),
            D::Array(arr, idx) => Self::Array(arr, idx),
            D::Map(arr, idx) => Self::Map(arr, idx),
            D::Union(arr, idx) => Self::Union(arr, idx),
            other => unimplemented!("unsupported duckdb value: {other:?}"),
        }
    }
}
