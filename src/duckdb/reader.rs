use crate::{
    AppResult,
    duckdb::{DuckDB, session::SessionConn, sql::quote},
    misc::config::config,
    reader::{Reader, RowSource},
};

pub struct ViewSource {
    conn: SessionConn,
    view: String,
    columns: Vec<(String, String)>,
    row_count: usize,
    select: String,
}

impl DuckDB {
    pub fn reader(&self, view: &str) -> AppResult<Reader<ViewSource>> {
        Ok(Reader::new(
            ViewSource::new(self.session.session()?, view)?,
            self.reader,
        ))
    }
}

impl ViewSource {
    pub fn new(conn: SessionConn, view: &str) -> AppResult<Self> {
        let columns: Vec<(String, String)> = conn
            .prepare(&format!("DESCRIBE {}", quote(view)))?
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
            .collect::<Result<_, _>>()?;
        let row_count =
            conn.query_row(&format!("SELECT count(*) FROM {}", quote(view)), [], |r| {
                r.get::<_, i64>(0)
            })? as usize;
        let select = columns
            .iter()
            .map(|(name, data_type)| match is_float(data_type) {
                true => quote(name),
                false => format!("CAST({} AS VARCHAR)", quote(name)),
            })
            .collect::<Vec<_>>()
            .join(", ");
        Ok(Self {
            conn,
            view: view.to_string(),
            columns,
            row_count,
            select,
        })
    }
    pub fn view(&self) -> &str {
        &self.view
    }
}

impl RowSource for ViewSource {
    fn columns(&self) -> Vec<(String, String)> {
        self.columns.clone()
    }
    fn row_count(&self) -> usize {
        self.row_count
    }
    fn fetch(&self, offset: usize, len: usize) -> AppResult<Vec<Vec<String>>> {
        let width = self.columns.len();
        let precision = config().fp_precision();
        let floats = self
            .columns
            .iter()
            .map(|(_, data_type)| is_float(data_type))
            .collect::<Vec<_>>();
        Ok(self
            .conn
            .prepare(&format!(
                "SELECT {} FROM {} LIMIT {len} OFFSET {offset}",
                self.select,
                quote(&self.view),
            ))?
            .query_map([], |r| {
                (0..width)
                    .map(|idx| {
                        Ok(match floats[idx] {
                            true => r
                                .get::<_, Option<f64>>(idx)?
                                .map(|value| match precision {
                                    Some(precision) => format!("{value:.precision$}"),
                                    None => value.to_string(),
                                })
                                .unwrap_or_default(),
                            false => r.get::<_, Option<String>>(idx)?.unwrap_or_default(),
                        })
                    })
                    .collect()
            })?
            .collect::<Result<_, _>>()?)
    }
}

fn is_float(data_type: &str) -> bool {
    matches!(data_type, "DOUBLE" | "FLOAT")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{duckdb::session::Session, reader::ReaderConfig};

    #[test]
    fn view_source_reads_typed_columns_as_text() {
        let mut session = Session::new(None).unwrap();
        let view = session
            .create_query(
                "numbers",
                &[],
                "SELECT range AS id, CASE WHEN range % 2 = 0 THEN 'even' END AS kind, range / 4 AS ratio FROM range(1000)",
            )
            .unwrap();
        let mut reader = Reader::new(
            ViewSource::new(session.session().unwrap(), &view).unwrap(),
            ReaderConfig {
                segment_rows: 100,
                segments: 3,
            },
        );
        assert_eq!(
            reader.columns(),
            [
                ("id".to_string(), "BIGINT".to_string()),
                ("kind".to_string(), "VARCHAR".to_string()),
                ("ratio".to_string(), "DOUBLE".to_string())
            ]
        );
        assert_eq!(reader.row_count(), 1000);
        let rows = reader
            .rows(499..502)
            .unwrap()
            .map(<[String]>::to_vec)
            .collect::<Vec<_>>();
        assert_eq!(
            rows,
            [
                ["499", "", "124.75"],
                ["500", "even", "125"],
                ["501", "", "125.25"]
            ]
        );
    }
}
