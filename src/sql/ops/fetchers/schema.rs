use crate::{
    AppResult,
    sql::{
        db::{Database, Type},
        ops::reply::{Receiver, channel},
        worker::submit,
    },
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Schema {
    columns: Vec<(String, Type)>,
}

impl Schema {
    pub fn new(columns: Vec<(String, Type)>) -> Self {
        Self { columns }
    }

    pub fn get(&self, idx: usize) -> Option<(&str, &Type)> {
        self.columns
            .get(idx)
            .map(|(name, dtype)| (name.as_str(), dtype))
    }

    pub fn position(&self, name: &str) -> Option<usize> {
        self.columns.iter().position(|(column, _)| column == name)
    }

    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.columns.iter().map(|(name, _)| name.as_str())
    }

    pub fn dtypes(&self) -> impl Iterator<Item = &Type> {
        self.columns.iter().map(|(_, dtype)| dtype)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &Type)> {
        self.columns
            .iter()
            .map(|(name, dtype)| (name.as_str(), dtype))
    }

    pub fn len(&self) -> usize {
        self.columns.len()
    }

    pub fn is_empty(&self) -> bool {
        self.columns.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct Fetcher {
    sql: String,
}

impl Fetcher {
    pub fn new(sql: impl Into<String>) -> Self {
        Self { sql: sql.into() }
    }

    pub fn run(&self) -> Receiver<Schema> {
        let this = self.clone();
        let (send, recv) = channel();
        submit(move |db| send.send(this.fetch(db)));
        recv
    }

    fn fetch(&self, db: &Database) -> AppResult<Schema> {
        let sql = self.sql.trim().trim_end_matches(';');
        let mut stmt = db.prepare(&format!("SELECT * FROM ({sql}) LIMIT 0"))?;
        let rows = stmt.query([])?;
        let mut columns = Vec::with_capacity(rows.column_count());
        for (idx, name) in rows.column_names().into_iter().enumerate() {
            columns.push((name, rows.column_type(idx)?));
        }
        Ok(Schema::new(columns))
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::*;
    use crate::sql::{db::DatabaseConfig, ops::Next};

    fn db() -> Database {
        DatabaseConfig::default().open_in_memory().unwrap()
    }

    fn wait<T>(recv: &Receiver<T>) -> AppResult<T> {
        let deadline = Instant::now() + Duration::from_secs(30);
        loop {
            match recv.recv() {
                Next::Value(value) => return value,
                Next::Done => panic!("sender dropped without sending a value"),
                Next::Empty if Instant::now() > deadline => panic!("timed out waiting for a value"),
                Next::Empty => std::thread::sleep(Duration::from_millis(5)),
            }
        }
    }

    fn dtype(expr: &str) -> Type {
        let schema = Fetcher::new(format!("SELECT {expr} AS c"))
            .fetch(&db())
            .unwrap();
        schema.get(0).unwrap().1.clone()
    }

    #[test]
    fn schema_methods() {
        let schema = Schema::new(vec![("a".into(), Type::Int), ("b".into(), Type::Text)]);
        assert_eq!(schema.len(), 2);
        assert!(!schema.is_empty());
        assert_eq!(schema.get(1), Some(("b", &Type::Text)));
        assert_eq!(schema.get(2), None);
        assert_eq!(schema.position("b"), Some(1));
        assert_eq!(schema.position("c"), None);
        assert_eq!(schema.names().collect::<Vec<_>>(), ["a", "b"]);
        assert_eq!(
            schema.dtypes().collect::<Vec<_>>(),
            [&Type::Int, &Type::Text]
        );
        assert!(Schema::default().is_empty());
    }

    #[test]
    fn scalars() {
        assert_eq!(dtype("NULL"), Type::Null);
        assert_eq!(dtype("true"), Type::Boolean);
        assert_eq!(dtype("1::TINYINT"), Type::TinyInt);
        assert_eq!(dtype("1::SMALLINT"), Type::SmallInt);
        assert_eq!(dtype("1::INTEGER"), Type::Int);
        assert_eq!(dtype("1::BIGINT"), Type::BigInt);
        assert_eq!(dtype("1::HUGEINT"), Type::HugeInt);
        assert_eq!(dtype("1::UHUGEINT"), Type::UHugeInt);
        assert_eq!(dtype("1::UTINYINT"), Type::UTinyInt);
        assert_eq!(dtype("1::USMALLINT"), Type::USmallInt);
        assert_eq!(dtype("1::UINTEGER"), Type::UInt);
        assert_eq!(dtype("1::UBIGINT"), Type::UBigInt);
        assert_eq!(dtype("1::FLOAT"), Type::Float);
        assert_eq!(dtype("1::DOUBLE"), Type::Double);
        assert_eq!(dtype("1.5::DECIMAL(4, 1)"), Type::Decimal);
        assert_eq!(dtype("'x'"), Type::Text);
        assert_eq!(dtype("'x'::BLOB"), Type::Blob);
    }

    #[test]
    fn temporals() {
        assert_eq!(dtype("DATE '2024-01-01'"), Type::Date32);
        assert_eq!(dtype("TIME '12:00:00'"), Type::Time64);
        assert_eq!(dtype("TIMESTAMP '2024-01-01'"), Type::Timestamp);
        assert_eq!(dtype("TIMESTAMP_S '2024-01-01'"), Type::Timestamp);
        assert_eq!(dtype("TIMESTAMP_MS '2024-01-01'"), Type::Timestamp);
        assert_eq!(dtype("TIMESTAMP_NS '2024-01-01'"), Type::Timestamp);
        assert_eq!(dtype("INTERVAL 1 DAY"), Type::Interval);
    }

    #[test]
    fn lossy_types() {
        assert_eq!(dtype("uuid()"), Type::Text);
        assert_eq!(dtype("'101'::BIT"), Type::Blob);
        assert_eq!(
            dtype("TIMESTAMPTZ '2024-01-01 00:00:00+00'"),
            Type::Timestamp
        );
        assert_eq!(dtype("TIMETZ '12:00:00+00'"), Type::Time64);
    }

    #[test]
    fn enums_and_unions() {
        assert_eq!(dtype("'a'::ENUM('a', 'b')"), Type::Enum);
        assert_eq!(dtype("union_value(n := 1)"), Type::Union);
    }

    #[test]
    fn nested() {
        assert_eq!(dtype("[1, 2]"), Type::List(Box::new(Type::Int)));
        assert_eq!(
            dtype("[1, 2]::INTEGER[2]"),
            Type::Array(Box::new(Type::Int), 2)
        );
        assert_eq!(
            dtype("{'a': 1, 'b': 'x'}"),
            Type::Struct(vec![("a".into(), Type::Int), ("b".into(), Type::Text)])
        );
        assert_eq!(
            dtype("MAP {'k': 1}"),
            Type::Map(Box::new(Type::Text), Box::new(Type::Int))
        );
        assert_eq!(
            dtype("[{'a': [1]::INTEGER[1]}]"),
            Type::List(Box::new(Type::Struct(vec![(
                "a".into(),
                Type::Array(Box::new(Type::Int), 1)
            )])))
        );
    }

    #[test]
    fn columns_keep_their_order_and_names() {
        let schema = Fetcher::new("SELECT 1 AS a, 'x' AS b, 2 AS a")
            .fetch(&db())
            .unwrap();
        assert_eq!(schema.names().collect::<Vec<_>>(), ["a", "b", "a_1"]);
        assert_eq!(
            schema.dtypes().collect::<Vec<_>>(),
            [&Type::Int, &Type::Text, &Type::Int]
        );
    }

    #[test]
    fn empty_result_still_has_a_schema() {
        let schema = Fetcher::new("SELECT 1 AS a WHERE false")
            .fetch(&db())
            .unwrap();
        assert_eq!(schema.get(0), Some(("a", &Type::Int)));
    }

    #[test]
    fn trailing_semicolon_is_ignored() {
        let schema = Fetcher::new("SELECT 42 AS n;  ").fetch(&db()).unwrap();
        assert_eq!(schema.len(), 1);
    }

    #[test]
    fn invalid_sql_fails() {
        assert!(Fetcher::new("SELEC 1").fetch(&db()).is_err());
    }

    #[test]
    fn run_returns_the_schema_through_the_worker() {
        let schema = wait(&Fetcher::new("SELECT 1 AS a").run()).unwrap();
        assert_eq!(schema.get(0), Some(("a", &Type::Int)));
    }
}
