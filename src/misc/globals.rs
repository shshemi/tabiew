use std::{
    io::{Cursor, Read},
    ops::DerefMut,
    sync::{LazyLock, Mutex},
};

use super::sql::SqlBackend;

pub fn sql() -> impl DerefMut<Target = SqlBackend> {
    static SQL_BACKEND: LazyLock<Mutex<SqlBackend>> =
        LazyLock::new(|| Mutex::new(SqlBackend::default()));
    SQL_BACKEND.lock().unwrap()
}

pub fn stdin() -> Cursor<&'static Vec<u8>> {
    static STDIN_CONTENT: LazyLock<Vec<u8>> = LazyLock::new(|| {
        let mut buf = Vec::new();
        std::io::stdin().read_to_end(&mut buf).unwrap();
        buf
    });
    Cursor::new(&STDIN_CONTENT)
}
