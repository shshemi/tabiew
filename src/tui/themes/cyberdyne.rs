use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Cyberdyne;

impl SixColorsTwoRowsStyler for Cyberdyne {
    const BACKGROUND: Color = Color::from_u32(0x00151144);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00353164);
    const FOREGROUND: Color = Color::from_u32(0x0000ff92);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00080808);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ffc4be),
        Color::from_u32(0x00d6fcba),
        Color::from_u32(0x00fffed5),
        Color::from_u32(0x00c2e3ff),
        Color::from_u32(0x00ffb2fe),
        Color::from_u32(0x00e6e7fe),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff8373),
        Color::from_u32(0x0000c172),
        Color::from_u32(0x00d2a700),
        Color::from_u32(0x000071cf),
        Color::from_u32(0x00ff90fe),
        Color::from_u32(0x006bffdd),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001B174A), Color::from_u32(0x00211D50)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0000DF7C);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF5343);
}
