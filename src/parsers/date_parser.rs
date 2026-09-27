use anyhow::bail;
use polars::{
    frame::column::{Column, IntoColumn},
    time::chunkedarray::StringMethods,
};

use crate::AppResult;

pub const PREDEFINED_FORMATS: [&str; 18] = [
    "%Y-%m-%d", "%Y/%m/%d", "%Y.%m.%d", "%Y %m %d", "%Y%m%d", "%d-%m-%Y", "%d/%m/%Y", "%d.%m.%Y",
    "%d %m %Y", "%d%m%Y", "%m-%d-%Y", "%m/%d/%Y", "%m.%d.%Y", "%m %d %Y", "%m%d%Y", "%B %d %Y",
    "%B-%d-%Y", "%Y-%j",
];

#[derive(Debug, Clone, Copy)]
pub struct DateParser {
    fmt: Option<&'static str>,
    use_cache: bool,
}

impl DateParser {
    pub fn set_format(&mut self, fmt: impl Into<Option<&'static str>>) {
        self.fmt = fmt.into()
    }

    pub fn parse(&self, column: &Column) -> AppResult<Column> {
        Ok(column
            .str()?
            .as_date(self.fmt, self.use_cache)?
            .into_column())
    }

    pub fn parse_strict(&self, column: &Column) -> AppResult<Column> {
        let parsed = self.parse(column)?;
        if parsed.null_count() == column.null_count() {
            Ok(parsed)
        } else {
            bail!(
                "Column '{}' could not be parsed as date with format '{}'",
                column.name(),
                self.fmt.unwrap_or("None")
            )
        }
    }
}

impl Default for DateParser {
    fn default() -> Self {
        Self {
            fmt: None,
            use_cache: true,
        }
    }
}
