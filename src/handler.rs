use crate::app::{App, AppResult, AppStatus};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match (&app.status, key_event.code) {
        // Exit application on `ESC` or `q`
        (_, KeyCode::Esc) => {
            app.state_normal();
        }

        (AppStatus::Normal, KeyCode::Char('q')) => app.quit(),
        (AppStatus::Normal, KeyCode::Up) => app.select_up(1),
        (AppStatus::Normal, KeyCode::Down) => app.select_down(1),
        (AppStatus::Normal, KeyCode::PageUp) => app.select_up(app.visible_rows.into()),
        (AppStatus::Normal, KeyCode::PageDown) => app.select_down(app.visible_rows.into()),
        (AppStatus::Normal, KeyCode::Char(':')) => {
            app.state_error("Commands are not supported yet!".to_owned(), 8)
        }

        _ => {}
    }
    Ok(())
}
