use std::{
    fs,
    ops::{Deref, DerefMut},
    sync::RwLock,
};

use serde::{Deserialize, Serialize};

use crate::{
    AppResult,
    misc::{globals::config, paths::config_path},
    tui::themes::theme::Theme,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    theme: RwLock<Theme>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            theme: RwLock::new(Theme::default()),
        }
    }

    pub fn load(&self, text: &str) -> AppResult<()> {
        let Config { theme } = toml::from_str(text)?;
        *self.theme_mut() = theme.into_inner()?;
        Ok(())
    }

    pub fn dump(&self) -> AppResult<String> {
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

pub fn store_config() {
    if let Some(config_path) = config_path()
        && let Some(parent) = config_path.parent()
        && let Ok(_) = fs::create_dir_all(parent)
        && let Ok(contents) = config().dump()
    {
        let _ = fs::write(config_path, contents);
    }
}
