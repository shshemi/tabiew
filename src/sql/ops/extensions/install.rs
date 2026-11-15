use crate::{
    AppResult,
    sql::{
        db::Database,
        ops::extensions::Extension,
        ops::{
            reply::{Receiver, channel},
            util::quote_literal,
        },
        worker::submit,
    },
};

#[derive(Debug, Clone, Default)]
pub struct Install {
    force: bool,
    from: Option<Repository>,
    version: Option<String>,
}

impl Install {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_force(mut self, force: bool) -> Self {
        self.force = force;
        self
    }

    pub fn with_from(mut self, from: Repository) -> Self {
        self.from = Some(from);
        self
    }

    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub fn run<E: Extension>(&self) -> Receiver<()> {
        let this = self.clone();
        let (send, recv) = channel();
        submit(move |db| send.send(this.install::<E>(db)));
        recv
    }

    fn install<E: Extension>(&self, db: &mut Database) -> AppResult<()> {
        let mut sql = String::new();
        if self.force {
            sql.push_str("FORCE ");
        }
        sql.push_str("INSTALL ");
        sql.push_str(E::as_sql());
        if let Some(from) = &self.from {
            sql.push_str(" FROM ");
            sql.push_str(&from.as_sql());
        }
        if let Some(version) = &self.version {
            sql.push_str(" VERSION ");
            sql.push_str(&quote_literal(version));
        }

        db.execute(&sql, [])?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Repository {
    Core,
    CoreNightly,
    Community,
    Custom(String),
}

impl Repository {
    pub fn as_sql(&self) -> String {
        match self {
            Repository::Core => "core".to_owned(),
            Repository::CoreNightly => "core_nightly".to_owned(),
            Repository::Community => "community".to_owned(),
            Repository::Custom(url) => quote_literal(url),
        }
    }
}
