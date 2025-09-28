
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Floraverse;

impl SixColorsTwoRowsStyler for Floraverse {
    const BACKGROUND: Color = Color::from_u32(0x000e0d15);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x002E2D35);
    const FOREGROUND: Color = Color::from_u32(0x00dbd1b9);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0008002e);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00d02063),
        Color::from_u32(0x00b4ce59),
        Color::from_u32(0x00fac357),
        Color::from_u32(0x0040a4cf),
        Color::from_u32(0x00f12aae),
        Color::from_u32(0x0062caa8),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x0064002c),
        Color::from_u32(0x005d731a),
        Color::from_u32(0x00cd751c),
        Color::from_u32(0x001d6da1),
        Color::from_u32(0x00b7077e),
        Color::from_u32(0x0042a38c),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0014131B), Color::from_u32(0x001A1921)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x009B9B9B);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x009D4500);
}
