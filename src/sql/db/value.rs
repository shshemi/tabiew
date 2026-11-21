use duckdb::types::{Decimal, TimeUnit};

use crate::sql::db::ValueRef;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
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
    Text(String),
    Blob(Vec<u8>),
    Geometry(Vec<u8>),
    Date32(i32),
    Time64(TimeUnit, i64),
    Interval { months: i32, days: i32, nanos: i64 },
    List(Vec<Value>),
    Enum(String),
    Struct(Vec<(String, Value)>),
    Array(Vec<Value>),
    Map(Vec<(Value, Value)>),
    Union(Box<Value>),
}

impl From<duckdb::types::Value> for Value {
    fn from(value: duckdb::types::Value) -> Self {
        use duckdb::types::Value as D;
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
            D::List(items) => Self::List(items.into_iter().map(Into::into).collect()),
            D::Enum(s) => Self::Enum(s),
            D::Struct(fields) => Self::Struct(
                fields
                    .iter()
                    .map(|(name, value)| (name.clone(), value.clone().into()))
                    .collect(),
            ),
            D::Array(items) => Self::Array(items.into_iter().map(Into::into).collect()),
            D::Map(entries) => Self::Map(
                entries
                    .iter()
                    .map(|(key, value)| (key.clone().into(), value.clone().into()))
                    .collect(),
            ),
            D::Union(inner) => Self::Union(Box::new((*inner).into())),
            other => unimplemented!("unsupported duckdb value: {other:?}"),
        }
    }
}

impl From<Value> for duckdb::types::Value {
    fn from(value: Value) -> Self {
        use duckdb::types::Value as D;
        match value {
            Value::Null => D::Null,
            Value::Boolean(b) => D::Boolean(b),
            Value::TinyInt(i) => D::TinyInt(i),
            Value::SmallInt(i) => D::SmallInt(i),
            Value::Int(i) => D::Int(i),
            Value::BigInt(i) => D::BigInt(i),
            Value::HugeInt(i) => D::HugeInt(i),
            Value::UHugeInt(u) => D::UHugeInt(u),
            Value::UTinyInt(u) => D::UTinyInt(u),
            Value::USmallInt(u) => D::USmallInt(u),
            Value::UInt(u) => D::UInt(u),
            Value::UBigInt(u) => D::UBigInt(u),
            Value::Float(f) => D::Float(f),
            Value::Double(f) => D::Double(f),
            Value::Decimal(d) => D::Decimal(d),
            Value::Timestamp(unit, t) => D::Timestamp(unit, t),
            Value::Text(s) => D::Text(s),
            Value::Blob(b) => D::Blob(b),
            Value::Geometry(wkb) => D::Geometry(wkb),
            Value::Date32(d) => D::Date32(d),
            Value::Time64(unit, t) => D::Time64(unit, t),
            Value::Interval {
                months,
                days,
                nanos,
            } => D::Interval {
                months,
                days,
                nanos,
            },
            Value::List(items) => D::List(items.into_iter().map(Into::into).collect()),
            Value::Enum(s) => D::Enum(s),
            Value::Struct(fields) => D::Struct(
                fields
                    .into_iter()
                    .map(|(name, value)| (name, value.into()))
                    .collect::<Vec<_>>()
                    .into(),
            ),
            Value::Array(items) => D::Array(items.into_iter().map(Into::into).collect()),
            Value::Map(entries) => D::Map(
                entries
                    .into_iter()
                    .map(|(key, value)| (key.into(), value.into()))
                    .collect::<Vec<_>>()
                    .into(),
            ),
            Value::Union(inner) => D::Union(Box::new((*inner).into())),
        }
    }
}

impl From<ValueRef<'_>> for Value {
    fn from(value: ValueRef<'_>) -> Self {
        duckdb::types::ValueRef::from(value).to_owned().into()
    }
}
