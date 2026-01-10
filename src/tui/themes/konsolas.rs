use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Konsolas;

impl SixColorsTwoRowsStyler for Konsolas {
    const BACKGROUND: Color = Color::from_u32(0x00060606);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00262626);
    const FOREGROUND: Color = Color::from_u32(0x00c8c1c1);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff4141),
        Color::from_u32(0x005fff5f),
        Color::from_u32(0x00ffff55),
        Color::from_u32(0x004b4bff),
        Color::from_u32(0x00ff54ff),
        Color::from_u32(0x0069ffff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00aa1717),
        Color::from_u32(0x0018b218),
        Color::from_u32(0x00ebae1f),
        Color::from_u32(0x002323a5),
        Color::from_u32(0x00ad1edc),
        Color::from_u32(0x0042b0c8),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x000C0C0C), Color::from_u32(0x00121212)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A8A1A1);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00BB7E00);
}
