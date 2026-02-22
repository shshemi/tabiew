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
    if let Some(before_dot) = trimmed.strip_suffix('.') {
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
                if tokens.len() >= 2
                    && let Token::Word(prev) = tokens[tokens.len() - 2]
                        && matches!(prev.keyword, Keyword::ORDER | Keyword::GROUP) {
                            return CompletionContext::Column;
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    // Empty / whitespace-only input.
    #[case::empty("", CompletionContext::None)]
    #[case::whitespace_only("   ", CompletionContext::None)]
    // Qualified column via trailing dot.
    #[case::qualified_column("SELECT t.", CompletionContext::QualifiedColumn("t".to_string()))]
    #[case::qualified_column_with_schema("SELECT schema.t.", CompletionContext::QualifiedColumn("schema.t".to_string()))]
    // Keywords that yield Column context.
    #[case::select("SELECT", CompletionContext::Column)]
    #[case::select_distinct("SELECT DISTINCT", CompletionContext::Column)]
    #[case::where_clause("SELECT * FROM t WHERE", CompletionContext::Column)]
    #[case::and("SELECT * FROM t WHERE a = 1 AND", CompletionContext::Column)]
    #[case::or("SELECT * FROM t WHERE a = 1 OR", CompletionContext::Column)]
    #[case::having("SELECT a FROM t GROUP BY a HAVING", CompletionContext::Column)]
    #[case::between("SELECT * FROM t WHERE a BETWEEN", CompletionContext::Column)]
    #[case::case_keyword("SELECT CASE", CompletionContext::Column)]
    #[case::when_keyword("SELECT CASE WHEN", CompletionContext::Column)]
    #[case::when_keyword("SELECT CASE WHEN a", CompletionContext::None)]
    #[case::then_keyword("SELECT CASE WHEN a THEN", CompletionContext::Column)]
    #[case::else_keyword("SELECT CASE WHEN a THEN b ELSE", CompletionContext::Column)]
    #[case::in_keyword("SELECT * FROM t WHERE a IN", CompletionContext::Column)]
    #[case::like_keyword("SELECT * FROM t WHERE a LIKE", CompletionContext::Column)]
    #[case::is_keyword("SELECT * FROM t WHERE a IS", CompletionContext::Column)]
    #[case::not_keyword("SELECT * FROM t WHERE NOT", CompletionContext::Column)]
    #[case::set_keyword("UPDATE t SET", CompletionContext::Column)]
    #[case::on_keyword("SELECT * FROM a JOIN b ON", CompletionContext::Column)]
    // Keywords that yield Table context.
    #[case::from("SELECT * FROM", CompletionContext::Table)]
    #[case::join("SELECT * FROM a JOIN", CompletionContext::Table)]
    #[case::into("INSERT INTO", CompletionContext::Table)]
    // ORDER BY / GROUP BY.
    #[case::order_by("SELECT * FROM t ORDER BY", CompletionContext::Column)]
    #[case::group_by("SELECT a FROM t GROUP BY", CompletionContext::Column)]
    // BY without ORDER/GROUP preceding it.
    #[case::bare_by("BY", CompletionContext::None)]
    // ASC / DESC.
    #[case::asc("SELECT * FROM t ORDER BY a ASC,", CompletionContext::Column)]
    #[case::desc("SELECT * FROM t ORDER BY a DESC", CompletionContext::Column)]
    // Keywords that yield None.
    #[case::as_keyword("SELECT a AS", CompletionContext::None)]
    #[case::limit("SELECT * FROM t LIMIT", CompletionContext::None)]
    #[case::offset("SELECT * FROM t LIMIT 10 OFFSET", CompletionContext::None)]
    // Comparison operators.
    #[case::eq("SELECT * FROM t WHERE a =", CompletionContext::Column)]
    #[case::neq("SELECT * FROM t WHERE a !=", CompletionContext::Column)]
    #[case::lt("SELECT * FROM t WHERE a <", CompletionContext::Column)]
    #[case::gt("SELECT * FROM t WHERE a >", CompletionContext::Column)]
    #[case::lte("SELECT * FROM t WHERE a <=", CompletionContext::Column)]
    #[case::gte("SELECT * FROM t WHERE a >=", CompletionContext::Column)]
    // Left paren.
    #[case::lparen("SELECT COUNT(", CompletionContext::Column)]
    // Comma context (scans back to enclosing clause).
    #[case::comma_in_select("SELECT a,", CompletionContext::Column)]
    #[case::comma_in_from("SELECT * FROM a,", CompletionContext::Table)]
    #[case::comma_in_where("SELECT * FROM t WHERE a = 1 AND b IN (1,", CompletionContext::Column)]
    #[case::comma_in_order_by("SELECT * FROM t ORDER BY a,", CompletionContext::Column)]
    // Identifier (not a keyword) yields None.
    #[case::plain_identifier("SELECT a", CompletionContext::None)]
    fn test_detect_sql_context(
        #[case] before_token: &str,
        #[case] expected: CompletionContext,
    ) {
        assert_eq!(detect_sql_context(before_token), expected);
    }
}
