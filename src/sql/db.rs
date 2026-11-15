use duckdb::{
    Connection, Params, RowIndex,
    arrow::array::{ArrayRef, FixedSizeListArray, MapArray, StructArray},
    types::{Decimal, EnumType, ListType, TimeUnit},
};

use crate::AppResult;

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

#[derive(Debug)]
pub struct Database {
    conn: Connection,
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

pub struct Statement<'conn> {
    stmt: duckdb::Statement<'conn>,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Null,
    Boolean,
    TinyInt,
    SmallInt,
    Int,
    BigInt,
    HugeInt,
    UHugeInt,
    UTinyInt,
    USmallInt,
    UInt,
    UBigInt,
    Float,
    Double,
    Decimal,
    Timestamp,
    Text,
    Blob,
    Geometry,
    Date32,
    Time64,
    Interval,
    List(Box<Type>),
    Enum,
    Struct(Vec<(String, Type)>),
    Map(Box<Type>, Box<Type>),
    Array(Box<Type>, u32),
    Union,
    Variant,
    Any,
}

impl From<duckdb::types::Type> for Type {
    fn from(value: duckdb::types::Type) -> Self {
        use duckdb::types::Type as D;
        match value {
            D::Null => Self::Null,
            D::Boolean => Self::Boolean,
            D::TinyInt => Self::TinyInt,
            D::SmallInt => Self::SmallInt,
            D::Int => Self::Int,
            D::BigInt => Self::BigInt,
            D::HugeInt => Self::HugeInt,
            D::UHugeInt => Self::UHugeInt,
            D::UTinyInt => Self::UTinyInt,
            D::USmallInt => Self::USmallInt,
            D::UInt => Self::UInt,
            D::UBigInt => Self::UBigInt,
            D::Float => Self::Float,
            D::Double => Self::Double,
            D::Decimal => Self::Decimal,
            D::Timestamp => Self::Timestamp,
            D::Text => Self::Text,
            D::Blob => Self::Blob,
            D::Geometry => Self::Geometry,
            D::Date32 => Self::Date32,
            D::Time64 => Self::Time64,
            D::Interval => Self::Interval,
            D::List(item) => Self::List(Box::new((*item).into())),
            D::Enum => Self::Enum,
            D::Struct(fields) => Self::Struct(
                fields
                    .into_iter()
                    .map(|(name, ty)| (name, ty.into()))
                    .collect(),
            ),
            D::Map(key, value) => Self::Map(Box::new((*key).into()), Box::new((*value).into())),
            D::Array(item, size) => Self::Array(Box::new((*item).into()), size),
            D::Union => Self::Union,
            D::Variant => Self::Variant,
            D::Any => Self::Any,
            other => unimplemented!("unsupported duckdb type: {other:?}"),
        }
    }
}

impl From<Type> for duckdb::types::Type {
    fn from(value: Type) -> Self {
        use duckdb::types::Type as D;
        match value {
            Type::Null => D::Null,
            Type::Boolean => D::Boolean,
            Type::TinyInt => D::TinyInt,
            Type::SmallInt => D::SmallInt,
            Type::Int => D::Int,
            Type::BigInt => D::BigInt,
            Type::HugeInt => D::HugeInt,
            Type::UHugeInt => D::UHugeInt,
            Type::UTinyInt => D::UTinyInt,
            Type::USmallInt => D::USmallInt,
            Type::UInt => D::UInt,
            Type::UBigInt => D::UBigInt,
            Type::Float => D::Float,
            Type::Double => D::Double,
            Type::Decimal => D::Decimal,
            Type::Timestamp => D::Timestamp,
            Type::Text => D::Text,
            Type::Blob => D::Blob,
            Type::Geometry => D::Geometry,
            Type::Date32 => D::Date32,
            Type::Time64 => D::Time64,
            Type::Interval => D::Interval,
            Type::List(item) => D::List(Box::new((*item).into())),
            Type::Enum => D::Enum,
            Type::Struct(fields) => D::Struct(
                fields
                    .into_iter()
                    .map(|(name, ty)| (name, ty.into()))
                    .collect(),
            ),
            Type::Map(key, value) => D::Map(Box::new((*key).into()), Box::new((*value).into())),
            Type::Array(item, size) => D::Array(Box::new((*item).into()), size),
            Type::Union => D::Union,
            Type::Variant => D::Variant,
            Type::Any => D::Any,
        }
    }
}
