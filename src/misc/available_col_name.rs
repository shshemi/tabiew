use anyhow::anyhow;

use crate::{iters::enumerate_names, misc::unwrap_or_graceful_shutdown::UnwrapOrGracefulShutdown};

pub fn available_col_name(mut suggestion: &str, predicate: impl Fn(&str) -> bool) -> String {
    if suggestion.is_empty() {
        suggestion = "column";
    }
    enumerate_names(suggestion)
        .find(|name| predicate(name))
        .ok_or(anyhow!("Suitable column name not found"))
        .unwrap_or_graceful_shutdown()
}
