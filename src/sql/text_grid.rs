use std::collections::BTreeMap;
use std::fmt::{Display, Write};

use chrono::{DateTime, Datelike, Timelike};
use duckdb::{
    arrow::array::{Array, StringArray},
    types::{Decimal, EnumType, ListType, TimeUnit, Value},
};

use crate::{misc::config::config, sql::db::ValueRef};

#[derive(Debug, Default)]
pub struct TextGrid {
    idx: BTreeMap<CellPos, Span>,
    buf: String,
}

impl TextGrid {
    pub fn get(&self, row: usize, col: usize) -> Option<&str> {
        self.idx
            .get(&CellPos { row, col })
            .and_then(|span| self.buf.get(span.start..span.end))
    }

    pub fn insert(&mut self, row: usize, col: usize, value: ValueRef) -> bool {
        let fp_prec = config().fp_precision();
        let start = self.buf.len();
        match value {
            ValueRef::Text(s) => {
                push_untabbed(&mut self.buf, first_line(&String::from_utf8_lossy(s)))
            }
            ValueRef::Enum(dict, idx) => {
                push_untabbed(&mut self.buf, first_line(enum_str(dict, idx)))
            }
            ValueRef::Blob(b) | ValueRef::Geometry(b) => {
                let _ = write!(self.buf, "[{} bytes]", b.len());
            }
            ValueRef::List(list, idx) => {
                let _ = write!(self.buf, "[{} items]", list_len(list, idx));
            }
            ValueRef::Array(arr, _) => {
                let _ = write!(self.buf, "[{} items]", arr.value_length());
            }
            ValueRef::Struct(arr, _) => {
                let _ = write!(self.buf, "{{{} fields}}", arr.num_columns());
            }
            ValueRef::Map(arr, idx) => {
                let _ = write!(self.buf, "{{{} entries}}", arr.value_length(idx));
            }
            ValueRef::Union(..) => push_owned(&mut self.buf, fp_prec, &to_owned(value), false),
            ValueRef::Null => {}
            ValueRef::Boolean(b) => push_display(&mut self.buf, if b { "true" } else { "false" }),
            ValueRef::TinyInt(i) => push_int(&mut self.buf, i),
            ValueRef::SmallInt(i) => push_int(&mut self.buf, i),
            ValueRef::Int(i) => push_int(&mut self.buf, i),
            ValueRef::BigInt(i) => push_int(&mut self.buf, i),
            ValueRef::HugeInt(i) => push_int(&mut self.buf, i),
            ValueRef::UTinyInt(u) => push_int(&mut self.buf, u),
            ValueRef::USmallInt(u) => push_int(&mut self.buf, u),
            ValueRef::UInt(u) => push_int(&mut self.buf, u),
            ValueRef::UBigInt(u) => push_int(&mut self.buf, u),
            ValueRef::UHugeInt(u) => push_int(&mut self.buf, u),
            ValueRef::Float(f) => push_float(&mut self.buf, fp_prec, f),
            ValueRef::Double(f) => push_float(&mut self.buf, fp_prec, f),
            ValueRef::Decimal(d) => push_decimal(&mut self.buf, d),
            ValueRef::Date32(days) => push_date(&mut self.buf, days),
            ValueRef::Time64(unit, t) => push_time(&mut self.buf, to_nanos(unit, t)),
            ValueRef::Timestamp(unit, t) => push_datetime(&mut self.buf, unit, t),
            ValueRef::Interval {
                months,
                days,
                nanos,
            } => push_interval(&mut self.buf, months, days, nanos),
        }
        let span = Span {
            start,
            end: self.buf.len(),
        };
        self.idx.insert(CellPos { row, col }, span).is_some()
    }

    pub fn count(&self) -> usize {
        self.idx.len()
    }

    pub fn mem_usage(&self) -> usize {
        self.idx.len() * size_of::<usize>() * 4 + self.buf.capacity()
    }

    pub fn merge(&mut self, other: TextGrid) {
        let offset = self.buf.len();
        for (pos, span) in other.idx {
            self.idx.insert(pos, span.offset(offset));
        }
        self.buf.push_str(&other.buf);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct CellPos {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

impl Span {
    fn offset(self, offset: usize) -> Self {
        Span {
            start: offset + self.start,
            end: offset + self.end,
        }
    }
}

fn push_scalar(buf: &mut String, fp_prec: Option<usize>, value: ValueRef) {
    match value {
        ValueRef::Null => {}
        ValueRef::Boolean(b) => push_display(buf, if b { "true" } else { "false" }),
        ValueRef::TinyInt(i) => push_int(buf, i),
        ValueRef::SmallInt(i) => push_int(buf, i),
        ValueRef::Int(i) => push_int(buf, i),
        ValueRef::BigInt(i) => push_int(buf, i),
        ValueRef::HugeInt(i) => push_int(buf, i),
        ValueRef::UTinyInt(u) => push_int(buf, u),
        ValueRef::USmallInt(u) => push_int(buf, u),
        ValueRef::UInt(u) => push_int(buf, u),
        ValueRef::UBigInt(u) => push_int(buf, u),
        ValueRef::UHugeInt(u) => push_int(buf, u),
        ValueRef::Float(f) => push_float(buf, fp_prec, f),
        ValueRef::Double(f) => push_float(buf, fp_prec, f),
        ValueRef::Decimal(d) => push_decimal(buf, d),
        ValueRef::Date32(days) => push_date(buf, days),
        ValueRef::Time64(unit, t) => push_time(buf, to_nanos(unit, t)),
        ValueRef::Timestamp(unit, t) => push_datetime(buf, unit, t),
        ValueRef::Interval {
            months,
            days,
            nanos,
        } => push_interval(buf, months, days, nanos),
        ValueRef::Text(s) => push_display(buf, String::from_utf8_lossy(s)),
        ValueRef::Enum(dict, idx) => push_display(buf, enum_str(dict, idx)),
        ValueRef::Blob(b) | ValueRef::Geometry(b) => push_bytes(buf, b),
        ValueRef::List(..)
        | ValueRef::Array(..)
        | ValueRef::Struct(..)
        | ValueRef::Map(..)
        | ValueRef::Union(..) => push_owned(buf, fp_prec, &to_owned(value), false),
    }
}

fn push_owned(buf: &mut String, fp_prec: Option<usize>, value: &Value, nested: bool) {
    match value {
        Value::Null if nested => push_display(buf, "NULL"),
        Value::Text(s) | Value::Enum(s) if nested => {
            let _ = write!(buf, "{s:?}");
        }
        Value::Enum(s) => push_display(buf, s),
        Value::List(items) | Value::Array(items) => {
            push_display(buf, '[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    push_display(buf, ", ");
                }
                push_owned(buf, fp_prec, item, true);
            }
            push_display(buf, ']');
        }
        Value::Struct(fields) => {
            push_display(buf, '{');
            for (i, (name, item)) in fields.iter().enumerate() {
                if i > 0 {
                    push_display(buf, ", ");
                }
                let _ = write!(buf, "{name}: ");
                push_owned(buf, fp_prec, item, true);
            }
            push_display(buf, '}');
        }
        Value::Map(entries) => {
            push_display(buf, '{');
            for (i, (key, item)) in entries.iter().enumerate() {
                if i > 0 {
                    push_display(buf, ", ");
                }
                push_owned(buf, fp_prec, key, true);
                push_display(buf, ": ");
                push_owned(buf, fp_prec, item, true);
            }
            push_display(buf, '}');
        }
        Value::Union(inner) => push_owned(buf, fp_prec, inner, nested),
        scalar => push_scalar(buf, fp_prec, duckdb::types::ValueRef::from(scalar).into()),
    }
}

fn to_owned(value: ValueRef) -> Value {
    duckdb::types::ValueRef::from(value).to_owned()
}

fn push_display(buf: &mut String, value: impl Display) {
    let _ = write!(buf, "{value}");
}

fn push_int(buf: &mut String, value: impl Display) {
    let _ = write!(buf, "{value}");
}

fn push_decimal(buf: &mut String, value: Decimal) {
    let _ = write!(buf, "{value}");
}

fn push_float(buf: &mut String, fp_prec: Option<usize>, value: impl Display) {
    match fp_prec {
        Some(prec) => {
            let _ = write!(buf, "{value:.prec$}");
        }
        None => push_display(buf, value),
    }
}

fn push_untabbed(buf: &mut String, text: &str) {
    for c in text.chars() {
        buf.push(if c == '\t' { ' ' } else { c });
    }
}

fn first_line(text: &str) -> &str {
    match text.find('\n') {
        Some(end) => text[..end].strip_suffix('\r').unwrap_or(&text[..end]),
        None => text,
    }
}

fn push_bytes(buf: &mut String, bytes: &[u8]) {
    for (i, byte) in bytes.iter().enumerate() {
        if i > 0 {
            push_display(buf, ' ');
        }
        let _ = write!(buf, "{byte:02x}");
    }
}

fn push_date(buf: &mut String, days: i32) {
    let date = DateTime::from_timestamp(i64::from(days) * 86_400, 0)
        .unwrap_or_default()
        .date_naive();
    let _ = write!(
        buf,
        "{:04}-{:02}-{:02}",
        date.year(),
        date.month(),
        date.day()
    );
}

fn push_time(buf: &mut String, nanos: i64) {
    let seconds = nanos / 1_000_000_000;
    let _ = write!(
        buf,
        "{:02}:{:02}:{:02}",
        seconds / 3_600,
        seconds / 60 % 60,
        seconds % 60
    );
}

fn push_datetime(buf: &mut String, unit: TimeUnit, value: i64) {
    let datetime = match unit {
        TimeUnit::Second => DateTime::from_timestamp(value, 0).unwrap_or_default(),
        TimeUnit::Millisecond => DateTime::from_timestamp_millis(value).unwrap_or_default(),
        TimeUnit::Microsecond => DateTime::from_timestamp_micros(value).unwrap_or_default(),
        TimeUnit::Nanosecond => DateTime::from_timestamp_nanos(value),
    };
    let _ = write!(
        buf,
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        datetime.year(),
        datetime.month(),
        datetime.day(),
        datetime.hour(),
        datetime.minute(),
        datetime.second()
    );
}

fn push_interval(buf: &mut String, months: i32, days: i32, nanos: i64) {
    let mut empty = true;
    for (n, unit) in [(months / 12, "year"), (months % 12, "month"), (days, "day")] {
        if n != 0 {
            if !empty {
                push_display(buf, ' ');
            }
            let _ = write!(buf, "{n} {unit}{}", if n.abs() == 1 { "" } else { "s" });
            empty = false;
        }
    }
    if nanos != 0 || empty {
        if !empty {
            push_display(buf, ' ');
        }
        if nanos < 0 {
            push_display(buf, '-');
        }
        push_time(buf, nanos.saturating_abs());
    }
}

fn to_nanos(unit: TimeUnit, value: i64) -> i64 {
    match unit {
        TimeUnit::Second => value.saturating_mul(1_000_000_000),
        TimeUnit::Millisecond => value.saturating_mul(1_000_000),
        TimeUnit::Microsecond => value.saturating_mul(1_000),
        TimeUnit::Nanosecond => value,
    }
}

fn enum_str<'a>(dict: EnumType<'a>, idx: usize) -> &'a str {
    let (values, key) = match dict {
        EnumType::UInt8(arr) => (arr.values(), arr.key(idx)),
        EnumType::UInt16(arr) => (arr.values(), arr.key(idx)),
        EnumType::UInt32(arr) => (arr.values(), arr.key(idx)),
    };
    match (values.as_any().downcast_ref::<StringArray>(), key) {
        (Some(values), Some(key)) => values.value(key),
        _ => "",
    }
}

fn list_len(list: ListType, idx: usize) -> i64 {
    match list {
        ListType::Regular(arr) => i64::from(arr.value_length(idx)),
        ListType::Large(arr) => arr.value_length(idx),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sql::db::DatabaseConfig;

    fn insert(value: ValueRef) -> String {
        let mut pool = TextGrid::default();
        pool.insert(0, 0, value);
        assert_eq!(pool.get(0, 0), Some(pool.buf.as_str()));
        pool.buf
    }

    fn insert_sql(expr: &str) -> String {
        let db = DatabaseConfig::default().open_in_memory().unwrap();
        let mut stmt = db.prepare(&format!("SELECT {expr}")).unwrap();
        let mut rows = stmt.query([]).unwrap();
        let row = rows.next().unwrap().unwrap();
        insert(row.get_ref(0).unwrap())
    }

    #[test]
    fn scalars() {
        assert_eq!(insert(ValueRef::Int(42)), "42");
        assert_eq!(insert(ValueRef::UHugeInt(u128::MAX)), u128::MAX.to_string());
        assert_eq!(insert(ValueRef::Boolean(true)), "true");
        assert_eq!(insert_sql("12.50::DECIMAL(4, 2)"), "12.50");
        assert_eq!(insert_sql("DATE '2024-03-05'"), "2024-03-05");
        assert_eq!(insert_sql("TIME '04:05:06'"), "04:05:06");
        assert_eq!(
            insert_sql("TIMESTAMP '2024-03-05 04:05:06'"),
            "2024-03-05 04:05:06"
        );
        assert_eq!(
            insert_sql("INTERVAL '1 year 2 months 3 days 04:05:06'"),
            "1 year 2 months 3 days 04:05:06"
        );
    }

    #[test]
    fn null_is_empty() {
        assert_eq!(insert(ValueRef::Null), "");
    }

    #[test]
    fn text_keeps_the_first_line_untabbed() {
        assert_eq!(insert(ValueRef::Text(b"first\tline\nsecond")), "first line");
        assert_eq!(insert(ValueRef::Text(b"a\r\nb")), "a");
        assert_eq!(insert(ValueRef::Text(b"single")), "single");
    }

    #[test]
    fn enums() {
        assert_eq!(insert_sql("'x'::ENUM ('x', 'y')"), "x");
    }

    #[test]
    fn blobs_show_their_size() {
        assert_eq!(insert(ValueRef::Blob(&[0xde, 0xad])), "[2 bytes]");
    }

    #[test]
    fn composites_show_their_size() {
        assert_eq!(insert_sql("[1, 2, 3]"), "[3 items]");
        assert_eq!(insert_sql("[1, 2]::INTEGER[2]"), "[2 items]");
        assert_eq!(insert_sql("{'a': 1, 'b': 'x'}"), "{2 fields}");
        assert_eq!(insert_sql("MAP {'k': 1}"), "{1 entries}");
    }

    #[test]
    fn unions_show_their_value() {
        assert_eq!(insert_sql("union_value(n := 5)"), "5");
    }

    #[test]
    fn pool_returns_pushed_values() {
        let mut pool = TextGrid::default();
        pool.insert(0, 0, ValueRef::Int(7));
        pool.insert(0, 1, ValueRef::Text(b"a\nb"));
        pool.insert(1, 0, ValueRef::Blob(&[1]));
        assert_eq!(pool.get(0, 0), Some("7"));
        assert_eq!(pool.get(0, 1), Some("a"));
        assert_eq!(pool.get(1, 0), Some("[1 bytes]"));
        assert_eq!(pool.get(2, 0), None);
        assert_eq!(pool.count(), 3);
    }

    #[test]
    fn push_value_reports_overwrites() {
        let mut pool = TextGrid::default();
        assert!(!pool.insert(0, 0, ValueRef::Int(1)));
        assert!(pool.insert(0, 0, ValueRef::Int(2)));
        assert_eq!(pool.get(0, 0), Some("2"));
    }

    #[test]
    fn merge_keeps_spans_valid() {
        let mut a = TextGrid::default();
        a.insert(0, 0, ValueRef::Text(b"left"));
        let mut b = TextGrid::default();
        b.insert(1, 0, ValueRef::Text(b"right\nside"));
        a.merge(b);
        assert_eq!(a.get(0, 0), Some("left"));
        assert_eq!(a.get(1, 0), Some("right"));
    }
}
