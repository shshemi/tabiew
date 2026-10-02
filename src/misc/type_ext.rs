use std::fmt::Display;

use ratatui::layout::Constraint;

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

pub trait ConstraintExt {
    fn value(&self) -> u16;
}

impl ConstraintExt for Constraint {
    fn value(&self) -> u16 {
        match self {
            Constraint::Min(val)
            | Constraint::Max(val)
            | Constraint::Length(val)
            | Constraint::Fill(val) => *val,
            Constraint::Percentage(_) | Constraint::Ratio(_, _) => 0,
        }
    }
}
