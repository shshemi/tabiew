use duckdb::Connection;

use crate::{AppResult, duckdb::sql::quote};

const SAMPLE: usize = 1000;

const DATES: [&str; 18] = [
    "%Y-%m-%d", "%Y/%m/%d", "%Y.%m.%d", "%Y %m %d", "%Y%m%d", "%d-%m-%Y", "%d/%m/%Y", "%d.%m.%Y",
    "%d %m %Y", "%d%m%Y", "%m-%d-%Y", "%m/%d/%Y", "%m.%d.%Y", "%m %d %Y", "%m%d%Y", "%B %d %Y",
    "%B-%d-%Y", "%Y-%j",
];

const DATETIMES: [&str; 17] = [
    "%Y-%m-%d %H:%M:%S",
    "%Y-%m-%dT%H:%M:%S",
    "%Y-%m-%dT%H:%M:%S.%f",
    "%Y/%m/%d %H:%M:%S",
    "%Y %m %d %H:%M:%S",
    "%Y.%m.%d %H:%M:%S",
    "%d-%m-%Y %H:%M:%S",
    "%d/%m/%Y %H:%M:%S",
    "%d %m %Y %H:%M:%S",
    "%d.%m.%Y %H:%M:%S",
    "%m-%d-%Y %H:%M:%S",
    "%m/%d/%Y %H:%M:%S",
    "%m %d %Y %H:%M:%S",
    "%m.%d.%Y %H:%M:%S",
    "%B %d %Y %H:%M:%S",
    "%B-%d-%Y %H:%M:%S",
    "%Y%m%dT%H%M%S",
];

pub fn refine(conn: &Connection, alias: &str, types: &[String]) -> AppResult<()> {
    let wanted = |kind: &str| types.iter().any(|t| t == kind);
    let formats = DATES
        .iter()
        .filter(|_| wanted("DATE"))
        .map(|format| (*format, "DATE"))
        .chain(
            DATETIMES
                .iter()
                .filter(|_| wanted("TIMESTAMP"))
                .map(|format| (*format, "TIMESTAMP")),
        )
        .collect::<Vec<_>>();
    if formats.is_empty() {
        return Ok(());
    }
    let columns = conn
        .prepare(
            "SELECT table_name, column_name FROM duckdb_columns() WHERE database_name = ? AND schema_name = 'main' AND data_type = 'VARCHAR'",
        )?
        .query_map([alias], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)))?
        .collect::<Result<Vec<_>, _>>()?;
    let checks = formats
        .iter()
        .map(|(format, _)| {
            format!(
                "coalesce(bool_and(try_strptime(v, {}) IS NOT NULL), false)",
                literal(format)
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    for (table, column) in columns {
        crate::duckdb::cancel::check()?;
        let table = format!("{}.{}", quote(alias), quote(&table));
        let column = quote(&column);
        let matched = conn.query_row(
            &format!(
                "SELECT {checks} FROM (SELECT {column} AS v FROM {table} WHERE {column} IS NOT NULL LIMIT {SAMPLE})"
            ),
            [],
            |row| (0..formats.len()).map(|idx| row.get::<_, bool>(idx)).collect::<Result<Vec<_>, _>>(),
        )?;
        let Some((format, kind)) = matched
            .iter()
            .position(|matched| *matched)
            .map(|idx| formats[idx])
        else {
            continue;
        };
        let format = literal(format);
        let misses: i64 = conn.query_row(
            &format!(
                "SELECT count(*) FROM {table} WHERE {column} IS NOT NULL AND try_strptime({column}, {format}) IS NULL"
            ),
            [],
            |row| row.get(0),
        )?;
        if misses == 0 {
            conn.execute_batch(&format!(
                "ALTER TABLE {table} ALTER {column} SET DATA TYPE {kind} USING CAST(strptime({column}, {format}) AS {kind})"
            ))?;
        }
    }
    Ok(())
}

fn literal(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}
