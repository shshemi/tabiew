
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Tearout;

impl SixColorsTwoRowsStyler for Tearout {
    const BACKGROUND: Color = Color::from_u32(0x0034392d);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x0054594D);
    const FOREGROUND: Color = Color::from_u32(0x00f4d2ae);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00685742);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00cc967b),
        Color::from_u32(0x0097976d),
        Color::from_u32(0x006c9861),
        Color::from_u32(0x00b5955e),
        Color::from_u32(0x00c9a554),
        Color::from_u32(0x00d7c483),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00cc967b),
        Color::from_u32(0x0097976d),
        Color::from_u32(0x006c9861),
        Color::from_u32(0x00b5955e),
        Color::from_u32(0x00c9a554),
        Color::from_u32(0x00d7c483),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x003A3F33), Color::from_u32(0x00404539)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00B7A463);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00A79453);
}
