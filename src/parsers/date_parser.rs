use anyhow::bail;
use polars::{
    series::{IntoSeries, Series},
    time::chunkedarray::StringMethods,
};

use crate::AppResult;

pub const PREDEFINED_FORMATS: [&str; 18] = [
    "%Y-%m-%d", "%Y/%m/%d", "%Y.%m.%d", "%Y %m %d", "%Y%m%d", "%d-%m-%Y", "%d/%m/%Y", "%d.%m.%Y",
    "%d %m %Y", "%d%m%Y", "%m-%d-%Y", "%m/%d/%Y", "%m.%d.%Y", "%m %d %Y", "%m%d%Y", "%B %d %Y",
    "%B-%d-%Y", "%Y-%j",
];

#[derive(Debug)]
pub struct DateParser {
    fmt: Option<&'static str>,
    use_cache: bool,
}

impl DateParser {
    pub fn set_format(&mut self, fmt: impl Into<Option<&'static str>>) {
        self.fmt = fmt.into()
    }

    pub fn parse(&self, series: &Series) -> AppResult<Series> {
        Ok(series
            .str()?
            .as_date(self.fmt, self.use_cache)?
            .into_series())
    }

    pub fn parse_strict(&self, series: &Series) -> AppResult<Series> {
        let parsed = self.parse(series)?;
        if parsed.null_count() == series.null_count() {
            Ok(parsed)
        } else {
            bail!(
                "Series '{}' could not be parsed as date with format '{}'",
                series.name(),
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
