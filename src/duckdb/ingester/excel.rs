use crate::{
    AppResult,
    duckdb::{
        helper::quote,
        ingester::{FileIngester, IngestProgress, progress::CountProgress},
        session::SessionConn,
    },
    misc::available_col_name::available_col_name,
};
use calamine::{Data, DataType, Range, Reader, open_workbook_auto};
use duckdb::{
    Connection, appender_params_from_iter,
    types::{TimeUnit, Value},
};
use std::{path::PathBuf, sync::Arc};

pub struct ExcelIngester {
    path: PathBuf,
    header: bool,
    progress: Arc<CountProgress>,
}

impl ExcelIngester {
    pub fn new(path: PathBuf, header: bool) -> Self {
        Self {
            path,
            header,
            progress: CountProgress::new(),
        }
    }
}

impl FileIngester for ExcelIngester {
    fn progress(&self) -> Arc<dyn IngestProgress> {
        self.progress.clone()
    }
    fn ingest(&mut self, conn: SessionConn, alias: &str) -> AppResult<()> {
        let sheets = open_workbook_auto(&self.path)?.worksheets();
        self.progress.total(sheets.len() as u64);
        for (name, sheet) in sheets {
            write_sheet(&conn, alias, &name, &sheet, self.header)?;
            self.progress.add(1);
        }
        Ok(())
    }
}

fn write_sheet(
    conn: &Connection,
    alias: &str,
    table: &str,
    sheet: &Range<Data>,
    header: bool,
) -> AppResult<()> {
    if sheet.is_empty() {
        return Ok(());
    }
    let offset = sheet.start().unwrap_or_default().1 as usize;
    let mut rows = sheet.rows();
    let head = if header { rows.next() } else { None };
    let rows = rows.collect::<Vec<_>>();
    let mut columns = Vec::<String>::new();
    for i in 0..sheet.width() {
        let name = head
            .and_then(|head| head.get(i))
            .map(|cell| cell.to_string().trim().to_owned())
            .filter(|name| !name.is_empty())
            .unwrap_or_else(|| col_letter(offset + i));
        let name = available_col_name(&name, |name| !columns.iter().any(|c| c == name));
        columns.push(name);
    }
    let kinds = (0..columns.len())
        .map(|i| {
            rows.iter()
                .filter_map(|row| kind(&row[i]))
                .reduce(merge)
                .unwrap_or("VARCHAR")
        })
        .collect::<Vec<_>>();
    let definition = columns
        .iter()
        .zip(&kinds)
        .map(|(column, kind)| format!("{} {kind}", quote(column)))
        .collect::<Vec<_>>();
    conn.execute_batch(&format!(
        "CREATE OR REPLACE TABLE {}.{} ({})",
        quote(alias),
        quote(table),
        definition.join(", ")
    ))?;
    let mut appender = conn.appender_to_catalog_and_db(table, alias, "main")?;
    for row in rows {
        let values = row.iter().zip(&kinds).map(|(cell, kind)| value(cell, kind));
        appender.append_row(appender_params_from_iter(values))?;
    }
    appender.flush()?;
    Ok(())
}

fn kind(cell: &Data) -> Option<&'static str> {
    match cell {
        Data::Empty => None,
        Data::Int(_) => Some("BIGINT"),
        Data::Float(float) if whole(*float) => Some("BIGINT"),
        Data::Float(_) => Some("DOUBLE"),
        Data::Bool(_) => Some("BOOLEAN"),
        Data::DateTime(datetime) if datetime.is_datetime() => Some("TIMESTAMP"),
        _ => Some("VARCHAR"),
    }
}

fn whole(float: f64) -> bool {
    float.fract() == 0.0 && float.abs() < 9.0e15
}

fn merge(a: &'static str, b: &'static str) -> &'static str {
    match (a, b) {
        _ if a == b => a,
        ("BIGINT", "DOUBLE") | ("DOUBLE", "BIGINT") => "DOUBLE",
        _ => "VARCHAR",
    }
}

fn value(cell: &Data, kind: &str) -> Value {
    match (cell, kind) {
        (Data::Empty, _) => Value::Null,
        (Data::Int(int), "BIGINT") => Value::BigInt(*int),
        (Data::Int(int), "DOUBLE") => Value::Double(*int as f64),
        (Data::Float(float), "BIGINT") => Value::BigInt(*float as i64),
        (Data::Float(float), "DOUBLE") => Value::Double(*float),
        (Data::Bool(bool), "BOOLEAN") => Value::Boolean(*bool),
        (Data::DateTime(_), "TIMESTAMP") => cell
            .as_datetime()
            .map(|datetime| {
                Value::Timestamp(TimeUnit::Microsecond, datetime.and_utc().timestamp_micros())
            })
            .unwrap_or(Value::Null),
        _ => Value::Text(cell.to_string()),
    }
}

fn col_letter(mut index: usize) -> String {
    let mut letter = String::new();
    loop {
        letter.insert(0, (b'A' + (index % 26) as u8) as char);
        index /= 26;
        if index == 0 {
            return letter;
        }
        index -= 1;
    }
}
