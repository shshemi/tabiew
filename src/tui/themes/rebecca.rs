
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Rebecca;

impl SixColorsTwoRowsStyler for Rebecca {
    const BACKGROUND: Color = Color::from_u32(0x00292a44);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00494A64);
    const FOREGROUND: Color = Color::from_u32(0x00e8e6ed);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0012131e);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff92cd),
        Color::from_u32(0x0001eac0),
        Color::from_u32(0x00fffca8),
        Color::from_u32(0x0069c0fa),
        Color::from_u32(0x00c17ff8),
        Color::from_u32(0x008bfde1),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00dd7755),
        Color::from_u32(0x0004dbb5),
        Color::from_u32(0x00f2e7b7),
        Color::from_u32(0x007aa5ff),
        Color::from_u32(0x00bf9cf9),
        Color::from_u32(0x0056d3c2),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002F304A), Color::from_u32(0x00353650)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00987BD9);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C2B787);
}
