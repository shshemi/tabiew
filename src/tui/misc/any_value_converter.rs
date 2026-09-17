use std::fmt::Display;
use std::fmt::Write;

use chrono::{DateTime, Datelike, Timelike};
use polars::datatypes::{AnyValue, TimeUnit};

pub fn format_any_value<'a, 'b, 'c>(
    buf: &'a mut String,
    fp_prec: &Option<usize>,
    value: AnyValue<'b>,
) -> &'c str
where
    'a: 'c,
    'b: 'c,
{
    match value {
        AnyValue::Null => "",
        AnyValue::Boolean(b) => bool(b),
        AnyValue::String(s) => s.lines().next().unwrap_or_default(),
        AnyValue::StringOwned(s) => display(buf, s.lines().next().unwrap_or_default()),
        AnyValue::UInt8(u) => display(buf, u),
        AnyValue::UInt16(u) => display(buf, u),
        AnyValue::UInt32(u) => display(buf, u),
        AnyValue::UInt64(u) => display(buf, u),
        AnyValue::UInt128(u) => display(buf, u),
        AnyValue::Int8(i) => display(buf, i),
        AnyValue::Int16(i) => display(buf, i),
        AnyValue::Int32(i) => display(buf, i),
        AnyValue::Int64(i) => display(buf, i),
        AnyValue::Int128(i) => display(buf, i),
        AnyValue::Float16(f) => display_with_precision(buf, *fp_prec, f),
        AnyValue::Float32(f) => display_with_precision(buf, *fp_prec, f),
        AnyValue::Float64(f) => display_with_precision(buf, *fp_prec, f),
        AnyValue::Date(d) => date(buf, d),
        AnyValue::Datetime(t, unit, _) | AnyValue::DatetimeOwned(t, unit, _) => {
            datetime(buf, t, unit)
        }
        AnyValue::Duration(_, _) => display(buf, value),
        AnyValue::Time(t) => time(buf, t),
        AnyValue::Categorical(cat, map) | AnyValue::Enum(cat, map) => {
            map.cat_to_str(cat).unwrap_or_default()
        }
        AnyValue::CategoricalOwned(cat, map) | AnyValue::EnumOwned(cat, map) => {
            display(buf, map.cat_to_str(cat).unwrap_or_default())
        }
        AnyValue::List(series) => display(buf, format_args!("[{} items]", series.len())),
        AnyValue::Array(_, n) => display(buf, format_args!("[{n} items]")),
        AnyValue::Struct(_, _, fields) => display(buf, format_args!("{{{} fields}}", fields.len())),
        AnyValue::StructOwned(st) => display(buf, format_args!("{{{} fields}}", st.0.len())),
        AnyValue::Binary(items) => display(buf, format_args!("[{} bytes]", items.len())),
        AnyValue::BinaryOwned(items) => display(buf, format_args!("[{} bytes]", items.len())),
        AnyValue::Decimal(_, _, _) => display(buf, value),
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use polars::prelude::*;
    use rstest::rstest;

    use super::format_any_value;

    fn conv(fp_prec: Option<usize>, value: AnyValue) -> String {
        let mut buf = String::new();
        format_any_value(&mut buf, &fp_prec, value).to_owned()
    }

    fn categorical_series() -> Series {
        Series::new("fruit".into(), &["apple", "kiwi", "banana"])
            .cast(&DataType::from_categories(Categories::global()))
            .unwrap()
    }

    fn enum_series() -> Series {
        let categories = FrozenCategories::new(["low", "mid", "high"]).unwrap();
        Series::new("level".into(), &["high", "low"])
            .cast(&DataType::from_frozen_categories(categories))
            .unwrap()
    }

    fn struct_series() -> Series {
        df!("x" => [1i32, 2, 3, 4], "y" => ["a", "b", "c", "d"])
            .unwrap()
            .into_struct("s".into())
            .into_series()
    }

    fn array_series() -> Series {
        Series::new("a".into(), &[1i32, 2, 3, 4, 5, 6])
            .reshape_array(&[
                ReshapeDimension::Infer,
                ReshapeDimension::Specified(Dimension::new(3)),
            ])
            .unwrap()
    }

    #[test]
    fn null_is_empty() {
        assert_eq!(conv(None, AnyValue::Null), "");
    }

    #[rstest]
    #[case::yes(true, "true")]
    #[case::no(false, "false")]
    fn boolean(#[case] value: bool, #[case] expected: &str) {
        assert_eq!(conv(None, AnyValue::Boolean(value)), expected);
    }

    #[rstest]
    #[case::plain("hello", "hello")]
    #[case::empty("", "")]
    #[case::multi_line("first\nsecond\nthird", "first")]
    #[case::crlf("first\r\nsecond", "first")]
    #[case::leading_newline("\nsecond", "")]
    #[case::trailing_newline("only\n", "only")]
    #[case::tab_preserved("a\tb", "a\tb")]
    #[case::unicode("日本語 café", "日本語 café")]
    fn string_first_line(#[case] value: &str, #[case] expected: &str) {
        assert_eq!(conv(None, AnyValue::String(value)), expected);
        assert_eq!(conv(None, AnyValue::StringOwned(value.into())), expected);
    }

    #[test]
    fn borrowed_string_is_returned_without_copying() {
        let text = String::from("borrowed");
        let mut buf = String::new();
        let out = format_any_value(&mut buf, &None, AnyValue::String(&text));
        assert_eq!(out.as_ptr(), text.as_ptr());
    }

    #[rstest]
    #[case::u8_min(AnyValue::UInt8(0), "0")]
    #[case::u8_max(AnyValue::UInt8(u8::MAX), "255")]
    #[case::u16_max(AnyValue::UInt16(u16::MAX), "65535")]
    #[case::u32_max(AnyValue::UInt32(u32::MAX), "4294967295")]
    #[case::u64_max(AnyValue::UInt64(u64::MAX), "18446744073709551615")]
    #[case::u128_max(
        AnyValue::UInt128(u128::MAX),
        "340282366920938463463374607431768211455"
    )]
    #[case::i8_min(AnyValue::Int8(i8::MIN), "-128")]
    #[case::i8_max(AnyValue::Int8(i8::MAX), "127")]
    #[case::i16_min(AnyValue::Int16(i16::MIN), "-32768")]
    #[case::i32_min(AnyValue::Int32(i32::MIN), "-2147483648")]
    #[case::i64_min(AnyValue::Int64(i64::MIN), "-9223372036854775808")]
    #[case::i64_zero(AnyValue::Int64(0), "0")]
    #[case::i64_negative(AnyValue::Int64(-42), "-42")]
    #[case::i128_min(
        AnyValue::Int128(i128::MIN),
        "-170141183460469231731687303715884105728"
    )]
    fn integers(#[case] value: AnyValue<'static>, #[case] expected: &str) {
        assert_eq!(conv(None, value.clone()), expected);
        assert_eq!(conv(Some(3), value), expected);
    }

    #[rstest]
    #[case::whole(1.0, "1")]
    #[case::negative_zero(-0.0, "-0")]
    #[case::fraction(2.5, "2.5")]
    #[case::binary_noise(0.1 + 0.2, "0.30000000000000004")]
    #[case::large(1e20, "100000000000000000000")]
    #[case::small(1e-7, "0.0000001")]
    #[case::nan(f64::NAN, "NaN")]
    #[case::infinity(f64::INFINITY, "inf")]
    #[case::negative_infinity(f64::NEG_INFINITY, "-inf")]
    fn float64_without_precision(#[case] value: f64, #[case] expected: &str) {
        assert_eq!(conv(None, AnyValue::Float64(value)), expected);
    }

    #[rstest]
    #[case::pads_zeros(1.0, 2, "1.00")]
    #[case::rounds(2.345678, 2, "2.35")]
    #[case::zero_precision(2.5, 0, "2")]
    #[case::zero_precision_rounds_half_to_even(3.5, 0, "4")]
    #[case::negative(-1.25, 1, "-1.2")]
    #[case::long_precision(0.5, 12, "0.500000000000")]
    #[case::nan(f64::NAN, 3, "NaN")]
    #[case::infinity(f64::INFINITY, 3, "inf")]
    fn float64_with_precision(
        #[case] value: f64,
        #[case] precision: usize,
        #[case] expected: &str,
    ) {
        assert_eq!(conv(Some(precision), AnyValue::Float64(value)), expected);
    }

    #[rstest]
    #[case::no_precision(None, "1.5")]
    #[case::with_precision(Some(3), "1.500")]
    fn float32(#[case] precision: Option<usize>, #[case] expected: &str) {
        assert_eq!(conv(precision, AnyValue::Float32(1.5)), expected);
    }

    #[rstest]
    #[case::no_precision(None, "1.5")]
    #[case::with_precision(Some(3), "1.500")]
    fn float16(#[case] precision: Option<usize>, #[case] expected: &str) {
        let half = pf16::from(1.5f32);
        assert_eq!(conv(precision, AnyValue::Float16(half)), expected);
    }

    #[rstest]
    #[case::epoch(0, "1970-01-01")]
    #[case::day_after_epoch(1, "1970-01-02")]
    #[case::day_before_epoch(-1, "1969-12-31")]
    #[case::leap_day(18321, "2020-02-29")]
    #[case::modern(18077, "2019-06-30")]
    #[case::year_one(-719162, "0001-01-01")]
    #[case::year_9999(2932896, "9999-12-31")]
    fn date(#[case] days: i32, #[case] expected: &str) {
        assert_eq!(conv(None, AnyValue::Date(days)), expected);
    }

    #[test]
    fn date_matches_polars_for_many_days() {
        let mut buf = String::new();
        for days in (-719_162..2_932_896).step_by(997) {
            assert_eq!(
                format_any_value(&mut buf, &None, AnyValue::Date(days)),
                AnyValue::Date(days).to_string(),
                "day {days}"
            );
        }
    }

    #[rstest]
    #[case::midnight(0, "00:00:00")]
    #[case::morning(28_145_000_000_000, "07:49:05")]
    #[case::fraction_truncated(28_145_999_999_999, "07:49:05")]
    #[case::last_nanosecond(86_399_999_999_999, "23:59:59")]
    #[case::single_digits(3_661_000_000_000, "01:01:01")]
    fn time(#[case] nanos: i64, #[case] expected: &str) {
        assert_eq!(conv(None, AnyValue::Time(nanos)), expected);
        assert_eq!(conv(Some(3), AnyValue::Time(nanos)), expected);
    }

    #[rstest]
    #[case::ms(1_561_880_945_000, TimeUnit::Milliseconds, "2019-06-30 07:49:05")]
    #[case::us(1_561_880_945_000_000, TimeUnit::Microseconds, "2019-06-30 07:49:05")]
    #[case::ns(
        1_561_880_945_000_000_000,
        TimeUnit::Nanoseconds,
        "2019-06-30 07:49:05"
    )]
    #[case::ms_fraction_truncated(1_561_880_945_999, TimeUnit::Milliseconds, "2019-06-30 07:49:05")]
    #[case::us_fraction_truncated(
        1_561_880_945_999_999,
        TimeUnit::Microseconds,
        "2019-06-30 07:49:05"
    )]
    #[case::ns_fraction_truncated(
        1_561_880_945_999_999_999,
        TimeUnit::Nanoseconds,
        "2019-06-30 07:49:05"
    )]
    #[case::epoch(0, TimeUnit::Milliseconds, "1970-01-01 00:00:00")]
    #[case::before_epoch_ms(-1, TimeUnit::Milliseconds, "1969-12-31 23:59:59")]
    #[case::before_epoch_ns(-1_000_000_001, TimeUnit::Nanoseconds, "1969-12-31 23:59:58")]
    #[case::far_past(-2_208_988_800_000, TimeUnit::Milliseconds, "1900-01-01 00:00:00")]
    #[case::far_future(4_102_444_800_000, TimeUnit::Milliseconds, "2100-01-01 00:00:00")]
    fn datetime(#[case] value: i64, #[case] unit: TimeUnit, #[case] expected: &str) {
        assert_eq!(conv(None, AnyValue::Datetime(value, unit, None)), expected);
        assert_eq!(
            conv(Some(6), AnyValue::Datetime(value, unit, None)),
            expected
        );
        assert_eq!(
            conv(None, AnyValue::DatetimeOwned(value, unit, None)),
            expected
        );
    }

    #[test]
    fn datetime_ignores_time_zone() {
        let tz = TimeZone::opt_try_new(Some("Europe/London")).unwrap();
        let value = AnyValue::Datetime(1_561_880_945_000, TimeUnit::Milliseconds, tz.as_ref());
        assert_eq!(conv(None, value), "2019-06-30 07:49:05");
        let owned =
            AnyValue::DatetimeOwned(1_561_880_945_000, TimeUnit::Milliseconds, tz.map(Arc::new));
        assert_eq!(conv(None, owned), "2019-06-30 07:49:05");
    }

    #[test]
    fn datetime_matches_polars_on_whole_seconds() {
        let mut buf = String::new();
        for secs in (-4_000_000_000i64..4_000_000_000).step_by(7_654_321) {
            for (unit, scale) in [
                (TimeUnit::Milliseconds, 1_000),
                (TimeUnit::Microseconds, 1_000_000),
                (TimeUnit::Nanoseconds, 1_000_000_000),
            ] {
                let value = AnyValue::Datetime(secs * scale, unit, None);
                assert_eq!(
                    format_any_value(&mut buf, &None, value.clone()),
                    value.to_string(),
                    "{secs}s {unit:?}"
                );
            }
        }
    }

    #[rstest]
    #[case::zero_ms(0, TimeUnit::Milliseconds, "0ms")]
    #[case::zero_us(0, TimeUnit::Microseconds, "0µs")]
    #[case::zero_ns(0, TimeUnit::Nanoseconds, "0ns")]
    #[case::ms(142, TimeUnit::Milliseconds, "142ms")]
    #[case::all_parts(90_061_001, TimeUnit::Milliseconds, "1d 1h 1m 1s 1ms")]
    #[case::negative(-90_061_001, TimeUnit::Milliseconds, "-1d -1h -1m -1s -1ms")]
    #[case::us_whole_ms(1_500, TimeUnit::Microseconds, "1500µs")]
    #[case::ns_whole_seconds(3_000_000_000, TimeUnit::Nanoseconds, "3s")]
    fn duration_uses_polars_format(
        #[case] value: i64,
        #[case] unit: TimeUnit,
        #[case] expected: &str,
    ) {
        assert_eq!(conv(None, AnyValue::Duration(value, unit)), expected);
    }

    #[test]
    fn categorical_returns_category_string() {
        let series = categorical_series();
        assert_eq!(conv(None, series.get(0).unwrap()), "apple");
        assert_eq!(conv(None, series.get(1).unwrap()), "kiwi");
        assert_eq!(conv(None, series.get(2).unwrap()), "banana");
        assert!(matches!(series.get(0).unwrap(), AnyValue::Categorical(..)));
    }

    #[test]
    fn categorical_owned_returns_category_string() {
        let series = categorical_series();
        let owned = series.get(2).unwrap().into_static();
        assert!(matches!(owned, AnyValue::CategoricalOwned(..)));
        assert_eq!(conv(None, owned), "banana");
    }

    #[test]
    fn enum_returns_category_string() {
        let series = enum_series();
        assert!(matches!(series.get(0).unwrap(), AnyValue::Enum(..)));
        assert_eq!(conv(None, series.get(0).unwrap()), "high");
        assert_eq!(conv(None, series.get(1).unwrap()), "low");
        let owned = series.get(0).unwrap().into_static();
        assert!(matches!(owned, AnyValue::EnumOwned(..)));
        assert_eq!(conv(None, owned), "high");
    }

    #[rstest]
    #[case::three(vec![1i32, 2, 3], "[3 items]")]
    #[case::one(vec![7], "[1 items]")]
    #[case::empty(vec![], "[0 items]")]
    fn list_reports_item_count(#[case] items: Vec<i32>, #[case] expected: &str) {
        let inner = Series::new("l".into(), &items);
        assert_eq!(conv(None, AnyValue::List(inner)), expected);
    }

    #[test]
    fn array_reports_width() {
        let series = array_series();
        let cell = series.get(1).unwrap();
        assert!(matches!(cell, AnyValue::Array(_, 3)));
        assert_eq!(conv(None, cell), "[3 items]");
    }

    #[test]
    fn struct_reports_field_count_not_row_count() {
        let series = struct_series();
        let cell = series.get(1).unwrap();
        assert!(matches!(cell, AnyValue::Struct(..)));
        assert_eq!(conv(None, cell), "{2 fields}");
        let owned = series.get(1).unwrap().into_static();
        assert!(matches!(owned, AnyValue::StructOwned(..)));
        assert_eq!(conv(None, owned), "{2 fields}");
    }

    #[rstest]
    #[case::three(&[1u8, 2, 3], "[3 bytes]")]
    #[case::empty(&[], "[0 bytes]")]
    #[case::many(&[0u8; 1024], "[1024 bytes]")]
    fn binary_reports_byte_count(#[case] bytes: &'static [u8], #[case] expected: &str) {
        assert_eq!(conv(None, AnyValue::Binary(bytes)), expected);
        assert_eq!(conv(None, AnyValue::BinaryOwned(bytes.to_vec())), expected);
    }

    #[rstest]
    #[case::two_places(1234, 2, "12.34")]
    #[case::leading_zero(5, 4, "0.0005")]
    #[case::negative_leading_zero(-50, 3, "-0.050")]
    #[case::trailing_zeros_kept(7000, 3, "7.000")]
    #[case::no_scale(42, 0, "42")]
    #[case::negative_no_scale(-42, 0, "-42")]
    #[case::zero(0, 2, "0.00")]
    #[case::scale_equals_digits(-999, 3, "-0.999")]
    #[case::huge(99_999_999_999_999_999_999, 6, "99999999999999.999999")]
    fn decimal(#[case] value: i128, #[case] scale: usize, #[case] expected: &str) {
        assert_eq!(conv(None, AnyValue::Decimal(value, 38, scale)), expected);
        assert_eq!(conv(Some(1), AnyValue::Decimal(value, 38, scale)), expected);
    }

    #[test]
    fn buffer_is_cleared_between_calls() {
        let mut buf = String::new();
        let prec = Some(2);
        assert_eq!(
            format_any_value(&mut buf, &prec, AnyValue::Int64(1234567890)),
            "1234567890"
        );
        assert_eq!(format_any_value(&mut buf, &prec, AnyValue::Int64(7)), "7");
        assert_eq!(
            format_any_value(&mut buf, &prec, AnyValue::Float64(0.5)),
            "0.50"
        );
        assert_eq!(
            format_any_value(&mut buf, &prec, AnyValue::String("static")),
            "static"
        );
        assert_eq!(format_any_value(&mut buf, &prec, AnyValue::Null), "");
        assert_eq!(
            format_any_value(&mut buf, &prec, AnyValue::Date(0)),
            "1970-01-01"
        );
        assert_eq!(
            format_any_value(&mut buf, &prec, AnyValue::Time(0)),
            "00:00:00"
        );
        assert_eq!(format_any_value(&mut buf, &prec, AnyValue::UInt8(9)), "9");
    }

    #[test]
    fn same_value_is_stable_across_repeated_calls() {
        let mut buf = String::new();
        let series = struct_series();
        for _ in 0..3 {
            assert_eq!(
                format_any_value(&mut buf, &None, series.get(0).unwrap()),
                "{2 fields}"
            );
            assert_eq!(
                format_any_value(&mut buf, &None, AnyValue::Decimal(-50, 10, 3)),
                "-0.050"
            );
            assert_eq!(
                format_any_value(
                    &mut buf,
                    &None,
                    AnyValue::Datetime(0, TimeUnit::Milliseconds, None)
                ),
                "1970-01-01 00:00:00"
            );
        }
    }
}
