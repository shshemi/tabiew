use crate::{
    app::{AppResult, StatusBar, StatusBarState, Table},
    command::ExecutionTable,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
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
    match (&status_bar.state, key_event.code) {
        (_, KeyCode::Esc) => status_bar.normal(),

        (StatusBarState::Command(_), KeyCode::Enter) => {
            if let Some(command) = status_bar.commit_prompt() {
                let (s1, s2) = command.split_once(' ').unwrap_or((command.as_str(), ""));
                if let Some(func) = exec_tbl.get(s1) {
                    match func(s2, tabular, sql_context, running) {
                        Ok(_) => status_bar.normal(),
                        Err(error) => status_bar.error(error),
                    }
                } else {
                    status_bar.error("command not found");
                }
            } else {
                status_bar.error("invalid state");
            }
        }

        (StatusBarState::Command(_), _) => {
            // status_bar.command().input(key_event);
            status_bar.input(key_event)
        }

        (StatusBarState::Normal, KeyCode::Char('q')) => *running = false,
        (StatusBarState::Normal, KeyCode::Char('v')) => tabular.switch_view(),
        (StatusBarState::Normal, KeyCode::Up | KeyCode::Char('k')) => {
            if let Some(scroll) = &mut tabular.detailed_view {
                scroll.up();
            } else {
                tabular.select_up(1);
            }
        }
        (StatusBarState::Normal, KeyCode::Down | KeyCode::Char('j')) => {
            if let Some(scroll) = &mut tabular.detailed_view {
                scroll.down();
            } else {
                tabular.select_down(1);
            }
        }
        (StatusBarState::Normal, KeyCode::Left | KeyCode::Char('h')) => {
            if tabular.detailed_view.is_some() {
                tabular.select_up(1)
            }
        }
        (StatusBarState::Normal, KeyCode::Right | KeyCode::Char('l')) => {
            if tabular.detailed_view.is_some() {
                tabular.select_down(1)
            }
        }
        (StatusBarState::Normal, KeyCode::PageUp) => {
            tabular.select_up(tabular.rendered_rows.into())
        }
        (StatusBarState::Normal, KeyCode::PageDown) => {
            tabular.select_down(tabular.rendered_rows.into())
        }
        (StatusBarState::Normal, KeyCode::Char('b'))
            if key_event.modifiers == KeyModifiers::CONTROL =>
        {
            tabular.select_up(tabular.rendered_rows.into())
        }
        (StatusBarState::Normal, KeyCode::Char('f'))
            if key_event.modifiers == KeyModifiers::CONTROL =>
        {
            tabular.select_down(tabular.rendered_rows.into())
        }
        (StatusBarState::Normal, KeyCode::Home | KeyCode::Char('g')) => tabular.select_first(),
        (StatusBarState::Normal, KeyCode::End | KeyCode::Char('G')) => tabular.select_last(),
        (StatusBarState::Normal, KeyCode::Char(':')) => {
            status_bar.command(":");
        }
        (
            StatusBarState::Normal,
            KeyCode::Char('1')
            | KeyCode::Char('2')
            | KeyCode::Char('3')
            | KeyCode::Char('4')
            | KeyCode::Char('5')
            | KeyCode::Char('6')
            | KeyCode::Char('7')
            | KeyCode::Char('8')
            | KeyCode::Char('9')
        ) => {
            status_bar.command(":goto ");
            status_bar.input(key_event);
        }
        (StatusBarState::Normal, KeyCode::Char('u'))
            if key_event.modifiers == KeyModifiers::CONTROL =>
        {
            tabular.select_up((tabular.rendered_rows / 2).into())
        }
        (StatusBarState::Normal, KeyCode::Char('d'))
            if key_event.modifiers == KeyModifiers::CONTROL =>
        {
            tabular.select_down((tabular.rendered_rows / 2).into())
        }
        (StatusBarState::Error(_), _) => {
            status_bar.normal();
        }

        _ => {}
    }
    Ok(())
}
