mod context;
mod data;
mod extraction;

use polars::frame::DataFrame;

pub use context::CompletionContext;
pub use data::{collect_all_columns, filter_by_prefix, get_table_columns, get_table_names};
pub use extraction::{extract_token_and_context, is_separator};

/// Return completion suggestions for the given input value at the given cursor
/// position, using `sql_prefix` for tokenizer context.
pub fn suggestions(
    value: &str,
    cursor: usize,
    sql_prefix: &str,
    all_columns: &[String],
    dataframe: Option<&DataFrame>,
) -> Vec<String> {
    let (token, context) = extract_token_and_context(value, cursor, sql_prefix);

    if token.is_empty() {
        return Vec::new();
    }

    let token_lower = token.to_lowercase();

    match &context {
        CompletionContext::Column => filter_by_prefix(all_columns.iter(), &token_lower),
        CompletionContext::QualifiedColumn(table) => {
            let columns = get_table_columns(table, dataframe);
            filter_by_prefix(columns.iter(), &token_lower)
        }
        CompletionContext::Table => {
            let tables = get_table_names(dataframe.is_some());
            filter_by_prefix(tables.iter(), &token_lower)
        }
        CompletionContext::None => Vec::new(),
    }
}
