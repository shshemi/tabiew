use std::{collections::HashMap, fmt::Debug};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{app::Context, handler::action::AppAction};

enum Action {
    Direct(AppAction),
    Closure(Box<dyn Fn(KeyEvent) -> AppAction>),
}

impl Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Direct(arg0) => f.debug_tuple("Direct").field(arg0).finish(),
            Self::Closure(_arg0) => f.debug_tuple("Closure").finish(),
        }
    }
}

impl From<AppAction> for Action {
    fn from(value: AppAction) -> Self {
        Action::Direct(value)
    }
}

impl<F: Fn(KeyEvent) -> AppAction + 'static> From<F> for Action {
    fn from(value: F) -> Self {
        Action::Closure(Box::new(value))
    }
}

#[derive(Debug)]
struct Keybind {
    code: KeyCode,
    modifiers: KeyModifiers,
    action: Action,
}

impl Default for Keybind {
    fn default() -> Self {
        Self {
            code: KeyCode::Null,
            modifiers: KeyModifiers::empty(),
            action: Action::Direct(AppAction::NoAction),
        }
    }
}

impl Keybind {
    fn code(mut self, code: KeyCode) -> Self {
        self.code = code;
        self
    }

    fn char(mut self, c: char) -> Self {
        self.code = KeyCode::Char(c);
        self
    }

    fn shift(mut self) -> Self {
        self.modifiers |= KeyModifiers::SHIFT;
        self
    }

    fn ctrl(mut self) -> Self {
        self.modifiers |= KeyModifiers::CONTROL;
        self
    }

    #[allow(dead_code)]
    fn alt(mut self) -> Self {
        self.modifiers |= KeyModifiers::ALT;
        self
    }

    #[allow(dead_code)]
    fn meta(mut self) -> Self {
        self.modifiers |= KeyModifiers::META;
        self
    }

    fn action(mut self, action: impl Into<Action>) -> Self {
        self.action = action.into();
        self
    }

    fn matches(&self, event: KeyEvent) -> Option<AppAction> {
        if self.code != event.code {
            return None;
        }

        // Ignore SHIFT for character keys (cross-platform/cross-keyboard compatibility)
        let dominated = match self.code {
            KeyCode::Char(_) => KeyModifiers::SHIFT,
            _ => KeyModifiers::empty(),
        };

        let self_mods = self.modifiers.difference(dominated);
        let event_mods = event.modifiers.difference(dominated);

        (self_mods == event_mods).then_some(match &self.action {
            Action::Direct(app_action) => app_action.clone(),
            Action::Closure(closure) => closure(event),
        })
    }
}

#[derive(Default)]
struct Keybinds {
    list: Vec<Keybind>,
    fall_back: Option<Box<dyn Fn(KeyEvent) -> Option<AppAction>>>,
}

impl Keybinds {
    fn find(&self, event: KeyEvent) -> Option<AppAction> {
        self.list
            .iter()
            .find_map(|kb| kb.matches(event))
            .or(self.fall_back.as_ref().and_then(|fb| fb(event)))
    }

    fn add(&mut self, kb: Keybind) -> &mut Self {
        self.list.push(kb);
        self
    }

    fn fallback(&mut self, closure: impl Fn(KeyEvent) -> Option<AppAction> + 'static) {
        self.fall_back = Some(Box::new(closure));
    }
}

pub struct KeyHandler {
    map: HashMap<Context, Keybinds>,
}

impl KeyHandler {
    pub fn action(&self, mut context: Context, event: KeyEvent) -> AppAction {
        loop {
            if let Some(act) = self.map.get(&context).and_then(|kbl| kbl.find(event)) {
                return act;
            } else if let Some(parent) = context.parent() {
                context = parent;
            } else {
                return AppAction::NoAction;
            }
        }
    }

    fn keybinds(&mut self, context: Context) -> &mut Keybinds {
        self.map.entry(context).or_default()
    }
}

impl Default for KeyHandler {
    fn default() -> Self {
        let mut hndl = Self {
            map: Default::default(),
        };

        // ----- empty keybindings
        hndl.keybinds(Context::Empty)
            // :
            .add(
                Keybind::default()
                    .char(':')
                    .action(AppAction::PaletteShow(String::default())),
            )
            .fallback(|event| match event.code {
                KeyCode::Char('1') => Some(AppAction::GoToLineShowWithValue(1)),
                KeyCode::Char('2') => Some(AppAction::GoToLineShowWithValue(2)),
                KeyCode::Char('3') => Some(AppAction::GoToLineShowWithValue(3)),
                KeyCode::Char('4') => Some(AppAction::GoToLineShowWithValue(4)),
                KeyCode::Char('5') => Some(AppAction::GoToLineShowWithValue(5)),
                KeyCode::Char('6') => Some(AppAction::GoToLineShowWithValue(6)),
                KeyCode::Char('7') => Some(AppAction::GoToLineShowWithValue(7)),
                KeyCode::Char('8') => Some(AppAction::GoToLineShowWithValue(8)),
                KeyCode::Char('9') => Some(AppAction::GoToLineShowWithValue(9)),
                _ => None,
            });

        // ----- error keybindings
        hndl.keybinds(Context::Error)
            .add(
                Keybind::default()
                    .char(':')
                    .action(AppAction::DismissErrorAndShowPalette),
            )
            .fallback(|_| Some(AppAction::DismissError));

        // ----- table keybindings
        hndl.keybinds(Context::Table)
            // F1
            .add(
                Keybind::default()
                    .code(KeyCode::F(1))
                    .action(AppAction::HelpShow),
            )
            // Q
            .add(Keybind::default().char('Q').action(AppAction::Quit))
            // q
            .add(
                Keybind::default()
                    .char('q')
                    .action(AppAction::TabRemoveOrQuit),
            )
            // enter
            .add(
                Keybind::default()
                    .code(KeyCode::Enter)
                    .action(AppAction::SheetShow),
            )
            //  / ?
            .add(
                Keybind::default()
                    .char('/')
                    .action(AppAction::SearchFuzzyShow),
            )
            //  / ?
            .add(
                Keybind::default()
                    .char('?')
                    .action(AppAction::SearchExactShow),
            )
            // f
            .add(
                Keybind::default()
                    .char('f')
                    .action(AppAction::ToggleBorders),
            )
            //  e
            .add(
                Keybind::default()
                    .char('e')
                    .action(AppAction::TableToggleExpansion),
            )
            //  arrow keys
            .add(
                Keybind::default()
                    .code(KeyCode::Up)
                    .action(AppAction::TableGoUp(1)),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::Down)
                    .action(AppAction::TableGoDown(1)),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::Left)
                    .action(AppAction::TableScrollLeft),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::Right)
                    .action(AppAction::TableScrollRight),
            )
            // hjkl keys
            .add(Keybind::default().char('k').action(AppAction::TableGoUp(1)))
            .add(
                Keybind::default()
                    .char('j')
                    .action(AppAction::TableGoDown(1)),
            )
            .add(
                Keybind::default()
                    .char('h')
                    .action(AppAction::TableScrollLeft),
            )
            .add(
                Keybind::default()
                    .char('l')
                    .action(AppAction::TableScrollRight),
            )
            // b w
            .add(
                Keybind::default()
                    .char('b')
                    .action(AppAction::TableScrollLeftColumn),
            )
            .add(
                Keybind::default()
                    .char('w')
                    .action(AppAction::TableScrollRightColumn),
            )
            // ctrl-u ctrl-d
            .add(
                Keybind::default()
                    .char('u')
                    .ctrl()
                    .action(AppAction::TableGoUpHalfPage),
            )
            .add(
                Keybind::default()
                    .char('d')
                    .ctrl()
                    .action(AppAction::TableGoDownHalfPage),
            )
            // ctrl-b ctrl-f pageup pagedown
            .add(
                Keybind::default()
                    .char('b')
                    .ctrl()
                    .action(AppAction::TableGoUpFullPage),
            )
            .add(
                Keybind::default()
                    .char('f')
                    .ctrl()
                    .action(AppAction::TableGoDownFullPage),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::PageUp)
                    .action(AppAction::TableGoUpFullPage),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::PageDown)
                    .action(AppAction::TableGoDownFullPage),
            )
            // ^_ $ line beginning end
            .add(
                Keybind::default()
                    .char('^')
                    .action(AppAction::TableScrollStart),
            )
            .add(
                Keybind::default()
                    .char('_')
                    .action(AppAction::TableScrollStart),
            )
            .add(
                Keybind::default()
                    .char('$')
                    .action(AppAction::TableScrollEnd),
            )
            // g G home end
            .add(
                Keybind::default()
                    .char('g')
                    .action(AppAction::TableGotoFirst),
            )
            .add(
                Keybind::default()
                    .char('G')
                    .action(AppAction::TableGotoLast),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::Home)
                    .action(AppAction::TableGotoFirst),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::End)
                    .action(AppAction::TableGotoLast),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::Char('r'))
                    .ctrl()
                    .action(AppAction::TableReset),
            ) // shift-h shift-l shift-left shift-right
            .add(Keybind::default().char('H').action(AppAction::TabPrev))
            .add(Keybind::default().char('L').action(AppAction::TabNext))
            .add(
                Keybind::default()
                    .code(KeyCode::Left)
                    .shift()
                    .action(AppAction::TabPrev),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::Right)
                    .shift()
                    .action(AppAction::TabNext),
            )
            // I
            .add(
                Keybind::default()
                    .char('I')
                    .action(AppAction::DataFrameInfoShow),
            )
            // t
            .add(Keybind::default().char('t').action(AppAction::TabShowPanel));

        // ---- schema keybindings
        hndl.keybinds(Context::Schema)
            // up & down
            .add(
                Keybind::default()
                    .code(KeyCode::Up)
                    .action(AppAction::SchemaNamesSelectPrev),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::Down)
                    .action(AppAction::SchemaNamesSelectNext),
            )
            // KJ
            .add(
                Keybind::default()
                    .code(KeyCode::Char('k'))
                    .action(AppAction::SchemaNamesSelectPrev),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::Char('j'))
                    .action(AppAction::SchemaNamesSelectNext),
            )
            // shift - up & down
            .add(
                Keybind::default()
                    .code(KeyCode::Up)
                    .shift()
                    .action(AppAction::SchemaFieldsScrollUp),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::Down)
                    .shift()
                    .action(AppAction::SchemaFieldsScrollDown),
            )
            // KJ
            .add(
                Keybind::default()
                    .char('K')
                    .action(AppAction::SchemaFieldsScrollUp),
            )
            .add(
                Keybind::default()
                    .char('J')
                    .action(AppAction::SchemaFieldsScrollDown),
            )
            // gG
            .add(
                Keybind::default()
                    .char('g')
                    .action(AppAction::SchemaNamesSelectFirst),
            )
            .add(
                Keybind::default()
                    .char('G')
                    .action(AppAction::SchemaNamesSelectLast),
            )
            // enter
            .add(
                Keybind::default()
                    .code(KeyCode::Enter)
                    .action(AppAction::SchemaOpenTable),
            )
            // delete
            .add(
                Keybind::default()
                    .code(KeyCode::Delete)
                    .action(AppAction::SchemaUnloadTable),
            )
            // q esc
            .add(
                Keybind::default()
                    .code(KeyCode::Esc)
                    .action(AppAction::SwitchToTabulars),
            )
            .add(
                Keybind::default()
                    .char('q')
                    .action(AppAction::SwitchToTabulars),
            );

        // ---- command keybindings
        hndl.keybinds(Context::Command)
            .add(
                Keybind::default()
                    .code(KeyCode::Up)
                    .action(AppAction::PaletteSelectPrevious),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::Down)
                    .action(AppAction::PaletteSelectNext),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::Char('p'))
                    .ctrl()
                    .action(AppAction::PaletteSelectPrevious),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::Char('n'))
                    .ctrl()
                    .action(AppAction::PaletteSelectNext),
            )
            // enter esc
            .add(
                Keybind::default()
                    .code(KeyCode::Enter)
                    .action(AppAction::PaletteInsertSelectedOrCommit),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::Esc)
                    .action(AppAction::PaletteDeselectOrDismiss),
            )
            .fallback(|event| Some(AppAction::PalleteHandleKeyEvent(event)));

        // ---- sheet keybindings
        hndl.keybinds(Context::Sheet)
            // q and esc
            .add(Keybind::default().char('q').action(AppAction::DismissModal))
            .add(
                Keybind::default()
                    .code(KeyCode::Esc)
                    .action(AppAction::DismissModal),
            )
            // shift up down j k
            .add(
                Keybind::default()
                    .code(KeyCode::Up)
                    .shift()
                    .action(AppAction::SheetScrollUp),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::Down)
                    .shift()
                    .action(AppAction::SheetScrollDown),
            )
            .add(
                Keybind::default()
                    .char('K')
                    .action(AppAction::SheetScrollUp),
            )
            .add(
                Keybind::default()
                    .char('J')
                    .action(AppAction::SheetScrollDown),
            );

        // ---- tabs keybindings
        hndl.keybinds(Context::TabSidePanel)
            .add(
                Keybind::default()
                    .code(KeyCode::Up)
                    .action(AppAction::TabPanelPrev),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::Down)
                    .action(AppAction::TabPanelNext),
            )
            .add(Keybind::default().char('k').action(AppAction::TabPanelPrev))
            .add(Keybind::default().char('j').action(AppAction::TabPanelNext))
            .add(
                Keybind::default()
                    .code(KeyCode::Enter)
                    .action(AppAction::TabPanelSelect),
            )
            .add(Keybind::default().char('Q').action(AppAction::Quit))
            .add(Keybind::default().char('q').action(AppAction::TabHidePanel))
            .add(
                Keybind::default()
                    .code(KeyCode::Esc)
                    .action(AppAction::TabHidePanel),
            );

        // ---- search keybindings
        hndl.keybinds(Context::Search)
            // enter esc
            .add(
                Keybind::default()
                    .code(KeyCode::Enter)
                    .action(AppAction::SearchCommit),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::Esc)
                    .action(AppAction::SearchRollback),
            )
            .fallback(|event| Some(AppAction::SearchHandleKeyEvent(event)));

        // ---- data frame info keybindings
        hndl.keybinds(Context::DataFrameInfo)
            // J K
            .add(
                Keybind::default()
                    .char('K')
                    .action(AppAction::DataFrameInfoScrollUp),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::Up)
                    .shift()
                    .action(AppAction::DataFrameInfoScrollUp),
            )
            .add(
                Keybind::default()
                    .char('J')
                    .action(AppAction::DataFrameInfoScrollDown),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::Down)
                    .shift()
                    .action(AppAction::DataFrameInfoScrollDown),
            )
            // q esc
            .add(Keybind::default().char('q').action(AppAction::DismissModal))
            .add(
                Keybind::default()
                    .code(KeyCode::Esc)
                    .action(AppAction::DismissModal),
            );

        // ---- scatter plot keybindings
        hndl.keybinds(Context::ScatterPlot)
            // q esc
            .add(Keybind::default().char('q').action(AppAction::DismissModal))
            .add(
                Keybind::default()
                    .code(KeyCode::Esc)
                    .action(AppAction::DismissModal),
            );

        // ---- histogram plot keybindings
        hndl.keybinds(Context::HistogramPlot)
            // up down
            .add(
                Keybind::default()
                    .code(KeyCode::Up)
                    .action(AppAction::HistogramScrollUp),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::Down)
                    .action(AppAction::HistogramScrollDown),
            )
            // jk
            .add(
                Keybind::default()
                    .char('k')
                    .action(AppAction::HistogramScrollUp),
            )
            .add(
                Keybind::default()
                    .char('j')
                    .action(AppAction::HistogramScrollDown),
            )
            // q esc
            .add(Keybind::default().char('q').action(AppAction::DismissModal))
            .add(
                Keybind::default()
                    .code(KeyCode::Esc)
                    .action(AppAction::DismissModal),
            );
        hndl.keybinds(Context::ThemeSelector)
            .add(
                Keybind::default()
                    .char('p')
                    .ctrl()
                    .action(AppAction::ThemeSelectorSelectPrev),
            )
            .add(
                Keybind::default()
                    .char('n')
                    .ctrl()
                    .action(AppAction::ThemeSelectorSelectNext),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::Up)
                    .action(AppAction::ThemeSelectorSelectPrev),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::Down)
                    .action(AppAction::ThemeSelectorSelectNext),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::Esc)
                    .action(AppAction::ThemeSelectorRollback),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::Enter)
                    .action(AppAction::ThemeSelectorCommit),
            )
            .fallback(|event| Some(AppAction::ThemeSelectorHandleKeyEvent(event)));

        // ---- inline query keybindings
        hndl.keybinds(Context::InlineQuery)
            .add(
                Keybind::default()
                    .code(KeyCode::Enter)
                    .action(AppAction::InlineQueryCommit),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::Esc)
                    .action(AppAction::DismissModal),
            )
            .fallback(|event| Some(AppAction::InlineQueryHandleKeyEvent(event)));

        hndl.keybinds(Context::GoToLine)
            .add(
                Keybind::default()
                    .code(KeyCode::Enter)
                    .action(AppAction::GoToLineCommit),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::Esc)
                    .action(AppAction::GoToLineRollback),
            )
            .fallback(|event| Some(AppAction::GoToLineHandleKeyEvent(event)));

        hndl.keybinds(Context::ExportWizard)
            .add(
                Keybind::default()
                    .code(KeyCode::Enter)
                    .action(AppAction::ExportWizardNextStep),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::Esc)
                    .action(AppAction::DismissModal),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::Up)
                    .action(AppAction::ExportWizardSelectPrev),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::Down)
                    .action(AppAction::ExportWizardSelectNext),
            )
            .add(
                Keybind::default()
                    .char('n')
                    .ctrl()
                    .action(AppAction::ExportWizardSelectNext),
            )
            .add(
                Keybind::default()
                    .char('p')
                    .ctrl()
                    .action(AppAction::ExportWizardSelectPrev),
            )
            .fallback(|event| Some(AppAction::ExportWizardHandleKeyEvent(event)));

        hndl.keybinds(Context::HistogramWizard)
            .add(
                Keybind::default()
                    .code(KeyCode::Enter)
                    .action(AppAction::HistogramWizardNextStep),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::Esc)
                    .action(AppAction::DismissModal),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::Up)
                    .action(AppAction::HistogramWizardSelectPrev),
            )
            .add(
                Keybind::default()
                    .code(KeyCode::Down)
                    .action(AppAction::HistogramWizardSelectNext),
            )
            .add(
                Keybind::default()
                    .char('n')
                    .ctrl()
                    .action(AppAction::HistogramWizardSelectNext),
            )
            .add(
                Keybind::default()
                    .char('p')
                    .ctrl()
                    .action(AppAction::HistogramWizardSelectPrev),
            )
            .fallback(|event| Some(AppAction::HistogramWizardHandleKeyEvent(event)));

        // ---- help modal keybindings
        hndl.keybinds(Context::Help)
            .add(Keybind::default().char('q').action(AppAction::HelpDismiss))
            .add(
                Keybind::default()
                    .code(KeyCode::Esc)
                    .action(AppAction::HelpDismiss),
            );

        hndl
    }
}
