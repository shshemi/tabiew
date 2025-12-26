use std::fmt::Display;

use ratatui::layout::Constraint;

use crate::{AppResult, handler::message::Message};

pub trait ToAscii {
    fn to_ascii(self) -> Option<u8>;
}

impl ToAscii for char {
    #[inline]
    fn to_ascii(self) -> Option<u8> {
        self.is_ascii().then_some(self as u8)
    }
}

pub trait HasSubsequence {
    fn has_subsequence(&self, other: &Self) -> bool;
}

impl HasSubsequence for str {
    fn has_subsequence(&self, other: &Self) -> bool {
        let mut oitr = other.chars();
        let mut current = oitr.next();
        for chr in self.chars() {
            if let Some(cur) = current {
                if chr == cur {
                    current = oitr.next();
                }
            } else {
                break;
            }
        }
        current.is_none()
    }
}

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
                eprintln!("Error: {err}");
                std::process::exit(1);
            }
        }
    }
}

pub trait UnwrapOrEnqueueError {
    fn unwrap_or_enqueue_error(&self);
}

impl UnwrapOrEnqueueError for AppResult<()> {
    fn unwrap_or_enqueue_error(&self) {
        match self {
            Ok(_) => (),
            Err(err) => Message::AppShowError(err.to_string()).enqueue(),
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

pub fn human_readable_size(volume: u64) -> String {
    if volume < 1024 {
        format!("{volume} B")
    } else if volume < 1024 * 1024 {
        format!("{:.2} KB", volume as f64 / 1024.0)
    } else if volume < 1024 * 1024 * 1024 {
        format!("{:.2} MB", volume as f64 / (1024.0 * 1024.0))
    } else if volume < 1024 * 1024 * 1024 * 1024 {
        format!("{:.2} GB", volume as f64 / (1024.0 * 1024.0 * 1024.0))
    } else if volume < 1024 * 1024 * 1024 * 1024 * 1024 {
        format!(
            "{:.2} TB",
            volume as f64 / (1024.0 * 1024.0 * 1024.0 * 1024.0)
        )
    } else {
        format!(
            "{:.2} PB",
            volume as f64 / (1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0)
        )
    }
}
