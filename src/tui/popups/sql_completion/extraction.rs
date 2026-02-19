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
