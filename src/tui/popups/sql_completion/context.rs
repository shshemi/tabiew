use sqlparser::keywords::Keyword;
use sqlparser::tokenizer::{Token, TokenWithSpan, Tokenizer};

use sqlparser::dialect::GenericDialect;

use super::extraction::is_separator;

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

/// Analyse the text preceding the current token to decide what kind of
/// completion is appropriate.
pub(super) fn detect_sql_context(before_token: &str) -> CompletionContext {
    let trimmed = before_token.trim_end();
    if trimmed.is_empty() {
        return CompletionContext::None;
    }

    // Qualified column: the character immediately before the token is `.`.
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
        .map(|token_with_span| &token_with_span.token)
        .filter(|token| !matches!(token, Token::Whitespace(_)))
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
                // ORDER BY / GROUP BY → columns.
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
