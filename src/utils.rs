use polars::{datatypes::AnyValue, frame::DataFrame};
use ratatui::{
    layout::Constraint, text::Span, widgets::{Cell, Row, Table}
};

use crate::theme::{Styler, Theme};

pub struct ZipIters<Iter> {
    iterators: Vec<Iter>,
}

impl<Iter, T> Iterator for ZipIters<Iter>
where
    Iter: Iterator<Item = T>,
    T: Clone + Default,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut items = Vec::new();
        let mut any_valid = false;

        for iter in self.iterators.iter_mut() {
            if let Some(item) = iter.next() {
                items.push(item);
                any_valid = true;
            } else {
                items.push(T::default()); // Using default to fill gaps
            }
        }

        if any_valid {
            Some(items)
        } else {
            None // If no valid items, all iterators are exhausted
        }
    }
}

pub fn zip_iters<I1: IntoIterator<Item = I2>, I2: Iterator<Item = T>, T: Clone + Default>(
    iter: I1,
) -> impl Iterator<Item = Vec<T>> {
    ZipIters {
        iterators: iter.into_iter().collect(),
    }
}

pub fn tabulate(data_frame: &polars::prelude::DataFrame) -> Table {
    Table::new(
        rows_from_dataframe(data_frame),
        widths_from_dataframe(data_frame),
    )
    .header(header_from_dataframe(data_frame))
    .highlight_symbol(Span::raw("-> ").style(Theme::table_cell(0, 0)))
    .highlight_style(Theme::table_highlight())
}

fn cell_from_value(value: polars::datatypes::AnyValue) -> Cell {
    Cell::new(match value {
        AnyValue::Null => "".to_owned(),
        AnyValue::Boolean(v) => format!("{}", v),
        AnyValue::String(v) => v.to_string(),
        AnyValue::UInt8(v) => format!("{}", v),
        AnyValue::UInt16(v) => format!("{}", v),
        AnyValue::UInt32(v) => format!("{}", v),
        AnyValue::UInt64(v) => format!("{}", v),
        AnyValue::Int8(v) => format!("{}", v),
        AnyValue::Int16(v) => format!("{}", v),
        AnyValue::Int32(v) => format!("{}", v),
        AnyValue::Int64(v) => format!("{}", v),
        AnyValue::Float32(v) => format!("{}", v),
        AnyValue::Float64(v) => format!("{}", v),
        AnyValue::Date(v) => format!("{}", v),
        AnyValue::Datetime(v1, v2, v3) => format!("{} {} {:?}", v1, v2, v3),
        AnyValue::Duration(v1, v2) => format!("{} {}", v1, v2),
        AnyValue::Time(v) => format!("{}", v),
        AnyValue::List(v) => format!("{}", v),
        AnyValue::StringOwned(v) => format!("{}", v),
        AnyValue::Binary(v) => format!("{:?}", v),
        AnyValue::BinaryOwned(v) => format!("{:?}", v),
        AnyValue::Decimal(v1, v2) => format!("{}.{}", v1, v2),
    })
}

fn widths_from_dataframe(df: &polars::frame::DataFrame) -> Vec<Constraint> {
    df.get_column_names()
        .into_iter()
        .map(|col| Constraint::Min(col.len() as u16))
        .collect::<Vec<_>>()
}

fn rows_from_dataframe(df: &DataFrame) -> Vec<Row> {
    zip_iters(df.iter().map(|series| series.iter()))
        .enumerate()
        .map(|(row_idx, row)| {
            Row::new(
                row.into_iter()
                    .enumerate()
                    .map(|(col_idx, value)| {
                        cell_from_value(value).style(Theme::table_cell(row_idx, col_idx))
                    })
                    .collect::<Vec<_>>(),
            )
            .style(Theme::table_row(row_idx))
        })
        .collect::<Vec<_>>()
}

fn header_from_dataframe(df: &DataFrame) -> Row {
    Row::new(
        df.get_column_names()
            .into_iter()
            .enumerate()
            .map(|(col_idx, name)| Cell::new(name).style(Theme::table_header_cell(col_idx)))
            .collect::<Vec<_>>(),
    )
    .style(Theme::table_header())
}
