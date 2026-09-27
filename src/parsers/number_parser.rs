use anyhow::bail;
use polars::{
    datatypes::DataType,
    frame::column::{Column, IntoColumn},
    prelude::ChunkCast,
};

use crate::AppResult;

#[derive(Debug, Clone)]
pub struct NumberParser {
    dtype: DataType,
}

impl NumberParser {
    pub fn int() -> Self {
        Self {
            dtype: DataType::Int64,
        }
    }

    pub fn float() -> Self {
        Self {
            dtype: DataType::Float64,
        }
    }

    pub fn parse(&self, column: &Column) -> AppResult<Column> {
        Ok(column.str()?.cast(&self.dtype)?.into_column())
    }

    pub fn parse_strict(&self, column: &Column) -> AppResult<Column> {
        let parsed = self.parse(column)?;
        if parsed.null_count() == column.null_count() {
            Ok(parsed)
        } else {
            bail!(
                "Column '{}' could not be parsed as {}",
                column.name(),
                self.dtype
            )
        }
    }
}
