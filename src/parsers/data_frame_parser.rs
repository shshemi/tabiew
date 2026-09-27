use polars::frame::DataFrame;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::parsers::auto_parser::AutoParser;

#[derive(Default, Clone)]
pub struct DataFrameParser<'a> {
    auto_parser: AutoParser<'a>,
}

impl<'a> DataFrameParser<'a> {
    pub fn with_bool(self, enabled: bool) -> Self {
        Self {
            auto_parser: self.auto_parser.with_bool(enabled),
        }
    }

    pub fn with_date(self, enabled: bool) -> Self {
        Self {
            auto_parser: self.auto_parser.with_date(enabled),
        }
    }

    pub fn with_datetime(self, enabled: bool) -> Self {
        Self {
            auto_parser: self.auto_parser.with_datetime(enabled),
        }
    }

    pub fn with_float(self, enabled: bool) -> Self {
        Self {
            auto_parser: self.auto_parser.with_float(enabled),
        }
    }

    pub fn with_int(self, enabled: bool) -> Self {
        Self {
            auto_parser: self.auto_parser.with_int(enabled),
        }
    }

    pub fn parse_and_update(&self, df: &mut DataFrame) {
        df.columns()
            .par_iter()
            .filter(|c| c.dtype().is_string())
            .filter_map(move |column| self.auto_parser.clone().parse_strict(column).ok())
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|column| {
                let _ = df.with_column(column);
            });
    }
}
