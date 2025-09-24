use std::{
    ops::{Deref, DerefMut},
    sync::RwLock,
};

use anyhow::Ok;
use serde::{Deserialize, Serialize};

use crate::{AppResult, tui::themes::theme::Theme};

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    theme: RwLock<Theme>,
}

impl Config {
    pub const fn new() -> Self {
        Self {
            theme: RwLock::new(Theme::default()),
        }
    }

    pub fn load(&self, text: &str) -> AppResult<()> {
        let Config { theme } = toml::from_str(text)?;
        *self.theme_mut() = theme.into_inner()?;
        Ok(())
    }

    pub fn store(&self) -> AppResult<String> {
        Ok(toml::to_string(self)?)
    }

    pub fn theme(&self) -> impl Deref<Target = Theme> {
        self.theme.read().unwrap()
    }

    pub fn theme_mut(&self) -> impl DerefMut<Target = Theme> {
        self.theme.write().unwrap()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
