use std::collections::BTreeMap;

use crate::sql::db::Value;

#[derive(Debug, Default)]
pub struct ValueSeries {
    idx: BTreeMap<usize, Value>,
}

impl ValueSeries {
    pub fn get(&self, row: usize) -> Option<&Value> {
        self.idx.get(&row)
    }

    pub fn insert(&mut self, row: usize, value: impl Into<Value>) -> bool {
        self.idx.insert(row, value.into()).is_some()
    }

    pub fn len(&self) -> usize {
        self.idx.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn mem_usage(&self) -> usize {
        self.idx.len() * size_of::<(usize, Value)>()
            + self.idx.values().map(heap_size).sum::<usize>()
    }

    pub fn merge(&mut self, other: ValueSeries) {
        self.idx.extend(other.idx);
    }
}

fn heap_size(value: &Value) -> usize {
    match value {
        Value::Null
        | Value::Boolean(_)
        | Value::TinyInt(_)
        | Value::SmallInt(_)
        | Value::Int(_)
        | Value::BigInt(_)
        | Value::HugeInt(_)
        | Value::UHugeInt(_)
        | Value::UTinyInt(_)
        | Value::USmallInt(_)
        | Value::UInt(_)
        | Value::UBigInt(_)
        | Value::Float(_)
        | Value::Double(_)
        | Value::Decimal(_)
        | Value::Timestamp(..)
        | Value::Date32(_)
        | Value::Time64(..)
        | Value::Interval { .. } => 0,
        Value::Text(s) | Value::Enum(s) => s.capacity(),
        Value::Blob(b) | Value::Geometry(b) => b.capacity(),
        Value::List(items) | Value::Array(items) => {
            items.capacity() * size_of::<Value>() + items.iter().map(heap_size).sum::<usize>()
        }
        Value::Struct(fields) => {
            fields.capacity() * size_of::<(String, Value)>()
                + fields
                    .iter()
                    .map(|(name, value)| name.capacity() + heap_size(value))
                    .sum::<usize>()
        }
        Value::Map(entries) => {
            entries.capacity() * size_of::<(Value, Value)>()
                + entries
                    .iter()
                    .map(|(key, value)| heap_size(key) + heap_size(value))
                    .sum::<usize>()
        }
        Value::Union(inner) => size_of::<Value>() + heap_size(inner),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sql::db::{DatabaseConfig, ValueRef};

    fn insert_sql(expr: &str) -> Value {
        let db = DatabaseConfig::default().open_in_memory().unwrap();
        let mut stmt = db.prepare(&format!("SELECT {expr}")).unwrap();
        let mut rows = stmt.query([]).unwrap();
        let row = rows.next().unwrap().unwrap();
        let mut series = ValueSeries::default();
        series.insert(0, row.get_ref(0).unwrap());
        series.get(0).unwrap().clone()
    }

    #[test]
    fn scalars() {
        assert_eq!(insert_sql("42"), Value::Int(42));
        assert_eq!(insert_sql("true"), Value::Boolean(true));
        assert_eq!(insert_sql("'x'"), Value::Text("x".into()));
        assert_eq!(insert_sql("NULL::INTEGER"), Value::Null);
    }

    #[test]
    fn composites() {
        assert_eq!(
            insert_sql("[1, 2]"),
            Value::List(vec![Value::Int(1), Value::Int(2)])
        );
        assert_eq!(
            insert_sql("{'a': 1, 'b': 'x'}"),
            Value::Struct(vec![
                ("a".into(), Value::Int(1)),
                ("b".into(), Value::Text("x".into())),
            ])
        );
        assert_eq!(
            insert_sql("MAP {'k': 1}"),
            Value::Map(vec![(Value::Text("k".into()), Value::Int(1))])
        );
    }

    #[test]
    fn insert_accepts_value_and_value_ref() {
        let mut series = ValueSeries::default();
        series.insert(0, Value::Int(1));
        series.insert(1, ValueRef::Text(b"a"));
        assert_eq!(series.get(0), Some(&Value::Int(1)));
        assert_eq!(series.get(1), Some(&Value::Text("a".into())));
        assert_eq!(series.get(2), None);
        assert_eq!(series.len(), 2);
    }

    #[test]
    fn insert_reports_overwrites() {
        let mut series = ValueSeries::default();
        assert!(!series.insert(0, Value::Int(1)));
        assert!(series.insert(0, Value::Int(2)));
        assert_eq!(series.get(0), Some(&Value::Int(2)));
    }

    #[test]
    fn merge_combines_rows() {
        let mut a = ValueSeries::default();
        a.insert(0, Value::Int(1));
        a.insert(1, Value::Int(2));
        let mut b = ValueSeries::default();
        b.insert(1, Value::Int(20));
        b.insert(2, Value::Int(30));
        a.merge(b);
        assert_eq!(a.get(0), Some(&Value::Int(1)));
        assert_eq!(a.get(1), Some(&Value::Int(20)));
        assert_eq!(a.get(2), Some(&Value::Int(30)));
        assert_eq!(a.len(), 3);
    }

    #[test]
    fn mem_usage_counts_heap_data() {
        let mut small = ValueSeries::default();
        small.insert(0, Value::Text(String::new()));
        let mut large = ValueSeries::default();
        large.insert(0, Value::Text("x".repeat(1000)));
        assert!(large.mem_usage() >= small.mem_usage() + 1000);
    }
}
