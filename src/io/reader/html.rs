use std::io::Read;

use polars::{
    frame::DataFrame,
    prelude::{AnyValue, Column},
};
use scraper::{ElementRef, Html, Selector};

use crate::{
    AppResult,
    args::Args,
    io::reader::{DataFrameReader, NamedFrames, ReaderSource},
    misc::stdin::stdin,
};

#[derive(Debug, Default)]
pub struct HtmlToDataFrame;

impl HtmlToDataFrame {
    pub fn from_args(_args: &Args) -> Self {
        Self
    }
}

impl DataFrameReader for HtmlToDataFrame {
    fn read_to_data_frames(&self, input: ReaderSource) -> AppResult<NamedFrames> {
        let contents = match &input {
            ReaderSource::File(path) => std::fs::read_to_string(path)?,
            ReaderSource::Stdin => {
                let mut s = String::new();
                stdin().read_to_string(&mut s)?;
                s
            }
        };

        let document = Html::parse_document(&contents);
        let table_selector = Selector::parse("table").unwrap();

        document
            .select(&table_selector)
            .enumerate()
            .map(|(idx, table)| {
                let df = table_to_data_frame(table)?;
                Ok((format!("html_table_{}", idx + 1), df))
            })
            .collect::<AppResult<Vec<_>>>()
            .map(Vec::into_boxed_slice)
    }
}

fn table_to_data_frame(table: ElementRef) -> AppResult<DataFrame> {
    let thead_th_selector = Selector::parse("thead th").unwrap();
    let tr_selector = Selector::parse("tr").unwrap();
    let cell_selector = Selector::parse("th, td").unwrap();

    let header_cells: Vec<String> = table
        .select(&thead_th_selector)
        .map(cell_text)
        .collect();

    let has_thead_headers = !header_cells.is_empty();

    let data_rows: Vec<Vec<String>> = table
        .select(&tr_selector)
        .filter(|tr| {
            if !has_thead_headers {
                return true;
            }
            !tr.ancestors().any(|node| {
                ElementRef::wrap(node)
                    .map(|e| e.value().name() == "thead")
                    .unwrap_or(false)
            })
        })
        .map(|tr| tr.select(&cell_selector).map(cell_text).collect())
        .collect();

    let column_count = if has_thead_headers {
        header_cells.len()
    } else {
        data_rows.iter().map(Vec::len).max().unwrap_or(0)
    };

    let headers: Vec<String> = if has_thead_headers {
        header_cells
    } else {
        (1..=column_count)
            .map(|i| format!("column_{i}"))
            .collect()
    };

    let mut columns: Vec<Vec<AnyValue<'static>>> =
        (0..column_count).map(|_| Vec::new()).collect();
    for row in data_rows {
        let mut row_iter = row.into_iter();
        for col in columns.iter_mut() {
            col.push(match row_iter.next() {
                Some(value) => AnyValue::StringOwned(value.into()),
                None => AnyValue::Null,
            });
        }
    }

    Ok(DataFrame::new_infer_height(
        headers
            .into_iter()
            .zip(columns)
            .map(|(name, values)| Column::new(name.into(), values))
            .collect(),
    )?)
}

fn cell_text(elem: ElementRef) -> String {
    elem.text().collect::<String>().trim().to_owned()
}
