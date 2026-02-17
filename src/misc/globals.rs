use std::{
    io::{Cursor, Read},
    sync::LazyLock,
};

pub fn stdin() -> Cursor<&'static Vec<u8>> {
    static STDIN_CONTENT: LazyLock<Vec<u8>> = LazyLock::new(|| {
        let mut buf = Vec::new();
        std::io::stdin().read_to_end(&mut buf).unwrap();
        buf
    });
    Cursor::new(&STDIN_CONTENT)
}
