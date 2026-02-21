use std::collections::HashSet;

use polars::frame::DataFrame;

use crate::misc::sql::sql;

/// Gather all unique column names: first from the given DataFrame (the `_`
/// table), then from every registered table in the SQL backend.
pub fn collect_all_columns(dataframe: Option<&DataFrame>) -> Vec<String> {
    let mut columns: Vec<String> = Vec::new();
    let mut seen = HashSet::new();

    if let Some(dataframe) = dataframe {
        for name in dataframe.get_column_names() {
            let name = name.to_string();
            if seen.insert(name.clone()) {
                columns.push(name);
            }
        }
    }

    let backend = sql();
    for (_, info) in backend.schema().iter() {
        for (name, _) in info.schema().iter() {
            if seen.insert(name.clone()) {
                columns.push(name.clone());
            }
        }
    }

    columns
}

/// Return column names for a specific table.
pub fn get_table_columns(table: &str, default_dataframe: Option<&DataFrame>) -> Vec<String> {
    if table == "_" {
        if let Some(dataframe) = default_dataframe {
            return dataframe
                .get_column_names()
                .into_iter()
                .map(|s| s.to_string())
                .collect();
        }
    }

    let backend = sql();
    if let Some(info) = backend.schema().get(table) {
        info.schema().iter().map(|(name, _)| name.clone()).collect()
    } else {
        Vec::new()
    }
}

/// Return all known table names (including `_` when a default DataFrame is
/// present).
pub fn get_table_names(has_default: bool) -> Vec<String> {
    let backend = sql();
    let mut names: Vec<String> = Vec::new();
    if has_default {
        names.push("_".to_string());
    }
    for (name, _) in backend.schema().iter() {
        names.push(name.clone());
    }
    names
}

/// Filter a candidate list by prefix, returning matches where the candidate
/// starts with `token_lower` but is not an exact match.
pub fn filter_by_prefix<'a>(
    candidates: impl Iterator<Item = &'a String>,
    token_lower: &str,
) -> Vec<String> {
    candidates
        .filter(|candidate| {
            let candidate_lower = candidate.to_lowercase();
            candidate_lower.starts_with(token_lower) && candidate_lower != token_lower
        })
        .cloned()
        .collect()
}
