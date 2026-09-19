use polars::{
    datatypes::{AnyValue, DataType},
    frame::DataFrame,
    series::Series,
};
use rayon::iter::{ParallelBridge, ParallelIterator};
use unicode_width::UnicodeWidthStr;

use crate::tui::misc::any_value_formatter::AnyValueFormatter;

#[derive(Default)]
pub struct DataFrameWidthsCalculator {
    wc: SeriesWidthCalculator,
}

impl DataFrameWidthsCalculator {
    pub fn calculate(&mut self, df: &DataFrame) -> Vec<u16> {
        df.columns()
            .iter()
            .map(|col| self.wc.calculate(col.as_materialized_series()))
            .collect()
    }
}

pub struct SeriesWidthCalculator {
    wc: Vec<AnyValueWidthCalculator>,
}

impl SeriesWidthCalculator {
    pub fn calculate(&mut self, series: &Series) -> u16 {
        let chunk_count = self.wc.len();
        let chunk_len = series.len().div_ceil(chunk_count);
        let name_width = series.name().width() as u16;
        let value_width = match series.dtype() {
            DataType::Null => Some(0),
            DataType::Boolean => Some(5),
            DataType::Date => Some(10),
            DataType::Time => Some(8),
            DataType::Datetime(_, _) => Some(19),
            _ => (0..chunk_count)
                .map(|i| (i * chunk_len) as i64)
                .map(|start| series.slice(start, chunk_len))
                .zip(self.wc.iter_mut())
                .par_bridge()
                .map(|(chunk, wc)| chunk.iter().map(|val| wc.calculate(val)).max())
                .flatten()
                .max(),
        }
        .unwrap_or_default();
        name_width.max(value_width)
    }
}

impl Default for SeriesWidthCalculator {
    fn default() -> Self {
        Self {
            wc: vec![AnyValueWidthCalculator::default(); num_cpus::get()],
        }
    }
}

#[derive(Clone, Default)]
pub struct AnyValueWidthCalculator {
    conv: AnyValueFormatter,
}

impl AnyValueWidthCalculator {
    #[inline]
    pub fn calculate(&mut self, value: AnyValue) -> u16 {
        match value {
            AnyValue::Null => 0,
            AnyValue::Boolean(_) => 5,
            AnyValue::Date(_) => 10, // 1970-10-10
            AnyValue::Time(_) => 8,  // 07:49:05
            AnyValue::Datetime(_, _, _) | AnyValue::DatetimeOwned(_, _, _) => 19, // 2019-06-30 07:49:05
            AnyValue::String(s) => first_line_width(s),
            AnyValue::UInt8(u) => uint_width(u),
            AnyValue::UInt16(u) => uint_width(u),
            AnyValue::UInt32(u) => uint_width(u),
            AnyValue::UInt64(u) => uint_width(u),
            AnyValue::UInt128(u) => uint_width(u),
            AnyValue::Int8(i) => int_width(i),
            AnyValue::Int16(i) => int_width(i),
            AnyValue::Int32(i) => int_width(i),
            AnyValue::Int64(i) => int_width(i),
            AnyValue::Int128(i) => int_width(i),
            AnyValue::Float16(_)
            | AnyValue::Float32(_)
            | AnyValue::Float64(_)
            | AnyValue::Duration(_, _)
            | AnyValue::Decimal(_, _, _)
            | AnyValue::Categorical(_, _)
            | AnyValue::CategoricalOwned(_, _)
            | AnyValue::Enum(_, _)
            | AnyValue::EnumOwned(_, _) => self.table_str_width(value),
            AnyValue::List(series) => uint_width(series.len() as u128) + 8, // [123 items]
            AnyValue::Array(_, n) => uint_width(n as u128) + 8,             // [123 items]
            AnyValue::Struct(_, _, fields) => uint_width(fields.len() as u128) + 9, // {5 fields}
            AnyValue::StructOwned(st) => uint_width(st.0.len() as u128) + 9, // {5 fields}
            AnyValue::StringOwned(s) => first_line_width(&s),
            AnyValue::Binary(items) => uint_width(items.len() as u128) + 8, // [20 bytes]
            AnyValue::BinaryOwned(items) => uint_width(items.len() as u128) + 8, // [20 bytes]
        }
    }

    #[inline]
    fn table_str_width(&mut self, value: AnyValue) -> u16 {
        self.conv.to_single_line(value).width() as u16
    }
}

#[inline]
fn first_line_width(s: &str) -> u16 {
    s.lines().next().unwrap_or_default().width() as u16
}

#[inline]
fn uint_width(value: impl Into<u128>) -> u16 {
    value.into().checked_ilog10().map_or(1, |log| log + 1) as u16
}

#[inline]
fn int_width(value: impl Into<i128>) -> u16 {
    let value = value.into();
    if value.is_negative() {
        uint_width(value.unsigned_abs()) + 1
    } else {
        uint_width(value.unsigned_abs())
    }
}
