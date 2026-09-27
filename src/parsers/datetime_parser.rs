use anyhow::bail;
use polars::{
    datatypes::{StringChunked, TimeUnit, TimeZone},
    series::{IntoSeries, Series},
    time::chunkedarray::StringMethods,
};

use crate::AppResult;

pub const AMBIGUOUS_EARLIEST: &str = "earliest";
pub const AMBIGUOUS_RAISE: &str = "raise";
pub const AMBIGUOUS_LATEST: &str = "latest";
pub const AMBIGUOUS_NULL: &str = "null";
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

#[derive(Debug)]
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

    pub fn parse(&self, series: &Series) -> AppResult<Series> {
        Ok(series
            .str()?
            .as_datetime(
                self.fmt,
                self.tu,
                self.use_cache,
                self.tz_aware,
                self.tz,
                &self.ambiguous,
            )?
            .into_series())
    }

    pub fn parse_strict(&self, series: &Series) -> AppResult<Series> {
        let parsed = self.parse(series)?;
        if parsed.null_count() == series.null_count() {
            Ok(parsed)
        } else {
            bail!(
                "Series '{}' could not be parsed as datetime with format '{}'",
                series.name(),
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
