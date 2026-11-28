use std::sync::{Mutex, OnceLock};

use anyhow::anyhow;

use crate::{
    AppResult,
    sql::db::{Database, DatabaseConfig, ValueRef},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Query {
    sql: String,
}

impl Query {
    pub fn new(sql: impl Into<String>) -> AppResult<Self> {
        let sql = sql.into();
        let db = validator()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        validate(&db, &sql)?;
        Ok(Self {
            sql: sql.trim().trim_end_matches(';').trim_end().to_owned(),
        })
    }

    pub fn with_offset(self, offset: u64) -> Self {
        Self {
            sql: format!("SELECT * FROM ({}) OFFSET {offset}", self.sql),
        }
    }

    pub fn with_limit(self, limit: u64) -> Self {
        Self {
            sql: format!("SELECT * FROM ({}) LIMIT {limit}", self.sql),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.sql
    }
}

fn validator() -> &'static Mutex<Database> {
    static VALIDATOR: OnceLock<Mutex<Database>> = OnceLock::new();
    VALIDATOR.get_or_init(|| Mutex::new(DatabaseConfig::default().open_in_memory().unwrap()))
}

fn validate(db: &Database, sql: &str) -> AppResult<()> {
    let mut stmt = db.prepare(
        "SELECT j->>'error_message', json_array_length(j->'statements') \
         FROM (SELECT json_serialize_sql(?)::JSON AS j)",
    )?;
    let mut rows = stmt.query([sql])?;
    let row = rows
        .next()?
        .ok_or_else(|| anyhow!("query validation returned no rows"))?;
    match (row.get_ref(0)?, row.get_ref(1)?) {
        (ValueRef::Null, ValueRef::UBigInt(1)) => Ok(()),
        (ValueRef::Null, ValueRef::UBigInt(0)) => Err(anyhow!("query is empty")),
        (ValueRef::Null, ValueRef::UBigInt(count)) => {
            Err(anyhow!("expected a single statement, found {count}"))
        }
        (ValueRef::Text(message), _) => Err(anyhow!("{}", String::from_utf8_lossy(message))),
        other => Err(anyhow!("unexpected query validation result: {other:?}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_is_valid() {
        let query = Query::new("SELECT a, b FROM t").unwrap();
        assert_eq!(query.as_str(), "SELECT a, b FROM t");
    }

    #[test]
    fn unknown_tables_and_columns_are_valid() {
        assert!(Query::new("SELECT missing FROM nowhere").is_ok());
    }

    #[test]
    fn duckdb_query_forms_are_valid() {
        for sql in [
            "FROM t SELECT a",
            "FROM t",
            "SUMMARIZE t",
            "DESCRIBE t",
            "VALUES (1)",
            "WITH c AS (SELECT a FROM t) SELECT * FROM c",
        ] {
            assert!(Query::new(sql).is_ok(), "{sql}");
        }
    }

    #[test]
    fn trailing_semicolons_and_whitespace_are_trimmed() {
        let query = Query::new("  SELECT * FROM t ;; \n").unwrap();
        assert_eq!(query.as_str(), "SELECT * FROM t");
    }

    #[test]
    fn semicolons_in_strings_and_comments_are_valid() {
        assert!(Query::new("SELECT ';DROP TABLE t;'").is_ok());
        assert!(Query::new("SELECT 1 /* ; DROP TABLE t; */").is_ok());
    }

    #[test]
    fn limit_and_offset_wrap_the_query() {
        let query = Query::new("SELECT * FROM t;").unwrap();
        assert_eq!(
            query.clone().with_limit(5).as_str(),
            "SELECT * FROM (SELECT * FROM t) LIMIT 5"
        );
        assert_eq!(
            query.clone().with_offset(3).as_str(),
            "SELECT * FROM (SELECT * FROM t) OFFSET 3"
        );
        assert_eq!(
            query.with_offset(3).with_limit(5).as_str(),
            "SELECT * FROM (SELECT * FROM (SELECT * FROM t) OFFSET 3) LIMIT 5"
        );
    }

    #[test]
    fn limit_and_offset_select_rows() {
        let db = DatabaseConfig::default().open_in_memory().unwrap();
        let sql = Query::new("SELECT range FROM range(10)")
            .unwrap()
            .with_offset(3)
            .with_limit(2);
        let mut stmt = db.prepare(sql.as_str()).unwrap();
        let mut rows = stmt.query([]).unwrap();
        let mut values = Vec::new();
        while let Some(row) = rows.next().unwrap() {
            match row.get_ref(0).unwrap() {
                ValueRef::BigInt(n) => values.push(n),
                other => panic!("unexpected value: {other:?}"),
            }
        }
        assert_eq!(values, [3, 4]);
    }

    #[test]
    fn syntax_errors_fail() {
        let err = Query::new("SELEC 1").unwrap_err();
        assert!(err.to_string().contains("syntax error"));
    }

    #[test]
    fn non_select_statements_fail() {
        assert!(Query::new("DROP TABLE t").is_err());
        assert!(Query::new("CREATE TABLE u (a INTEGER)").is_err());
        assert!(Query::new("INSERT INTO t VALUES (1, 2)").is_err());
    }

    #[test]
    fn multiple_selects_fail() {
        let err = Query::new("SELECT 1; SELECT 2").unwrap_err();
        assert!(err.to_string().contains("single statement"));
    }

    #[test]
    fn multiple_statements_fail() {
        assert!(Query::new("SELECT 1; DROP TABLE t; SELECT 2").is_err());
    }

    #[test]
    fn wrapper_escapes_fail() {
        assert!(Query::new("t); DROP TABLE t; SELECT (1").is_err());
    }

    #[test]
    fn empty_query_fails() {
        assert!(Query::new("").is_err());
        assert!(Query::new("  ;  ").is_err());
    }

    #[test]
    fn validation_does_not_execute() {
        let db = DatabaseConfig::default().open_in_memory().unwrap();
        db.execute("CREATE TABLE t (a INTEGER)", []).unwrap();
        assert!(validate(&db, "SELECT * FROM t; DROP TABLE t").is_err());
        db.execute("INSERT INTO t VALUES (1)", []).unwrap();
    }
}
