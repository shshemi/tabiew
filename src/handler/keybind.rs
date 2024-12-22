use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{app::AppAction, app::AppState};

#[derive(Debug, PartialEq, Eq, Hash)]
enum Keybind {
    Exact(AppState, KeyCode, KeyModifiers),
    KeyOnly(KeyCode, KeyModifiers),
    StateOnly(AppState),
}
enum Action {
    Direct(AppAction),
    Closure(Box<dyn Fn(AppState, KeyEvent) -> AppAction>),
}

impl From<AppAction> for Action {
    fn from(value: AppAction) -> Self {
        Action::Direct(value)
    }
}

impl<F: Fn(AppState, KeyEvent) -> AppAction + 'static> From<F> for Action {
    fn from(value: F) -> Self {
        Action::Closure(Box::new(value))
    }
}
#[derive(Default)]
pub struct KeyMap {
    map: HashMap<Keybind, Action>,
}

pub fn default_keymap() -> KeyMap {
    let mut key_map = KeyMap::default();
    // Clear error
    key_map.add(
        Keybind::StateOnly(AppState::Error),
        AppAction::StatusBarStats,
    );
    // Close app/tab/sheet
    key_map.add(
        Keybind::Exact(AppState::Empty, KeyCode::Char('q'), KeyModifiers::empty()),
        AppAction::Quit,
    );
    key_map.add(
        Keybind::Exact(AppState::Sheet, KeyCode::Char('q'), KeyModifiers::empty()),
        AppAction::TabularTableView,
    );
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::Char('q'), KeyModifiers::empty()),
        AppAction::TabRemoveOrQuit,
    );
    // Switch tab/sheet/enter
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::Enter, KeyModifiers::empty()),
        AppAction::TabularEnterPress,
    );
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::Char('v'), KeyModifiers::empty()),
        AppAction::TabularSheetView,
    );
    key_map.add(
        Keybind::Exact(AppState::Sheet, KeyCode::Char('v'), KeyModifiers::empty()),
        AppAction::TabularTableView,
    );
    // Move half page
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::Char('u'), KeyModifiers::CONTROL),
        AppAction::TabularGoUpHalfPage,
    );
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::Char('d'), KeyModifiers::CONTROL),
        AppAction::TabularGoDownHalfPage,
    );
    // Move full page
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::PageUp, KeyModifiers::empty()),
        AppAction::TabularGoUpFullPage,
    );
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::PageDown, KeyModifiers::empty()),
        AppAction::TabularGoDownFullPage,
    );
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::Char('b'), KeyModifiers::CONTROL),
        AppAction::TabularGoUpFullPage,
    );
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::Char('f'), KeyModifiers::CONTROL),
        AppAction::TabularGoDownFullPage,
    );
    // Move to prev/next record
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::Up, KeyModifiers::empty()),
        AppAction::TabularGoUp(1),
    );
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::Down, KeyModifiers::empty()),
        AppAction::TabularGoDown(1),
    );
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::Char('k'), KeyModifiers::empty()),
        AppAction::TabularGoUp(1),
    );
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::Char('j'), KeyModifiers::empty()),
        AppAction::TabularGoDown(1),
    );
    key_map.add(
        Keybind::Exact(AppState::Sheet, KeyCode::Right, KeyModifiers::empty()),
        AppAction::TabularGoDown(1),
    );
    key_map.add(
        Keybind::Exact(AppState::Sheet, KeyCode::Left, KeyModifiers::empty()),
        AppAction::TabularGoUp(1),
    );
    key_map.add(
        Keybind::Exact(AppState::Sheet, KeyCode::Char('h'), KeyModifiers::empty()),
        AppAction::TabularGoUp(1),
    );
    key_map.add(
        Keybind::Exact(AppState::Sheet, KeyCode::Char('l'), KeyModifiers::empty()),
        AppAction::TabularGoDown(1),
    );
    // Move to first/last record
    key_map.add(
        Keybind::Exact(AppState::Sheet, KeyCode::Home, KeyModifiers::empty()),
        AppAction::TabularGotoFirst,
    );
    key_map.add(
        Keybind::Exact(AppState::Sheet, KeyCode::End, KeyModifiers::empty()),
        AppAction::TabularGotoLast,
    );
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::Home, KeyModifiers::empty()),
        AppAction::TabularGotoFirst,
    );
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::End, KeyModifiers::empty()),
        AppAction::TabularGotoLast,
    );
    key_map.add(
        Keybind::Exact(AppState::Sheet, KeyCode::Char('g'), KeyModifiers::empty()),
        AppAction::TabularGotoFirst,
    );
    key_map.add(
        Keybind::Exact(AppState::Sheet, KeyCode::Char('G'), KeyModifiers::SHIFT),
        AppAction::TabularGotoLast,
    );
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::Char('g'), KeyModifiers::empty()),
        AppAction::TabularGotoFirst,
    );
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::Char('G'), KeyModifiers::SHIFT),
        AppAction::TabularGotoLast,
    );
    // Scroll up/down in sheets
    key_map.add(
        Keybind::Exact(AppState::Sheet, KeyCode::Up, KeyModifiers::empty()),
        AppAction::SheetScrollUp,
    );
    key_map.add(
        Keybind::Exact(AppState::Sheet, KeyCode::Down, KeyModifiers::empty()),
        AppAction::SheetScrollDown,
    );
    key_map.add(
        Keybind::Exact(AppState::Sheet, KeyCode::Char('k'), KeyModifiers::empty()),
        AppAction::SheetScrollUp,
    );
    key_map.add(
        Keybind::Exact(AppState::Sheet, KeyCode::Char('j'), KeyModifiers::empty()),
        AppAction::SheetScrollDown,
    );
    // Move prev/next tab
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::Char('H'), KeyModifiers::SHIFT),
        AppAction::TabSelectedPrev,
    );
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::Char('L'), KeyModifiers::SHIFT),
        AppAction::TabSelectedNext,
    );
    key_map.add(
        Keybind::Exact(AppState::Sheet, KeyCode::Char('H'), KeyModifiers::SHIFT),
        AppAction::TabSelectedPrev,
    );
    key_map.add(
        Keybind::Exact(AppState::Sheet, KeyCode::Char('L'), KeyModifiers::SHIFT),
        AppAction::TabSelectedNext,
    );
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::Left, KeyModifiers::SHIFT),
        AppAction::TabSelectedPrev,
    );
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::Right, KeyModifiers::SHIFT),
        AppAction::TabSelectedNext,
    );
    key_map.add(
        Keybind::Exact(AppState::Sheet, KeyCode::Left, KeyModifiers::SHIFT),
        AppAction::TabSelectedPrev,
    );
    key_map.add(
        Keybind::Exact(AppState::Sheet, KeyCode::Right, KeyModifiers::SHIFT),
        AppAction::TabSelectedNext,
    );
    // Move to line by number
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::Char('1'), KeyModifiers::empty()),
        AppAction::StatusBarCommand("goto 1".to_owned()),
    );
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::Char('2'), KeyModifiers::empty()),
        AppAction::StatusBarCommand("goto 2".to_owned()),
    );
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::Char('3'), KeyModifiers::empty()),
        AppAction::StatusBarCommand("goto 3".to_owned()),
    );
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::Char('4'), KeyModifiers::empty()),
        AppAction::StatusBarCommand("goto 4".to_owned()),
    );
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::Char('5'), KeyModifiers::empty()),
        AppAction::StatusBarCommand("goto 5".to_owned()),
    );
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::Char('6'), KeyModifiers::empty()),
        AppAction::StatusBarCommand("goto 6".to_owned()),
    );
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::Char('7'), KeyModifiers::empty()),
        AppAction::StatusBarCommand("goto 7".to_owned()),
    );
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::Char('8'), KeyModifiers::empty()),
        AppAction::StatusBarCommand("goto 8".to_owned()),
    );
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::Char('9'), KeyModifiers::empty()),
        AppAction::StatusBarCommand("goto 9".to_owned()),
    );
    // Select random
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::Char('R'), KeyModifiers::SHIFT),
        AppAction::TabularGotoRandom,
    );
    // Reset dataframe
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::Char('r'), KeyModifiers::CONTROL),
        AppAction::TabularReset,
    );
    // Command start, stop, and commit
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::Char(':'), KeyModifiers::empty()),
        AppAction::StatusBarCommand("".to_owned()),
    );
    key_map.add(
        Keybind::Exact(AppState::Error, KeyCode::Char(':'), KeyModifiers::empty()),
        AppAction::StatusBarCommand("".to_owned()),
    );
    key_map.add(
        Keybind::Exact(AppState::Sheet, KeyCode::Char(':'), KeyModifiers::empty()),
        AppAction::StatusBarCommand("".to_owned()),
    );
    key_map.add(
        Keybind::Exact(AppState::Command, KeyCode::Esc, KeyModifiers::empty()),
        AppAction::StatusBarStats,
    );
    key_map.add(
        Keybind::Exact(AppState::Command, KeyCode::Enter, KeyModifiers::empty()),
        AppAction::PromptCommit,
    );
    // Search start, stop, and commit
    key_map.add(
        Keybind::Exact(AppState::Table, KeyCode::Char('/'), KeyModifiers::empty()),
        AppAction::StatusBarSearch("".to_owned()),
    );
    key_map.add(
        Keybind::Exact(AppState::Search, KeyCode::Esc, KeyModifiers::empty()),
        AppAction::SearchRollback,
    );
    key_map.add(
        Keybind::Exact(AppState::Search, KeyCode::Enter, KeyModifiers::empty()),
        AppAction::SearchCommit,
    );
    // Keyboard input in command and search
    key_map.add(Keybind::StateOnly(AppState::Search), |_, key_event| {
        AppAction::StatusBarHandleSearch(key_event)
    });
    key_map.add(Keybind::StateOnly(AppState::Command), |_, key_event| {
        AppAction::StatusBarHandleCommand(key_event)
    });
    key_map
}

impl KeyMap {
    fn add(&mut self, keybind: Keybind, action: impl Into<Action>) {
        self.map.insert(keybind, action.into());
    }
    pub fn get(&self, state: AppState, key_event: KeyEvent) -> Option<AppAction> {
        self.map
            .get(&Keybind::Exact(state, key_event.code, key_event.modifiers))
            .or(self
                .map
                .get(&Keybind::KeyOnly(key_event.code, key_event.modifiers)))
            .or(self.map.get(&Keybind::StateOnly(state)))
            .map(|action| match action {
                Action::Direct(action) => action.to_owned(),
                Action::Closure(closure) => closure(state, key_event),
            })
    }
}
