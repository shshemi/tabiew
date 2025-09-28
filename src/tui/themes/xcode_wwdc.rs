
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct XcodeWwdc;

impl SixColorsTwoRowsStyler for XcodeWwdc {
    const BACKGROUND: Color = Color::from_u32(0x00292c36);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00494C56);
    const FOREGROUND: Color = Color::from_u32(0x00e7e8eb);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00494d5c);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00bb383a),
        Color::from_u32(0x0094c66e),
        Color::from_u32(0x00d28e5d),
        Color::from_u32(0x008884c5),
        Color::from_u32(0x00b73999),
        Color::from_u32(0x0000aba4),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00bb383a),
        Color::from_u32(0x0094c66e),
        Color::from_u32(0x00d28e5d),
        Color::from_u32(0x008884c5),
        Color::from_u32(0x00b73999),
        Color::from_u32(0x0000aba4),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002F323C), Color::from_u32(0x00353842)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00C7C8CB);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00A25E2D);
}
