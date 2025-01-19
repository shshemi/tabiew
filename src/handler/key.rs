use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{app::AppContext, handler::action::AppAction};

#[derive(Debug, PartialEq, Eq, Hash)]
enum Keybind {
    Exact(AppContext, KeyCode, KeyModifiers),
    KeyOnly(KeyCode, KeyModifiers),
    StateOnly(AppContext),
}
enum Action {
    Direct(AppAction),
    Closure(Box<dyn Fn(AppContext, KeyEvent) -> AppAction>),
}

impl From<AppAction> for Action {
    fn from(value: AppAction) -> Self {
        Action::Direct(value)
    }
}

impl<F: Fn(AppContext, KeyEvent) -> AppAction + 'static> From<F> for Action {
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
        Keybind::StateOnly(AppContext::Error),
        AppAction::StatusBarInfo,
    );
    // Close app/tab/sheet
    key_map.add(
        Keybind::Exact(AppContext::Empty, KeyCode::Char('q'), KeyModifiers::empty()),
        AppAction::Quit,
    );
    key_map.add(
        Keybind::Exact(AppContext::Sheet, KeyCode::Char('q'), KeyModifiers::empty()),
        AppAction::TabularTableMode,
    );
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Char('q'), KeyModifiers::empty()),
        AppAction::TabRemoveOrQuit,
    );
    // Switch tab/sheet/enter
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Enter, KeyModifiers::empty()),
        AppAction::TabularSheetMode,
    );
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Char('v'), KeyModifiers::empty()),
        AppAction::TabularSheetMode,
    );
    key_map.add(
        Keybind::Exact(AppContext::Sheet, KeyCode::Char('v'), KeyModifiers::empty()),
        AppAction::TabularTableMode,
    );
    // Move half page
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Char('u'), KeyModifiers::CONTROL),
        AppAction::TabularGoUpHalfPage,
    );
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Char('d'), KeyModifiers::CONTROL),
        AppAction::TabularGoDownHalfPage,
    );
    // Move full page
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::PageUp, KeyModifiers::empty()),
        AppAction::TabularGoUpFullPage,
    );
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::PageDown, KeyModifiers::empty()),
        AppAction::TabularGoDownFullPage,
    );
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Char('b'), KeyModifiers::CONTROL),
        AppAction::TabularGoUpFullPage,
    );
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Char('f'), KeyModifiers::CONTROL),
        AppAction::TabularGoDownFullPage,
    );
    // Move to prev/next record
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Up, KeyModifiers::empty()),
        AppAction::TabularGoUp(1),
    );
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Down, KeyModifiers::empty()),
        AppAction::TabularGoDown(1),
    );
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Char('k'), KeyModifiers::empty()),
        AppAction::TabularGoUp(1),
    );
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Char('j'), KeyModifiers::empty()),
        AppAction::TabularGoDown(1),
    );
    key_map.add(
        Keybind::Exact(AppContext::Sheet, KeyCode::Right, KeyModifiers::empty()),
        AppAction::TabularGoDown(1),
    );
    key_map.add(
        Keybind::Exact(AppContext::Sheet, KeyCode::Left, KeyModifiers::empty()),
        AppAction::TabularGoUp(1),
    );
    key_map.add(
        Keybind::Exact(AppContext::Sheet, KeyCode::Char('h'), KeyModifiers::empty()),
        AppAction::TabularGoUp(1),
    );
    key_map.add(
        Keybind::Exact(AppContext::Sheet, KeyCode::Char('l'), KeyModifiers::empty()),
        AppAction::TabularGoDown(1),
    );


    // Table view toggle expansion
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Char('e'), KeyModifiers::empty()),
        AppAction::TabularToggleExpansion,
    );

    // Scroll table left / right and start / end
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Char('h'), KeyModifiers::empty()),
        AppAction::TabularScrollLeft,
    );
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Char('l'), KeyModifiers::empty()),
        AppAction::TabularScrollRight,
    );
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Left, KeyModifiers::empty()),
        AppAction::TabularScrollLeft,
    );
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Right, KeyModifiers::empty()),
        AppAction::TabularScrollRight,
    );
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Char('$'), KeyModifiers::empty()),
        AppAction::TabularScrollEnd,
    );
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Char('_'), KeyModifiers::empty()),
        AppAction::TabularScrollStart,
    );

    // Move to first/last record
    key_map.add(
        Keybind::Exact(AppContext::Sheet, KeyCode::Home, KeyModifiers::empty()),
        AppAction::TabularGotoFirst,
    );
    key_map.add(
        Keybind::Exact(AppContext::Sheet, KeyCode::End, KeyModifiers::empty()),
        AppAction::TabularGotoLast,
    );
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Home, KeyModifiers::empty()),
        AppAction::TabularGotoFirst,
    );
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::End, KeyModifiers::empty()),
        AppAction::TabularGotoLast,
    );
    key_map.add(
        Keybind::Exact(AppContext::Sheet, KeyCode::Char('g'), KeyModifiers::empty()),
        AppAction::TabularGotoFirst,
    );
    key_map.add(
        Keybind::Exact(AppContext::Sheet, KeyCode::Char('G'), KeyModifiers::SHIFT),
        AppAction::TabularGotoLast,
    );
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Char('g'), KeyModifiers::empty()),
        AppAction::TabularGotoFirst,
    );
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Char('G'), KeyModifiers::SHIFT),
        AppAction::TabularGotoLast,
    );
    // Scroll up/down in sheets
    key_map.add(
        Keybind::Exact(AppContext::Sheet, KeyCode::Up, KeyModifiers::empty()),
        AppAction::SheetScrollUp,
    );
    key_map.add(
        Keybind::Exact(AppContext::Sheet, KeyCode::Down, KeyModifiers::empty()),
        AppAction::SheetScrollDown,
    );
    key_map.add(
        Keybind::Exact(AppContext::Sheet, KeyCode::Char('k'), KeyModifiers::empty()),
        AppAction::SheetScrollUp,
    );
    key_map.add(
        Keybind::Exact(AppContext::Sheet, KeyCode::Char('j'), KeyModifiers::empty()),
        AppAction::SheetScrollDown,
    );
    // Move prev/next tab
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Char('H'), KeyModifiers::SHIFT),
        AppAction::TabPrev,
    );
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Char('L'), KeyModifiers::SHIFT),
        AppAction::TabNext,
    );
    key_map.add(
        Keybind::Exact(AppContext::Sheet, KeyCode::Char('H'), KeyModifiers::SHIFT),
        AppAction::TabPrev,
    );
    key_map.add(
        Keybind::Exact(AppContext::Sheet, KeyCode::Char('L'), KeyModifiers::SHIFT),
        AppAction::TabNext,
    );
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Left, KeyModifiers::SHIFT),
        AppAction::TabPrev,
    );
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Right, KeyModifiers::SHIFT),
        AppAction::TabNext,
    );
    key_map.add(
        Keybind::Exact(AppContext::Sheet, KeyCode::Left, KeyModifiers::SHIFT),
        AppAction::TabPrev,
    );
    key_map.add(
        Keybind::Exact(AppContext::Sheet, KeyCode::Right, KeyModifiers::SHIFT),
        AppAction::TabNext,
    );
    // Move to line by number
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Char('1'), KeyModifiers::empty()),
        AppAction::StatusBarCommand("goto 1".to_owned()),
    );
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Char('2'), KeyModifiers::empty()),
        AppAction::StatusBarCommand("goto 2".to_owned()),
    );
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Char('3'), KeyModifiers::empty()),
        AppAction::StatusBarCommand("goto 3".to_owned()),
    );
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Char('4'), KeyModifiers::empty()),
        AppAction::StatusBarCommand("goto 4".to_owned()),
    );
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Char('5'), KeyModifiers::empty()),
        AppAction::StatusBarCommand("goto 5".to_owned()),
    );
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Char('6'), KeyModifiers::empty()),
        AppAction::StatusBarCommand("goto 6".to_owned()),
    );
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Char('7'), KeyModifiers::empty()),
        AppAction::StatusBarCommand("goto 7".to_owned()),
    );
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Char('8'), KeyModifiers::empty()),
        AppAction::StatusBarCommand("goto 8".to_owned()),
    );
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Char('9'), KeyModifiers::empty()),
        AppAction::StatusBarCommand("goto 9".to_owned()),
    );
    // Select random
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Char('R'), KeyModifiers::SHIFT),
        AppAction::TabularGotoRandom,
    );
    // Reset dataframe
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Char('r'), KeyModifiers::CONTROL),
        AppAction::TabularReset,
    );
    // Command start, stop, and commit
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Char(':'), KeyModifiers::empty()),
        AppAction::StatusBarCommand("".to_owned()),
    );
    key_map.add(
        Keybind::Exact(AppContext::Error, KeyCode::Char(':'), KeyModifiers::empty()),
        AppAction::StatusBarCommand("".to_owned()),
    );
    key_map.add(
        Keybind::Exact(AppContext::Sheet, KeyCode::Char(':'), KeyModifiers::empty()),
        AppAction::StatusBarCommand("".to_owned()),
    );
    key_map.add(
        Keybind::Exact(AppContext::Command, KeyCode::Esc, KeyModifiers::empty()),
        AppAction::StatusBarInfo,
    );
    key_map.add(
        Keybind::Exact(AppContext::Command, KeyCode::Enter, KeyModifiers::empty()),
        AppAction::PromptCommit,
    );
    key_map.add(Keybind::StateOnly(AppContext::Command), |_, key_event| {
        AppAction::StatusBarHandle(key_event)
    });

    // Search start, stop, and commit
    key_map.add(
        Keybind::Exact(AppContext::Table, KeyCode::Char('/'), KeyModifiers::empty()),
        AppAction::TabularSearchMode,
    );
    key_map.add(
        Keybind::Exact(AppContext::Search, KeyCode::Home, KeyModifiers::empty()),
        AppAction::SearchGotoStart,
    );
    key_map.add(
        Keybind::Exact(AppContext::Search, KeyCode::End, KeyModifiers::empty()),
        AppAction::SearchGotoEnd,
    );
    key_map.add(
        Keybind::Exact(AppContext::Search, KeyCode::Left, KeyModifiers::empty()),
        AppAction::SearchGotoPrev,
    );
    key_map.add(
        Keybind::Exact(AppContext::Search, KeyCode::Right, KeyModifiers::empty()),
        AppAction::SearchGotoNext,
    );
    key_map.add(
        Keybind::Exact(AppContext::Search, KeyCode::Backspace, KeyModifiers::empty()),
        AppAction::SearchDeletePrev,
    );
    key_map.add(
        Keybind::Exact(AppContext::Search, KeyCode::Delete, KeyModifiers::empty()),
        AppAction::SearchDeleteNext,
    );
    key_map.add(
        Keybind::Exact(AppContext::Search, KeyCode::Esc, KeyModifiers::empty()),
        AppAction::SearchRollback,
    );
    key_map.add(
        Keybind::Exact(AppContext::Search, KeyCode::Enter, KeyModifiers::empty()),
        AppAction::SearchCommit,
    );
    key_map.add(
        Keybind::StateOnly(AppContext::Search),
        |_, key_event: KeyEvent| {
            if let KeyCode::Char(c) = key_event.code {
                AppAction::SearchInsert(c)
            } else {
                AppAction::NoAction
            }
        },
    );

    //
    // Pallete
    // key_map.add(
    //     Keybind::Exact(AppContext::Table, KeyCode::Char('P'), KeyModifiers::SHIFT),
    //     AppAction::CommandPalleteShow,
    // );
    // key_map.add(
    //     Keybind::Exact(AppContext::Pallete, KeyCode::Enter, KeyModifiers::NONE),
    //     AppAction::CommandPalleteHide,
    // );
    // key_map.add(
    //     Keybind::Exact(AppContext::Pallete, KeyCode::Right, KeyModifiers::NONE),
    //     AppAction::CommandPalleteNext,
    // );
    // key_map.add(
    //     Keybind::Exact(AppContext::Pallete, KeyCode::Left, KeyModifiers::NONE),
    //     AppAction::CommandPalletePrev,
    // );
    // key_map.add(
    //     Keybind::Exact(AppContext::Pallete, KeyCode::Home, KeyModifiers::NONE),
    //     AppAction::CommandPalleteStart,
    // );
    // key_map.add(
    //     Keybind::Exact(AppContext::Pallete, KeyCode::End, KeyModifiers::NONE),
    //     AppAction::CommandPalleteEnd,
    // );
    // key_map.add(
    //     Keybind::Exact(AppContext::Pallete, KeyCode::Up, KeyModifiers::NONE),
    //     AppAction::CommandPalleteAbove,
    // );
    // key_map.add(
    //     Keybind::Exact(AppContext::Pallete, KeyCode::Down, KeyModifiers::NONE),
    //     AppAction::CommandPalleteBelow,
    // );
    // key_map.add(
    //     Keybind::Exact(AppContext::Pallete, KeyCode::Backspace, KeyModifiers::NONE),
    //     AppAction::CommandPalleteDeletePrev,
    // );
    // key_map.add(
    //     Keybind::Exact(AppContext::Pallete, KeyCode::Delete, KeyModifiers::NONE),
    //     AppAction::CommandPalleteDeleteNext,
    // );
    // key_map.add(
    //     Keybind::StateOnly(AppContext::Pallete),
    //     |_, event: KeyEvent| {
    //         if let KeyCode::Char(c) = event.code {
    //             AppAction::CommandPalleteInsert(c)
    //         } else {
    //             AppAction::NoAction
    //         }
    //     },
    // );
    key_map
}

impl KeyMap {
    fn add(&mut self, keybind: Keybind, action: impl Into<Action>) {
        self.map.insert(keybind, action.into());
    }
    fn get(&self, state: AppContext, key_event: KeyEvent) -> Option<AppAction> {
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

    pub fn get_action(&self, state: AppContext, key_event: KeyEvent) -> AppAction {
        self.get(state, key_event).unwrap_or(AppAction::NoAction)
    }
}
