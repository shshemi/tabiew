use anyhow::bail;
use polars::{
    frame::column::{Column, IntoColumn},
    prelude::{ChunkSet, DataType},
    series::ChunkCompareEq,
};

use crate::AppResult;

#[derive(Debug, Clone, Copy)]
pub struct BoolParser {
    true_value: &'static str,
    false_value: &'static str,
}

impl BoolParser {
    pub fn parse(&self, column: &Column) -> AppResult<Column> {
        let ca = column.str()?;
        let is_true = ca.equal(self.true_value);
        let is_false = ca.equal(self.false_value);
        let invalid = !(&is_true | &is_false);
        Ok(is_true
            .set(&invalid, None)?
            .with_name(column.name().clone())
            .into_column())
    }

    pub fn parse_strict(&self, column: &Column) -> AppResult<Column> {
        let ca = column.str()?;
        let is_true = ca.equal(self.true_value);
        if (&is_true | &ca.equal(self.false_value)).all() {
            Ok(is_true.with_name(column.name().clone()).into_column())
        } else {
            bail!(
                "Column '{}' could not be parsed as {}",
                column.name(),
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
