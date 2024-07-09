use std::{collections::BTreeMap, path::PathBuf};

use polars::{
    error::PolarsResult,
    frame::DataFrame,
    prelude::{IntoLazy, LazyFrame, NamedFrom},
    series::Series,
};
use polars_sql::SQLContext;

pub struct SqlBackend {
    sql: SQLContext,
    default: Option<String>,
    tables: BTreeMap<String, PathBuf>,
}

impl SqlBackend {
    pub fn new() -> Self {
        Self {
            sql: SQLContext::new(),
            default: None,
            tables: Default::default(),
        }
    }

    pub fn default_table(&self) -> Option<&str> {
        self.default.as_deref()
    }

    pub fn table_df(&self) -> DataFrame {
        let (tables, paths): (Vec<String>, Vec<String>) = self
            .tables
            .iter()
            .map(|(n, p)| (n.to_owned(), p.to_string_lossy().into_owned()))
            .unzip();

        DataFrame::new([Series::new("Table", tables), Series::new("Path", paths)].into())
            .expect("Invalid SQL backed state")
    }

    pub fn register(&mut self, name: &str, data_frame: DataFrame, path: PathBuf) {
        if let Some(name) = TableNameGen::with(name).find(|name| !self.tables.contains_key(name)) {
            self.sql.register(&name, data_frame.lazy());
            self.tables.insert(name.clone(), path);
            if self.default.is_none() {
                self.default = name.into();
            }
        }
    }

    pub fn execute(&mut self, query: &str) -> PolarsResult<DataFrame> {
        self.sql.execute(query).and_then(LazyFrame::collect)
    }

    pub fn default_df(&mut self) -> Option<DataFrame> {
        let def = self.default.as_deref()?;
        self.execute(format!("SELECT * FROM '{}'", def).as_str()).ok()
    }
}

impl Default for SqlBackend {
    fn default() -> Self {
        Self::new()
    }
}

pub struct TableNameGen<'a> {
    base: &'a str,
    stage: u32,
}

impl<'a> TableNameGen<'a> {
    pub fn with(base: &'a str) -> Self {
        Self { base, stage: 0 }
    }
}

impl<'a> Iterator for TableNameGen<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.stage += 1;
        match self.stage {
            1 => self.base.to_owned().into(),
            2.. => format!("{}_{}", self.base, self.stage).into(),
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_name_gen() {
        let mut name_gen = TableNameGen::with("student");
        assert_eq!(name_gen.next().unwrap(), "student");
        assert_eq!(name_gen.next().unwrap(), "student_2");
        assert_eq!(name_gen.next().unwrap(), "student_3");
        assert_eq!(name_gen.next().unwrap(), "student_4");
    }
}
