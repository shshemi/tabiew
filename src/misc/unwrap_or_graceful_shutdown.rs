use std::fmt::Display;

use crate::tui::terminal::forece_stop_tui;

pub trait UnwrapOrGracefulShutdown<T> {
    fn unwrap_or_graceful_shutdown(self) -> T;
}

impl<T, E> UnwrapOrGracefulShutdown<T> for Result<T, E>
where
    E: Display,
{
    fn unwrap_or_graceful_shutdown(self) -> T {
        match self {
            Ok(val) => val,
            Err(err) => {
                forece_stop_tui();
                eprintln!("Error: {err}");
                std::process::exit(1);
            }
        }
    }
}
