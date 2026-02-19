use std::collections::HashSet;

use polars::frame::DataFrame;
use sqlparser::dialect::GenericDialect;
use sqlparser::keywords::Keyword;
use sqlparser::tokenizer::{Token, TokenWithSpan, Tokenizer};

use crate::misc::sql::sql;

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// The kind of completion expected at the current cursor position.
#[derive(Debug, PartialEq, Eq)]
pub enum CompletionContext {
    /// Suggest column names.
    Column,
    /// Suggest column names from a specific table (after `table.`).
    QualifiedColumn(String),
    /// Suggest registered table names.
    Table,
    /// No contextual suggestions.
    None,
}

// ---------------------------------------------------------------------------
// Token / context extraction
// ---------------------------------------------------------------------------

/// Characters that delimit the "current word" being typed.
pub fn is_separator(c: char) -> bool {
    c.is_whitespace()
        || matches!(
            c,
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
        .find(|(_, c)| is_separator(*c))
        .map(|(i, c)| i + c.len_utf8())
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

// ---------------------------------------------------------------------------
// Context detection (powered by sqlparser)
// ---------------------------------------------------------------------------

/// Analyse the text preceding the current token to decide what kind of
/// completion is appropriate.
fn detect_sql_context(before_token: &str) -> CompletionContext {
    let trimmed = before_token.trim_end();
    if trimmed.is_empty() {
        return CompletionContext::None;
    }

    // Qualified column: the character immediately before the token is `.`
    if trimmed.ends_with('.') {
        let before_dot = &trimmed[..trimmed.len() - 1];
        let table_name = before_dot
            .rsplit(|c: char| is_separator(c) && c != '.')
            .next()
            .unwrap_or("");
        if !table_name.is_empty() {
            return CompletionContext::QualifiedColumn(table_name.to_string());
        }
    }

    // Tokenize with sqlparser (partial results on error via _into_buf).
    let dialect = GenericDialect {};
    let mut tokenizer = Tokenizer::new(&dialect, trimmed);
    let mut token_buf: Vec<TokenWithSpan> = Vec::new();
    let _ = tokenizer.tokenize_with_location_into_buf(&mut token_buf);

    let significant: Vec<&Token> = token_buf
        .iter()
        .map(|t| &t.token)
        .filter(|t| !matches!(t, Token::Whitespace(_)))
        .collect();

    if significant.is_empty() {
        return CompletionContext::None;
    }

    classify_last_token(significant.last().unwrap(), &significant)
}

/// Given the last significant SQL token, decide the completion context.
fn classify_last_token(last: &Token, tokens: &[&Token]) -> CompletionContext {
    match last {
        Token::Word(word) => match word.keyword {
            Keyword::SELECT | Keyword::DISTINCT => CompletionContext::Column,
            Keyword::FROM | Keyword::JOIN | Keyword::INTO => CompletionContext::Table,
            Keyword::WHERE | Keyword::AND | Keyword::OR | Keyword::NOT | Keyword::ON
            | Keyword::HAVING | Keyword::BETWEEN | Keyword::CASE | Keyword::WHEN
            | Keyword::THEN | Keyword::ELSE | Keyword::IN | Keyword::LIKE | Keyword::IS
            | Keyword::SET => CompletionContext::Column,
            Keyword::BY => {
                // ORDER BY / GROUP BY → columns
                if tokens.len() >= 2 {
                    if let Token::Word(prev) = tokens[tokens.len() - 2] {
                        if matches!(prev.keyword, Keyword::ORDER | Keyword::GROUP) {
                            return CompletionContext::Column;
                        }
                    }
                }
                CompletionContext::None
            }
            Keyword::ASC | Keyword::DESC => CompletionContext::Column,
            Keyword::AS | Keyword::LIMIT | Keyword::OFFSET => CompletionContext::None,
            _ => CompletionContext::None,
        },
        Token::Eq | Token::Neq | Token::Lt | Token::Gt | Token::LtEq | Token::GtEq => {
            CompletionContext::Column
        }
        Token::LParen => CompletionContext::Column,
        Token::Comma => find_clause_for_comma(tokens),
        _ => CompletionContext::None,
    }
}

/// When the cursor follows a comma, scan backward through the tokens to find
/// the enclosing SQL clause and return the appropriate context.
fn find_clause_for_comma(tokens: &[&Token]) -> CompletionContext {
    for token in tokens.iter().rev().skip(1) {
        if let Token::Word(word) = token {
            match word.keyword {
                Keyword::SELECT | Keyword::DISTINCT => return CompletionContext::Column,
                Keyword::FROM | Keyword::JOIN => return CompletionContext::Table,
                Keyword::WHERE | Keyword::HAVING | Keyword::ON => return CompletionContext::Column,
                Keyword::BY | Keyword::ORDER | Keyword::GROUP => return CompletionContext::Column,
                _ => continue,
            }
        }
    }
    CompletionContext::None
}

// ---------------------------------------------------------------------------
// Data helpers – collect column / table names from the SQL backend
// ---------------------------------------------------------------------------

/// Gather all unique column names: first from the given DataFrame (the `_`
/// table), then from every registered table in the SQL backend.
pub fn collect_all_columns(df: Option<&DataFrame>) -> Vec<String> {
    let mut columns: Vec<String> = Vec::new();
    let mut seen = HashSet::new();

    if let Some(df) = df {
        for name in df.get_column_names() {
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
pub fn get_table_columns(table: &str, default_df: Option<&DataFrame>) -> Vec<String> {
    if table == "_" {
        if let Some(df) = default_df {
            return df
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
        .filter(|c| {
            let c_lower = c.to_lowercase();
            c_lower.starts_with(token_lower) && c_lower != token_lower
        })
        .cloned()
        .collect()
}
