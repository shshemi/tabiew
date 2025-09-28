
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Obsidian;

impl SixColorsTwoRowsStyler for Obsidian {
    const BACKGROUND: Color = Color::from_u32(0x00283033);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00485053);
    const FOREGROUND: Color = Color::from_u32(0x00cdcdcd);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff0003),
        Color::from_u32(0x0093c863),
        Color::from_u32(0x00fef874),
        Color::from_u32(0x00a1d7ff),
        Color::from_u32(0x00ff55ff),
        Color::from_u32(0x0055ffff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00a60001),
        Color::from_u32(0x0000bb00),
        Color::from_u32(0x00fecd22),
        Color::from_u32(0x003a9bdb),
        Color::from_u32(0x00bb00bb),
        Color::from_u32(0x0000bbbb),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002E3639), Color::from_u32(0x00343C3F)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A0AAB0);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CE9D00);
}
