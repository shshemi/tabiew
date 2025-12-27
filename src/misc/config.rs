use std::{
    fs,
    ops::Deref,
    sync::{
        OnceLock, RwLock,
        atomic::{AtomicBool, Ordering},
    },
};

use serde::{Deserialize, Serialize};

use crate::{AppResult, misc::paths::config_path, tui::themes::theme::LoadedTheme};

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    theme: RwLock<LoadedTheme>,
    show_table_borders: AtomicBool,
    show_table_row_numbers: AtomicBool,
}

impl Config {
    pub fn reload(&self) -> AppResult<()> {
        let path = config_path()?;
        let contents = fs::read_to_string(path)?;
        let Config {
            theme,
            show_table_borders: table_borders,
            show_table_row_numbers: table_row_numbers,
        } = toml::from_str(&contents)?;
        self.set_theme(theme.into_inner()?);
        self.show_table_borders
            .swap(table_borders.into_inner(), Ordering::Relaxed);
        self.show_table_row_numbers
            .swap(table_row_numbers.into_inner(), Ordering::Relaxed);
        Ok(())
    }

    pub fn store(&self) -> AppResult<()> {
        let config_path = config_path()?;
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let contents = toml::to_string_pretty(self)?;
        Ok(fs::write(config_path, contents)?)
    }

    pub fn theme(&self) -> impl Deref<Target = LoadedTheme> {
        self.theme.read().unwrap()
    }

    pub fn set_theme(&self, theme: impl Into<LoadedTheme>) {
        *self.theme.write().unwrap() = theme.into();
    }

    pub fn show_table_borders(&self) -> bool {
        self.show_table_borders.load(Ordering::Relaxed)
    }

    pub fn toggle_show_table_borders(&self) {
        self.show_table_borders.fetch_xor(true, Ordering::Relaxed);
    }

    pub fn show_table_row_numbers(&self) -> bool {
        self.show_table_row_numbers.load(Ordering::Relaxed)
    }

    pub fn toggle_show_table_row_numbers(&self) {
        self.show_table_row_numbers
            .fetch_xor(true, Ordering::Relaxed);
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: RwLock::new(LoadedTheme::default()),
            show_table_borders: AtomicBool::new(true),
            show_table_row_numbers: AtomicBool::new(true),
        }
    }
}

pub fn config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(Config::default)
}

pub fn theme() -> impl Deref<Target = LoadedTheme> {
    config().theme()
}
