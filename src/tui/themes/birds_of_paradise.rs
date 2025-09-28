
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct BirdsOfParadise;

impl SixColorsTwoRowsStyler for BirdsOfParadise {
    const BACKGROUND: Color = Color::from_u32(0x002a1f1d);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x004A3F3D);
    const FOREGROUND: Color = Color::from_u32(0x00e0dbb7);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00573d26);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00e84627),
        Color::from_u32(0x0095d8ba),
        Color::from_u32(0x00d0d150),
        Color::from_u32(0x00b8d3ed),
        Color::from_u32(0x00d19ecb),
        Color::from_u32(0x0093cfd7),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00be2d26),
        Color::from_u32(0x006ba18a),
        Color::from_u32(0x00e99d2a),
        Color::from_u32(0x005a86ad),
        Color::from_u32(0x00ac80a6),
        Color::from_u32(0x0074a6ad),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00302523), Color::from_u32(0x00362B29)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00371D06);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B96D00);
}
