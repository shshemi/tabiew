use std::path::PathBuf;

use polars::{
    error::PolarsResult,
    frame::DataFrame,
    prelude::{AnyValue, DataType, IntoLazy, LazyFrame},
    series::Series,
};
use polars_sql::SQLContext;

use crate::misc::type_ext::SnakeCaseNameGenExt;

use super::{polars_ext::AnyValueExt, vec_map::VecMap};

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

    pub fn register(
        &mut self,
        name: &str,
        data_frame: DataFrame,
        input: impl Into<Source>,
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

    pub fn execute(
        &mut self,
        query: &str,
        default_table: impl Into<Option<DataFrame>>,
    ) -> PolarsResult<DataFrame> {
        if let Some(data_frame) = default_table.into() {
            self.sql.register("_", data_frame.lazy());
        }
        let mut df = self.sql.execute(query).and_then(LazyFrame::collect)?;
        df.as_single_chunk_par();
        Ok(df)
    }
}

impl Default for SqlBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Default)]
pub struct BackendSchema {
    schema: VecMap<String, TableInfo>,
}

impl BackendSchema {
    pub fn insert(&mut self, name: String, info: TableInfo) {
        self.schema.insert(name, info);
    }

    pub fn remove(&mut self, name: &str) {
        self.schema.remove(name);
    }

    pub fn available_name(&self, preferred: &str) -> String {
        preferred
            .snake_case_names()
            .find(|name| !self.schema.contains(name) && name != DEFAULT_TABLE_NAME)
            .expect("Unable to find a name")
    }

    pub fn get(&self, name: &str) -> Option<&TableInfo> {
        self.schema.get(name)
    }

    pub fn get_by_index(&self, idx: usize) -> Option<(&String, &TableInfo)> {
        self.schema.get_by_index(idx)
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

#[derive(Debug)]
pub struct TableInfo {
    source: Source,
    height: usize,
    width: usize,
    total_null: usize,
    total_est_size: usize,
    schema: TableSchema,
}

impl TableInfo {
    pub fn new(input: Source, df: &DataFrame) -> Self {
        let schema = TableSchema::new(df);
        Self {
            source: input,
            height: df.height(),
            width: df.width(),
            total_null: schema.iter().map(|(_, info)| info.null_count()).sum(),
            total_est_size: schema.iter().map(|(_, info)| info.estimated_size()).sum(),
            schema,
        }
    }

    pub fn source(&self) -> &Source {
        &self.source
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

    pub fn total_est_size(&self) -> usize {
        self.total_est_size
    }

    pub fn schema(&self) -> &TableSchema {
        &self.schema
    }
}

#[derive(Debug, Clone)]
pub enum Source {
    File(PathBuf),
    Stdin,
    User,
}

impl Source {
    pub fn display_path(&self) -> String {
        match self {
            Source::File(path_buf) => path_buf.to_string_lossy().into_owned(),
            Source::Stdin => "Standard Input".to_owned(),
            Source::User => "User".to_owned(),
        }
    }
}

impl From<crate::reader::Source> for Source {
    fn from(value: crate::reader::Source) -> Self {
        match value {
            crate::reader::Source::File(path_buf) => Source::File(path_buf),
            crate::reader::Source::Stdin => Source::Stdin,
        }
    }
}

#[derive(Debug)]
pub struct TableSchema {
    schema: VecMap<String, FieldInfo>,
}

impl TableSchema {
    pub fn new(df: &DataFrame) -> Self {
        Self {
            schema: df
                .iter()
                .map(|ser| (ser.name().to_string(), FieldInfo::new(ser)))
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

#[derive(Debug)]
pub struct FieldInfo {
    dtype: DataType,
    est_size: usize,
    null_count: usize,
    min: String,
    max: String,
}

impl FieldInfo {
    pub fn new(series: &Series) -> Self {
        let (min, max) = min_max(series);
        Self {
            dtype: series.dtype().to_owned(),
            est_size: series.estimated_size(),
            null_count: series.null_count(),
            min,
            max,
        }
    }
    pub fn dtype(&self) -> &DataType {
        &self.dtype
    }

    pub fn estimated_size(&self) -> usize {
        self.est_size
    }

    pub fn null_count(&self) -> usize {
        self.null_count
    }

    pub fn min(&self) -> &str {
        &self.min
    }

    pub fn max(&self) -> &str {
        &self.max
    }
}

fn min_max(series: &Series) -> (String, String) {
    let dtype = series.dtype();
    if dtype.is_primitive_numeric()
        || matches!(
            dtype,
            DataType::Time | DataType::Date | DataType::Datetime(_, _)
        )
    {
        let (a, b) =
            series
                .iter()
                .fold((AnyValue::Null, AnyValue::Null), |(mut min, mut max), a| {
                    if a < min || matches!(min, AnyValue::Null) {
                        min = a;
                    } else if a > max || matches!(max, AnyValue::Null) {
                        max = a;
                    }
                    (min, max)
                });
        (a.into_single_line(), b.into_single_line())
    } else {
        ("-".to_owned(), "-".to_owned())
    }
}
