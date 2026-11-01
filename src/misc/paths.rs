use std::path::PathBuf;

use anyhow::anyhow;

use crate::AppResult;

pub fn theme_path() -> AppResult<PathBuf> {
    std::env::home_dir()
        .map(|path| path.join(".config").join("tabiew").join("theme.toml"))
        .ok_or(anyhow!("Home dir not found"))
}

pub fn config_path() -> AppResult<PathBuf> {
    std::env::home_dir()
        .map(|path| path.join(".config").join("tabiew").join("config.toml"))
        .ok_or(anyhow!("Home dir not found"))
}

pub fn cache_path() -> AppResult<PathBuf> {
    let path = if cfg!(target_os = "windows") {
        let local_path =
            std::env::var("LOCALAPPDATA").map_err(|_| anyhow!("LOCALAPPDATA env not found"))?;
        PathBuf::from(local_path).join("tabiew")
    } else {
        let home_path = std::env::home_dir().ok_or_else(|| anyhow!("Home dir not found"))?;
        home_path.join(".cache").join("tabiew")
    };
    Ok(path)
}
