use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Pnevma;

impl SixColorsTwoRowsStyler for Pnevma {
    const BACKGROUND: Color = Color::from_u32(0x001c1c1c);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003C3C3C);
    const FOREGROUND: Color = Color::from_u32(0x00d0d0d0);
    const DARK_FOREGROUND: Color = Color::from_u32(0x002f2e2d);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00d78787),
        Color::from_u32(0x00afbea2),
        Color::from_u32(0x00e4c9af),
        Color::from_u32(0x00a1bdce),
        Color::from_u32(0x00d7beda),
        Color::from_u32(0x00b1e7dd),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00a36666),
        Color::from_u32(0x0090a57d),
        Color::from_u32(0x00d7af87),
        Color::from_u32(0x007fa5bd),
        Color::from_u32(0x00c79ec4),
        Color::from_u32(0x008adbb4),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00222222), Color::from_u32(0x00282828)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00C4A98F);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00A77F57);
}
