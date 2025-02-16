use itertools::Itertools;
use polars::{
    frame::DataFrame,
    prelude::{AnyValue, DataType, PlSmallStr, Schema},
};
use rusqlite::Connection;
use tempfile::NamedTempFile;

use crate::AppResult;

use super::{NamedFrames, ReadToDataFrames};

pub struct SqliteToDataFrames;

impl ReadToDataFrames for SqliteToDataFrames {
    fn named_frames(&self, input: super::Input) -> AppResult<NamedFrames> {
        let path = match input {
            crate::reader::Input::File(path) => path,
            crate::reader::Input::Stdin => NamedTempFile::new()?.keep()?.1,
        };

        let conn = Connection::open(path)?;

        // Fetch table names
        let names = conn
            .prepare(
                "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%';",
            )?
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<String>, _>>()?;

        // Iterate over tables and transform them into data_frames
        names
            .into_iter()
            .map(|tbl| {
                let df = get_data_frame(&conn, &tbl)?;
                Ok((Some(tbl), df))
            })
            .collect::<AppResult<Vec<_>>>()
            .map(|vec| vec.into_boxed_slice())
    }
}

fn get_data_frame(conn: &Connection, table_name: &str) -> AppResult<DataFrame> {
    let schema = Schema::from_iter(
        conn.prepare(&format!("PRAGMA table_info({})", table_name))?
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
            .prepare(&format!("SELECT * FROM {}", table_name))?
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
