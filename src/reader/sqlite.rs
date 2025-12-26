use std::{io::Read, path::Path};

use itertools::Itertools;
use polars::{
    frame::DataFrame,
    prelude::{AnyValue, DataType, PlSmallStr, Schema},
};
use rusqlite::Connection;
use tempfile::NamedTempFile;

use crate::{AppResult, args::Args, misc::globals::stdin, reader::Source};

use super::{NamedFrames, ReadToDataFrames};

#[derive(Default)]
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

impl ReadToDataFrames for SqliteToDataFrames {
    fn named_frames(&self, input: super::Source) -> AppResult<NamedFrames> {
        match input {
            Source::File(path) => path_to_name_frames(path, self.key.as_deref()),
            Source::Stdin => {
                let temp_file = NamedTempFile::new()?;
                let mut buf = Vec::new();
                stdin().read_to_end(&mut buf).unwrap();
                std::fs::write(temp_file.path(), buf).unwrap();
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
    let schema = Schema::from_iter(
        conn.prepare(&format!("PRAGMA table_info({table_name})"))?
            .query_map([], |row| {
                let name = PlSmallStr::from_string(row.get(1)?);
                let dtype = match row.get::<_, String>(2)?.as_str() {
                    "INTEGER" => DataType::Int64,
                    "REAL" => DataType::Float64,
                    "BLOB" => DataType::Binary,
                    _ => DataType::String,
                };
                Ok((name, dtype))
            })?
            .collect::<Result<Vec<_>, _>>()?,
    );
    Ok(DataFrame::from_rows_and_schema(
        &conn
            .prepare(&format!("SELECT * FROM {table_name}"))?
            .query_map([], sqlite_to_polars_row)?
            .collect::<Result<Vec<_>, _>>()?,
        &schema,
    )?)
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
