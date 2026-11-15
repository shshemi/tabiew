use std::path::Path;

use duckdb::types::Value;

use crate::{
    AppResult,
    sql::{
        db::Database,
        ops::{
            reply::{Receiver, channel},
            util::{pairs_to_duck_db_value, quote_ident, table_name, utf8_path},
        },
        worker::submit,
    },
};

#[derive(Debug, Clone, Default)]
pub struct ImportParquet {
    binary_as_string: Option<Value>,
    can_have_nan: Option<Value>,
    int96_as: Option<Value>,
    utf8_validation: Option<Value>,
    encryption_config: Option<Value>,

    explicit_cardinality: Option<Value>,
    prefetch_strategy: Option<Value>,

    filename: Option<Value>,
    file_row_number: Option<Value>,
    union_by_name: Option<Value>,
    hive_partitioning: Option<Value>,
    hive_types: Option<Value>,
    hive_types_autocast: Option<Value>,
    allow_empty: Option<Value>,
}

impl ImportParquet {
    pub fn with_binary_as_string(mut self, binary_as_string: bool) -> Self {
        self.binary_as_string = Some(Value::Boolean(binary_as_string));
        self
    }

    pub fn with_can_have_nan(mut self, can_have_nan: bool) -> Self {
        self.can_have_nan = Some(Value::Boolean(can_have_nan));
        self
    }

    pub fn with_int96_as(mut self, int96_as: impl Into<String>) -> Self {
        self.int96_as = Some(Value::Text(int96_as.into()));
        self
    }

    pub fn with_utf8_validation(mut self, utf8_validation: impl Into<String>) -> Self {
        self.utf8_validation = Some(Value::Text(utf8_validation.into()));
        self
    }

    pub fn with_encryption_config(mut self, footer_key: impl Into<String>) -> Self {
        self.encryption_config =
            pairs_to_duck_db_value(vec![("footer_key".to_owned(), footer_key.into())]);
        self
    }

    pub fn with_explicit_cardinality(mut self, explicit_cardinality: u64) -> Self {
        self.explicit_cardinality = Some(Value::UBigInt(explicit_cardinality));
        self
    }

    pub fn with_prefetch_strategy(mut self, prefetch_strategy: impl Into<String>) -> Self {
        self.prefetch_strategy = Some(Value::Text(prefetch_strategy.into()));
        self
    }

    pub fn with_filename(mut self, filename: bool) -> Self {
        self.filename = Some(Value::Boolean(filename));
        self
    }

    pub fn with_file_row_number(mut self, file_row_number: bool) -> Self {
        self.file_row_number = Some(Value::Boolean(file_row_number));
        self
    }

    pub fn with_union_by_name(mut self, union_by_name: bool) -> Self {
        self.union_by_name = Some(Value::Boolean(union_by_name));
        self
    }

    pub fn with_hive_partitioning(mut self, hive_partitioning: bool) -> Self {
        self.hive_partitioning = Some(Value::Boolean(hive_partitioning));
        self
    }

    pub fn with_hive_types(mut self, hive_types: Vec<(String, String)>) -> Self {
        self.hive_types = pairs_to_duck_db_value(hive_types);
        self
    }

    pub fn with_hive_types_autocast(mut self, hive_types_autocast: bool) -> Self {
        self.hive_types_autocast = Some(Value::Boolean(hive_types_autocast));
        self
    }

    pub fn with_allow_empty(mut self, allow_empty: bool) -> Self {
        self.allow_empty = Some(Value::Boolean(allow_empty));
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

        let mut sql = format!(
            "CREATE TABLE {} AS SELECT * FROM read_parquet(?",
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
            ("binary_as_string", &self.binary_as_string),
            ("can_have_nan", &self.can_have_nan),
            ("int96_as", &self.int96_as),
            ("utf8_validation", &self.utf8_validation),
            ("encryption_config", &self.encryption_config),
            ("explicit_cardinality", &self.explicit_cardinality),
            ("prefetch_strategy", &self.prefetch_strategy),
            ("filename", &self.filename),
            ("file_row_number", &self.file_row_number),
            ("union_by_name", &self.union_by_name),
            ("hive_partitioning", &self.hive_partitioning),
            ("hive_types", &self.hive_types),
            ("hive_types_autocast", &self.hive_types_autocast),
            ("allow_empty", &self.allow_empty),
        ]
        .into_iter()
        .filter_map(|(option, value)| Some((option, value.clone()?)))
        .collect()
    }
}
