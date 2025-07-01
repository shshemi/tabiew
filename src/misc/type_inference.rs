use std::collections::HashMap;

use chrono::{NaiveDate, NaiveDateTime};
use polars::{
    frame::DataFrame,
    prelude::{AnyValue, DataType, PlSmallStr, TimeUnit},
    series::Series,
};

use crate::{AppResult, misc::polars_ext::TryMapAll};

#[derive(Debug)]
pub struct TypeInference {
    int: bool,
    float: bool,
    boolean: bool,
    date: bool,
    datetime: bool,
}

impl Default for TypeInference {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeInference {
    pub fn new() -> Self {
        Self {
            int: false,
            float: false,
            boolean: false,
            date: false,
            datetime: false,
        }
    }

    pub fn infer_int(mut self) -> Self {
        self.int = true;
        self
    }

    pub fn infer_float(mut self) -> Self {
        self.float = true;
        self
    }

    pub fn infer_boolean(mut self) -> Self {
        self.boolean = true;
        self
    }

    pub fn infer_date(mut self) -> Self {
        self.date = true;
        self
    }

    pub fn infer_datetime(mut self) -> Self {
        self.datetime = true;
        self
    }

    pub fn infer(&self, mut data_frame: DataFrame) -> AppResult<DataFrame> {
        let cast_fns = {
            let mut vec = Vec::<fn(&Series) -> Option<Series>>::new();
            if self.int {
                vec.push(cast_int);
            }

            if self.float {
                vec.push(cast_float);
            }

            if self.boolean {
                vec.push(cast_boolean);
            }

            if self.date {
                vec.push(cast_date);
            }

            if self.datetime {
                vec.push(cast_datetime);
            }
            vec
        };

        let updates = data_frame
            .iter()
            .filter_map(|ser| {
                cast_fns
                    .iter()
                    .find_map(|cast| cast(ser).map(|new_ser| (ser.name().to_owned(), new_ser)))
            })
            .collect::<HashMap<PlSmallStr, Series>>();

        for (col, ser) in updates.into_iter() {
            data_frame.replace(&col, ser)?;
        }

        Ok(data_frame)
    }
}

fn cast_int(ser: &Series) -> Option<Series> {
    ser.cast(&DataType::Int64).ok()
}

fn cast_float(ser: &Series) -> Option<Series> {
    ser.cast(&DataType::Float64).ok()
}

fn cast_boolean(ser: &Series) -> Option<Series> {
    ser.cast(&DataType::Boolean).ok()
}

fn cast_date(series: &Series) -> Option<Series> {
    [
        "%Y-%m-%d", "%Y/%m/%d", "%Y.%m.%d", "%Y %m %d", "%Y%m%d", "%d-%m-%Y", "%d/%m/%Y",
        "%d.%m.%Y", "%d %m %Y", "%d%m%Y", "%m-%d-%Y", "%m/%d/%Y", "%m.%d.%Y", "%m %d %Y", "%m%d%Y",
        "%B %d %Y", "%B-%d-%Y", "%Y-%j",
    ]
    .into_iter()
    .find_map(|fmt| cast_date_custom(series, fmt))
}

fn cast_datetime(series: &Series) -> Option<Series> {
    [
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%dT%H:%M:%S%.f",
        "%Y/%m/%d %H:%M:%S",
        "%Y %m %d %H:%M:%S",
        "%Y.%m.%d %H:%M:%S",
        "%d-%m-%Y %H:%M:%S",
        "%d/%m/%Y %H:%M:%S",
        "%d %m %Y %H:%M:%S",
        "%d.%m.%Y %H:%M:%S",
        "%m-%d-%Y %H:%M:%S",
        "%m/%d/%Y %H:%M:%S",
        "%m %d %Y %H:%M:%S",
        "%m.%d.%Y %H:%M:%S",
        "%B %d %Y %H:%M:%S",
        "%B-%d-%Y %H:%M:%S",
        "%Y%m%dT%H%M%S",
    ]
    .into_iter()
    .find_map(|fmt| cast_datetime_custom(series, fmt))
}

fn cast_date_custom(series: &Series, fmt: &'static str) -> Option<Series> {
    series.try_map_all(|val| match val {
        AnyValue::String(s) => parse_date(s, fmt),
        AnyValue::StringOwned(s) => parse_date(s.as_str(), fmt),
        AnyValue::Date(days) => Some(AnyValue::Date(days)),
        AnyValue::Null => Some(AnyValue::Null),
        _ => None,
    })
}

fn cast_datetime_custom(series: &Series, fmt: &'static str) -> Option<Series> {
    series.try_map_all(|val| match val {
        AnyValue::String(s) => parse_datetime(s, fmt),
        AnyValue::StringOwned(s) => parse_datetime(s.as_str(), fmt),
        AnyValue::Datetime(ts, unit, zone) => Some(AnyValue::DatetimeOwned(
            ts,
            unit,
            zone.map(|sm| sm.to_owned().into()),
        )),
        AnyValue::DatetimeOwned(ts, unit, zone) => Some(AnyValue::DatetimeOwned(ts, unit, zone)),
        AnyValue::Null => Some(AnyValue::Null),
        _ => None,
    })
}

fn parse_datetime(slice: &str, fmt: &str) -> Option<AnyValue<'static>> {
    NaiveDateTime::parse_from_str(slice, fmt)
        .map(|date| {
            AnyValue::DatetimeOwned(
                date.and_utc().timestamp_millis(),
                TimeUnit::Milliseconds,
                None,
            )
        })
        .ok()
}

fn parse_date(slice: &str, fmt: &str) -> Option<AnyValue<'static>> {
    NaiveDate::parse_from_str(slice, fmt)
        .map(|date| {
            AnyValue::Date(
                date.signed_duration_since(NaiveDate::from_ymd_opt(1970, 1, 1).unwrap())
                    .num_days() as i32,
            )
        })
        .ok()
}
