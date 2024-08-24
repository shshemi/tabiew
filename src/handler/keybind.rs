use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{app::AppAction, app::AppState};

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
                (StateKey::State(AppState::Error), AppAction::StatusBarStats),
                // Close app/tab/sheet
                (
                    StateKey::Exact(AppState::Empty, KeyCode::Char('q'), KeyModifiers::empty()),
                    AppAction::Quit,
                ),
                (
                    StateKey::Exact(AppState::Sheet, KeyCode::Char('q'), KeyModifiers::empty()),
                    AppAction::TabularTableView,
                ),
                (
                    StateKey::Exact(AppState::Table, KeyCode::Char('q'), KeyModifiers::empty()),
                    AppAction::TabRemoveOrQuit,
                ),
                // Switch tab/sheet
                (
                    StateKey::Exact(AppState::Table, KeyCode::Char('v'), KeyModifiers::empty()),
                    AppAction::TabularSheetView,
                ),
                (
                    StateKey::Exact(AppState::Sheet, KeyCode::Char('v'), KeyModifiers::empty()),
                    AppAction::TabularTableView,
                ),
                // Move half page
                (
                    StateKey::Exact(AppState::Table, KeyCode::Char('u'), KeyModifiers::CONTROL),
                    AppAction::TabularGoUpHalfPage,
                ),
                (
                    StateKey::Exact(AppState::Table, KeyCode::Char('d'), KeyModifiers::CONTROL),
                    AppAction::TabularGoDownHalfPage,
                ),
                // Move full page
                (
                    StateKey::Exact(AppState::Table, KeyCode::PageUp, KeyModifiers::empty()),
                    AppAction::TabularGoUpFullPage,
                ),
                (
                    StateKey::Exact(AppState::Table, KeyCode::PageDown, KeyModifiers::empty()),
                    AppAction::TabularGoDownFullPage,
                ),
                (
                    StateKey::Exact(AppState::Table, KeyCode::Char('b'), KeyModifiers::CONTROL),
                    AppAction::TabularGoUpFullPage,
                ),
                (
                    StateKey::Exact(AppState::Table, KeyCode::Char('f'), KeyModifiers::CONTROL),
                    AppAction::TabularGoDownFullPage,
                ),
                // Move to prev/next record
                (
                    StateKey::Exact(AppState::Table, KeyCode::Up, KeyModifiers::empty()),
                    AppAction::TabularGoUp(1),
                ),
                (
                    StateKey::Exact(AppState::Table, KeyCode::Down, KeyModifiers::empty()),
                    AppAction::TabularGoDown(1),
                ),
                (
                    StateKey::Exact(AppState::Table, KeyCode::Char('k'), KeyModifiers::empty()),
                    AppAction::TabularGoUp(1),
                ),
                (
                    StateKey::Exact(AppState::Table, KeyCode::Char('j'), KeyModifiers::empty()),
                    AppAction::TabularGoDown(1),
                ),
                (
                    StateKey::Exact(AppState::Sheet, KeyCode::Right, KeyModifiers::empty()),
                    AppAction::TabularGoDown(1),
                ),
                (
                    StateKey::Exact(AppState::Sheet, KeyCode::Left, KeyModifiers::empty()),
                    AppAction::TabularGoUp(1),
                ),
                (
                    StateKey::Exact(AppState::Sheet, KeyCode::Char('h'), KeyModifiers::empty()),
                    AppAction::TabularGoUp(1),
                ),
                (
                    StateKey::Exact(AppState::Sheet, KeyCode::Char('l'), KeyModifiers::empty()),
                    AppAction::TabularGoDown(1),
                ),
                // Move to first/last record
                (
                    StateKey::Exact(AppState::Sheet, KeyCode::Home, KeyModifiers::empty()),
                    AppAction::TabularGotoFirst,
                ),
                (
                    StateKey::Exact(AppState::Sheet, KeyCode::End, KeyModifiers::empty()),
                    AppAction::TabularGotoLast,
                ),
                (
                    StateKey::Exact(AppState::Table, KeyCode::Home, KeyModifiers::empty()),
                    AppAction::TabularGotoFirst,
                ),
                (
                    StateKey::Exact(AppState::Table, KeyCode::End, KeyModifiers::empty()),
                    AppAction::TabularGotoLast,
                ),
                (
                    StateKey::Exact(AppState::Sheet, KeyCode::Char('g'), KeyModifiers::empty()),
                    AppAction::TabularGotoFirst,
                ),
                (
                    StateKey::Exact(AppState::Sheet, KeyCode::Char('G'), KeyModifiers::SHIFT),
                    AppAction::TabularGotoLast,
                ),
                (
                    StateKey::Exact(AppState::Table, KeyCode::Char('g'), KeyModifiers::empty()),
                    AppAction::TabularGotoFirst,
                ),
                (
                    StateKey::Exact(AppState::Table, KeyCode::Char('G'), KeyModifiers::SHIFT),
                    AppAction::TabularGotoLast,
                ),
                // Scroll up/down in sheets
                (
                    StateKey::Exact(AppState::Sheet, KeyCode::Up, KeyModifiers::empty()),
                    AppAction::SheetScrollUp,
                ),
                (
                    StateKey::Exact(AppState::Sheet, KeyCode::Down, KeyModifiers::empty()),
                    AppAction::SheetScrollDown,
                ),
                (
                    StateKey::Exact(AppState::Sheet, KeyCode::Char('k'), KeyModifiers::empty()),
                    AppAction::SheetScrollUp,
                ),
                (
                    StateKey::Exact(AppState::Sheet, KeyCode::Char('j'), KeyModifiers::empty()),
                    AppAction::SheetScrollDown,
                ),
                // Move prev/next tab
                (
                    StateKey::Exact(AppState::Table, KeyCode::Char('H'), KeyModifiers::SHIFT),
                    AppAction::TabSelectedPrev,
                ),
                (
                    StateKey::Exact(AppState::Table, KeyCode::Char('L'), KeyModifiers::SHIFT),
                    AppAction::TabSelectedNext,
                ),
                (
                    StateKey::Exact(AppState::Sheet, KeyCode::Char('H'), KeyModifiers::SHIFT),
                    AppAction::TabSelectedPrev,
                ),
                (
                    StateKey::Exact(AppState::Sheet, KeyCode::Char('L'), KeyModifiers::SHIFT),
                    AppAction::TabSelectedNext,
                ),
                // Move to line by number
                (
                    StateKey::Exact(AppState::Table, KeyCode::Char('1'), KeyModifiers::empty()),
                    AppAction::StatusBarCommand("goto 1".to_owned()),
                ),
                (
                    StateKey::Exact(AppState::Table, KeyCode::Char('2'), KeyModifiers::empty()),
                    AppAction::StatusBarCommand("goto 2".to_owned()),
                ),
                (
                    StateKey::Exact(AppState::Table, KeyCode::Char('3'), KeyModifiers::empty()),
                    AppAction::StatusBarCommand("goto 3".to_owned()),
                ),
                (
                    StateKey::Exact(AppState::Table, KeyCode::Char('4'), KeyModifiers::empty()),
                    AppAction::StatusBarCommand("goto 4".to_owned()),
                ),
                (
                    StateKey::Exact(AppState::Table, KeyCode::Char('5'), KeyModifiers::empty()),
                    AppAction::StatusBarCommand("goto 5".to_owned()),
                ),
                (
                    StateKey::Exact(AppState::Table, KeyCode::Char('6'), KeyModifiers::empty()),
                    AppAction::StatusBarCommand("goto 6".to_owned()),
                ),
                (
                    StateKey::Exact(AppState::Table, KeyCode::Char('7'), KeyModifiers::empty()),
                    AppAction::StatusBarCommand("goto 7".to_owned()),
                ),
                (
                    StateKey::Exact(AppState::Table, KeyCode::Char('8'), KeyModifiers::empty()),
                    AppAction::StatusBarCommand("goto 8".to_owned()),
                ),
                (
                    StateKey::Exact(AppState::Table, KeyCode::Char('9'), KeyModifiers::empty()),
                    AppAction::StatusBarCommand("goto 9".to_owned()),
                ),
                // Select Random
                (
                    StateKey::Exact(AppState::Table, KeyCode::Char('R'), KeyModifiers::SHIFT),
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
            .or(self
                .map
                .get(&StateKey::KeyCode(key_event.code, key_event.modifiers)))
            .or(self.map.get(&StateKey::State(state)))
    }
}
