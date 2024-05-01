use crate::app::{App, AppResult, AppStatus};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match (&app.status, key_event.code) {
        // Exit application on `ESC` or `q`
        (_, KeyCode::Esc) => {
            app.status.normal()
        }

        (AppStatus::Command(text), KeyCode::Enter) => {
            app.status.error(format!("Unsupported command: '{}'", text.lines()[0]), 10)
        }

        (AppStatus::Command(text), KeyCode::Backspace) => {
            if text.lines()[0].len() > 1 {
                // app.status.command();
            } else {
                app.status.normal()
            }
        }

        (AppStatus::Command(_), _) => {
            app.status.command().input(key_event);
        }

        (AppStatus::Normal, KeyCode::Char('q')) => app.quit(),
        (AppStatus::Normal, KeyCode::Up) => app.select_up(1),
        (AppStatus::Normal, KeyCode::Down) => app.select_down(1),
        (AppStatus::Normal, KeyCode::PageUp) => app.select_up(app.visible_rows.into()),
        (AppStatus::Normal, KeyCode::PageDown) => app.select_down(app.visible_rows.into()),
        (AppStatus::Normal, KeyCode::Home) => app.select_first(),
        (AppStatus::Normal, KeyCode::End) => app.select_last(),
        (AppStatus::Normal, KeyCode::Char(':')) => {
            app.status.command().input(key_event);
        }

        _ => {}
    }
    Ok(())
}
