use std::path::PathBuf;

use anyhow::anyhow;

use crate::AppResult;

pub fn theme_path() -> AppResult<PathBuf> {
    home::home_dir()
        .map(|path| path.join(".config").join("tabiew").join("theme.toml"))
        .ok_or(anyhow!("Home dir not found"))
}

pub fn config_path() -> AppResult<PathBuf> {
    home::home_dir()
        .map(|path| path.join(".config").join("tabiew").join("config.toml"))
        .ok_or(anyhow!("Home dir not found"))
}
