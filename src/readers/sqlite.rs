use std::{io::Read, path::Path};

use itertools::Itertools;
use polars::{
    frame::DataFrame,
    prelude::{AnyValue, DataType, PlSmallStr, Schema},
};
use rusqlite::Connection;
use tempfile::NamedTempFile;

use crate::{AppResult, args::Args, misc::stdin::stdin, readers::ReaderSource};

use super::{DataFrameReader, NamedFrames};

#[derive(Debug, Default)]
pub struct SqliteToDataFrames {
    key: Option<String>,
}

impl SqliteToDataFrames {
    pub fn from_args(args: &Args) -> Self {
        Self {
            key: args.sqlite_key.clone(),
        }
    }

    pub fn key(self, key: String) -> Self {
        SqliteToDataFrames { key: Some(key) }
    }
}

impl DataFrameReader for SqliteToDataFrames {
    fn read_to_data_frames(&self, input: ReaderSource) -> AppResult<NamedFrames> {
        match input {
            ReaderSource::File(path) => path_to_name_frames(path, self.key.as_deref()),
            ReaderSource::Stdin => {
                let temp_file = NamedTempFile::new()?;
                let mut buf = Vec::new();
                stdin().read_to_end(&mut buf)?;
                std::fs::write(temp_file.path(), buf)?;
                path_to_name_frames(temp_file.path(), self.key.as_deref())
            }
        }
    }
}

fn path_to_name_frames(path: impl AsRef<Path>, key: Option<&str>) -> AppResult<NamedFrames> {
    let conn = Connection::open(path)?;

    if let Some(key) = key {
        conn.pragma_update(None, "key", key)?;
    }
    // Fetch table names
    let names = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%';")?
        .query_map([], |row| row.get(0))?
        .collect::<Result<Vec<String>, _>>()?;

    // Iterate over tables and transform them into data_frames
    names
        .into_iter()
        .map(|name| {
            let df = get_data_frame(&conn, &name)?;
            Ok((name, df))
        })
        .collect::<AppResult<Vec<_>>>()
        .map(|vec| vec.into_boxed_slice())
}

fn get_data_frame(conn: &Connection, table_name: &str) -> AppResult<DataFrame> {
    let mut stmt = conn.prepare(&format!(
        "SELECT * FROM \"{}\"",
        table_name.replace('"', "\"\"")
    ))?;
    let names = stmt
        .column_names()
        .into_iter()
        .map(PlSmallStr::from)
        .collect_vec();
    let rows = stmt
        .query_map([], sqlite_to_polars_row)?
        .collect::<Result<Vec<_>, _>>()?;
    let schema = Schema::from_iter(names.into_iter().enumerate().map(|(idx, name)| {
        let dtype = rows
            .iter()
            .filter_map(|row| row.0.get(idx))
            .map(AnyValue::dtype)
            .fold(DataType::Null, merge_dtypes);
        match dtype {
            DataType::Null => (name, DataType::String),
            dtype => (name, dtype),
        }
    }));
    let rows = rows
        .into_iter()
        .map(|row| {
            row.0
                .into_iter()
                .zip(schema.iter_values())
                .map(|(value, dtype)| match (value, dtype) {
                    (AnyValue::Int64(n), DataType::String) => {
                        AnyValue::StringOwned(n.to_string().into())
                    }
                    (AnyValue::Float64(f), DataType::String) => {
                        AnyValue::StringOwned(f.to_string().into())
                    }
                    (AnyValue::BinaryOwned(buf), DataType::String) => {
                        AnyValue::StringOwned(String::from_utf8_lossy(&buf).into_owned().into())
                    }
                    (value, _) => value,
                })
                .collect_vec()
        })
        .map(polars::frame::row::Row)
        .collect_vec();
    Ok(DataFrame::from_rows_and_schema(&rows, &schema)?)
}

fn merge_dtypes(lhs: DataType, rhs: DataType) -> DataType {
    match (lhs, rhs) {
        (DataType::Null, dtype) | (dtype, DataType::Null) => dtype,
        (lhs, rhs) if lhs == rhs => lhs,
        (DataType::Int64, DataType::Float64) | (DataType::Float64, DataType::Int64) => {
            DataType::Float64
        }
        _ => DataType::String,
    }
}

fn sqlite_to_polars_row<'a>(
    row: &rusqlite::Row,
) -> Result<polars::frame::row::Row<'a>, rusqlite::Error> {
    (0..row.as_ref().column_count())
        .map(|idx| row.get_ref(idx))
        .map_ok(|value| match value {
            rusqlite::types::ValueRef::Null => AnyValue::Null,
            rusqlite::types::ValueRef::Integer(n) => AnyValue::Int64(n),
            rusqlite::types::ValueRef::Real(f) => AnyValue::Float64(f),
            rusqlite::types::ValueRef::Text(buf) => {
                AnyValue::StringOwned(String::from_utf8_lossy(buf).into_owned().into())
            }
            rusqlite::types::ValueRef::Blob(buf) => AnyValue::BinaryOwned(buf.to_owned()),
        })
        .collect::<Result<Vec<_>, _>>()
        .map(polars::frame::row::Row)
}
