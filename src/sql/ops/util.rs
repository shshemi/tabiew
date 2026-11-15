use std::path::Path;

use anyhow::anyhow;
use duckdb::types::{OrderedMap, Value};

use crate::AppResult;

pub fn list_to_duck_db_value(items: Vec<String>) -> Option<Value> {
    (!items.is_empty()).then(|| Value::List(items.into_iter().map(Value::Text).collect()))
}

pub fn pairs_to_duck_db_value(items: Vec<(String, String)>) -> Option<Value> {
    (!items.is_empty()).then(|| {
        Value::Struct(OrderedMap::from(
            items
                .into_iter()
                .map(|(name, ty)| (name, Value::Text(ty)))
                .collect::<Vec<_>>(),
        ))
    })
}

pub fn table_name(path: &Path) -> AppResult<String> {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .map(ToOwned::to_owned)
        .ok_or_else(|| anyhow!("invalid file name: {}", path.display()))
}

pub fn utf8_path(path: &Path) -> AppResult<&str> {
    path.to_str()
        .ok_or_else(|| anyhow!("non UTF-8 path: {}", path.display()))
}

pub fn quote_ident(ident: &str) -> String {
    format!("\"{}\"", ident.replace('"', "\"\""))
}
