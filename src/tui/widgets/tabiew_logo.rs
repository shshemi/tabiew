use itertools::Itertools;
use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};
use unicode_width::UnicodeWidthStr;

use crate::misc::config::theme;

const LARGE: &[&str] = &[
    "╭──────────╮     ████████╗ █████╗ ██████╗ ██╗███████╗██╗    ██╗",
    "│ >_ ▀▀ ▀▀ │     ╚══██╔══╝██╔══██╗██╔══██╗██║██╔════╝██║    ██║",
    "│ ▀▀ ▀▀ ▀▀ │        ██║   ███████║██████╔╝██║█████╗  ██║ █╗ ██║",
    "│ ▀▀ ▀▀ ▀▀ │        ██║   ██╔══██║██╔══██╗██║██╔══╝  ██║███╗██║",
    "╰──────────╯        ██║   ██║  ██║██████╔╝██║███████╗╚███╔███╔╝",
    "                    ╚═╝   ╚═╝  ╚═╝╚═════╝ ╚═╝╚══════╝ ╚══╝╚══╝ ",
];

const MEDIUM: &[&str] = &[
    "╭──────────╮   ████████╗██╗    ██╗",
    "│ >_ ▀▀ ▀▀ │   ╚══██╔══╝██║    ██║",
    "│ ▀▀ ▀▀ ▀▀ │      ██║   ██║ █╗ ██║",
    "│ ▀▀ ▀▀ ▀▀ │      ██║   ██║███╗██║",
    "╰──────────╯      ██║   ╚███╔███╔╝",
    "                  ╚═╝    ╚══╝╚══╝ ",
];

const SMALL: &[&str] = &[
    "╭──────────╮",
    "│ >_ ▀▀ ▀▀ │",
    "│ ▀▀ ▀▀ ▀▀ │",
    "│ ▀▀ ▀▀ ▀▀ │",
    "╰──────────╯",
];

const FALLBACK: &[&str] = &["tabiew"];

#[derive(Debug, Default, Clone, Copy)]
pub struct TabiewLogo;

impl TabiewLogo {
    /// Size of the variant that would be drawn in `area`, if any of them fits.
    pub fn size(area: Rect) -> Option<(u16, u16)> {
        Self::best_fit(area).map(dimensions)
    }

    fn best_fit(area: Rect) -> Option<&'static [&'static str]> {
        [LARGE, MEDIUM, SMALL, FALLBACK].into_iter().find(|art| {
            let (width, height) = dimensions(art);
            width <= area.width && height <= area.height
        })
    }
}

impl Widget for TabiewLogo {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let Some(art) = Self::best_fit(area) else {
            return;
        };
        let (width, height) = dimensions(art);
        let centered = Rect {
            x: area.x + (area.width - width) / 2,
            y: area.y + (area.height - height) / 2,
            width,
            height,
        };
        let theme = theme();
        let style = |ink| match ink {
            Ink::Border => theme.block(),
            Ink::Shadow => theme.subtext(),
            Ink::Body => theme.text(),
        };
        let lines = art
            .iter()
            .map(|line| {
                line.chars()
                    .chunk_by(|chr| ink(*chr))
                    .into_iter()
                    .map(|(ink, chrs)| Span::styled(chrs.collect::<String>(), style(ink)))
                    .collect::<Line>()
            })
            .collect::<Vec<_>>();
        Paragraph::new(lines)
            .style(style(Ink::Body))
            .render(centered, buf);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ink {
    Border,
    Shadow,
    Body,
}

fn ink(chr: char) -> Ink {
    match chr {
        '\u{256d}' | '\u{256e}' | '\u{2570}' | '\u{256f}' | '\u{2500}' | '\u{2502}' => Ink::Border,
        '\u{2550}' | '\u{2551}' | '\u{2554}' | '\u{2557}' | '\u{255a}' | '\u{255d}' => Ink::Shadow,
        _ => Ink::Body,
    }
}

fn dimensions(art: &[&str]) -> (u16, u16) {
    let width = art.iter().map(|line| line.width()).max().unwrap_or(0) as u16;
    (width, art.len() as u16)
}
