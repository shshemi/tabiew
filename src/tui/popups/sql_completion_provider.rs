use polars::frame::DataFrame;

use crate::{
    sql_completion::{
        self, CompletionContext, collect_all_columns, filter_by_prefix, get_table_columns,
        get_table_names,
    },
    tui::pickers::text_picker_with_suggestion::Provider,
};

/// A `Provider` implementation that uses SQL context detection to generate
/// autocompletion suggestions for column names, table names, etc.
#[derive(Debug)]
pub struct SqlCompletionProvider {
    sql_prefix: &'static str,
    dataframe: Option<DataFrame>,
    all_columns: Vec<String>,
}

impl SqlCompletionProvider {
    pub fn new(sql_prefix: &'static str, dataframe: Option<DataFrame>) -> Self {
        let all_columns = collect_all_columns(dataframe.as_ref());
        Self {
            sql_prefix,
            dataframe,
            all_columns,
        }
    }
}

impl Provider for SqlCompletionProvider {
    fn suggestions(&self, value: &str, cursor: usize) -> Vec<String> {
        let (token, context) =
            sql_completion::extract_token_and_context(value, cursor, self.sql_prefix);

        if token.is_empty() {
            return Vec::new();
        }

        let token_lower = token.to_lowercase();

        match &context {
            CompletionContext::Column => filter_by_prefix(self.all_columns.iter(), &token_lower),
            CompletionContext::QualifiedColumn(table) => {
                let columns = get_table_columns(table, self.dataframe.as_ref());
                filter_by_prefix(columns.iter(), &token_lower)
            }
            CompletionContext::Table => {
                let tables = get_table_names(self.dataframe.is_some());
                filter_by_prefix(tables.iter(), &token_lower)
            }
            CompletionContext::None => Vec::new(),
        }
    }

    fn is_separator(&self, character: char) -> bool {
        sql_completion::is_separator(character)
    }
}
