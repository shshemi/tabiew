use crate::{
    AppResult,
    duckdb::{ingester::options::TYPES, sql::quote},
    misc::available_col_name::available_col_name,
};
use chrono::{DateTime, NaiveDate, NaiveDateTime};
use duckdb::{
    Connection, appender_params_from_iter,
    types::{TimeUnit, Value},
};

const BATCH: usize = 122_880;

pub(super) struct TextTable<'a> {
    conn: &'a Connection,
    alias: &'a str,
    name: String,
    types: &'a [String],
    columns: Vec<String>,
    kinds: Vec<&'static str>,
    created: usize,
    batch: Vec<Vec<(usize, String)>>,
}

impl<'a> TextTable<'a> {
    pub(super) fn new(
        conn: &'a Connection,
        alias: &'a str,
        name: String,
        types: &'a [String],
    ) -> Self {
        Self {
            conn,
            alias,
            name,
            types,
            columns: Vec::new(),
            kinds: Vec::new(),
            created: 0,
            batch: Vec::new(),
        }
    }
    pub(super) fn column(&mut self, name: &str) -> usize {
        let column = available_col_name(name, |name| !self.columns.iter().any(|c| c == name));
        self.columns.push(column);
        self.columns.len() - 1
    }
    pub(super) fn width(&self) -> usize {
        self.columns.len()
    }
    pub(super) fn push(&mut self, row: Vec<(usize, String)>) -> AppResult<()> {
        self.batch.push(row);
        if self.batch.len() >= BATCH {
            self.flush()?;
        }
        Ok(())
    }
    pub(super) fn finish(mut self) -> AppResult<()> {
        self.flush()
    }
    fn flush(&mut self) -> AppResult<()> {
        if self.columns.is_empty() {
            self.batch.clear();
            return Ok(());
        }
        let mut values = vec![Vec::<&str>::new(); self.columns.len()];
        for row in &self.batch {
            for (index, value) in row {
                if !value.is_empty() {
                    values[*index].push(value);
                }
            }
        }
        let mut changed = Vec::new();
        for (index, values) in values.iter().enumerate() {
            if index == self.kinds.len() {
                self.kinds.push(infer(self.types, values));
            } else if values
                .iter()
                .any(|value| typed(self.kinds[index], value).is_none())
            {
                self.kinds[index] = widen(self.types, self.kinds[index], values);
                changed.push(index);
            }
        }
        let table = format!("{}.{}", quote(self.alias), quote(&self.name));
        if self.created == 0 {
            let definition = self
                .columns
                .iter()
                .zip(&self.kinds)
                .map(|(column, kind)| format!("{} {kind}", quote(column)))
                .collect::<Vec<_>>();
            self.conn.execute_batch(&format!(
                "CREATE OR REPLACE TABLE {table} ({})",
                definition.join(", ")
            ))?;
        } else {
            for index in changed.into_iter().filter(|index| *index < self.created) {
                self.conn.execute_batch(&format!(
                    "ALTER TABLE {table} ALTER {} TYPE {}",
                    quote(&self.columns[index]),
                    self.kinds[index]
                ))?;
            }
            for (column, kind) in self.columns.iter().zip(&self.kinds).skip(self.created) {
                self.conn.execute_batch(&format!(
                    "ALTER TABLE {table} ADD COLUMN {} {kind}",
                    quote(column)
                ))?;
            }
        }
        self.created = self.columns.len();
        let mut appender = self
            .conn
            .appender_to_catalog_and_db(&self.name, self.alias, "main")?;
        for row in self.batch.drain(..) {
            let mut values = vec![Value::Null; self.columns.len()];
            for (index, value) in row {
                values[index] = match self.kinds[index] {
                    "VARCHAR" => Value::Text(value),
                    _ if value.is_empty() => Value::Null,
                    kind => typed(kind, &value).unwrap_or(Value::Text(value)),
                };
            }
            appender.append_row(appender_params_from_iter(values))?;
        }
        appender.flush()?;
        Ok(())
    }
}

fn infer(types: &[String], values: &[&str]) -> &'static str {
    TYPES
        .iter()
        .filter(|kind| types.iter().any(|t| t == *kind))
        .find(|kind| !values.is_empty() && values.iter().all(|value| typed(kind, value).is_some()))
        .copied()
        .unwrap_or("VARCHAR")
}

fn widen(types: &[String], kind: &'static str, values: &[&str]) -> &'static str {
    let wider = match kind {
        "BIGINT" => "DOUBLE",
        "DATE" => "TIMESTAMP",
        _ => "VARCHAR",
    };
    if types.iter().any(|t| t == wider) && values.iter().all(|value| typed(wider, value).is_some())
    {
        wider
    } else {
        "VARCHAR"
    }
}

fn typed(kind: &str, value: &str) -> Option<Value> {
    match kind {
        "BIGINT" => value.parse().ok().map(Value::BigInt),
        "DOUBLE" => value.parse().ok().map(Value::Double),
        "BOOLEAN" if value.eq_ignore_ascii_case("true") => Some(Value::Boolean(true)),
        "BOOLEAN" if value.eq_ignore_ascii_case("false") => Some(Value::Boolean(false)),
        "DATE" => NaiveDate::parse_from_str(value, "%Y-%m-%d")
            .ok()
            .map(|date| (date - DateTime::UNIX_EPOCH.date_naive()).num_days() as i32)
            .map(Value::Date32),
        "TIMESTAMP" => DateTime::parse_from_rfc3339(value)
            .map(|datetime| datetime.naive_utc())
            .or_else(|_| NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M:%S%.f"))
            .or_else(|_| NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S%.f"))
            .ok()
            .map(|datetime| {
                Value::Timestamp(TimeUnit::Microsecond, datetime.and_utc().timestamp_micros())
            }),
        "VARCHAR" => Some(Value::Text(value.to_owned())),
        _ => None,
    }
}
