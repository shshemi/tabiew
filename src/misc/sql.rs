use polars::{
    error::PolarsResult,
    frame::DataFrame,
    prelude::{DataType, IntoLazy, LazyFrame},
    series::Series,
};
use polars_sql::SQLContext;

use crate::{misc::type_ext::SnakeCaseNameGenExt, reader::InputSource};

use super::vec_map::VecMap;

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

    pub fn register(&mut self, name: &str, data_frame: DataFrame, input: InputSource) -> String {
        let name = self.schema.available_name(name);
        self.schema
            .insert(name.clone(), TableInfo::new(input, &data_frame));
        self.sql.register(&name, data_frame.lazy());
        name
    }

    pub fn set_default(&mut self, data_frame: DataFrame) {
        self.sql.register("_", data_frame.lazy());
    }

    pub fn unset_default(&mut self) {
        self.sql.unregister("_");
    }

    pub fn execute(&mut self, query: &str) -> PolarsResult<DataFrame> {
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
    source: InputSource,
    height: usize,
    width: usize,
    total_null: usize,
    total_est_size: usize,
    schema: TableSchema,
}

impl TableInfo {
    pub fn new(input: InputSource, df: &DataFrame) -> Self {
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

    pub fn source(&self) -> &InputSource {
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
}

impl FieldInfo {
    pub fn new(series: &Series) -> Self {
        Self {
            dtype: series.dtype().to_owned(),
            est_size: series.estimated_size(),
            null_count: series.null_count(),
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
}
