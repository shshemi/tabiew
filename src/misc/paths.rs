use std::path::PathBuf;

pub fn theme_path() -> Option<PathBuf> {
    home::home_dir().map(|path| path.join(".config").join("tabiew").join("theme.toml"))
}

pub fn config_gath() -> Option<PathBuf> {
    home::home_dir().map(|path| path.join(".config").join("tabiew").join("config.toml"))
}
