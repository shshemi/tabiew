use crate::{
    AppResult,
    duckdb::{DuckDB, session::SessionConn, sql::quote},
    reader::{Reader, RowSource},
};

pub struct ViewSource {
    conn: SessionConn,
    view: String,
    columns: Vec<String>,
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
        let columns: Vec<String> = conn
            .prepare(&format!("DESCRIBE {}", quote(view)))?
            .query_map([], |r| r.get(0))?
            .collect::<Result<_, _>>()?;
        let row_count =
            conn.query_row(&format!("SELECT count(*) FROM {}", quote(view)), [], |r| {
                r.get::<_, i64>(0)
            })? as usize;
        let select = columns
            .iter()
            .map(|name| format!("CAST({} AS VARCHAR)", quote(name)))
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
    fn columns(&self) -> Vec<String> {
        self.columns.clone()
    }
    fn row_count(&self) -> usize {
        self.row_count
    }
    fn fetch(&self, offset: usize, len: usize) -> AppResult<Vec<Vec<String>>> {
        let width = self.columns.len();
        Ok(self
            .conn
            .prepare(&format!(
                "SELECT {} FROM {} LIMIT {len} OFFSET {offset}",
                self.select,
                quote(&self.view),
            ))?
            .query_map([], |r| {
                (0..width)
                    .map(|idx| Ok(r.get::<_, Option<String>>(idx)?.unwrap_or_default()))
                    .collect()
            })?
            .collect::<Result<_, _>>()?)
    }
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
                "SELECT range AS id, CASE WHEN range % 2 = 0 THEN 'even' END AS kind FROM range(1000)",
            )
            .unwrap();
        let mut reader = Reader::new(
            ViewSource::new(session.session().unwrap(), &view).unwrap(),
            ReaderConfig {
                segment_rows: 100,
                segments: 3,
            },
        );
        assert_eq!(reader.columns(), ["id", "kind"]);
        assert_eq!(reader.row_count(), 1000);
        let rows = reader
            .rows(499..502)
            .unwrap()
            .map(<[String]>::to_vec)
            .collect::<Vec<_>>();
        assert_eq!(rows, [["499", ""], ["500", "even"], ["501", ""]]);
    }
}
