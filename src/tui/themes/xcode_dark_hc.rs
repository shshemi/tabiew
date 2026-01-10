use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct XcodeDarkHc;

impl SixColorsTwoRowsStyler for XcodeDarkHc {
    const BACKGROUND: Color = Color::from_u32(0x001f1f24);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003F3F44);
    const FOREGROUND: Color = Color::from_u32(0x00ffffff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0043454b);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff8a7a),
        Color::from_u32(0x00b1faeb),
        Color::from_u32(0x00ffa14f),
        Color::from_u32(0x006bdfff),
        Color::from_u32(0x00ff85b8),
        Color::from_u32(0x00e5cfff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff8a7a),
        Color::from_u32(0x0083c9bc),
        Color::from_u32(0x00d9c668),
        Color::from_u32(0x004ec4e6),
        Color::from_u32(0x00ff85b8),
        Color::from_u32(0x00cda1ff),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0025252A), Color::from_u32(0x002B2B30)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DFDFDF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF5A4A);
}
