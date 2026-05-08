use std::io::Read;

use polars::{
    frame::DataFrame,
    prelude::{AnyValue, Column},
};
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

use crate::{
    AppResult,
    args::Args,
    io::reader::{DataFrameReader, NamedFrames, ReaderSource},
    misc::stdin::stdin,
};

#[derive(Debug, Default)]
pub struct MarkdownToDataFrame;

impl MarkdownToDataFrame {
    pub fn from_args(_args: &Args) -> Self {
        Self
    }
}

impl DataFrameReader for MarkdownToDataFrame {
    fn read_to_data_frames(&self, input: ReaderSource) -> AppResult<NamedFrames> {
        let contents = match &input {
            ReaderSource::File(path) => std::fs::read_to_string(path)?,
            ReaderSource::Stdin => {
                let mut s = String::new();
                stdin().read_to_string(&mut s)?;
                s
            }
        };

        let parser = Parser::new_ext(&contents, Options::ENABLE_TABLES);

        collect_tables(parser)
            .into_iter()
            .enumerate()
            .map(|(idx, (headers, rows))| {
                let df = build_data_frame(headers, rows)?;
                Ok((format!("markdown_table_{}", idx + 1), df))
            })
            .collect::<AppResult<Vec<_>>>()
            .map(Vec::into_boxed_slice)
    }
}

fn collect_tables(parser: Parser) -> Vec<(Vec<String>, Vec<Vec<String>>)> {
    let mut tables: Vec<(Vec<String>, Vec<Vec<String>>)> = Vec::new();
    let mut headers: Vec<String> = Vec::new();
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut row: Vec<String> = Vec::new();
    let mut cell = String::new();

    let mut in_head = false;
    let mut in_cell = false;

    for event in parser {
        match event {
            Event::Start(Tag::Table(_)) => {
                headers.clear();
                rows.clear();
            }
            Event::End(TagEnd::Table) => {
                tables.push((std::mem::take(&mut headers), std::mem::take(&mut rows)));
            }
            Event::Start(Tag::TableHead) => in_head = true,
            Event::End(TagEnd::TableHead) => in_head = false,
            Event::Start(Tag::TableRow) => row.clear(),
            Event::End(TagEnd::TableRow) if !in_head => {
                rows.push(std::mem::take(&mut row));
            }
            Event::Start(Tag::TableCell) => {
                in_cell = true;
                cell.clear();
            }
            Event::End(TagEnd::TableCell) => {
                in_cell = false;
                let value = std::mem::take(&mut cell).trim().to_owned();
                if in_head {
                    headers.push(value);
                } else {
                    row.push(value);
                }
            }
            Event::Text(s) | Event::Code(s) if in_cell => cell.push_str(&s),
            Event::SoftBreak | Event::HardBreak if in_cell => cell.push(' '),
            _ => {}
        }
    }

    tables
}

fn build_data_frame(headers: Vec<String>, rows: Vec<Vec<String>>) -> AppResult<DataFrame> {
    let column_count = headers.len();
    let mut columns: Vec<Vec<AnyValue<'static>>> = (0..column_count).map(|_| Vec::new()).collect();
    for row in rows {
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
