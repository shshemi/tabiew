use std::path::PathBuf;

use indexmap::IndexMap;
use polars::prelude::{DataType, Scalar};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::misc::sql;

#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    pub schema: IndexMap<String, TableInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableInfo {
    pub origin: TableSource,
    pub height: usize,
    pub width: usize,
    pub total_null: usize,
    pub total_est_size: Size,
    pub schema: TableSchema,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TableSource {
    Url(Url),
    File(PathBuf),
    Stdin,
    User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableSchema {
    pub schema: IndexMap<String, FieldInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldInfo {
    pub dtype: DataType,
    pub est_size: Size,
    pub null_count: usize,
    pub min: Scalar,
    pub max: Scalar,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Size(pub usize);

impl From<&sql::BackendSchema> for Schema {
    fn from(value: &sql::BackendSchema) -> Self {
        Self {
            schema: value
                .iter()
                .map(|(name, info)| (name.clone(), info.into()))
                .collect(),
        }
    }
}

impl From<&sql::TableInfo> for TableInfo {
    fn from(value: &sql::TableInfo) -> Self {
        Self {
            origin: value.source().into(),
            height: value.height(),
            width: value.width(),
            total_null: value.total_null(),
            total_est_size: value.total_est_size().into(),
            schema: value.schema().into(),
        }
    }
}

impl From<&sql::TableSource> for TableSource {
    fn from(value: &sql::TableSource) -> Self {
        match value {
            sql::TableSource::Url(url) => Self::Url(url.clone()),
            sql::TableSource::File(path) => Self::File(path.clone()),
            sql::TableSource::Stdin => Self::Stdin,
            sql::TableSource::User => Self::User,
        }
    }
}

impl From<&sql::TableSchema> for TableSchema {
    fn from(value: &sql::TableSchema) -> Self {
        Self {
            schema: value
                .iter()
                .map(|(name, field)| (name.clone(), field.into()))
                .collect(),
        }
    }
}

impl From<&sql::FieldInfo> for FieldInfo {
    fn from(value: &sql::FieldInfo) -> Self {
        Self {
            dtype: value.dtype().clone(),
            est_size: value.estimated_size().into(),
            null_count: value.null_count(),
            min: value.min().clone(),
            max: value.max().clone(),
        }
    }
}

impl From<sql::Size> for Size {
    fn from(value: sql::Size) -> Self {
        Self(value.into())
    }
}
