use polars::{
    datatypes::{AnyValue, CategoricalMapping, DataType},
    frame::DataFrame,
    series::Series,
};
use rayon::iter::{ParallelBridge, ParallelIterator};
use unicode_width::UnicodeWidthStr;

use crate::{misc::config::config, tui::misc::any_value_converter::format_any_value};

#[derive(Default)]
pub struct DataFrameWidthsCalculator {
    wc: SeriesWidthCalculator,
}

impl DataFrameWidthsCalculator {
    pub fn calculate(&mut self, df: &DataFrame) -> Vec<u16> {
        df.columns()
            .iter()
            .map(|col| self.wc.calculate(col.as_materialized_series()))
            .collect()
    }
}

pub struct SeriesWidthCalculator {
    wc: Vec<AnyValueWidthCalculator>,
}

impl SeriesWidthCalculator {
    pub fn calculate(&mut self, series: &Series) -> u16 {
        let chunk_count = self.wc.len();
        let chunk_len = series.len().div_ceil(chunk_count);
        let name_width = series.name().width() as u16;
        let value_width = match series.dtype() {
            DataType::Null => Some(0),
            DataType::Boolean => Some(5),
            DataType::Date => Some(10),
            DataType::Time => Some(8),
            DataType::Datetime(_, _) => Some(19),
            _ => (0..chunk_count)
                .map(|i| (i * chunk_len) as i64)
                .map(|start| series.slice(start, chunk_len))
                .zip(self.wc.iter_mut())
                .par_bridge()
                .map(|(chunk, wc)| chunk.iter().map(|val| wc.calculate(val)).max())
                .flatten()
                .max(),
        }
        .unwrap_or_default();
        name_width.max(value_width)
    }
}

impl Default for SeriesWidthCalculator {
    fn default() -> Self {
        Self {
            wc: vec![AnyValueWidthCalculator::default(); num_cpus::get()],
        }
    }
}

#[derive(Clone)]
pub struct AnyValueWidthCalculator {
    buf: String,
    fp_prec: Option<usize>,
}

impl Default for AnyValueWidthCalculator {
    fn default() -> Self {
        Self {
            buf: Default::default(),
            fp_prec: config().fp_precision(),
        }
    }
}

impl AnyValueWidthCalculator {
    #[inline]
    pub fn calculate(&mut self, value: AnyValue) -> u16 {
        match value {
            AnyValue::Null => 0,
            AnyValue::Boolean(_) => 5,
            AnyValue::Date(_) => 10, // 1970-10-10
            AnyValue::Time(_) => 8,  // 07:49:05
            AnyValue::Datetime(_, _, _) | AnyValue::DatetimeOwned(_, _, _) => 19, // 2019-06-30 07:49:05
            AnyValue::String(s) => Self::first_line_width(s),
            AnyValue::UInt8(u) => Self::uint_width(u),
            AnyValue::UInt16(u) => Self::uint_width(u),
            AnyValue::UInt32(u) => Self::uint_width(u),
            AnyValue::UInt64(u) => Self::uint_width(u),
            AnyValue::UInt128(u) => Self::uint_width(u),
            AnyValue::Int8(i) => Self::int_width(i.into()),
            AnyValue::Int16(i) => Self::int_width(i.into()),
            AnyValue::Int32(i) => Self::int_width(i.into()),
            AnyValue::Int64(i) => Self::int_width(i.into()),
            AnyValue::Int128(i) => Self::int_width(i),
            AnyValue::Float16(_)
            | AnyValue::Float32(_)
            | AnyValue::Float64(_)
            | AnyValue::Duration(_, _)
            | AnyValue::Decimal(_, _, _)
            | AnyValue::Categorical(_, _)
            | AnyValue::CategoricalOwned(_, _)
            | AnyValue::Enum(_, _)
            | AnyValue::EnumOwned(_, _) => self.str_width(value),
            AnyValue::List(series) => Self::uint_width(series.len() as u128) + 8, // [123 items]
            AnyValue::Array(_, n) => Self::uint_width(n as u128) + 8,             // [123 items]
            AnyValue::Struct(_, _, fields) => Self::uint_width(fields.len() as u128) + 9, // {5 fields}
            AnyValue::StructOwned(st) => Self::uint_width(st.0.len() as u128) + 9, // {5 fields}
            AnyValue::StringOwned(s) => Self::first_line_width(&s),
            AnyValue::Binary(items) => Self::uint_width(items.len() as u128) + 8, // [20 bytes]
            AnyValue::BinaryOwned(items) => Self::uint_width(items.len() as u128) + 8, // [20 bytes]
        }
    }

    #[inline]
    fn first_line_width(s: &str) -> u16 {
        s.lines().next().unwrap_or_default().width() as u16
    }

    #[inline]
    fn uint_width(value: impl Into<u128>) -> u16 {
        value.into().checked_ilog10().map_or(1, |log| log + 1) as u16
    }

    #[inline]
    fn int_width(value: i128) -> u16 {
        if value.is_negative() {
            Self::uint_width(value.unsigned_abs()) + 1
        } else {
            Self::uint_width(value.unsigned_abs())
        }
    }

    #[inline]
    fn str_width(&mut self, value: AnyValue) -> u16 {
        format_any_value(&mut self.buf, &self.fp_prec, value).width() as u16
    }
}

#[cfg(test)]
mod tests {
    use polars::prelude::*;
    use rstest::rstest;
    use unicode_width::UnicodeWidthStr;

    use super::{AnyValueWidthCalculator, DataFrameWidthsCalculator, SeriesWidthCalculator};
    use crate::{misc::config::config, tui::misc::any_value_converter::format_any_value};

    fn width(value: AnyValue) -> u16 {
        AnyValueWidthCalculator::default().calculate(value)
    }

    fn rendered_width(value: AnyValue) -> u16 {
        let mut buf = String::new();
        format_any_value(&mut buf, &config().fp_precision(), value).width() as u16
    }

    fn series_calculator(chunks: usize) -> SeriesWidthCalculator {
        SeriesWidthCalculator {
            wc: vec![AnyValueWidthCalculator::default(); chunks],
        }
    }

    fn categorical_series() -> Series {
        Series::new("fruit".into(), &["apple", "kiwi", "日本の果物"])
            .cast(&DataType::from_categories(Categories::global()))
            .unwrap()
    }

    fn enum_series() -> Series {
        let categories = FrozenCategories::new(["low", "medium", "high"]).unwrap();
        Series::new("level".into(), &["medium", "low"])
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

    #[rstest]
    #[case::zero(0, 1)]
    #[case::one(1, 1)]
    #[case::nine(9, 1)]
    #[case::ten(10, 2)]
    #[case::ninety_nine(99, 2)]
    #[case::hundred(100, 3)]
    #[case::u64_max(u64::MAX as u128, 20)]
    #[case::u128_max(u128::MAX, 39)]
    fn uint_width_boundaries(#[case] value: u128, #[case] expected: u16) {
        assert_eq!(AnyValueWidthCalculator::uint_width(value), expected);
    }

    #[test]
    fn uint_width_around_every_power_of_ten() {
        let mut power = 1u128;
        for digits in 1..=38u16 {
            assert_eq!(AnyValueWidthCalculator::uint_width(power), digits);
            assert_eq!(
                AnyValueWidthCalculator::uint_width(power - 1),
                digits.max(2) - 1
            );
            power *= 10;
        }
        assert_eq!(AnyValueWidthCalculator::uint_width(power), 39);
    }

    #[rstest]
    #[case::zero(0, 1)]
    #[case::positive(7, 1)]
    #[case::negative(-7, 2)]
    #[case::negative_ten(-10, 3)]
    #[case::i64_min(i64::MIN as i128, 20)]
    #[case::i128_max(i128::MAX, 39)]
    #[case::i128_min(i128::MIN, 40)]
    fn int_width_boundaries(#[case] value: i128, #[case] expected: u16) {
        assert_eq!(AnyValueWidthCalculator::int_width(value), expected);
    }

    #[test]
    fn integer_widths_match_display_length() {
        let mut x = 0x2545F4914F6CDD1Du64;
        for _ in 0..5000 {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            let shift = (x % 64) as u32;
            let v = (x >> shift) as i64 * if x & 1 == 0 { 1 } else { -1 };
            assert_eq!(
                width(AnyValue::Int64(v)) as usize,
                v.to_string().len(),
                "{v}"
            );
            assert_eq!(
                width(AnyValue::UInt64(v as u64)) as usize,
                (v as u64).to_string().len()
            );
            assert_eq!(
                width(AnyValue::Int32(v as i32)) as usize,
                (v as i32).to_string().len()
            );
            assert_eq!(
                width(AnyValue::Int16(v as i16)) as usize,
                (v as i16).to_string().len()
            );
            assert_eq!(
                width(AnyValue::UInt8(v as u8)) as usize,
                (v as u8).to_string().len()
            );
        }
    }

    #[rstest]
    #[case::null(AnyValue::Null, 0)]
    #[case::boolean_true(AnyValue::Boolean(true), 5)]
    #[case::boolean_false(AnyValue::Boolean(false), 5)]
    #[case::date(AnyValue::Date(18077), 10)]
    #[case::time(AnyValue::Time(28_145_123_456_789), 8)]
    #[case::datetime(
        AnyValue::Datetime(1_561_880_945_123, TimeUnit::Milliseconds, None),
        19
    )]
    #[case::datetime_owned(
        AnyValue::DatetimeOwned(1_561_880_945_123, TimeUnit::Milliseconds, None),
        19
    )]
    #[case::u8_max(AnyValue::UInt8(u8::MAX), 3)]
    #[case::u16_max(AnyValue::UInt16(u16::MAX), 5)]
    #[case::u32_max(AnyValue::UInt32(u32::MAX), 10)]
    #[case::u64_max(AnyValue::UInt64(u64::MAX), 20)]
    #[case::u128_max(AnyValue::UInt128(u128::MAX), 39)]
    #[case::i8_min(AnyValue::Int8(i8::MIN), 4)]
    #[case::i16_min(AnyValue::Int16(i16::MIN), 6)]
    #[case::i32_min(AnyValue::Int32(i32::MIN), 11)]
    #[case::i64_min(AnyValue::Int64(i64::MIN), 20)]
    #[case::i128_min(AnyValue::Int128(i128::MIN), 40)]
    #[case::list_three(AnyValue::List(Series::new("l".into(), &[1i32, 2, 3])), 9)]
    #[case::list_empty(AnyValue::List(Series::new_empty("l".into(), &DataType::Int32)), 9)]
    #[case::list_many(AnyValue::List(Series::new("l".into(), &vec![0u8; 12345])), 13)]
    #[case::binary_three(AnyValue::Binary(&[1, 2, 3]), 9)]
    #[case::binary_empty(AnyValue::Binary(&[]), 9)]
    #[case::binary_owned(AnyValue::BinaryOwned(vec![0u8; 1024]), 12)]
    #[case::duration(AnyValue::Duration(142, TimeUnit::Milliseconds), 5)]
    #[case::duration_all_parts(AnyValue::Duration(90_061_001, TimeUnit::Milliseconds), 15)]
    #[case::duration_micro(AnyValue::Duration(1_500, TimeUnit::Microseconds), 6)]
    #[case::decimal(AnyValue::Decimal(1234, 10, 2), 5)]
    #[case::decimal_leading_zero(AnyValue::Decimal(5, 10, 4), 6)]
    #[case::decimal_negative(AnyValue::Decimal(-50, 10, 3), 6)]
    #[case::decimal_no_scale(AnyValue::Decimal(-42, 10, 0), 3)]
    fn fixed_and_counted_widths(#[case] value: AnyValue<'static>, #[case] expected: u16) {
        assert_eq!(width(value), expected);
    }

    #[test]
    fn datetime_with_time_zone_is_still_nineteen() {
        let tz = TimeZone::opt_try_new(Some("Europe/London")).unwrap();
        let value = AnyValue::Datetime(1_561_880_945_000, TimeUnit::Milliseconds, tz.as_ref());
        assert_eq!(width(value), 19);
    }

    #[rstest]
    #[case::ascii("hello", 5)]
    #[case::empty("", 0)]
    #[case::first_line_only("ab\nlonger second line", 2)]
    #[case::leading_newline("\nsecond", 0)]
    #[case::wide_chars("日本語", 6)]
    #[case::mixed("café 日本", 9)]
    fn string_uses_first_line_display_width(#[case] value: &str, #[case] expected: u16) {
        assert_eq!(width(AnyValue::String(value)), expected);
        assert_eq!(width(AnyValue::StringOwned(value.into())), expected);
    }

    #[test]
    fn categorical_uses_category_display_width() {
        let series = categorical_series();
        assert_eq!(width(series.get(0).unwrap()), 5);
        assert_eq!(width(series.get(1).unwrap()), 4);
        assert_eq!(width(series.get(2).unwrap()), 10);
        assert_eq!(width(series.get(2).unwrap().into_static()), 10);
    }

    #[test]
    fn enum_uses_category_display_width() {
        let series = enum_series();
        assert!(matches!(series.get(0).unwrap(), AnyValue::Enum(..)));
        assert_eq!(width(series.get(0).unwrap()), 6);
        assert_eq!(width(series.get(1).unwrap()), 3);
        assert_eq!(width(series.get(0).unwrap().into_static()), 6);
    }

    #[test]
    fn array_uses_declared_width_not_index() {
        let series = array_series();
        let cell = series.get(0).unwrap();
        assert!(matches!(cell, AnyValue::Array(_, 3)));
        assert_eq!(width(cell), 9);
    }

    #[test]
    fn struct_counts_fields_not_rows() {
        let series = struct_series();
        assert_eq!(width(series.get(0).unwrap()), 10);
        assert_eq!(width(series.get(0).unwrap().into_static()), 10);
    }

    #[test]
    fn every_variant_matches_rendered_width() {
        let tz = TimeZone::opt_try_new(Some("Europe/London")).unwrap();
        let categorical = categorical_series();
        let enums = enum_series();
        let structs = struct_series();
        let arrays = array_series();
        let mut values: Vec<AnyValue> = vec![
            AnyValue::Null,
            AnyValue::Boolean(false),
            AnyValue::String("plain"),
            AnyValue::String("多行\n二"),
            AnyValue::StringOwned("owned".into()),
            AnyValue::UInt8(200),
            AnyValue::UInt16(60000),
            AnyValue::UInt32(4_000_000_000),
            AnyValue::UInt64(u64::MAX),
            AnyValue::UInt128(u128::MAX),
            AnyValue::Int8(-100),
            AnyValue::Int16(-30000),
            AnyValue::Int32(i32::MIN),
            AnyValue::Int64(i64::MIN),
            AnyValue::Int128(i128::MIN),
            AnyValue::Float16(pf16::from(1.5f32)),
            AnyValue::Float32(-2.25),
            AnyValue::Float64(0.1 + 0.2),
            AnyValue::Float64(1e20),
            AnyValue::Float64(f64::NAN),
            AnyValue::Float64(f64::NEG_INFINITY),
            AnyValue::Date(0),
            AnyValue::Date(2_932_896),
            AnyValue::Time(86_399_999_999_999),
            AnyValue::Datetime(1_561_880_945_123, TimeUnit::Milliseconds, None),
            AnyValue::Datetime(1_561_880_945_123, TimeUnit::Milliseconds, tz.as_ref()),
            AnyValue::DatetimeOwned(-1, TimeUnit::Nanoseconds, None),
            AnyValue::Duration(0, TimeUnit::Microseconds),
            AnyValue::Duration(-90_061_001, TimeUnit::Milliseconds),
            AnyValue::Duration(3_723_001_000_000, TimeUnit::Nanoseconds),
            categorical.get(2).unwrap(),
            categorical.get(0).unwrap().into_static(),
            enums.get(0).unwrap(),
            enums.get(1).unwrap().into_static(),
            AnyValue::List(Series::new("l".into(), &[1i32, 2, 3])),
            arrays.get(1).unwrap(),
            structs.get(3).unwrap(),
            structs.get(3).unwrap().into_static(),
            AnyValue::Binary(&[1, 2, 3]),
            AnyValue::BinaryOwned(vec![0u8; 100]),
            AnyValue::Decimal(5, 10, 4),
            AnyValue::Decimal(-50, 10, 3),
            AnyValue::Decimal(i128::MAX, 38, 38),
            AnyValue::Decimal(i128::MIN + 1, 38, 0),
        ];
        let mut x = 0x9E3779B97F4A7C15u64;
        for _ in 0..3000 {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            let f = f64::from_bits(x);
            if f.is_finite() {
                values.push(AnyValue::Float64(f));
            }
            values.push(AnyValue::Decimal(
                (x as i128) % 10i128.pow((x % 30) as u32),
                38,
                (x % 12) as usize,
            ));
            values.push(AnyValue::Duration(
                x as i64 % 400_000_000_000,
                TimeUnit::Nanoseconds,
            ));
        }
        for value in values {
            assert_eq!(
                width(value.clone()),
                rendered_width(value.clone()),
                "{value:?}"
            );
        }
    }

    #[test]
    fn series_width_is_max_of_values() {
        let series = Series::new("n".into(), &[1i64, 22, 333, 4444, 55]);
        assert_eq!(series_calculator(2).calculate(&series), 4);
    }

    #[test]
    fn series_width_uses_name_when_longer() {
        let series = Series::new("a_rather_long_name".into(), &[1i64, 2, 3]);
        assert_eq!(series_calculator(2).calculate(&series), 18);
    }

    #[test]
    fn series_name_width_counts_wide_characters() {
        let series = Series::new("名前".into(), &[1i64]);
        assert_eq!(series_calculator(1).calculate(&series), 4);
    }

    #[test]
    fn empty_series_falls_back_to_name() {
        let series = Series::new_empty("empty".into(), &DataType::Int64);
        assert_eq!(series_calculator(4).calculate(&series), 5);
    }

    #[test]
    fn nulls_do_not_contribute_width() {
        let series = Series::new("n".into(), &[None, Some(12345i64), None]);
        assert_eq!(series_calculator(2).calculate(&series), 5);
    }

    #[rstest]
    #[case::boolean("is_active", DataType::Boolean, 9)]
    #[case::boolean_short_name("b", DataType::Boolean, 5)]
    #[case::null("nulls", DataType::Null, 5)]
    #[case::null_short_name("n", DataType::Null, 1)]
    #[case::date("d", DataType::Date, 10)]
    #[case::date_long_name("a_column_of_dates", DataType::Date, 17)]
    #[case::time("t", DataType::Time, 8)]
    #[case::datetime("ts", DataType::Datetime(TimeUnit::Milliseconds, None), 19)]
    #[case::datetime_long_name(
        "created_at_timestamp_utc",
        DataType::Datetime(TimeUnit::Nanoseconds, None),
        24
    )]
    fn fixed_width_dtypes_still_honor_name(
        #[case] name: &str,
        #[case] dtype: DataType,
        #[case] expected: u16,
    ) {
        let series = Series::full_null(name.into(), 3, &dtype);
        assert_eq!(series_calculator(2).calculate(&series), expected);
    }

    #[rstest]
    #[case::one_chunk(1)]
    #[case::three_chunks(3)]
    #[case::more_chunks_than_rows(50)]
    fn widest_value_found_regardless_of_chunking(#[case] chunks: usize) {
        let mut values = vec![1i64; 7];
        for pos in 0..7 {
            values.fill(1);
            values[pos] = 1_000_000;
            let series = Series::new("n".into(), &values);
            assert_eq!(
                series_calculator(chunks).calculate(&series),
                7,
                "max at {pos}"
            );
        }
    }

    #[test]
    fn large_series_with_max_in_last_row() {
        let mut values = vec![5i64; 10_007];
        values[10_006] = -123_456_789;
        let series = Series::new("n".into(), &values);
        assert_eq!(series_calculator(8).calculate(&series), 10);
    }

    #[test]
    fn string_series_uses_first_line_widths() {
        let series = Series::new("s".into(), &["ab", "第一行\n第二行はとても長い", "xyz"]);
        assert_eq!(series_calculator(2).calculate(&series), 6);
    }

    #[test]
    fn categorical_series_width() {
        let series = categorical_series();
        assert_eq!(series_calculator(2).calculate(&series), 10);
    }

    #[test]
    fn default_series_calculator_has_one_worker_per_cpu() {
        assert_eq!(SeriesWidthCalculator::default().wc.len(), num_cpus::get());
    }

    #[test]
    fn data_frame_widths_follow_column_order() {
        let df = df!(
            "id" => [1i64, 22, 333],
            "name" => ["ann", "bob", "christopher"],
            "ok" => [true, false, true],
            "when" => [0i32, 1, 2],
        )
        .unwrap()
        .lazy()
        .with_column(col("when").cast(DataType::Date))
        .collect()
        .unwrap();
        assert_eq!(
            DataFrameWidthsCalculator::default().calculate(&df),
            vec![3, 11, 5, 10]
        );
    }

    #[test]
    fn data_frame_without_columns_yields_no_widths() {
        let df = DataFrame::empty();
        assert!(
            DataFrameWidthsCalculator::default()
                .calculate(&df)
                .is_empty()
        );
    }

    #[test]
    fn data_frame_calculator_is_reusable() {
        let mut calc = DataFrameWidthsCalculator::default();
        let wide = df!("v" => [1_000_000i64]).unwrap();
        let narrow = df!("v" => [1i64]).unwrap();
        assert_eq!(calc.calculate(&wide), vec![7]);
        assert_eq!(calc.calculate(&narrow), vec![1]);
        assert_eq!(calc.calculate(&wide), vec![7]);
    }
}
