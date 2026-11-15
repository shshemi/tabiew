use crate::{
    AppResult,
    sql::{
        db::{Database, ValueRef},
        ops::{
            extensions::Info,
            reply::{Receiver, channel},
        },
        worker::submit,
    },
};

#[derive(Debug, Clone, Default)]
pub struct GetAll;

impl GetAll {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self) -> Receiver<Vec<Info>> {
        let this = self.clone();
        let (send, recv) = channel();
        submit(move |db| send.send(this.get(db)));
        recv
    }

    fn get(&self, db: &Database) -> AppResult<Vec<Info>> {
        let mut stmt = db.prepare(
            "SELECT extension_name, installed, loaded, coalesce(description, '') \
             FROM duckdb_extensions() ORDER BY extension_name",
        )?;
        let mut rows = stmt.query([])?;
        let mut extensions = Vec::new();
        while let Some(row) = rows.next()? {
            extensions.push(Info {
                name: text(row.get_ref(0)?),
                installed: matches!(row.get_ref(1)?, ValueRef::Boolean(true)),
                loaded: matches!(row.get_ref(2)?, ValueRef::Boolean(true)),
                description: text(row.get_ref(3)?),
            });
        }
        Ok(extensions)
    }
}

fn text(value: ValueRef) -> String {
    match value {
        ValueRef::Text(bytes) => String::from_utf8_lossy(bytes).into_owned(),
        _ => String::new(),
    }
}
