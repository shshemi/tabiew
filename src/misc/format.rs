use crate::args::Format;
use clap::ValueEnum;
use std::path::Path;

pub fn resolve_format(format: Option<&Format>, path: &Path) -> Option<Format> {
    format.cloned().or_else(|| {
        path.extension()
            .and_then(|ext| Format::from_str(&ext.to_string_lossy(), true).ok())
    })
}
