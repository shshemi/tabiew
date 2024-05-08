use std::error::Error;

use crate::app::{Table, AppResult, StatusBar};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use polars::frame::DataFrame;
use polars_sql::SQLContext;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(
    key_event: KeyEvent,
    tabular: &mut Table,
    status_bar: &mut StatusBar,
    sql_context: &mut SQLContext,
    running: &mut bool,
) -> AppResult<()> {
    match (&status_bar, key_event.code) {
        (_, KeyCode::Esc) => status_bar.normal(),

        (StatusBar::Command(text), KeyCode::Enter) => {
            let command = &text.lines()[0];
            if let Some((s1, s2)) = command.split_once(' ') {
                match (s1, s2) {

                    // Handle SQL queris with prefix of :e
                    (":e", query) => match handle_query(sql_context, query) {
                        Ok(data_frame) => {
                            tabular.set_data_frame(data_frame);
                            status_bar.normal()
                        }
                        Err(err) => status_bar.error(err, 12),
                    },

                    (_, _) => status_bar.error("Invalid command", 8),
                }
            } else {
                status_bar.error("Invalid command", 8)
            }
        }

        (StatusBar::Command(text), KeyCode::Backspace) => {
            if text.lines()[0].len() > 1 {
                status_bar.command().input(key_event);
            } else {
                status_bar.normal()
            }
        }

        (StatusBar::Command(_), _) => {
            status_bar.command().input(key_event);
        }

        (StatusBar::Normal, KeyCode::Char('q')) => *running = false,
        (StatusBar::Normal, KeyCode::Char('v')) => tabular.toggle_detailed_view(),
        (StatusBar::Normal, KeyCode::Char('w')) => tabular.detailed_view_scroll_up(),
        (StatusBar::Normal, KeyCode::Char('s')) => tabular.detailed_view_scroll_down(),
        (StatusBar::Normal, KeyCode::Up) => tabular.select_up(1),
        (StatusBar::Normal, KeyCode::Down) => tabular.select_down(1),
        (StatusBar::Normal, KeyCode::PageUp) => tabular.select_up(tabular.rendered_rows.into()),
        (StatusBar::Normal, KeyCode::PageDown) => tabular.select_down(tabular.rendered_rows.into()),
        (StatusBar::Normal, KeyCode::Home) => tabular.select_first(),
        (StatusBar::Normal, KeyCode::End) => tabular.select_last(),
        (StatusBar::Normal, KeyCode::Char(':')) => {
            status_bar.command().input(key_event);
        }

        _ => {}
    }
    Ok(())
}

fn handle_query(sql_context: &mut SQLContext, query: &str) -> Result<DataFrame, Box<dyn Error>> {
    Ok(sql_context.execute(query)?.collect()?)
}
