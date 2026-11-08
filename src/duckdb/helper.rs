use rand::seq::IndexedRandom;
use sqlparser::{
    ast::{ObjectNamePart, Statement},
    dialect::DuckDbDialect,
    parser::Parser,
};

const ADJECTIVES: [&str; 24] = [
    "brave", "calm", "clever", "eager", "fancy", "gentle", "happy", "jolly", "keen", "lucid",
    "merry", "nifty", "proud", "quiet", "quirky", "sharp", "silly", "sleepy", "swift", "tender",
    "vivid", "witty", "zealous", "zen",
];

const NAMES: [&str; 24] = [
    "babbage",
    "bohr",
    "curie",
    "darwin",
    "dijkstra",
    "einstein",
    "euler",
    "faraday",
    "fermi",
    "galileo",
    "gauss",
    "hopper",
    "kepler",
    "knuth",
    "lovelace",
    "newton",
    "noether",
    "pascal",
    "ramanujan",
    "ritchie",
    "shannon",
    "tesla",
    "turing",
    "wozniak",
];

pub fn quote(name: &str) -> String {
    format!("\"{}\"", name.replace('"', "\"\""))
}

pub struct SplitSql {
    pub setup: Vec<String>,
    pub name: Option<String>,
    pub query: String,
}

pub fn split_sql(sql: &str) -> SplitSql {
    let comment_name = sql
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .and_then(|line| line.strip_prefix("--"))
        .and_then(|line| line.trim().strip_prefix("name:"))
        .map(|name| name.trim().to_string())
        .filter(|name| !name.is_empty());
    let Some(mut statements) = Parser::parse_sql(&DuckDbDialect {}, sql)
        .ok()
        .filter(|statements| !statements.is_empty())
    else {
        return SplitSql {
            setup: Vec::new(),
            name: comment_name,
            query: sql.to_string(),
        };
    };
    let last = statements.pop().expect("statements is not empty");
    let named = match &last {
        Statement::CreateView(view) => Some((&view.name, view.query.as_ref())),
        Statement::CreateTable(table) => table.query.as_deref().map(|query| (&table.name, query)),
        _ => None,
    };
    let (name, query) = match named {
        Some((name, query)) => (
            name.0
                .last()
                .and_then(ObjectNamePart::as_ident)
                .map(|ident| ident.value.clone()),
            query.to_string(),
        ),
        None => (None, last.to_string()),
    };
    SplitSql {
        setup: statements.iter().map(ToString::to_string).collect(),
        name: name.or(comment_name),
        query,
    }
}

pub fn random_name() -> String {
    let mut rng = rand::rng();
    format!(
        "{}_{}",
        ADJECTIVES
            .choose(&mut rng)
            .expect("ADJECTIVES is not empty"),
        NAMES.choose(&mut rng).expect("NAMES is not empty")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn split(sql: &str) -> (Vec<String>, Option<String>, String) {
        let SplitSql { setup, name, query } = split_sql(sql);
        (setup, name, query)
    }

    #[test]
    fn plain_select_has_no_name() {
        assert_eq!(split("SELECT 1"), (vec![], None, "SELECT 1".into()));
    }

    #[test]
    fn create_view_or_table_gives_the_name_and_the_query() {
        assert_eq!(
            split("CREATE VIEW best AS SELECT 1"),
            (vec![], Some("best".into()), "SELECT 1".into())
        );
        assert_eq!(
            split("create table main.best as select 1;"),
            (vec![], Some("best".into()), "SELECT 1".into())
        );
    }

    #[test]
    fn comment_names_the_query_unless_create_view_does() {
        let sql = "\n-- name: top\nSELECT 1";
        assert_eq!(split(sql), (vec![], Some("top".into()), "SELECT 1".into()));
        assert_eq!(
            split("-- name: top\nCREATE VIEW best AS SELECT 1").1,
            Some("best".into())
        );
        assert_eq!(split("-- name:\nSELECT 1").1, None);
    }

    #[test]
    fn all_but_the_last_statement_are_setup() {
        assert_eq!(
            split("CREATE TABLE t AS SELECT 1 AS a; INSERT INTO t VALUES (2); SELECT a FROM t;"),
            (
                vec![
                    "CREATE TABLE t AS SELECT 1 AS a".into(),
                    "INSERT INTO t VALUES (2)".into()
                ],
                None,
                "SELECT a FROM t".into()
            )
        );
    }

    #[test]
    fn unparseable_or_empty_sql_is_kept_as_written() {
        assert_eq!(split("SELEC 1"), (vec![], None, "SELEC 1".into()));
        assert_eq!(split("-- name: x\nSELEC 1").1, Some("x".into()));
        assert_eq!(split(""), (vec![], None, "".into()));
    }

    #[test]
    fn random_name_is_an_adjective_and_a_name() {
        let name = random_name();
        let (adjective, last) = name.split_once('_').unwrap();
        assert!(ADJECTIVES.contains(&adjective));
        assert!(NAMES.contains(&last));
    }
}
