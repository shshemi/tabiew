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
    pub fn run(&self, ext: Extension) -> Receiver<bool> {
        let this = self.clone();
        let (send, recv) = channel();
        submit(move |db| send.send(this.check(db, ext)));
        recv
    }

    pub(in crate::sql::ops) fn check(&self, db: &Database, ext: Extension) -> AppResult<bool> {
        let mut stmt =
            db.prepare("SELECT installed FROM duckdb_extensions() WHERE extension_name = ?")?;
        let mut rows = stmt.query([ext.as_sql()])?;
        Ok(match rows.next()? {
            Some(row) => matches!(row.get_ref(0)?, ValueRef::Boolean(true)),
            None => false,
        })
    }
}
