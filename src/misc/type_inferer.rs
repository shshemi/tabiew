use std::collections::HashMap;

use chrono::{NaiveDate, NaiveDateTime};
use polars::{
    frame::DataFrame,
    prelude::{AnyValue, DataType, PlSmallStr, TimeUnit},
    series::{ChunkCompareEq, Series},
};

use crate::{
    args::{Args, Type},
    misc::polars_ext::TryMapAll,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct TypeInferer {
    int: bool,
    float: bool,
    boolean: bool,
    date: bool,
    datetime: bool,
}

impl TypeInferer {
    pub fn from_args(args: &Args) -> Self {
        if args.no_type_inference {
            Self::default()
        } else {
            let mut type_infer = TypeInferer::default();
            for t in args.infer_types.inner() {
                type_infer = match t {
                    Type::Int => type_infer.int(),
                    Type::Float => type_infer.float(),
                    Type::Boolean => type_infer.boolean(),
                    Type::Date => type_infer.date(),
                    Type::Datetime => type_infer.datetime(),
                    Type::All => type_infer.int().float().boolean().date().datetime(),
                };
            }
            type_infer
        }
    }

    pub fn update(&self, data_frame: &mut DataFrame) {
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
            data_frame.replace(&col, ser).unwrap();
        }
    }

    pub fn int(mut self) -> Self {
        self.int = true;
        self
    }

    pub fn float(mut self) -> Self {
        self.float = true;
        self
    }

    pub fn boolean(mut self) -> Self {
        self.boolean = true;
        self
    }

    pub fn date(mut self) -> Self {
        self.date = true;
        self
    }

    pub fn datetime(mut self) -> Self {
        self.datetime = true;
        self
    }
}

fn cast_int(ser: &Series) -> Option<Series> {
    ser.cast(&DataType::Int64).ok().and_then(|casted| {
        casted
            .is_null()
            .equal(&ser.is_null())
            .all()
            .then_some(casted)
    })
}

fn cast_float(ser: &Series) -> Option<Series> {
    ser.cast(&DataType::Float64).ok().and_then(|casted| {
        casted
            .is_null()
            .equal(&ser.is_null())
            .all()
            .then_some(casted)
    })
}

fn cast_boolean(ser: &Series) -> Option<Series> {
    ser.try_map_all(|val| match val {
        AnyValue::String(s) => parse_boolean(s),
        AnyValue::StringOwned(s) => parse_boolean(s.as_str()),
        AnyValue::Date(days) => Some(AnyValue::Date(days)),
        AnyValue::Null => Some(AnyValue::Null),
        _ => None,
    })
}

fn parse_boolean(s: &str) -> Option<AnyValue<'static>> {
    match s {
        "true" => Some(AnyValue::Boolean(true)),
        "false" => Some(AnyValue::Boolean(false)),
        _ => None,
    }
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
