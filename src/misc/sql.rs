use std::{
    borrow::Cow,
    fmt::Display,
    ops::DerefMut,
    path::PathBuf,
    sync::{LazyLock, Mutex},
};

use indexmap::IndexMap;
use polars::{
    error::PolarsResult,
    frame::DataFrame,
    prelude::{DataType, IntoLazy, LazyFrame, Scalar},
    series::Series,
};
use polars_sql::SQLContext;
use url::Url;

use crate::{
    duckdb::{DataSource, TableEntry},
    iters::enumerate_names,
};

use super::unwrap_or_graceful_shutdown::UnwrapOrGracefulShutdown;

const DEFAULT_TABLE_NAME: &str = "_";

pub struct SqlBackend {
    sql: SQLContext,
    schema: BackendSchema,
}

impl SqlBackend {
    pub fn new() -> Self {
        Self {
            sql: SQLContext::new(),
            schema: Default::default(),
        }
    }

    pub fn schema(&self) -> &BackendSchema {
        &self.schema
    }

    pub fn set_schema(&mut self, schema: BackendSchema) {
        self.schema = schema;
    }

    pub fn register(
        &mut self,
        name: &str,
        data_frame: DataFrame,
        input: impl Into<TableSource>,
    ) -> String {
        let name = self.schema.available_name(name);
        self.schema
            .insert(name.clone(), TableInfo::new(input.into(), &data_frame));
        self.sql.register(&name, data_frame.lazy());
        name
    }

    pub fn unregister(&mut self, name: &str) {
        self.schema.remove(name);
        self.sql.unregister(name);
    }

    pub fn unset_default(&mut self) {
        self.sql.unregister("_");
    }

    pub fn execute_with_default(
        &mut self,
        query: &str,
        default_table: impl Into<Option<DataFrame>>,
    ) -> PolarsResult<DataFrame> {
        if let Some(data_frame) = default_table.into() {
            self.sql.register("_", data_frame.lazy());
        }
        let mut df = self.sql.execute(query).and_then(LazyFrame::collect)?;
        df.rechunk_mut_par();
        Ok(df)
    }
}

impl Default for SqlBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Default, Clone)]
pub struct BackendSchema {
    schema: IndexMap<String, TableInfo>,
}

impl BackendSchema {
    pub fn insert(&mut self, name: String, info: TableInfo) {
        self.schema.insert(name, info);
    }

    pub fn remove(&mut self, name: &str) {
        self.schema.shift_remove(name);
    }

    pub fn available_name(&self, preferred: &str) -> String {
        enumerate_names(preferred)
            .find(|name| !self.schema.contains_key(name) && name != DEFAULT_TABLE_NAME)
            .expect("Unable to find a name")
    }

    pub fn get(&self, name: &str) -> Option<&TableInfo> {
        self.schema.get(name)
    }

    pub fn get_by_index(&self, idx: usize) -> Option<(&String, &TableInfo)> {
        self.schema.get_index(idx)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &TableInfo)> {
        self.schema.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.schema.is_empty()
    }

    pub fn len(&self) -> usize {
        self.schema.len()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableInfo {
    origin: TableSource,
    height: usize,
    width: usize,
    total_null: usize,
    total_est_size: Size,
    schema: TableSchema,
    duckdb: Option<bool>,
    known: (bool, bool),
    disk: Option<u64>,
}

impl TableInfo {
    pub fn new(input: TableSource, df: &DataFrame) -> Self {
        let schema = TableSchema::new(df);
        Self {
            origin: input,
            height: df.height(),
            width: df.width(),
            total_null: schema.iter().map(|(_, info)| info.null_count()).sum(),
            total_est_size: schema.iter().map(|(_, info)| info.estimated_size()).sum(),
            schema,
            duckdb: None,
            known: (true, true),
            disk: None,
        }
    }

    pub fn duckdb(entry: &TableEntry) -> Self {
        let nulls = |idx: usize| {
            entry
                .stats
                .as_ref()
                .and_then(|stats| stats.get(idx))
                .map(|stats| stats.nulls as usize)
        };
        let bytes = |idx: usize| {
            entry
                .stats
                .as_ref()
                .and_then(|stats| stats.get(idx))
                .and_then(|stats| stats.bytes)
        };
        let total = (0..entry.columns.len()).map(bytes).sum::<Option<u64>>();
        Self {
            origin: entry
                .source
                .clone()
                .map(TableSource::File)
                .unwrap_or(TableSource::User),
            height: entry.rows.unwrap_or_default(),
            width: entry.columns.len(),
            total_null: (0..entry.columns.len()).filter_map(nulls).sum(),
            total_est_size: Size(total.unwrap_or_default()),
            schema: TableSchema {
                schema: entry
                    .columns
                    .iter()
                    .enumerate()
                    .map(|(idx, (name, data_type))| {
                        let stats = entry.stats.as_ref().and_then(|stats| stats.get(idx));
                        (
                            name.clone(),
                            FieldInfo {
                                dtype: DataType::Null,
                                est_size: Size(bytes(idx).unwrap_or_default()),
                                null_count: nulls(idx).unwrap_or_default(),
                                min: Scalar::null(DataType::Null),
                                max: Scalar::null(DataType::Null),
                                text: Some(FieldText {
                                    dtype: data_type.clone(),
                                    nulls: nulls(idx),
                                    min: stats.and_then(|stats| stats.min.clone()),
                                    max: stats.and_then(|stats| stats.max.clone()),
                                    size: bytes(idx).map(|bytes| Size(bytes).to_string()),
                                }),
                            },
                        )
                    })
                    .collect(),
            },
            duckdb: Some(entry.stats.is_some()),
            known: (entry.rows.is_some(), total.is_some()),
            disk: entry.disk,
        }
    }

    pub fn needs_stats(&self) -> bool {
        self.duckdb == Some(false)
    }

    pub fn total_null_text(&self) -> String {
        match self.duckdb {
            Some(false) => "-".to_string(),
            _ => self.total_null.to_string(),
        }
    }

    pub fn total_est_size_text(&self) -> String {
        match self.known.1 {
            true => self.total_est_size.to_string(),
            false => "-".to_string(),
        }
    }

    pub fn disk_text(&self) -> Option<String> {
        self.disk.map(|disk| match disk {
            0 => "0 B (in memory)".to_string(),
            disk => Size(disk).to_string(),
        })
    }

    pub fn shape_text(&self) -> String {
        match self.known.0 {
            true => format!("{} x {}", self.height, self.width),
            false => format!("- x {}", self.width),
        }
    }

    pub fn source(&self) -> &TableSource {
        &self.origin
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn total_null(&self) -> usize {
        self.total_null
    }

    pub fn total_est_size(&self) -> Size {
        self.total_est_size
    }

    pub fn schema(&self) -> &TableSchema {
        &self.schema
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TableSource {
    Url(Url),
    File(PathBuf),
    Stdin,
    User,
}

impl TableSource {
    pub fn display_path<'a>(&'a self) -> Cow<'a, str> {
        match self {
            TableSource::User => "User".into(),
            TableSource::File(path_buf) => path_buf.to_string_lossy(),
            TableSource::Stdin => "Stdin".into(),
            TableSource::Url(url) => url.as_str().into(),
        }
    }
}

impl From<DataSource> for TableSource {
    fn from(value: DataSource) -> Self {
        match value {
            DataSource::Stdin => Self::Stdin,
            DataSource::File(path_buf) => Self::File(path_buf),
            DataSource::Url(url) => Self::Url(url),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableSchema {
    schema: IndexMap<String, FieldInfo>,
}

impl TableSchema {
    pub fn new(df: &DataFrame) -> Self {
        Self {
            schema: df
                .columns()
                .iter()
                .map(|col| {
                    (
                        col.name().to_string(),
                        FieldInfo::new(col.as_materialized_series()),
                    )
                })
                .collect(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &FieldInfo)> {
        self.schema.iter()
    }

    pub fn len(&self) -> usize {
        self.schema.len()
    }

    pub fn is_empty(&self) -> bool {
        self.schema.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldInfo {
    dtype: DataType,
    est_size: Size,
    null_count: usize,
    min: Scalar,
    max: Scalar,
    text: Option<FieldText>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldText {
    pub dtype: String,
    pub nulls: Option<usize>,
    pub min: Option<String>,
    pub max: Option<String>,
    pub size: Option<String>,
}

impl FieldInfo {
    pub fn new(series: &Series) -> Self {
        // let (min, max) = min_max(series);
        let min = series.min_reduce().unwrap_or_default().into_value();
        let max = series.max_reduce().unwrap_or_default().into_value();
        Self {
            dtype: series.dtype().to_owned(),
            est_size: series.estimated_size().into(),
            null_count: series.null_count(),
            min: Scalar::new(min.dtype(), min),
            max: Scalar::new(max.dtype(), max),
            text: None,
        }
    }

    pub fn text(&self) -> Option<&FieldText> {
        self.text.as_ref()
    }
    pub fn dtype(&self) -> &DataType {
        &self.dtype
    }

    pub fn estimated_size(&self) -> Size {
        self.est_size
    }

    pub fn null_count(&self) -> usize {
        self.null_count
    }

    pub fn min(&self) -> &Scalar {
        &self.min
    }

    pub fn max(&self) -> &Scalar {
        &self.max
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Size(u64);

impl std::iter::Sum for Size {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Size(iter.map(|s| s.0).sum())
    }
}

impl From<usize> for Size {
    fn from(value: usize) -> Self {
        Size(value as u64)
    }
}

impl From<Size> for u64 {
    fn from(value: Size) -> Self {
        value.0
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 < 1024 {
            write!(f, "{} B", self.0)
        } else if self.0 < 1024 * 1024 {
            write!(f, "{:.2} KB", self.0 as f64 / 1024.0)
        } else if self.0 < 1024 * 1024 * 1024 {
            write!(f, "{:.2} MB", self.0 as f64 / (1024.0 * 1024.0))
        } else if self.0 < 1024 * 1024 * 1024 * 1024 {
            write!(f, "{:.2} GB", self.0 as f64 / (1024.0 * 1024.0 * 1024.0))
        } else if self.0 < 1024 * 1024 * 1024 * 1024 * 1024 {
            write!(
                f,
                "{:.2} TB",
                self.0 as f64 / (1024.0 * 1024.0 * 1024.0 * 1024.0)
            )
        } else {
            write!(
                f,
                "{:.2} PB",
                self.0 as f64 / (1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0)
            )
        }
    }
}

pub fn sql() -> impl DerefMut<Target = SqlBackend> {
    static SQL_BACKEND: LazyLock<Mutex<SqlBackend>> =
        LazyLock::new(|| Mutex::new(SqlBackend::default()));
    SQL_BACKEND.lock().unwrap_or_graceful_shutdown()
}
