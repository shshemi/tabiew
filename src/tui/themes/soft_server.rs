use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct SoftServer;

impl SixColorsTwoRowsStyler for SoftServer {
    const BACKGROUND: Color = Color::from_u32(0x00242626);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00444646);
    const FOREGROUND: Color = Color::from_u32(0x0099a3a2);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00dd5c60),
        Color::from_u32(0x00bfdf55),
        Color::from_u32(0x00deb360),
        Color::from_u32(0x0062b1df),
        Color::from_u32(0x00606edf),
        Color::from_u32(0x0064e39c),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00a2686a),
        Color::from_u32(0x009aa56a),
        Color::from_u32(0x00a3906a),
        Color::from_u32(0x006b8fa3),
        Color::from_u32(0x006a71a3),
        Color::from_u32(0x006ba58f),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002A2C2C), Color::from_u32(0x00303232)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00B2C0BE);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x0073603A);
}
