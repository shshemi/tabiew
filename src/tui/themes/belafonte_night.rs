
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct BelafonteNight;

impl SixColorsTwoRowsStyler for BelafonteNight {
    const BACKGROUND: Color = Color::from_u32(0x0020111b);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x0040313B);
    const FOREGROUND: Color = Color::from_u32(0x00968c83);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0020111b);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00be100e),
        Color::from_u32(0x00858162),
        Color::from_u32(0x00eaa549),
        Color::from_u32(0x00426a79),
        Color::from_u32(0x0097522c),
        Color::from_u32(0x00989a9c),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00be100e),
        Color::from_u32(0x00858162),
        Color::from_u32(0x00eaa549),
        Color::from_u32(0x00426a79),
        Color::from_u32(0x0097522c),
        Color::from_u32(0x00989a9c),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00261721), Color::from_u32(0x002C1D27)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00766C63);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00BA7519);
}
