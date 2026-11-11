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
