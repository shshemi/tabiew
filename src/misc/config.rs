use serde::Serialize;

use crate::tui::{Styler, theme::Monokai};

pub struct Config {
    theme: Box<dyn Styler>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: Box::new(Monokai::default()),
        }
    }
}

impl Serialize for Config {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}
