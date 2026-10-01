use clap::Parser;
use polars::frame::DataFrame;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    args::{Args, Type},
    parsers::auto_parser::AutoParser,
};

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

    pub fn from_env_args() -> Self {
        let args = Args::parse_from(std::env::args_os());
        if args.no_type_inference {
            DataFrameParser::default()
        } else {
            args.infer_types
                .inner()
                .iter()
                .fold(DataFrameParser::default(), |p, t| match t {
                    Type::All => p
                        .with_int(true)
                        .with_float(true)
                        .with_bool(true)
                        .with_date(true)
                        .with_datetime(true),
                    Type::Int => p.with_int(true),
                    Type::Float => p.with_float(true),
                    Type::Boolean => p.with_bool(true),
                    Type::Date => p.with_date(true),
                    Type::Datetime => p.with_datetime(true),
                })
        }
    }
}
