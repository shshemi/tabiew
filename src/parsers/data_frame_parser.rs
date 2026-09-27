use polars::frame::DataFrame;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::parsers::auto_parser::AutoParser;

pub struct DataFrameParser<'a> {
    auto_parser: AutoParser<'a>,
}

impl<'a> DataFrameParser<'a> {
    pub fn parse_strict(&self, df: &mut DataFrame) {
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
