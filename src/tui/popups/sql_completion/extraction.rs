use super::context::{detect_sql_context, CompletionContext};

/// Characters that delimit the "current word" being typed.
pub fn is_separator(character: char) -> bool {
    character.is_whitespace()
        || matches!(
            character,
            ',' | '(' | ')' | '=' | '<' | '>' | '!' | '+' | '-' | '/' | ';' | '\'' | '"' | '.'
        )
}

/// Extract the partial token at `cursor` and determine the completion context
/// by analysing the preceding SQL with `sqlparser`'s tokenizer.
///
/// `sql_prefix` is an optional SQL fragment prepended before the actual value
/// so that the tokenizer sees a syntactically meaningful context.  For example,
/// the *Select* command passes `"SELECT "` and the *Filter* command passes
/// `"SELECT * FROM _ WHERE "`.
pub fn extract_token_and_context(
    value: &str,
    cursor: usize,
    sql_prefix: &str,
) -> (String, CompletionContext) {
    let cursor = cursor.min(value.len());
    let before_cursor = &value[..cursor];

    // Find the start of the current partial word.
    let token_start = before_cursor
        .char_indices()
        .rev()
        .find(|(_, character)| is_separator(*character))
        .map(|(index, character)| index + character.len_utf8())
        .unwrap_or(0);

    let token = before_cursor[token_start..].to_string();
    let before_token = &before_cursor[..token_start];

    // Build the full SQL text that the tokenizer will analyse.  The prefix
    // gives the tokenizer enough leading context so that e.g. a bare
    // `col1, col2` typed in the *Select* input is seen as part of a SELECT
    // clause.
    let full_before = format!("{sql_prefix}{before_token}");

    let context = detect_sql_context(&full_before);

    (token, context)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::empty_input("", 0, "", "", CompletionContext::None)]
    #[case::simple_column_after_select("col", 3, "SELECT ", "col", CompletionContext::Column)]
    #[case::cursor_in_middle_of_word("column_name", 3, "SELECT ", "col", CompletionContext::Column)]
    #[case::after_comma_in_select("a, b", 4, "SELECT ", "b", CompletionContext::Column)]
    #[case::table_after_from("tab", 3, "SELECT * FROM ", "tab", CompletionContext::Table)]
    #[case::qualified_column("t.col", 5, "SELECT ", "col", CompletionContext::QualifiedColumn("t".to_string()))]
    #[case::after_where("x", 1, "SELECT * FROM _ WHERE ", "x", CompletionContext::Column)]
    #[case::cursor_beyond_length("ab", 100, "SELECT ", "ab", CompletionContext::Column)]
    #[case::at_separator_boundary("a ", 2, "SELECT ", "", CompletionContext::None)]
    fn test_extract_token_and_context(
        #[case] value: &str,
        #[case] cursor: usize,
        #[case] sql_prefix: &str,
        #[case] expected_token: &str,
        #[case] expected_context: CompletionContext,
    ) {
        let (token, context) = extract_token_and_context(value, cursor, sql_prefix);
        assert_eq!(token, expected_token);
        assert_eq!(context, expected_context);
    }
}
