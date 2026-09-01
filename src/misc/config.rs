use std::{
    fs,
    ops::Deref,
    sync::{
        OnceLock, RwLock,
        atomic::{AtomicBool, AtomicI8, Ordering},
    },
};

use serde::{Deserialize, Serialize};

use crate::{
    AppResult,
    misc::{http::HttpConfig, paths::config_path},
    tui::themes::theme::LoadedTheme,
};

use super::type_ext::UnwrapOrGracefulShutdown;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    theme: RwLock<LoadedTheme>,
    http: RwLock<HttpConfig>,
    show_table_borders: AtomicBool,
    show_table_row_numbers: AtomicBool,
    fp_precision: AtomicI8,
}

impl Config {
    pub fn reload(&self) -> AppResult<()> {
        let path = config_path()?;
        let contents = fs::read_to_string(path)?;
        let Config {
            theme,
            http,
            show_table_borders: table_borders,
            show_table_row_numbers: table_row_numbers,
            fp_precision,
        } = toml::from_str(&contents)?;
        self.set_theme(theme.into_inner()?);
        self.set_http_config(http.into_inner()?);
        self.show_table_borders
            .swap(table_borders.into_inner(), Ordering::Relaxed);
        self.show_table_row_numbers
            .swap(table_row_numbers.into_inner(), Ordering::Relaxed);
        self.fp_precision
            .swap(fp_precision.into_inner(), Ordering::Relaxed);
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
        self.theme.read().unwrap_or_graceful_shutdown()
    }

    pub fn set_theme(&self, theme: impl Into<LoadedTheme>) {
        *self.theme.write().unwrap_or_graceful_shutdown() = theme.into();
    }

    pub fn http_config(&self) -> impl Deref<Target = HttpConfig> {
        self.http.read().unwrap_or_graceful_shutdown()
    }

    pub fn set_http_config(&self, http_config: impl Into<HttpConfig>) {
        *self.http.write().unwrap_or_graceful_shutdown() = http_config.into();
    }

    pub fn show_table_borders(&self) -> bool {
        self.show_table_borders.load(Ordering::Relaxed)
    }

    pub fn toggle_show_table_borders(&self) {
        self.show_table_borders.fetch_xor(true, Ordering::Relaxed);
    }

    pub fn fp_precision(&self) -> Option<usize> {
        match self.fp_precision.load(Ordering::Relaxed) {
            precision if precision < 0 => None,
            precision => Some(precision as usize),
        }
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
            http: RwLock::new(HttpConfig::default()),
            fp_precision: AtomicI8::new(-1),
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
