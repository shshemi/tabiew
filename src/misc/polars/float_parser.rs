use anyhow::bail;
use polars::{datatypes::DataType, prelude::ChunkCast, series::Series};

use crate::AppResult;

#[derive(Debug)]
pub struct FloatParser {
    dtype: DataType,
}

impl FloatParser {
    pub fn parse(&self, series: &Series) -> AppResult<Series> {
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

impl Default for FloatParser {
    fn default() -> Self {
        Self {
            dtype: DataType::Float64,
        }
    }
}
