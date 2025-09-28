
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Kanagawabones;

impl SixColorsTwoRowsStyler for Kanagawabones {
    const BACKGROUND: Color = Color::from_u32(0x001f1f28);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003F3F48);
    const FOREGROUND: Color = Color::from_u32(0x00ddd8bb);
    const DARK_FOREGROUND: Color = Color::from_u32(0x001f1f28);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ec818c),
        Color::from_u32(0x009ec967),
        Color::from_u32(0x00f1c982),
        Color::from_u32(0x007bc2df),
        Color::from_u32(0x00a98fd2),
        Color::from_u32(0x007bc2df),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00e46a78),
        Color::from_u32(0x0098bc6d),
        Color::from_u32(0x00e5c283),
        Color::from_u32(0x007eb3c9),
        Color::from_u32(0x00957fb8),
        Color::from_u32(0x007eb3c9),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0025252E), Color::from_u32(0x002B2B34)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00C6C0A2);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B59253);
}
