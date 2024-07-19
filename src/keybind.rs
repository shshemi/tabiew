use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{AppAction, AppState};

#[derive(Debug, PartialEq, Eq, Hash)]
enum StateKey {
    Exact(AppState, KeyCode, KeyModifiers),
    KeyCode(KeyCode, KeyModifiers),
    State(AppState),
}
pub type Action = AppAction;
pub struct Keybind {
    map: HashMap<StateKey, Action>,
}

impl Default for Keybind {
    fn default() -> Self {
        Self {
            map: [
                // Clear error
                (
                    StateKey::State(AppState::Error),
                    AppAction::StatusBarStats,
                ),
                
                // Table view navigation
                (
                    StateKey::Exact(AppState::Tabular, KeyCode::Char('q'), KeyModifiers::empty()),
                    AppAction::Quit,
                ),
                (
                    StateKey::Exact(AppState::Tabular, KeyCode::Char('v'), KeyModifiers::empty()),
                    AppAction::TabularDetailView,
                ),
                (
                    StateKey::Exact(AppState::Tabular, KeyCode::Up, KeyModifiers::empty()),
                    AppAction::TabularGoUp(1),
                ),
                (
                    StateKey::Exact(AppState::Tabular, KeyCode::Down, KeyModifiers::empty()),
                    AppAction::TabularGoDown(1),
                ),
                (
                    StateKey::Exact(AppState::Tabular, KeyCode::Char('k'), KeyModifiers::empty()),
                    AppAction::TabularGoUp(1),
                ),
                (
                    StateKey::Exact(AppState::Tabular, KeyCode::Char('j'), KeyModifiers::empty()),
                    AppAction::TabularGoDown(1),
                ),
                (
                    StateKey::Exact(AppState::Tabular, KeyCode::PageUp, KeyModifiers::empty()),
                    AppAction::TabularGoUpFullPage,
                ),
                (
                    StateKey::Exact(AppState::Tabular, KeyCode::PageDown, KeyModifiers::empty()),
                    AppAction::TabularGoDownFullPage,
                ),
                (
                    StateKey::Exact(AppState::Tabular, KeyCode::Char('b'), KeyModifiers::CONTROL),
                    AppAction::TabularGoUpFullPage,
                ),
                (
                    StateKey::Exact(AppState::Tabular, KeyCode::Char('f'), KeyModifiers::CONTROL),
                    AppAction::TabularGoDownFullPage,
                ),
                (
                    StateKey::Exact(AppState::Tabular, KeyCode::Char('u'), KeyModifiers::CONTROL),
                    AppAction::TabularGoUpHalfPage,
                ),
                (
                    StateKey::Exact(AppState::Tabular, KeyCode::Char('d'), KeyModifiers::CONTROL),
                    AppAction::TabularGoDownHalfPage,
                ),
                (
                    StateKey::Exact(AppState::Tabular, KeyCode::Char('g'), KeyModifiers::empty()),
                    AppAction::TabularGotoFirst,
                ),
                (
                    StateKey::Exact(AppState::Tabular, KeyCode::Char('G'), KeyModifiers::SHIFT),
                    AppAction::TabularGotoLast,
                ),
                (
                    StateKey::Exact(AppState::Tabular, KeyCode::Home, KeyModifiers::empty()),
                    AppAction::TabularGotoFirst,
                ),
                (
                    StateKey::Exact(AppState::Tabular, KeyCode::End, KeyModifiers::SHIFT),
                    AppAction::TabularGotoLast,
                ),

                // Detail view navigation
                (
                    StateKey::Exact(AppState::Detail, KeyCode::Char('q'), KeyModifiers::empty()),
                    AppAction::TabularTableView,
                ),
                (
                    StateKey::Exact(AppState::Detail, KeyCode::Char('v'), KeyModifiers::empty()),
                    AppAction::TabularTableView,
                ),
                (
                    StateKey::Exact(AppState::Detail, KeyCode::Up, KeyModifiers::empty()),
                    AppAction::DetailScrollUp
                ),
                (
                    StateKey::Exact(AppState::Detail, KeyCode::Down, KeyModifiers::empty()),
                    AppAction::DetailScrollDown
                ),
                (
                    StateKey::Exact(AppState::Detail, KeyCode::Right, KeyModifiers::empty()),
                    AppAction::TabularGoDown(1),
                ),
                (
                    StateKey::Exact(AppState::Detail, KeyCode::Left, KeyModifiers::empty()),
                    AppAction::TabularGoUp(1),
                ),
                (
                    StateKey::Exact(AppState::Detail, KeyCode::Char('h'), KeyModifiers::empty()),
                    AppAction::TabularGoUp(1),
                ),
                (
                    StateKey::Exact(AppState::Detail, KeyCode::Char('l'), KeyModifiers::empty()),
                    AppAction::TabularGoDown(1),
                ),
                (
                    StateKey::Exact(AppState::Detail, KeyCode::Char('k'), KeyModifiers::empty()),
                    AppAction::DetailScrollUp
                ),
                (
                    StateKey::Exact(AppState::Detail, KeyCode::Char('j'), KeyModifiers::empty()),
                    AppAction::DetailScrollDown
                ),
                (
                    StateKey::Exact(AppState::Detail, KeyCode::Char('g'), KeyModifiers::empty()),
                    AppAction::TabularGotoFirst,
                ),
                (
                    StateKey::Exact(AppState::Detail, KeyCode::Char('G'), KeyModifiers::SHIFT),
                    AppAction::TabularGotoLast,
                ),
                (
                    StateKey::Exact(AppState::Detail, KeyCode::Home, KeyModifiers::empty()),
                    AppAction::TabularGotoFirst,
                ),
                (
                    StateKey::Exact(AppState::Detail, KeyCode::End, KeyModifiers::SHIFT),
                    AppAction::TabularGotoLast,
                ),

                // Goto line
                (
                    StateKey::Exact(AppState::Tabular, KeyCode::Char('1'), KeyModifiers::empty()),
                    AppAction::StatusBarCommand("goto 1".to_owned()),
                ),
                (
                    StateKey::Exact(AppState::Tabular, KeyCode::Char('2'), KeyModifiers::empty()),
                    AppAction::StatusBarCommand("goto 2".to_owned()),
                ),
                (
                    StateKey::Exact(AppState::Tabular, KeyCode::Char('3'), KeyModifiers::empty()),
                    AppAction::StatusBarCommand("goto 3".to_owned()),
                ),
                (
                    StateKey::Exact(AppState::Tabular, KeyCode::Char('4'), KeyModifiers::empty()),
                    AppAction::StatusBarCommand("goto 4".to_owned()),
                ),
                (
                    StateKey::Exact(AppState::Tabular, KeyCode::Char('5'), KeyModifiers::empty()),
                    AppAction::StatusBarCommand("goto 5".to_owned()),
                ),
                (
                    StateKey::Exact(AppState::Tabular, KeyCode::Char('6'), KeyModifiers::empty()),
                    AppAction::StatusBarCommand("goto 6".to_owned()),
                ),
                (
                    StateKey::Exact(AppState::Tabular, KeyCode::Char('7'), KeyModifiers::empty()),
                    AppAction::StatusBarCommand("goto 7".to_owned()),
                ),
                (
                    StateKey::Exact(AppState::Tabular, KeyCode::Char('8'), KeyModifiers::empty()),
                    AppAction::StatusBarCommand("goto 8".to_owned()),
                ),
                (
                    StateKey::Exact(AppState::Tabular, KeyCode::Char('9'), KeyModifiers::empty()),
                    AppAction::StatusBarCommand("goto 9".to_owned()),
                ),

                // Select Random
                (
                    StateKey::Exact(AppState::Tabular, KeyCode::Char('R'), KeyModifiers::SHIFT),
                    AppAction::TabularGotoRandom,
                ),
            ]
            .into_iter()
            .collect(),
        }
    }
}

impl Keybind {
    pub fn get_action(&self, state: AppState, key_event: KeyEvent) -> Option<&Action> {
        self.map
            .get(&StateKey::Exact(state, key_event.code, key_event.modifiers))
            .or(self.map.get(&StateKey::KeyCode(key_event.code, key_event.modifiers)))
            .or(self.map.get(&StateKey::State(state)))
    }
}
