use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Moonfly;

impl SixColorsTwoRowsStyler for Moonfly {
    const BACKGROUND: Color = Color::from_u32(0x00080808);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00282828);
    const FOREGROUND: Color = Color::from_u32(0x00bdbdbd);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00323437);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff5189),
        Color::from_u32(0x0036c692),
        Color::from_u32(0x00c6c684),
        Color::from_u32(0x0074b2ff),
        Color::from_u32(0x00ae81ff),
        Color::from_u32(0x0085dc85),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff5454),
        Color::from_u32(0x008cc85f),
        Color::from_u32(0x00e3c78a),
        Color::from_u32(0x0080a0ff),
        Color::from_u32(0x00cf87e8),
        Color::from_u32(0x0079dac8),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x000E0E0E), Color::from_u32(0x00141414)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x007E7E7E);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF2424);
}
