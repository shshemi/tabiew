use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct AlienBlood;

impl SixColorsTwoRowsStyler for AlienBlood {
    const BACKGROUND: Color = Color::from_u32(0x000f1610);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x002F3630);
    const FOREGROUND: Color = Color::from_u32(0x00637d75);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00112616);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00e08009),
        Color::from_u32(0x0018e000),
        Color::from_u32(0x00bde000),
        Color::from_u32(0x0000aae0),
        Color::from_u32(0x000058e0),
        Color::from_u32(0x0000e0c4),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x007f2b27),
        Color::from_u32(0x002f7e25),
        Color::from_u32(0x00717f24),
        Color::from_u32(0x002f6a7f),
        Color::from_u32(0x0047587f),
        Color::from_u32(0x00327f77),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00151C16), Color::from_u32(0x001B221C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0053DA71);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x004F0000);
}
