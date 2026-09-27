use anyhow::anyhow;
use polars::frame::column::Column;

use crate::{
    AppResult,
    parsers::{
        bool_parser::BoolParser, date_parser::DateParser, datetime_parser::DatetimeParser,
        number_parser::NumberParser,
    },
};

#[derive(Default, Clone)]
pub struct AutoParser<'a> {
    bool_parser: Option<BoolParser>,
    date_parser: Option<DateParser>,
    datetime_parser: Option<DatetimeParser<'a>>,
    float_parser: Option<NumberParser>,
    int_parser: Option<NumberParser>,
}

impl<'a> AutoParser<'a> {
    pub fn with_bool(self, enabled: bool) -> Self {
        Self {
            bool_parser: enabled.then(BoolParser::default),
            ..self
        }
    }

    pub fn with_date(self, enabled: bool) -> Self {
        Self {
            date_parser: enabled.then(DateParser::default),
            ..self
        }
    }

    pub fn with_datetime(self, enabled: bool) -> Self {
        Self {
            datetime_parser: enabled.then(DatetimeParser::default),
            ..self
        }
    }

    pub fn with_float(self, enabled: bool) -> Self {
        Self {
            float_parser: enabled.then(NumberParser::float),
            ..self
        }
    }

    pub fn with_int(self, enabled: bool) -> Self {
        Self {
            int_parser: enabled.then(NumberParser::int),
            ..self
        }
    }

    pub fn parse_strict(&mut self, column: &Column) -> AppResult<Column> {
        self.parse_int(column)
            .or_else(|| self.parse_float(column))
            .or_else(|| self.parse_bool(column))
            .or_else(|| self.parse_date(column))
            .or_else(|| self.parse_datetime(column))
            .ok_or_else(|| anyhow!("Column '{}' could not be parsed", column.name()))
    }

    fn parse_datetime(&mut self, column: &Column) -> Option<Column> {
        let parser = self.datetime_parser.as_mut()?;
        parser.set_format(None);
        if let Ok(s) = parser.parse_strict(column) {
            return Some(s);
        }
        for fmt in crate::parsers::datetime_parser::PREDEFINED_FORMATS {
            parser.set_format(fmt);
            if let Ok(s) = parser.parse_strict(column) {
                return Some(s);
            }
        }
        None
    }

    fn parse_date(&mut self, column: &Column) -> Option<Column> {
        let parser = self.date_parser.as_mut()?;
        parser.set_format(None);
        if let Ok(s) = parser.parse_strict(column) {
            return Some(s);
        }
        for fmt in crate::parsers::date_parser::PREDEFINED_FORMATS {
            parser.set_format(fmt);
            if let Ok(s) = parser.parse_strict(column) {
                return Some(s);
            }
        }
        None
    }

    fn parse_bool(&self, column: &Column) -> Option<Column> {
        self.bool_parser.as_ref()?.parse_strict(column).ok()
    }

    fn parse_float(&self, column: &Column) -> Option<Column> {
        self.float_parser.as_ref()?.parse_strict(column).ok()
    }

    fn parse_int(&self, column: &Column) -> Option<Column> {
        self.int_parser.as_ref()?.parse_strict(column).ok()
    }
}
