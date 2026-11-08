use crate::{
    AppResult,
    duckdb::{file_db::FileDBMetadata, helper::quote},
};
use duckdb::{AccessMode, Config, Connection};
use std::path::Path;

pub fn inspect(path: &Path, rows: usize) -> AppResult<()> {
    match FileDBMetadata::read(path)? {
        Some(metadata) => println!("{}", serde_json::to_string_pretty(&metadata)?),
        None => println!("no metadata"),
    }
    let conn =
        Connection::open_with_flags(path, Config::default().access_mode(AccessMode::ReadOnly)?)?;
    let tables = conn
        .prepare("SELECT table_name FROM duckdb_tables() ORDER BY table_name")?
        .query_map([], |r| r.get::<_, String>(0))?
        .collect::<Result<Vec<_>, _>>()?;
    for table in tables {
        let table = quote(&table);
        let count: i64 =
            conn.query_row(&format!("SELECT count(*) FROM {table}"), [], |r| r.get(0))?;
        println!("\n{table}: {count} rows");
        let columns = conn
            .prepare(&format!(
                "SELECT column_name, column_type FROM (DESCRIBE {table})"
            ))?
            .query_map([], |r| {
                Ok(format!(
                    "  {}: {}",
                    r.get::<_, String>(0)?,
                    r.get::<_, String>(1)?
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?;
        println!("{}", columns.join("\n"));
        let sample = conn
            .prepare(&format!(
                "SELECT CAST(t AS VARCHAR) FROM (SELECT * FROM {table} LIMIT ?) t"
            ))?
            .query_map([rows], |r| r.get::<_, String>(0))?
            .collect::<Result<Vec<_>, _>>()?;
        for row in sample {
            println!("  {row}");
        }
    }
    Ok(())
}
