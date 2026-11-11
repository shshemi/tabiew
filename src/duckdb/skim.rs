use duckdb::{
    core::{DataChunkHandle, Inserter, LogicalTypeId},
    ffi::duckdb_string_t,
    types::DuckString,
    vscalar::{ScalarFunctionSignature, VScalar},
    vtab::arrow::WritableVector,
};
use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use std::{error::Error, sync::LazyLock};

pub(super) const NAME: &str = "tw_skim_score";
pub(super) const FLOAT_TEXT: &str = "tw_float_text";

static MATCHER: LazyLock<SkimMatcherV2> = LazyLock::new(SkimMatcherV2::default);

pub(super) struct SkimScore;

pub(super) struct FloatText;

impl VScalar for FloatText {
    type State = ();
    fn invoke(
        _: &Self::State,
        input: &mut DataChunkHandle,
        output: &mut dyn WritableVector,
    ) -> Result<(), Box<dyn Error>> {
        let len = input.len();
        let values = input.flat_vector(0);
        let floats = unsafe { values.as_slice_with_len::<f64>(len) };
        let mut out = output.flat_vector();
        for (row, value) in floats.iter().enumerate() {
            match values.row_is_null(row as u64) {
                true => out.set_null(row),
                false => out.insert(row, value.to_string().as_str()),
            }
        }
        Ok(())
    }
    fn signatures() -> Vec<ScalarFunctionSignature> {
        vec![ScalarFunctionSignature::exact(
            vec![LogicalTypeId::Double.into()],
            LogicalTypeId::Varchar.into(),
        )]
    }
}

impl VScalar for SkimScore {
    type State = ();
    fn invoke(
        _: &Self::State,
        input: &mut DataChunkHandle,
        output: &mut dyn WritableVector,
    ) -> Result<(), Box<dyn Error>> {
        let len = input.len();
        let texts = input.flat_vector(0);
        let patterns = input.flat_vector(1);
        let text_values = unsafe { texts.as_slice_with_len::<duckdb_string_t>(len) };
        let pattern_values = unsafe { patterns.as_slice_with_len::<duckdb_string_t>(len) };
        let mut out = output.flat_vector();
        for row in 0..len {
            if texts.row_is_null(row as u64) || patterns.row_is_null(row as u64) {
                out.set_null(row);
                continue;
            }
            let (mut text, mut pattern) = (text_values[row], pattern_values[row]);
            let text = DuckString::new(&mut text).as_str();
            let pattern = DuckString::new(&mut pattern).as_str();
            let score = match text == pattern {
                true => Some(i64::MAX),
                false => MATCHER.fuzzy_match(&text, &pattern),
            };
            match score {
                Some(score) => unsafe { out.as_mut_slice_with_len::<i64>(len)[row] = score },
                None => out.set_null(row),
            }
        }
        Ok(())
    }
    fn signatures() -> Vec<ScalarFunctionSignature> {
        vec![ScalarFunctionSignature::exact(
            vec![LogicalTypeId::Varchar.into(), LogicalTypeId::Varchar.into()],
            LogicalTypeId::Bigint.into(),
        )]
    }
}
