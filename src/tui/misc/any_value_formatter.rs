use std::borrow::Cow;
use std::fmt::Display;
use std::fmt::Write;
use std::iter::Peekable;
use std::ops::Deref;
use std::str::Chars;

use chrono::{DateTime, Datelike, Timelike};
use polars::datatypes::{AnyValue, TimeUnit};

use crate::misc::config::config;

#[derive(Clone)]
pub struct AnyValueFormatter {
    buf: String,
    fp_prec: Option<usize>,
}

impl AnyValueFormatter {
    pub fn new(fp_precision: Option<usize>) -> Self {
        Self {
            buf: Default::default(),
            fp_prec: fp_precision,
        }
    }

    pub fn into_single_line(mut self, value: AnyValue) -> Cow<'static, str> {
        match self.to_single_line(value) {
            Formatted::Static(s) => Cow::Borrowed(s),
            Formatted::Owned(s) => Cow::Owned(s),
            Formatted::Buffer(_) => Cow::Owned(self.buf),
        }
    }

    pub fn to_single_line<'a>(&'a mut self, value: AnyValue) -> Formatted<'a> {
        let fp_prec = self.fp_prec;
        match value {
            AnyValue::Null => Formatted::Static(""),
            AnyValue::Boolean(b) => Formatted::Static(bool(b)),
            AnyValue::String(s) => Formatted::Buffer(untabbed(&mut self.buf, s)),
            AnyValue::StringOwned(s) => Formatted::Buffer(untabbed(&mut self.buf, &s)),
            AnyValue::UInt8(u) => Formatted::Buffer(display(&mut self.buf, u)),
            AnyValue::UInt16(u) => Formatted::Buffer(display(&mut self.buf, u)),
            AnyValue::UInt32(u) => Formatted::Buffer(display(&mut self.buf, u)),
            AnyValue::UInt64(u) => Formatted::Buffer(display(&mut self.buf, u)),
            AnyValue::UInt128(u) => Formatted::Buffer(display(&mut self.buf, u)),
            AnyValue::Int8(i) => Formatted::Buffer(display(&mut self.buf, i)),
            AnyValue::Int16(i) => Formatted::Buffer(display(&mut self.buf, i)),
            AnyValue::Int32(i) => Formatted::Buffer(display(&mut self.buf, i)),
            AnyValue::Int64(i) => Formatted::Buffer(display(&mut self.buf, i)),
            AnyValue::Int128(i) => Formatted::Buffer(display(&mut self.buf, i)),
            AnyValue::Float16(f) => {
                Formatted::Buffer(display_with_precision(&mut self.buf, fp_prec, f))
            }
            AnyValue::Float32(f) => {
                Formatted::Buffer(display_with_precision(&mut self.buf, fp_prec, f))
            }
            AnyValue::Float64(f) => {
                Formatted::Buffer(display_with_precision(&mut self.buf, fp_prec, f))
            }
            AnyValue::Date(d) => Formatted::Buffer(date(&mut self.buf, d)),
            AnyValue::Datetime(t, unit, _) | AnyValue::DatetimeOwned(t, unit, _) => {
                Formatted::Buffer(datetime(&mut self.buf, t, unit))
            }
            AnyValue::Duration(_, _) => Formatted::Buffer(display(&mut self.buf, value)),
            AnyValue::Time(t) => Formatted::Buffer(time(&mut self.buf, t)),
            AnyValue::Categorical(cat, map) | AnyValue::Enum(cat, map) => {
                if let Some(s) = map.cat_to_str(cat) {
                    Formatted::Buffer(untabbed(&mut self.buf, s))
                } else {
                    Formatted::Static("")
                }
            }
            AnyValue::CategoricalOwned(cat, map) | AnyValue::EnumOwned(cat, map) => {
                if let Some(s) = map.cat_to_str(cat) {
                    Formatted::Buffer(untabbed(&mut self.buf, s))
                } else {
                    Formatted::Static("")
                }
            }
            AnyValue::List(series) => Formatted::Buffer(display(
                &mut self.buf,
                format_args!("[{} items]", series.len()),
            )),
            AnyValue::Array(_, n) => {
                Formatted::Buffer(display(&mut self.buf, format_args!("[{n} items]")))
            }
            AnyValue::Struct(_, _, fields) => Formatted::Buffer(display(
                &mut self.buf,
                format_args!("{{{} fields}}", fields.len()),
            )),
            AnyValue::StructOwned(st) => Formatted::Buffer(display(
                &mut self.buf,
                format_args!("{{{} fields}}", st.0.len()),
            )),
            AnyValue::Binary(items) => Formatted::Buffer(display(
                &mut self.buf,
                format_args!("[{} bytes]", items.len()),
            )),
            AnyValue::BinaryOwned(items) => Formatted::Buffer(display(
                &mut self.buf,
                format_args!("[{} bytes]", items.len()),
            )),
            AnyValue::Decimal(_, _, _) => Formatted::Buffer(display(&mut self.buf, value)),
        }
    }
}

impl Default for AnyValueFormatter {
    fn default() -> Self {
        Self {
            buf: Default::default(),
            fp_prec: config().fp_precision(),
        }
    }
}

pub enum Formatted<'a> {
    Static(&'static str),
    Owned(String),
    Buffer(&'a str),
}

impl<'a> Formatted<'a> {
    pub fn into_string(self) -> String {
        match self {
            Formatted::Static(s) => s.to_owned(),
            Formatted::Owned(s) => s,
            Formatted::Buffer(s) => s.to_owned(),
        }
    }
}

impl Deref for Formatted<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            Formatted::Static(s) => s,
            Formatted::Owned(s) => s,
            Formatted::Buffer(s) => s,
        }
    }
}

struct FirstLineUntabbed<'a> {
    chars: Peekable<Chars<'a>>,
    done: bool,
}

impl<'a> FirstLineUntabbed<'a> {
    fn new(s: &'a str) -> Self {
        FirstLineUntabbed {
            chars: s.chars().peekable(),
            done: false,
        }
    }
}

impl<'a> Iterator for FirstLineUntabbed<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let c = self.chars.next()?;
        match c {
            '\t' => Some(' '),
            '\n' => {
                self.done = true;
                None
            }
            '\r' if self.chars.peek() == Some(&'\n') => {
                self.done = true;
                None
            }
            c => Some(c),
        }
    }
}

#[inline]
fn display(buf: &mut String, value: impl Display) -> &str {
    buf.clear();
    let _ = write!(buf, "{value}");
    buf
}

#[inline]
fn display_with_precision(buf: &mut String, fp_prec: Option<usize>, value: impl Display) -> &str {
    if let Some(precision) = fp_prec {
        buf.clear();
        let _ = write!(buf, "{value:.precision$}");
        buf
    } else {
        display(buf, value)
    }
}

#[inline]
fn bool(b: bool) -> &'static str {
    if b { "true" } else { "false" }
}

#[inline]
fn date(buf: &mut String, days: i32) -> &str {
    let date = DateTime::from_timestamp(i64::from(days) * 86_400, 0)
        .unwrap_or_default()
        .date_naive();
    buf.clear();
    let _ = write!(
        buf,
        "{:04}-{:02}-{:02}",
        date.year(),
        date.month(),
        date.day()
    );
    buf
}

#[inline]
fn time(buf: &mut String, nanos: i64) -> &str {
    let seconds = nanos / 1_000_000_000;
    buf.clear();
    let _ = write!(
        buf,
        "{:02}:{:02}:{:02}",
        seconds / 3_600,
        seconds / 60 % 60,
        seconds % 60
    );
    buf
}

#[inline]
fn datetime(buf: &mut String, value: i64, unit: TimeUnit) -> &str {
    let datetime = match unit {
        TimeUnit::Nanoseconds => DateTime::from_timestamp_nanos(value),
        TimeUnit::Microseconds => DateTime::from_timestamp_micros(value).unwrap_or_default(),
        TimeUnit::Milliseconds => DateTime::from_timestamp_millis(value).unwrap_or_default(),
    };
    buf.clear();
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
    buf
}

#[inline]
fn untabbed<'a>(buf: &'a mut String, s: &str) -> &'a str {
    buf.clear();
    buf.extend(FirstLineUntabbed::new(s));
    buf
}
