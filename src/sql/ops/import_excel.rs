use std::path::Path;

use duckdb::types::Value;

use crate::{
    AppResult,
    sql::{
        db::Database,
        ops::{
            extensions::{Excel, Token},
            reply::{Receiver, channel},
            util::{quote_ident, table_name, utf8_path},
        },
        worker::submit,
    },
};

#[derive(Debug, Clone)]
pub struct ImportExcel {
    sheet: Option<Value>,
    range: Option<Value>,
    header: Option<Value>,
    stop_at_empty: Option<Value>,

    all_varchar: Option<Value>,
    empty_as_varchar: Option<Value>,
    ignore_errors: Option<Value>,
    normalize_names: Option<Value>,
}

impl ImportExcel {
    pub fn new(_token: Token<Excel>) -> Self {
        Self {
            sheet: None,
            range: None,
            header: None,
            stop_at_empty: None,
            all_varchar: None,
            empty_as_varchar: None,
            ignore_errors: None,
            normalize_names: None,
        }
    }

    pub fn with_sheet(mut self, sheet: impl Into<String>) -> Self {
        self.sheet = Some(Value::Text(sheet.into()));
        self
    }

    pub fn with_range(mut self, range: impl Into<String>) -> Self {
        self.range = Some(Value::Text(range.into()));
        self
    }

    pub fn with_header(mut self, header: bool) -> Self {
        self.header = Some(Value::Boolean(header));
        self
    }

    pub fn with_stop_at_empty(mut self, stop_at_empty: bool) -> Self {
        self.stop_at_empty = Some(Value::Boolean(stop_at_empty));
        self
    }

    pub fn with_all_varchar(mut self, all_varchar: bool) -> Self {
        self.all_varchar = Some(Value::Boolean(all_varchar));
        self
    }

    pub fn with_empty_as_varchar(mut self, empty_as_varchar: bool) -> Self {
        self.empty_as_varchar = Some(Value::Boolean(empty_as_varchar));
        self
    }

    pub fn with_ignore_errors(mut self, ignore_errors: bool) -> Self {
        self.ignore_errors = Some(Value::Boolean(ignore_errors));
        self
    }

    pub fn with_normalize_names(mut self, normalize_names: bool) -> Self {
        self.normalize_names = Some(Value::Boolean(normalize_names));
        self
    }

    pub fn run(&self, path: impl AsRef<Path>) -> Receiver<()> {
        let this = self.clone();
        let path = path.as_ref().to_path_buf();
        let (send, recv) = channel();
        submit(move |db| send.send(this.import(db, &path)));
        recv
    }

    fn import(&self, db: &mut Database, path: &Path) -> AppResult<()> {
        let name = table_name(path)?;
        let path = utf8_path(path)?;

        db.prepare("LOAD excel")?.execute([])?;

        let mut sql = format!(
            "CREATE TABLE {} AS SELECT * FROM read_xlsx(?",
            quote_ident(&name)
        );
        let mut params = vec![Value::Text(path.to_owned())];
        for (option, value) in self.options() {
            sql.push_str(", ");
            sql.push_str(option);
            sql.push_str(" = ?");
            params.push(value);
        }
        sql.push(')');

        db.prepare(&sql)?
            .execute(duckdb::params_from_iter(params))?;
        Ok(())
    }

    fn options(&self) -> Vec<(&'static str, Value)> {
        [
            ("sheet", &self.sheet),
            ("range", &self.range),
            ("header", &self.header),
            ("stop_at_empty", &self.stop_at_empty),
            ("all_varchar", &self.all_varchar),
            ("empty_as_varchar", &self.empty_as_varchar),
            ("ignore_errors", &self.ignore_errors),
            ("normalize_names", &self.normalize_names),
        ]
        .into_iter()
        .filter_map(|(option, value)| Some((option, value.clone()?)))
        .collect()
    }
}
