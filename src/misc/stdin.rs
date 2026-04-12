use std::{
    io::{Cursor, Read},
    sync::LazyLock,
};

use super::type_ext::UnwrapOrGracefulShutdown;

pub fn stdin() -> Cursor<&'static [u8]> {
    static STDIN_CONTENT: LazyLock<Vec<u8>> = LazyLock::new(|| {
        let mut buf = Vec::new();
        std::io::stdin()
            .read_to_end(&mut buf)
            .unwrap_or_graceful_shutdown();
        buf
    });
    Cursor::new(&STDIN_CONTENT)
}
