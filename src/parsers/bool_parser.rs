use anyhow::bail;
use polars::{
    prelude::{ChunkSet, DataType},
    series::{ChunkCompareEq, IntoSeries, Series},
};

use crate::AppResult;

#[derive(Debug)]
pub struct BoolParser {
    true_value: &'static str,
    false_value: &'static str,
}

impl BoolParser {
    pub fn parse(&self, series: &Series) -> AppResult<Series> {
        if series.dtype() == &DataType::Boolean {
            return Ok(series.clone());
        }
        let ca = series.str()?;
        let is_true = ca.equal(self.true_value);
        let is_false = ca.equal(self.false_value);
        let invalid = !(&is_true | &is_false);
        Ok(is_true
            .set(&invalid, None)?
            .with_name(series.name().clone())
            .into_series())
    }

    pub fn parse_strict(&self, series: &Series) -> AppResult<Series> {
        if series.dtype() == &DataType::Boolean {
            return Ok(series.clone());
        }
        let ca = series.str()?;
        let is_true = ca.equal(self.true_value);
        if (&is_true | &ca.equal(self.false_value)).all() {
            Ok(is_true.with_name(series.name().clone()).into_series())
        } else {
            bail!(
                "Series '{}' could not be parsed as {}",
                series.name(),
                DataType::Boolean
            )
        }
    }
}

impl Default for BoolParser {
    fn default() -> Self {
        Self {
            true_value: "true",
            false_value: "false",
        }
    }
}
