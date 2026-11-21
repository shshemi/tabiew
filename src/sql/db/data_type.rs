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
