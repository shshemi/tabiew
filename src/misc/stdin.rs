use std::{
    io::{Cursor, Read, Stdin, StdinLock},
    sync::LazyLock,
};

use super::type_ext::UnwrapOrGracefulShutdown;

pub fn stdin() -> Cursor<&'static Vec<u8>> {
    static STDIN_CONTENT: LazyLock<Vec<u8>> = LazyLock::new(|| {
        let mut buf = Vec::new();
        std::io::stdin()
            .read_to_end(&mut buf)
            .unwrap_or_graceful_shutdown();
        buf
    });
    Cursor::new(&STDIN_CONTENT)
}

/// Acquire a blocking, locked handle on the real stdin stream. Unlike
/// [`stdin`], this does NOT go through the eager `LazyLock<Vec<u8>>` cache —
/// each call returns a fresh lock on the process's standard input, so the
/// streaming readers can read line-by-line without materializing the whole
/// input up front.
pub fn stdin_raw_locked() -> StdinLock<'static> {
    static STDIN: LazyLock<Stdin> = LazyLock::new(std::io::stdin);
    STDIN.lock()
}
