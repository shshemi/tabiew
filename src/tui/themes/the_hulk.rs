use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct TheHulk;

impl SixColorsTwoRowsStyler for TheHulk {
    const BACKGROUND: Color = Color::from_u32(0x001b1d1e);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003B3D3E);
    const FOREGROUND: Color = Color::from_u32(0x00b5b5b5);
    const DARK_FOREGROUND: Color = Color::from_u32(0x001b1d1e);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x008dff2a),
        Color::from_u32(0x0048ff77),
        Color::from_u32(0x003afe16),
        Color::from_u32(0x00506b95),
        Color::from_u32(0x0072589d),
        Color::from_u32(0x004085a6),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00269d1b),
        Color::from_u32(0x0013ce30),
        Color::from_u32(0x0063e457),
        Color::from_u32(0x002525f5),
        Color::from_u32(0x00641f74),
        Color::from_u32(0x00378ca9),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00212324), Color::from_u32(0x0027292A)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00009600);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00340044);
}
