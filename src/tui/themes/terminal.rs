use std::sync::OnceLock;

use ratatui::style::{Color, Modifier, Style};
use terminal_colorsaurus::{QueryOptions, color_palette};

use crate::tui::themes::styler::Styler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Terminal;

type Rgb = (u8, u8, u8);
type Palette = (Rgb, Rgb);

const STRIPE_SHIFT: u8 = 8;
const STRIPE_LIGHTEN: u8 = 16;
const DARK_FOREGROUND_MIX: u32 = 44;
const TAG_BACKGROUND_MIX: u32 = 20;

const COLORS: [Color; 6] = [
    Color::Red,
    Color::Yellow,
    Color::Green,
    Color::Cyan,
    Color::Blue,
    Color::Magenta,
];

impl Styler for Terminal {
    fn table_header(&self) -> Style {
        Style::default()
            .bg(Color::Reset)
            .add_modifier(Modifier::BOLD)
    }

    fn row(&self, row: usize) -> Style {
        Style::default()
            .bg(stripes()[row % stripes().len()])
            .fg(Color::Reset)
    }

    fn row_highlighted(&self) -> Style {
        Style::default()
            .bg(Color::Reset)
            .fg(Color::Reset)
            .add_modifier(Modifier::REVERSED)
    }

    fn header(&self, idx: usize) -> Style {
        Style::default().fg(COLORS[idx % COLORS.len()]).bold()
    }

    fn tag(&self, idx: usize) -> Style {
        Style::default()
            .bg(COLORS[idx % COLORS.len()])
            .fg(tag_background())
    }

    fn block_tag(&self) -> Style {
        Style::default().bg(Color::Yellow).fg(tag_background())
    }

    fn block(&self) -> Style {
        Style::default().bg(Color::Reset).fg(Color::Yellow)
    }

    fn text(&self) -> Style {
        Style::default().bg(Color::Reset).fg(Color::Reset)
    }

    fn subtext(&self) -> Style {
        Style::default().bg(Color::Reset).fg(dark_foreground())
    }

    fn error(&self) -> Style {
        Style::default()
            .bg(Color::Red)
            .fg(Color::Reset)
            .add_modifier(Modifier::REVERSED)
    }

    fn graph(&self, idx: usize) -> Style {
        Style::default().fg(COLORS[idx % COLORS.len()]).bold()
    }

    fn text_highlighted(&self) -> Style {
        Style::default()
            .bg(Color::Reset)
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    }

    fn gutter(&self, idx: usize) -> Style {
        Style::default()
            .bg(stripes()[idx % stripes().len()])
            .fg(dark_foreground())
    }

    fn background(&self) -> Style {
        Style::default().bg(Color::Black)
    }
}

#[inline]
fn stripes() -> &'static [Color; 2] {
    static STRIPES: OnceLock<[Color; 2]> = OnceLock::new();
    STRIPES.get_or_init(|| {
        let Some((r, g, b)) = background() else {
            return [Color::Reset; 2];
        };
        let luma = luma(r, g, b);
        [
            Color::Rgb(
                shift(r, luma, STRIPE_SHIFT),
                shift(g, luma, STRIPE_SHIFT),
                shift(b, luma, STRIPE_SHIFT),
            ),
            Color::Rgb(
                shift(r, luma, STRIPE_LIGHTEN),
                shift(g, luma, STRIPE_LIGHTEN),
                shift(b, luma, STRIPE_LIGHTEN),
            ),
        ]
    })
}

#[inline]
fn dark_foreground() -> Color {
    static DARK_FOREGROUND: OnceLock<Color> = OnceLock::new();
    *DARK_FOREGROUND.get_or_init(|| {
        let Some((foreground, background)) = palette() else {
            return Color::Reset;
        };
        Color::Rgb(
            mix(foreground.0, background.0, DARK_FOREGROUND_MIX),
            mix(foreground.1, background.1, DARK_FOREGROUND_MIX),
            mix(foreground.2, background.2, DARK_FOREGROUND_MIX),
        )
    })
}

#[inline]
fn tag_background() -> Color {
    static TAG_BACKGROUND: OnceLock<Color> = OnceLock::new();
    *TAG_BACKGROUND.get_or_init(|| {
        let Some((foreground, background)) = palette() else {
            return Color::Reset;
        };
        Color::Rgb(
            mix(background.0, foreground.0, TAG_BACKGROUND_MIX),
            mix(background.1, foreground.1, TAG_BACKGROUND_MIX),
            mix(background.2, foreground.2, TAG_BACKGROUND_MIX),
        )
    })
}

fn background() -> Option<Rgb> {
    palette().map(|(_, background)| background)
}

fn palette() -> &'static Option<Palette> {
    static PALETTE: OnceLock<Option<Palette>> = OnceLock::new();
    PALETTE.get_or_init(|| {
        let palette = color_palette(QueryOptions::default()).ok()?;
        Some((
            palette.foreground.scale_to_8bit(),
            palette.background.scale_to_8bit(),
        ))
    })
}

fn luma(r: u8, g: u8, b: u8) -> u32 {
    (r as u32 * 299 + g as u32 * 587 + b as u32 * 114) / 1000
}

fn mix(from: u8, to: u8, percent: u32) -> u8 {
    ((from as u32 * (100 - percent) + to as u32 * percent) / 100) as u8
}

fn shift(value: u8, luma: u32, amount: u8) -> u8 {
    if luma < 128 {
        value.saturating_add(amount)
    } else {
        value.saturating_sub(amount)
    }
}
