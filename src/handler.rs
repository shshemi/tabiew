use crate::{
    app::{AppResult, StatusBar, Table},
    command::ExecutionTable,
};
use crossterm::event::{KeyCode, KeyEvent};
use polars_sql::SQLContext;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(
    key_event: KeyEvent,
    tabular: &mut Table,
    status_bar: &mut StatusBar,
    sql_context: &mut SQLContext,
    running: &mut bool,
    exec_tbl: &ExecutionTable,
) -> AppResult<()> {
    match (&status_bar, key_event.code) {
        (_, KeyCode::Esc) => status_bar.normal(),

        (StatusBar::Command(text), KeyCode::Enter) => {
            let command = &text.lines()[0];
            let (s1, s2) = command.split_once(' ').unwrap_or((command.as_str(), ""));
            if let Some(func) = exec_tbl.get(s1) {
                match func(s2, tabular, sql_context, running) {
                    Ok(_) => status_bar.normal(),
                    Err(error) => status_bar.error(error, 10),
                }
            } else {
                status_bar.error("command not found", 8);
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
        (StatusBar::Normal, KeyCode::Char('v')) => tabular.switch_view(),
        (StatusBar::Normal, KeyCode::Up) => {
            if let Some(scroll) = &mut tabular.detailed_view {
                scroll.up();
            } else {
                tabular.select_up(1);
            }
        }
        (StatusBar::Normal, KeyCode::Down) => {
            if let Some(scroll) = &mut tabular.detailed_view {
                scroll.down();
            } else {
                tabular.select_down(1);
            }
        }
        (StatusBar::Normal, KeyCode::Left) => {
            if tabular.detailed_view.is_some() {
                tabular.select_up(1)
            }
        }
        (StatusBar::Normal, KeyCode::Right) => {
            if tabular.detailed_view.is_some() {
                tabular.select_down(1)
            }
        }
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
