use anyhow::bail;
use polars::{
    datatypes::{StringChunked, TimeUnit, TimeZone},
    frame::column::{Column, IntoColumn},
    time::chunkedarray::StringMethods,
};

use crate::AppResult;

pub const AMBIGUOUS_EARLIEST: &str = "earliest";
pub const PREDEFINED_FORMATS: [&str; 17] = [
    "%Y-%m-%d %H:%M:%S",
    "%Y-%m-%dT%H:%M:%S",
    "%Y-%m-%dT%H:%M:%S%.f",
    "%Y/%m/%d %H:%M:%S",
    "%Y %m %d %H:%M:%S",
    "%Y.%m.%d %H:%M:%S",
    "%d-%m-%Y %H:%M:%S",
    "%d/%m/%Y %H:%M:%S",
    "%d %m %Y %H:%M:%S",
    "%d.%m.%Y %H:%M:%S",
    "%m-%d-%Y %H:%M:%S",
    "%m/%d/%Y %H:%M:%S",
    "%m %d %Y %H:%M:%S",
    "%m.%d.%Y %H:%M:%S",
    "%B %d %Y %H:%M:%S",
    "%B-%d-%Y %H:%M:%S",
    "%Y%m%dT%H%M%S",
];

#[derive(Debug, Clone)]
pub struct DatetimeParser<'a> {
    fmt: Option<&'static str>,
    tu: TimeUnit,
    use_cache: bool,
    tz_aware: bool,
    tz: Option<&'a TimeZone>,
    ambiguous: StringChunked,
}

impl DatetimeParser<'_> {
    pub fn set_format(&mut self, fmt: impl Into<Option<&'static str>>) {
        self.fmt = fmt.into()
    }

    pub fn parse(&self, column: &Column) -> AppResult<Column> {
        Ok(column
            .str()?
            .as_datetime(
                self.fmt,
                self.tu,
                self.use_cache,
                self.tz_aware,
                self.tz,
                &self.ambiguous,
            )?
            .into_column())
    }

    pub fn parse_strict(&self, column: &Column) -> AppResult<Column> {
        let parsed = self.parse(column)?;
        if parsed.null_count() == column.null_count() {
            Ok(parsed)
        } else {
            bail!(
                "Column '{}' could not be parsed as datetime with format '{}'",
                column.name(),
                self.fmt.unwrap_or("None")
            )
        }
    }
}

impl Default for DatetimeParser<'_> {
    fn default() -> Self {
        Self {
            fmt: None,
            tu: TimeUnit::Milliseconds,
            use_cache: true,
            tz_aware: false,
            tz: None,
            ambiguous: StringChunked::from_iter(std::iter::once(AMBIGUOUS_EARLIEST)),
        }
    }
}
