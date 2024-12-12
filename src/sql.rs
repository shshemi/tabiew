use std::{collections::BTreeMap, path::PathBuf};

use itertools::Itertools;
use polars::{
    error::PolarsResult,
    frame::DataFrame,
    prelude::{IntoLazy, LazyFrame, NamedFrom},
    series::Series,
};
use polars_sql::SQLContext;

use crate::utils::type_ext::SnakeCaseNameGenExt;

pub struct SqlBackend {
    sql: SQLContext,
    tables: BTreeMap<String, (String, PathBuf)>,
}

impl SqlBackend {
    pub fn new() -> Self {
        Self {
            sql: SQLContext::new(),
            tables: Default::default(),
        }
    }

    pub fn schema(&self) -> DataFrame {
        let (tables, structures, paths) = self.tables.iter().fold(
            (Vec::new(), Vec::new(), Vec::new()),
            |(mut vt, mut vs, mut vp), (t, (s, p))| {
                vt.push(t.to_owned());
                vs.push(s.to_owned());
                vp.push(p.to_string_lossy().into_owned());
                (vt, vs, vp)
            },
        );
        [
            Series::new("Table".into(), tables),
            Series::new("Structure".into(), structures),
            Series::new("Path".into(), paths),
        ]
        .into_iter()
        .collect()
    }

    pub fn contains_dataframe(&self, name: &str) -> bool {
        self.tables.contains_key(name)
    }

    pub fn register(&mut self, name: &str, data_frame: DataFrame, path: PathBuf) -> String {
        if let Some(name) = name.snake_case_names().find(|name| !self.tables.contains_key(name)) {
            self.tables
                .insert(name.clone(), (data_frame_structure(&data_frame), path));
            self.sql.register(&name, data_frame.lazy());
            name
        } else {
            panic!("Not implemented")
        }
    }

    pub fn execute(&mut self, query: &str) -> PolarsResult<DataFrame> {
        self.sql.execute(query).and_then(LazyFrame::collect)
    }
}

impl Default for SqlBackend {
    fn default() -> Self {
        Self::new()
    }
}


fn data_frame_structure(df: &DataFrame) -> String {
    format!(
        "({})",
        df.iter()
            .map(|series| format!("{} {}", series.name().trim(), series.dtype()))
            .join(", ")
    )
}

#[cfg(test)]
mod tests {
    use polars::df;

    use super::*;

    #[test]
    fn test_data_frame_structure() {
        // Create a sample DataFrame
        let df = df![
            "name" => ["Alice", "Bob", "Charlie"],
            "age" => [25, 30, 35],
            " space " => [1, 1, 1],
            "salary" => [50000.0, 60000.0, 70000.0],
            "married" => [true, false, false],
        ]
        .unwrap();

        // Expected output
        assert_eq!(
            data_frame_structure(&df),
            "(name str, age i32, space i32, salary f64, married bool)"
        );
    }
}
