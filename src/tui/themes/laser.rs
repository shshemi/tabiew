use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Laser;

impl SixColorsTwoRowsStyler for Laser {
    const BACKGROUND: Color = Color::from_u32(0x00030d18);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00232D38);
    const FOREGROUND: Color = Color::from_u32(0x00f106e3);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00626262);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ffc4be),
        Color::from_u32(0x00d6fcba),
        Color::from_u32(0x00fffed5),
        Color::from_u32(0x00f92883),
        Color::from_u32(0x00ffb2fe),
        Color::from_u32(0x00e6e7fe),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff8373),
        Color::from_u32(0x00b4fb73),
        Color::from_u32(0x0009b4bd),
        Color::from_u32(0x00fed300),
        Color::from_u32(0x00ff90fe),
        Color::from_u32(0x00d1d1fe),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0009131E), Color::from_u32(0x000F1924)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0000DF7C);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF5343);
}
