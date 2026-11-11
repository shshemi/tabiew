use crate::{
    AppResult,
    args::{Args, Type},
    duckdb::{
        ingester::{FileIngester, IngestProgress, progress::StepProgress},
        session::SessionConn,
        sql::quote,
    },
};
use duckdb::params;
use serde::Serialize;
use std::{path::PathBuf, sync::Arc};

const REJECTS: &str = "_tw_rejects";

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CsvOptions {
    pub delimiter: char,
    pub quote: char,
    pub header: bool,
    pub ignore_errors: bool,
    pub null_padding: bool,
    pub all_varchar: bool,
    pub type_candidates: Option<Vec<String>>,
}

impl Default for CsvOptions {
    fn default() -> Self {
        Self {
            delimiter: ',',
            quote: '"',
            header: true,
            ignore_errors: false,
            null_padding: false,
            all_varchar: false,
            type_candidates: Some(vec!["BIGINT".to_string(), "DOUBLE".to_string()]),
        }
    }
}

impl CsvOptions {
    pub fn from_args(args: &Args) -> Self {
        Self {
            delimiter: args.separator,
            quote: args.quote_char,
            header: !args.no_header,
            ignore_errors: args.ignore_errors,
            null_padding: args.truncate_ragged_lines,
            all_varchar: args.no_type_inference,
            type_candidates: type_candidates(args.infer_types.inner()),
        }
    }
}

pub struct CsvIngester {
    path: PathBuf,
    options: CsvOptions,
    progress: Arc<StepProgress>,
}

impl CsvIngester {
    pub fn new(path: PathBuf, options: CsvOptions) -> Self {
        Self {
            path,
            options,
            progress: StepProgress::new(1),
        }
    }
    fn sql(&self, alias: &str, varchar: &[String], rejects: bool) -> String {
        let mut options = String::new();
        if let Some(types) = &self.options.type_candidates {
            let types = types
                .iter()
                .map(String::as_str)
                .chain(["VARCHAR"])
                .map(literal)
                .collect::<Vec<_>>();
            options.push_str(&format!(", auto_type_candidates = [{}]", types.join(", ")));
        }
        if !varchar.is_empty() {
            let types = varchar
                .iter()
                .map(|column| format!("{}: 'VARCHAR'", literal(column)))
                .collect::<Vec<_>>();
            options.push_str(&format!(", types = {{{}}}", types.join(", ")));
        }
        let errors = match rejects {
            true => format!(
                "store_rejects = true, rejects_table = '{REJECTS}', rejects_scan = '{REJECTS}_scan', rejects_limit = 1000"
            ),
            false => format!("ignore_errors = {}", self.options.ignore_errors),
        };
        format!(
            "CREATE OR REPLACE TABLE {}.data AS SELECT * FROM read_csv(?, delim = ?, quote = ?, header = ?, {errors}, null_padding = ?, all_varchar = ?{options})",
            quote(alias)
        )
    }
    fn run(
        &self,
        conn: &SessionConn,
        alias: &str,
        varchar: &[String],
        rejects: bool,
    ) -> duckdb::Result<usize> {
        conn.execute(
            &self.sql(alias, varchar, rejects),
            params![
                self.path.to_string_lossy(),
                self.options.delimiter.to_string(),
                self.options.quote.to_string(),
                self.options.header,
                self.options.null_padding,
                self.options.all_varchar,
            ],
        )
    }
    fn rejected(&self, conn: &SessionConn, alias: &str) -> AppResult<Option<Vec<String>>> {
        self.run(conn, alias, &[], true)?;
        let rejects = conn
            .prepare(&format!(
                "SELECT DISTINCT CAST(error_type AS VARCHAR), column_name FROM {REJECTS}"
            ))?
            .query_map([], |r| {
                Ok((r.get::<_, String>(0)?, r.get::<_, Option<String>>(1)?))
            })?
            .collect::<Result<Vec<_>, _>>()?;
        if rejects.is_empty() {
            return Ok(None);
        }
        Ok(Some(
            rejects
                .into_iter()
                .filter(|(kind, _)| kind == "CAST")
                .filter_map(|(_, column)| column)
                .collect(),
        ))
    }
}

impl FileIngester for CsvIngester {
    fn progress(&self) -> Arc<dyn IngestProgress> {
        self.progress.clone()
    }
    fn ingest(&mut self, conn: SessionConn, alias: &str) -> AppResult<()> {
        self.progress.start(&conn);
        let mut varchar = Vec::<String>::new();
        if !self.options.ignore_errors {
            conn.execute_batch("BEGIN")?;
            match self.rejected(&conn, alias) {
                Ok(None) => {
                    conn.execute_batch(&format!(
                        "COMMIT; DROP TABLE IF EXISTS {REJECTS}; DROP TABLE IF EXISTS {REJECTS}_scan"
                    ))?;
                    return Ok(());
                }
                Ok(Some(columns)) => {
                    conn.execute_batch("ROLLBACK")?;
                    varchar = columns;
                }
                Err(e) => {
                    let _ = conn.execute_batch("ROLLBACK");
                    return Err(e);
                }
            }
        }
        loop {
            match self.run(&conn, alias, &varchar, false) {
                Ok(_) => return Ok(()),
                Err(e) => match failed_column(&e.to_string()) {
                    Some(column) if !varchar.contains(&column) => varchar.push(column),
                    _ => return Err(e.into()),
                },
            }
        }
    }
}

pub(in crate::duckdb::ingester) fn type_candidates(types: &[Type]) -> Option<Vec<String>> {
    types
        .iter()
        .map(|t| {
            match t {
                Type::All => None,
                Type::Int => Some("BIGINT"),
                Type::Float => Some("DOUBLE"),
                Type::Boolean => Some("BOOLEAN"),
                Type::Date => Some("DATE"),
                Type::Datetime => Some("TIMESTAMP"),
            }
            .map(String::from)
        })
        .collect()
}

fn literal(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}

fn failed_column(error: &str) -> Option<String> {
    let rest = error.split_once("Error when converting column \"")?.1;
    Some(rest.split_once('"')?.0.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::duckdb::session::Session;
    use tempfile::TempDir;

    fn load(data: &str, ignore_errors: bool) -> AppResult<Vec<(String, String, i64)>> {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("late.csv");
        std::fs::write(&path, data).unwrap();
        let mut session = Session::new(None).unwrap();
        let alias = session.attach(None).unwrap();
        CsvIngester::new(
            path,
            CsvOptions {
                ignore_errors,
                ..CsvOptions::default()
            },
        )
        .ingest(session.session()?, &alias)?;
        let conn = session.session()?;
        let types = conn
            .prepare(&format!("DESCRIBE {}.data", quote(&alias)))?
            .query_map([], |r| r.get::<_, String>(1))?
            .collect::<Result<Vec<_>, _>>()?;
        let mut rows = conn.prepare(&format!(
            "SELECT CAST(a AS VARCHAR), CAST(b AS VARCHAR), id FROM {}.data",
            quote(&alias)
        ))?;
        let rows = rows
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
            .collect::<Result<Vec<_>, _>>()?;
        assert_eq!(types[0], "BIGINT");
        Ok(rows
            .into_iter()
            .map(|(a, b, id): (String, String, i64)| (a + ":" + &types[1], b + ":" + &types[2], id))
            .collect())
    }

    fn csv(late: &[(usize, &str)]) -> String {
        let mut data = String::from("id,a,b\n");
        for i in 0..30_000 {
            let line = late
                .iter()
                .find(|(at, _)| *at == i)
                .map(|(_, line)| line.to_string())
                .unwrap_or_else(|| format!("{i},{},{}.5", i % 7, i % 3));
            data.push_str(&line);
            data.push('\n');
        }
        data
    }

    #[test]
    fn late_values_that_break_a_type_keep_every_row_in_order() {
        let rows = load(&csv(&[(25_000, "25000,x,y"), (29_000, "29000,1,z")]), false).unwrap();
        assert_eq!(rows.len(), 30_000);
        assert!(rows.iter().enumerate().all(|(i, row)| row.2 == i as i64));
        assert_eq!(rows[25_000].0, "x:VARCHAR");
        assert_eq!(rows[29_000].1, "z:VARCHAR");
        assert_eq!(rows[1].0, "1:VARCHAR");
    }
}
