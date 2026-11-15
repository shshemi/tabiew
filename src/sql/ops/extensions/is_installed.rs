use std::marker::PhantomData;

use crate::{
    AppResult,
    sql::{
        db::{Database, ValueRef},
        ops::extensions::Extension,
        ops::reply::{Receiver, channel},
        worker::submit,
    },
};

#[derive(Debug, Clone, Default)]
pub struct IsInstalled;

impl IsInstalled {
    pub fn new() -> Self {
        Self
    }

    pub fn run<E: Extension>(&self) -> Receiver<Option<Token<E>>> {
        let this = self.clone();
        let (send, recv) = channel();
        submit(move |db| send.send(this.check::<E>(db)));
        recv
    }

    fn check<E: Extension>(&self, db: &Database) -> AppResult<Option<Token<E>>> {
        let mut stmt =
            db.prepare("SELECT installed FROM duckdb_extensions() WHERE extension_name = ?")?;
        let mut rows = stmt.query([E::as_sql()])?;
        match rows.next()? {
            Some(row) if matches!(row.get_ref(0)?, ValueRef::Boolean(true)) => Ok(Some(Token {
                _phantom: PhantomData,
            })),
            _ => Ok(None),
        }
    }
}

pub struct Token<E> {
    _phantom: PhantomData<E>,
}
