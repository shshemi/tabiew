use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::Margin,
    text::{Line, Span},
    widgets::{Block, Clear, Paragraph, Widget},
};
use unicode_width::UnicodeWidthStr;

use crate::{
    handler::message::Message,
    misc::config::theme,
    tui::{
        app_default::{AppDefault, AppTitle},
        component::Component,
        icons,
        utils::Scroll,
    },
};

type Section = (&'static str, &'static [(&'static str, &'static str)]);

const SECTIONS: &[Section] = &[
    (
        "Table navigation",
        &[
            ("k j / Up Down / Ctrl+p Ctrl+n", "Previous / next row"),
            ("h l / Left Right", "Scroll columns left / right"),
            ("b w / Alt+Left Alt+Right", "Previous / next column"),
            ("_ / $", "First / last column"),
            ("g / Home", "First row"),
            ("G / End", "Last row"),
            ("Ctrl+u / Ctrl+d", "Half page up / down"),
            ("Ctrl+b Ctrl+f / PageUp PageDown", "Page up / down"),
            ("1-9", "Go to line, starting with that digit"),
            ("R", "Select a random row"),
            ("e", "Toggle auto-fit column widths"),
            ("i", "Table info"),
            ("Enter", "Open sheet for the selected row"),
        ],
    ),
    (
        "Sheet",
        &[
            ("J K / Shift+Down Shift+Up", "Scroll"),
            ("f", "Switch between JSON and YAML"),
            ("c", "Copy the row to the clipboard"),
            ("Esc / q", "Close sheet"),
        ],
    ),
    (
        "Search",
        &[
            ("/", "Fuzzy search"),
            ("?", "Exact search"),
            ("Enter", "Keep the result and close"),
            ("Esc", "Cancel and restore the table"),
        ],
    ),
    (
        "Tabs",
        &[
            ("H L / Shift+Left Shift+Right", "Previous / next tab"),
            ("t", "Tab switcher"),
            ("q", "Close the pushed table, then the tab; quit when none"),
            ("Q", "Quit"),
            ("k j / Up Down", "In switcher: select tab"),
            ("Backspace / Delete", "In switcher: close tab"),
            ("Enter / Esc", "In switcher: keep / revert selection"),
        ],
    ),
    (
        "Commands",
        &[
            (":", "Command palette"),
            ("F1", "This help"),
            ("q + Space", "In palette: SQL query"),
            ("s + Space", "In palette: select columns"),
            ("o + Space", "In palette: order rows"),
            ("f + Space", "In palette: filter rows"),
        ],
    ),
];

#[derive(Debug, Default)]
pub struct Keybindings {
    scroll: Scroll,
    page: u16,
}

impl Keybindings {
    fn scroll_by(&mut self, up: bool, lines: u16) {
        for _ in 0..lines {
            if up {
                self.scroll.up();
            } else {
                self.scroll.down();
            }
        }
    }
}

impl Component for Keybindings {
    fn render(
        &mut self,
        _area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        _focus_state: crate::tui::component::FocusState,
    ) {
        let area = buf.area.inner(Margin::new(7, 3));
        Clear.render(area, buf);

        let key_width = SECTIONS
            .iter()
            .flat_map(|(_, entries)| entries.iter())
            .map(|(key, _)| key.width())
            .max()
            .unwrap_or(0);
        let mut lines = Vec::new();
        for (idx, (title, entries)) in SECTIONS.iter().enumerate() {
            if idx > 0 {
                lines.push(Line::default());
            }
            lines.push(Line::styled(*title, theme().header(0)));
            lines.extend(entries.iter().map(|(key, desc)| {
                Line::from(vec![
                    Span::styled(format!("  {key:<key_width$}  "), theme().text_highlighted()),
                    Span::styled(*desc, theme().text()),
                ])
            }));
        }

        let block = Block::app_default().app_title(icons::BOOK.title("Keybindings"));
        let inner = block.inner(area);
        self.page = inner.height;
        self.scroll.adjust(lines.len(), inner.height);
        Paragraph::new(lines)
            .style(theme().text())
            .block(block)
            .scroll((self.scroll.val_u16(), 0))
            .render(area, buf);
    }

    fn handle(&mut self, event: KeyEvent) -> bool {
        match (event.code, event.modifiers) {
            (KeyCode::Up, KeyModifiers::NONE) | (KeyCode::Char('k'), KeyModifiers::NONE) => {
                self.scroll_by(true, 1);
                true
            }
            (KeyCode::Down, KeyModifiers::NONE) | (KeyCode::Char('j'), KeyModifiers::NONE) => {
                self.scroll_by(false, 1);
                true
            }
            (KeyCode::PageUp, KeyModifiers::NONE) => {
                self.scroll_by(true, self.page);
                true
            }
            (KeyCode::PageDown, KeyModifiers::NONE) => {
                self.scroll_by(false, self.page);
                true
            }
            (KeyCode::Esc, KeyModifiers::NONE)
            | (KeyCode::Char('q'), KeyModifiers::NONE)
            | (KeyCode::F(1), KeyModifiers::NONE) => {
                Message::AppDismissOverlay.enqueue();
                true
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::component::FocusState;

    #[test]
    fn closes_with_esc_q_and_f1() {
        for code in [KeyCode::Esc, KeyCode::Char('q'), KeyCode::F(1)] {
            assert!(Keybindings::default().handle(KeyEvent::new(code, KeyModifiers::NONE)));
        }
    }

    #[test]
    fn page_down_stops_at_last_line() {
        let mut keybindings = Keybindings::default();
        let mut buf = ratatui::buffer::Buffer::empty(ratatui::layout::Rect::new(0, 0, 80, 12));
        keybindings.render(buf.area, &mut buf, FocusState::Focused);
        for _ in 0..100 {
            keybindings.handle(KeyEvent::new(KeyCode::PageDown, KeyModifiers::NONE));
        }
        keybindings.render(buf.area, &mut buf, FocusState::Focused);
        let total = SECTIONS.iter().map(|(_, e)| e.len() + 2).sum::<usize>() - 1;
        assert_eq!(keybindings.scroll.val_u16() as usize, total - 4);
    }
}
