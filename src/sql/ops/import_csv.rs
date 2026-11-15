use std::path::Path;

use duckdb::types::Value;

use crate::{
    AppResult,
    sql::{
        db::Database,
        ops::{
            reply::{Receiver, channel},
            util::{
                Compression, list_to_duck_db_value, pairs_to_duck_db_value, quote_ident,
                table_name, utf8_path,
            },
        },
        worker::submit,
    },
};

#[derive(Debug, Clone, Default)]
pub struct ImportCsv {
    delim: Option<Value>,
    quote: Option<Value>,
    escape: Option<Value>,
    new_line: Option<Value>,
    comment: Option<Value>,
    header: Option<Value>,
    skip: Option<Value>,
    encoding: Option<Value>,
    compression: Option<Value>,

    auto_detect: Option<Value>,
    sample_size: Option<Value>,
    files_to_sniff: Option<Value>,
    auto_type_candidates: Option<Value>,

    columns: Option<Value>,
    types: Option<Value>,
    names: Option<Value>,
    all_varchar: Option<Value>,
    normalize_names: Option<Value>,
    date_format: Option<Value>,
    timestamp_format: Option<Value>,
    decimal_separator: Option<Value>,
    thousands: Option<Value>,

    nullstr: Option<Value>,
    allow_quoted_nulls: Option<Value>,
    force_not_null: Option<Value>,

    ignore_errors: Option<Value>,
    null_padding: Option<Value>,
    strict_mode: Option<Value>,
    max_line_size: Option<Value>,
    store_rejects: Option<Value>,
    rejects_table: Option<Value>,
    rejects_scan: Option<Value>,
    rejects_limit: Option<Value>,
    rejects_line_size_limit: Option<Value>,

    buffer_size: Option<Value>,
    parallel: Option<Value>,

    filename: Option<Value>,
    file_row_number: Option<Value>,
    union_by_name: Option<Value>,
    hive_partitioning: Option<Value>,
    hive_types: Option<Value>,
    hive_types_autocast: Option<Value>,
    allow_empty: Option<Value>,
}

impl ImportCsv {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_delim(mut self, delim: impl Into<String>) -> Self {
        self.delim = Some(Value::Text(delim.into()));
        self
    }

    pub fn with_quote(mut self, quote: char) -> Self {
        self.quote = Some(Value::Text(quote.to_string()));
        self
    }

    pub fn with_escape(mut self, escape: char) -> Self {
        self.escape = Some(Value::Text(escape.to_string()));
        self
    }

    pub fn with_new_line(mut self, new_line: NewLine) -> Self {
        self.new_line = Some(Value::Text(new_line.as_sql().to_owned()));
        self
    }

    pub fn with_comment(mut self, comment: char) -> Self {
        self.comment = Some(Value::Text(comment.to_string()));
        self
    }

    pub fn with_header(mut self, header: bool) -> Self {
        self.header = Some(Value::Boolean(header));
        self
    }

    pub fn with_skip(mut self, skip: i64) -> Self {
        self.skip = Some(Value::BigInt(skip));
        self
    }

    pub fn with_encoding(mut self, encoding: impl Into<String>) -> Self {
        self.encoding = Some(Value::Text(encoding.into()));
        self
    }

    pub fn with_compression(mut self, compression: Compression) -> Self {
        self.compression = Some(Value::Text(compression.as_sql().to_owned()));
        self
    }

    pub fn with_auto_detect(mut self, auto_detect: bool) -> Self {
        self.auto_detect = Some(Value::Boolean(auto_detect));
        self
    }

    pub fn with_sample_size(mut self, sample_size: i64) -> Self {
        self.sample_size = Some(Value::BigInt(sample_size));
        self
    }

    pub fn with_files_to_sniff(mut self, files_to_sniff: i64) -> Self {
        self.files_to_sniff = Some(Value::BigInt(files_to_sniff));
        self
    }

    pub fn with_auto_type_candidates(mut self, auto_type_candidates: Vec<String>) -> Self {
        self.auto_type_candidates = list_to_duck_db_value(auto_type_candidates);
        self
    }

    pub fn with_columns(mut self, columns: Vec<(String, String)>) -> Self {
        self.columns = pairs_to_duck_db_value(columns);
        self
    }

    pub fn with_types(mut self, types: Vec<(String, String)>) -> Self {
        self.types = pairs_to_duck_db_value(types);
        self
    }

    pub fn with_names(mut self, names: Vec<String>) -> Self {
        self.names = list_to_duck_db_value(names);
        self
    }

    pub fn with_all_varchar(mut self, all_varchar: bool) -> Self {
        self.all_varchar = Some(Value::Boolean(all_varchar));
        self
    }

    pub fn with_normalize_names(mut self, normalize_names: bool) -> Self {
        self.normalize_names = Some(Value::Boolean(normalize_names));
        self
    }

    pub fn with_date_format(mut self, date_format: impl Into<String>) -> Self {
        self.date_format = Some(Value::Text(date_format.into()));
        self
    }

    pub fn with_timestamp_format(mut self, timestamp_format: impl Into<String>) -> Self {
        self.timestamp_format = Some(Value::Text(timestamp_format.into()));
        self
    }

    pub fn with_decimal_separator(mut self, decimal_separator: char) -> Self {
        self.decimal_separator = Some(Value::Text(decimal_separator.to_string()));
        self
    }

    pub fn with_thousands(mut self, thousands: char) -> Self {
        self.thousands = Some(Value::Text(thousands.to_string()));
        self
    }

    pub fn with_nullstr(mut self, nullstr: Vec<String>) -> Self {
        self.nullstr = list_to_duck_db_value(nullstr);
        self
    }

    pub fn with_allow_quoted_nulls(mut self, allow_quoted_nulls: bool) -> Self {
        self.allow_quoted_nulls = Some(Value::Boolean(allow_quoted_nulls));
        self
    }

    pub fn with_force_not_null(mut self, force_not_null: Vec<String>) -> Self {
        self.force_not_null = list_to_duck_db_value(force_not_null);
        self
    }

    pub fn with_ignore_errors(mut self, ignore_errors: bool) -> Self {
        self.ignore_errors = Some(Value::Boolean(ignore_errors));
        self
    }

    pub fn with_null_padding(mut self, null_padding: bool) -> Self {
        self.null_padding = Some(Value::Boolean(null_padding));
        self
    }

    pub fn with_strict_mode(mut self, strict_mode: bool) -> Self {
        self.strict_mode = Some(Value::Boolean(strict_mode));
        self
    }

    pub fn with_max_line_size(mut self, max_line_size: i64) -> Self {
        self.max_line_size = Some(Value::BigInt(max_line_size));
        self
    }

    pub fn with_store_rejects(mut self, store_rejects: bool) -> Self {
        self.store_rejects = Some(Value::Boolean(store_rejects));
        self
    }

    pub fn with_rejects_table(mut self, rejects_table: impl Into<String>) -> Self {
        self.rejects_table = Some(Value::Text(rejects_table.into()));
        self
    }

    pub fn with_rejects_scan(mut self, rejects_scan: impl Into<String>) -> Self {
        self.rejects_scan = Some(Value::Text(rejects_scan.into()));
        self
    }

    pub fn with_rejects_limit(mut self, rejects_limit: i64) -> Self {
        self.rejects_limit = Some(Value::BigInt(rejects_limit));
        self
    }

    pub fn with_rejects_line_size_limit(mut self, rejects_line_size_limit: i64) -> Self {
        self.rejects_line_size_limit = Some(Value::BigInt(rejects_line_size_limit));
        self
    }

    pub fn with_buffer_size(mut self, buffer_size: i64) -> Self {
        self.buffer_size = Some(Value::BigInt(buffer_size));
        self
    }

    pub fn with_parallel(mut self, parallel: bool) -> Self {
        self.parallel = Some(Value::Boolean(parallel));
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
            "CREATE TABLE {} AS SELECT * FROM read_csv(?",
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
            ("delim", &self.delim),
            ("quote", &self.quote),
            ("escape", &self.escape),
            ("new_line", &self.new_line),
            ("comment", &self.comment),
            ("header", &self.header),
            ("skip", &self.skip),
            ("encoding", &self.encoding),
            ("compression", &self.compression),
            ("auto_detect", &self.auto_detect),
            ("sample_size", &self.sample_size),
            ("files_to_sniff", &self.files_to_sniff),
            ("auto_type_candidates", &self.auto_type_candidates),
            ("columns", &self.columns),
            ("types", &self.types),
            ("names", &self.names),
            ("all_varchar", &self.all_varchar),
            ("normalize_names", &self.normalize_names),
            ("date_format", &self.date_format),
            ("timestamp_format", &self.timestamp_format),
            ("decimal_separator", &self.decimal_separator),
            ("thousands", &self.thousands),
            ("nullstr", &self.nullstr),
            ("allow_quoted_nulls", &self.allow_quoted_nulls),
            ("force_not_null", &self.force_not_null),
            ("ignore_errors", &self.ignore_errors),
            ("null_padding", &self.null_padding),
            ("strict_mode", &self.strict_mode),
            ("max_line_size", &self.max_line_size),
            ("store_rejects", &self.store_rejects),
            ("rejects_table", &self.rejects_table),
            ("rejects_scan", &self.rejects_scan),
            ("rejects_limit", &self.rejects_limit),
            ("rejects_line_size_limit", &self.rejects_line_size_limit),
            ("buffer_size", &self.buffer_size),
            ("parallel", &self.parallel),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NewLine {
    Lf,
    CrLf,
    Cr,
}

impl NewLine {
    pub fn as_sql(&self) -> &'static str {
        match self {
            NewLine::Lf => "\\n",
            NewLine::CrLf => "\\r\\n",
            NewLine::Cr => "\\r",
        }
    }
}
