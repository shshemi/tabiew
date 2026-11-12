use std::{env, path::Path, process::Command};

use anyhow::anyhow;

use crate::{
    AppResult,
    handler::event::{disable_event_read, enable_event_read},
    tui::terminal::{invalidate_tui, start_tui, stop_tui},
};

pub fn run_editor(path: &Path) -> AppResult<()> {
    let editor = env::var("EDITOR").map_err(|_| anyhow!("$EDITOR is not set"))?;
    disable_event_read();
    stop_tui()?;
    let status = Command::new(editor).arg(path).status();
    start_tui()?;
    invalidate_tui();
    enable_event_read();
    match status?.success() {
        true => Ok(()),
        false => Err(anyhow!("Editor failed")),
    }
}
