use anyhow::bail;
use polars::series::Series;

use crate::{
    AppResult,
    misc::{
        self,
        polars::{
            bool_parser::BoolParser, date_parser::DateParser, datetime_parser::DatetimeParser,
            general_parser::GeneralParser,
        },
    },
};

pub struct AutoParser<'a> {
    bool_parser: BoolParser,
    date_parser: DateParser,
    datetime_parser: DatetimeParser<'a>,
    float_parser: GeneralParser,
    int_parser: GeneralParser,
}

impl<'a> AutoParser<'a> {
    pub fn parse_strict(&mut self, series: &Series) -> AppResult<Series> {
        if let Ok(s) = self.parse_int(series) {
            return Ok(s);
        }
        if let Ok(s) = self.parse_float(series) {
            return Ok(s);
        }
        if let Ok(s) = self.parse_bool(series) {
            return Ok(s);
        }
        if let Ok(s) = self.parse_date(series) {
            return Ok(s);
        }
        if let Ok(s) = self.parse_datetime(series) {
            return Ok(s);
        }
        bail!("Series '{}' could not be parsed", series.name())
    }

    fn parse_datetime(&mut self, series: &Series) -> AppResult<Series> {
        self.datetime_parser.set_format(None);
        if let Ok(s) = self.datetime_parser.parse_strict(series) {
            return Ok(s);
        }
        for fmt in misc::polars::datetime_parser::PREDEFINED_FORMATS {
            self.datetime_parser.set_format(fmt);
            if let Ok(s) = self.datetime_parser.parse_strict(series) {
                return Ok(s);
            }
        }
        bail!("No matching format found")
    }

    fn parse_date(&mut self, series: &Series) -> AppResult<Series> {
        self.date_parser.set_format(None);
        if let Ok(s) = self.date_parser.parse_strict(series) {
            return Ok(s);
        }
        for fmt in misc::polars::date_parser::PREDEFINED_FORMATS {
            self.date_parser.set_format(fmt);
            if let Ok(s) = self.date_parser.parse_strict(series) {
                return Ok(s);
            }
        }
        bail!("No matching format found")
    }

    fn parse_bool(&self, series: &Series) -> AppResult<Series> {
        self.bool_parser.parse_strict(series)
    }

    fn parse_float(&self, series: &Series) -> AppResult<Series> {
        self.float_parser.parse_strict(series)
    }

    fn parse_int(&self, series: &Series) -> AppResult<Series> {
        self.int_parser.parse_strict(series)
    }
}

impl Default for AutoParser<'_> {
    fn default() -> Self {
        Self {
            bool_parser: BoolParser::default(),
            date_parser: DateParser::default(),
            datetime_parser: DatetimeParser::default(),
            float_parser: GeneralParser::float(),
            int_parser: GeneralParser::int(),
        }
    }
}
