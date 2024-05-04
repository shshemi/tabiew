use std::error::Error;

use crate::app::{App, AppResult, AppStatus};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use polars::frame::DataFrame;
use polars_sql::SQLContext;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(
    key_event: KeyEvent,
    app: &mut App,
    sql_context: &mut SQLContext,
) -> AppResult<()> {
    match (&app.status, key_event.code) {
        (_, KeyCode::Esc) => app.status.normal(),

        (AppStatus::Command(text), KeyCode::Enter) => {
            let command = &text.lines()[0];
            if let Some((s1, s2)) = command.split_once(' ') {
                match (s1, s2) {
                    // Handle SQL queris with prefix of :q
                    (":q", query) => match handle_query(sql_context, query) {
                        Ok(data_frame) => {
                            app.set_data_frame(data_frame);
                            app.status.normal()
                        }
                        Err(err) => app.status.error(err, 12),
                    },

                    (_, _) => app.status.error("Invalid command", 8),
                }
            } else {
                app.status.error("Invalid command", 8)
            }
        }

        (AppStatus::Command(text), KeyCode::Backspace) => {
            if text.lines()[0].len() > 1 {
                app.status.command().input(key_event);
            } else {
                app.status.normal()
            }
        }

        (AppStatus::Command(_), _) => {
            app.status.command().input(key_event);
        }

        (AppStatus::Normal, KeyCode::Char('q')) => app.quit(),
        (AppStatus::Normal, KeyCode::Char('v')) => app.toggle_detailed_view(),
        (AppStatus::Normal, KeyCode::Char('w')) => app.detailed_view_scroll_up(),
        (AppStatus::Normal, KeyCode::Char('s')) => app.detailed_view_scroll_down(),
        (AppStatus::Normal, KeyCode::Up) => app.select_up(1),
        (AppStatus::Normal, KeyCode::Down) => app.select_down(1),
        (AppStatus::Normal, KeyCode::PageUp) => app.select_up(app.rendered_rows.into()),
        (AppStatus::Normal, KeyCode::PageDown) => app.select_down(app.rendered_rows.into()),
        (AppStatus::Normal, KeyCode::Home) => app.select_first(),
        (AppStatus::Normal, KeyCode::End) => app.select_last(),
        (AppStatus::Normal, KeyCode::Char(':')) => {
            app.status.command().input(key_event);
        }

        _ => {}
    }
    Ok(())
}

fn handle_query(sql_context: &mut SQLContext, query: &str) -> Result<DataFrame, Box<dyn Error>> {
    Ok(sql_context.execute(query)?.collect()?)
}
