use anyhow::bail;
use polars::{datatypes::DataType, prelude::ChunkCast, series::Series};

use crate::AppResult;

#[derive(Debug)]
pub struct GeneralParser {
    dtype: DataType,
}

impl GeneralParser {
    pub fn new(dtype: DataType) -> Self {
        Self { dtype }
    }

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

    pub fn parse(&self, series: &Series) -> AppResult<Series> {
        if series.dtype() == &self.dtype {
            return Ok(series.clone());
        }
        Ok(series.str()?.cast(&self.dtype)?)
    }

    pub fn parse_strict(&self, series: &Series) -> AppResult<Series> {
        let parsed = self.parse(series)?;
        if parsed.null_count() == series.null_count() {
            Ok(parsed)
        } else {
            bail!(
                "Series '{}' could not be parsed as {}",
                series.name(),
                self.dtype
            )
        }
    }
}
