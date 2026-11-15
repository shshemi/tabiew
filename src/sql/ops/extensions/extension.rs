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
