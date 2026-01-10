use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct MonokaiRemastered;

impl SixColorsTwoRowsStyler for MonokaiRemastered {
    const BACKGROUND: Color = Color::from_u32(0x000c0c0c);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x002C2C2C);
    const FOREGROUND: Color = Color::from_u32(0x00d9d9d9);
    const DARK_FOREGROUND: Color = Color::from_u32(0x001a1a1a);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f4005f),
        Color::from_u32(0x0098e024),
        Color::from_u32(0x00e0d561),
        Color::from_u32(0x009d65ff),
        Color::from_u32(0x00f4005f),
        Color::from_u32(0x0058d1eb),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f4005f),
        Color::from_u32(0x0098e024),
        Color::from_u32(0x00fd971f),
        Color::from_u32(0x009d65ff),
        Color::from_u32(0x00f4005f),
        Color::from_u32(0x0058d1eb),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00121212), Color::from_u32(0x00181818)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DC7700);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CD6700);
}
