use duckdb::{
    arrow::array::{ArrayRef, FixedSizeListArray, MapArray, StructArray},
    types::{Decimal, EnumType, ListType, TimeUnit},
};

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

impl<'a> From<ValueRef<'a>> for duckdb::types::ValueRef<'a> {
    fn from(value: ValueRef<'a>) -> Self {
        use duckdb::types::ValueRef as D;
        match value {
            ValueRef::Null => D::Null,
            ValueRef::Boolean(b) => D::Boolean(b),
            ValueRef::TinyInt(i) => D::TinyInt(i),
            ValueRef::SmallInt(i) => D::SmallInt(i),
            ValueRef::Int(i) => D::Int(i),
            ValueRef::BigInt(i) => D::BigInt(i),
            ValueRef::HugeInt(i) => D::HugeInt(i),
            ValueRef::UHugeInt(u) => D::UHugeInt(u),
            ValueRef::UTinyInt(u) => D::UTinyInt(u),
            ValueRef::USmallInt(u) => D::USmallInt(u),
            ValueRef::UInt(u) => D::UInt(u),
            ValueRef::UBigInt(u) => D::UBigInt(u),
            ValueRef::Float(f) => D::Float(f),
            ValueRef::Double(f) => D::Double(f),
            ValueRef::Decimal(d) => D::Decimal(d),
            ValueRef::Timestamp(unit, t) => D::Timestamp(unit, t),
            ValueRef::Text(s) => D::Text(s),
            ValueRef::Blob(b) => D::Blob(b),
            ValueRef::Geometry(wkb) => D::Geometry(wkb),
            ValueRef::Date32(d) => D::Date32(d),
            ValueRef::Time64(unit, t) => D::Time64(unit, t),
            ValueRef::Interval {
                months,
                days,
                nanos,
            } => D::Interval {
                months,
                days,
                nanos,
            },
            ValueRef::List(list, idx) => D::List(list, idx),
            ValueRef::Enum(dict, idx) => D::Enum(dict, idx),
            ValueRef::Struct(arr, idx) => D::Struct(arr, idx),
            ValueRef::Array(arr, idx) => D::Array(arr, idx),
            ValueRef::Map(arr, idx) => D::Map(arr, idx),
            ValueRef::Union(arr, idx) => D::Union(arr, idx),
        }
    }
}
