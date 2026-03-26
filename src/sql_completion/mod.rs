mod context;
mod data;
mod extraction;
mod suggestion;

use polars::frame::DataFrame;

pub use context::CompletionContext;
pub use data::{collect_all_columns, filter_by_prefix, get_table_columns, get_table_names};
pub use extraction::extract_token_and_context;
pub use suggestion::SqlSuggestion;

const SQL_KEYWORDS: &[&str] = &[
    "AS",
    "ASC",
    "BY",
    "CASE",
    "CREATE",
    "DELETE",
    "DESC",
    "DISTINCT",
    "DROP",
    "ELSE",
    "EXCEPT",
    "EXPLAIN",
    "FROM",
    "FULL",
    "GROUP",
    "HAVING",
    "IN",
    "INNER",
    "INTERSECT",
    "IS",
    "JOIN",
    "LEFT",
    "LIKE",
    "LIMIT",
    "NOT",
    "OFFSET",
    "ON",
    "OR",
    "ORDER",
    "OUTER",
    "RIGHT",
    "SELECT",
    "SHOW",
    "TABLE",
    "THEN",
    "TRUNCATE",
    "UNION",
    "USING",
    "WHEN",
    "WHERE",
    "WITH",
];

/// Return completion suggestions for the given input value at the given cursor
/// position, using `sql_prefix` for tokenizer context.
pub fn suggestions(
    value: &str,
    cursor: usize,
    sql_prefix: &str,
    all_columns: &[String],
    dataframe: Option<&DataFrame>,
) -> Vec<SqlSuggestion> {
    let (token, context) = extract_token_and_context(value, cursor, sql_prefix);

    if token.is_empty() {
        return Vec::new();
    }

    let token_lower = token.to_lowercase();

    let strings = match &context {
        CompletionContext::Column => filter_by_prefix(all_columns.iter(), &token_lower),
        CompletionContext::Keyword => keyword_suggestions(&token_lower),
        CompletionContext::QualifiedColumn(table) => {
            let columns = get_table_columns(table, dataframe);
            filter_by_prefix(columns.iter(), &token_lower)
        }
        CompletionContext::Table => {
            let tables = get_table_names(dataframe.is_some());
            filter_by_prefix(tables.iter(), &token_lower)
        }
        CompletionContext::None => Vec::new(),
    };

    strings.into_iter().map(SqlSuggestion::new).collect()
}

fn keyword_suggestions(token_lower: &str) -> Vec<String> {
    SQL_KEYWORDS
        .iter()
        .filter(|keyword| {
            let keyword_lower = keyword.to_lowercase();
            keyword_lower.starts_with(token_lower) && keyword_lower != token_lower
        })
        .map(|keyword| (*keyword).to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::pickers::text_picker_with_suggestion::Suggestion;

    fn suggestion_titles(suggestions: Vec<SqlSuggestion>) -> Vec<String> {
        suggestions
            .into_iter()
            .map(|suggestion| suggestion.title().to_string())
            .collect()
    }

    #[test]
    fn suggests_keywords_at_statement_start() {
        let suggestions = suggestions("sel", 3, "", &[], None);

        assert!(suggestion_titles(suggestions).contains(&"SELECT".to_string()));
    }

    #[test]
    fn suggests_keywords_after_completed_select_item() {
        let suggestions = suggestions("a fr", 4, "SELECT ", &[], None);

        assert!(suggestion_titles(suggestions).contains(&"FROM".to_string()));
    }

    #[test]
    fn does_not_suggest_keywords_for_aliases_after_as() {
        let suggestions = suggestions("SELECT a AS al", "SELECT a AS al".len(), "", &[], None);

        assert!(suggestions.is_empty());
    }

    #[test]
    fn does_not_suggest_unsupported_keywords() {
        let suggestions = suggestions("al", 2, "", &[], None);

        assert!(!suggestion_titles(suggestions).contains(&"ALTER".to_string()));
    }
}
