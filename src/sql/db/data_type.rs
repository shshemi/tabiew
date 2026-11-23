use anyhow::anyhow;

use crate::AppResult;

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

impl Type {
    pub(super) fn from_column(
        logical: &duckdb::core::LogicalTypeHandle,
        arrow: &duckdb::arrow::datatypes::DataType,
    ) -> AppResult<Self> {
        use duckdb::arrow::datatypes::DataType as A;
        use duckdb::core::LogicalTypeId as L;
        Ok(match logical.id() {
            L::SqlNull => Self::Null,
            L::Boolean => Self::Boolean,
            L::Tinyint => Self::TinyInt,
            L::Smallint => Self::SmallInt,
            L::Integer => Self::Int,
            L::Bigint => Self::BigInt,
            L::Hugeint => Self::HugeInt,
            L::UHugeint => Self::UHugeInt,
            L::UTinyint => Self::UTinyInt,
            L::USmallint => Self::USmallInt,
            L::UInteger => Self::UInt,
            L::UBigint => Self::UBigInt,
            L::Float => Self::Float,
            L::Double => Self::Double,
            L::Decimal => Self::Decimal,
            L::Timestamp => Self::Timestamp,
            L::TimestampS => Self::Timestamp,
            L::TimestampMs => Self::Timestamp,
            L::TimestampNs => Self::Timestamp,
            L::TimestampTZ => Self::Timestamp,
            L::Varchar => Self::Text,
            L::Uuid => Self::Text,
            L::Blob => Self::Blob,
            L::Bit => Self::Blob,
            L::Bignum => Self::Blob,
            L::Geometry => Self::Geometry,
            L::Date => Self::Date32,
            L::Time => Self::Time64,
            L::TimeNs => Self::Time64,
            L::TimeTZ => Self::Time64,
            L::Interval => Self::Interval,
            L::Enum => Self::Enum,
            L::Union => Self::Union,
            L::Variant => Self::Variant,
            L::Any => Self::Any,
            L::List => match arrow {
                A::List(item) | A::LargeList(item) => Self::List(Box::new(Self::from_column(
                    &logical.child(0),
                    item.data_type(),
                )?)),
                other => return Err(anyhow!("list column has arrow type {other}")),
            },
            L::Array => match arrow {
                A::FixedSizeList(item, size) => Self::Array(
                    Box::new(Self::from_column(&logical.child(0), item.data_type())?),
                    u32::try_from(*size)?,
                ),
                other => return Err(anyhow!("array column has arrow type {other}")),
            },
            L::Struct => match arrow {
                A::Struct(fields) => {
                    let mut types = Vec::with_capacity(fields.len());
                    for (idx, field) in fields.iter().enumerate() {
                        types.push((
                            logical.child_name(idx),
                            Self::from_column(&logical.child(idx), field.data_type())?,
                        ));
                    }
                    Self::Struct(types)
                }
                other => return Err(anyhow!("struct column has arrow type {other}")),
            },
            L::Map => match arrow {
                A::Map(entries, _) => match entries.data_type() {
                    A::Struct(fields) if fields.len() == 2 => Self::Map(
                        Box::new(Self::from_column(&logical.child(0), fields[0].data_type())?),
                        Box::new(Self::from_column(&logical.child(1), fields[1].data_type())?),
                    ),
                    other => return Err(anyhow!("map column has arrow entries {other}")),
                },
                other => return Err(anyhow!("map column has arrow type {other}")),
            },
            L::Invalid => return Err(anyhow!("invalid column type")),
            L::StringLiteral => return Err(anyhow!("unresolved string literal type")),
            L::IntegerLiteral => return Err(anyhow!("unresolved integer literal type")),
            L::Unsupported => return Err(anyhow!("unsupported column type")),
            other => return Err(anyhow!("unknown column type {other:?}")),
        })
    }
}
