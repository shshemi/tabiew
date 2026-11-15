use std::fmt::Debug;

pub trait Extension: Send + 'static {
    fn as_sql() -> &'static str;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CoreFunctions;

impl Extension for CoreFunctions {
    fn as_sql() -> &'static str {
        "core_functions"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Icu;

impl Extension for Icu {
    fn as_sql() -> &'static str {
        "icu"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Excel;

impl Extension for Excel {
    fn as_sql() -> &'static str {
        "excel"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Parquet;

impl Extension for Parquet {
    fn as_sql() -> &'static str {
        "parquet"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Tpch;

impl Extension for Tpch {
    fn as_sql() -> &'static str {
        "tpch"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Tpcds;

impl Extension for Tpcds {
    fn as_sql() -> &'static str {
        "tpcds"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Httpfs;

impl Extension for Httpfs {
    fn as_sql() -> &'static str {
        "httpfs"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Json;

impl Extension for Json {
    fn as_sql() -> &'static str {
        "json"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Autocomplete;

impl Extension for Autocomplete {
    fn as_sql() -> &'static str {
        "autocomplete"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Motherduck;

impl Extension for Motherduck {
    fn as_sql() -> &'static str {
        "motherduck"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MysqlScanner;

impl Extension for MysqlScanner {
    fn as_sql() -> &'static str {
        "mysql_scanner"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct OdbcScanner;

impl Extension for OdbcScanner {
    fn as_sql() -> &'static str {
        "odbc_scanner"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SqliteScanner;

impl Extension for SqliteScanner {
    fn as_sql() -> &'static str {
        "sqlite_scanner"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PostgresScanner;

impl Extension for PostgresScanner {
    fn as_sql() -> &'static str {
        "postgres_scanner"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Inet;

impl Extension for Inet {
    fn as_sql() -> &'static str {
        "inet"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Spatial;

impl Extension for Spatial {
    fn as_sql() -> &'static str {
        "spatial"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Aws;

impl Extension for Aws {
    fn as_sql() -> &'static str {
        "aws"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Azure;

impl Extension for Azure {
    fn as_sql() -> &'static str {
        "azure"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Encodings;

impl Extension for Encodings {
    fn as_sql() -> &'static str {
        "encodings"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Iceberg;

impl Extension for Iceberg {
    fn as_sql() -> &'static str {
        "iceberg"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Vss;

impl Extension for Vss {
    fn as_sql() -> &'static str {
        "vss"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Delta;

impl Extension for Delta {
    fn as_sql() -> &'static str {
        "delta"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Fts;

impl Extension for Fts {
    fn as_sql() -> &'static str {
        "fts"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Ui;

impl Extension for Ui {
    fn as_sql() -> &'static str {
        "ui"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Ducklake;

impl Extension for Ducklake {
    fn as_sql() -> &'static str {
        "ducklake"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Quack;

impl Extension for Quack {
    fn as_sql() -> &'static str {
        "quack"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Vortex;

impl Extension for Vortex {
    fn as_sql() -> &'static str {
        "vortex"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Lance;

impl Extension for Lance {
    fn as_sql() -> &'static str {
        "lance"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Avro;

impl Extension for Avro {
    fn as_sql() -> &'static str {
        "avro"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct UnityCatalog;

impl Extension for UnityCatalog {
    fn as_sql() -> &'static str {
        "unity_catalog"
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;
    use crate::sql::db::{Database, DatabaseConfig, ValueRef};

    /// A fresh in-memory database with the default config.
    fn db() -> Database {
        DatabaseConfig::default().open_in_memory().unwrap()
    }

    fn all_names() -> Vec<&'static str> {
        vec![
            CoreFunctions::as_sql(),
            Icu::as_sql(),
            Excel::as_sql(),
            Parquet::as_sql(),
            Tpch::as_sql(),
            Tpcds::as_sql(),
            Httpfs::as_sql(),
            Json::as_sql(),
            Autocomplete::as_sql(),
            Motherduck::as_sql(),
            MysqlScanner::as_sql(),
            OdbcScanner::as_sql(),
            SqliteScanner::as_sql(),
            PostgresScanner::as_sql(),
            Inet::as_sql(),
            Spatial::as_sql(),
            Aws::as_sql(),
            Azure::as_sql(),
            Encodings::as_sql(),
            Iceberg::as_sql(),
            Vss::as_sql(),
            Delta::as_sql(),
            Fts::as_sql(),
            Ui::as_sql(),
            Ducklake::as_sql(),
            Quack::as_sql(),
            Vortex::as_sql(),
            Lance::as_sql(),
            Avro::as_sql(),
            UnityCatalog::as_sql(),
        ]
    }

    fn duckdb_extension_names() -> BTreeSet<String> {
        let db = db();
        let mut stmt = db
            .prepare("SELECT extension_name FROM duckdb_extensions()")
            .unwrap();
        let mut rows = stmt.query([]).unwrap();
        let mut names = BTreeSet::new();
        while let Some(row) = rows.next().unwrap() {
            if let ValueRef::Text(name) = row.get_ref(0).unwrap() {
                names.insert(String::from_utf8_lossy(name).into_owned());
            }
        }
        names
    }

    #[test]
    fn names_are_unique() {
        let names = all_names();
        let unique: BTreeSet<_> = names.iter().collect();
        assert_eq!(unique.len(), names.len());
    }

    #[test]
    fn every_name_is_known_to_duckdb() {
        let known = duckdb_extension_names();
        for name in all_names() {
            assert!(known.contains(name), "{name} is not a DuckDB extension");
        }
    }

    #[test]
    fn every_duckdb_extension_has_a_type() {
        let ours: BTreeSet<String> = all_names().into_iter().map(str::to_owned).collect();
        let missing: Vec<_> = duckdb_extension_names()
            .difference(&ours)
            .cloned()
            .collect();
        assert!(missing.is_empty(), "no Extension type for {missing:?}");
    }
}
