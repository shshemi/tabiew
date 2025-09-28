
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct AardvarkBlue;

impl SixColorsTwoRowsStyler for AardvarkBlue {
    const BACKGROUND: Color = Color::from_u32(0x00102040);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00304060);
    const FOREGROUND: Color = Color::from_u32(0x00dddddd);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00191919);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f05b50),
        Color::from_u32(0x0095dc55),
        Color::from_u32(0x00ffe763),
        Color::from_u32(0x0060a4ec),
        Color::from_u32(0x00e26be2),
        Color::from_u32(0x0060b6cb),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00aa342e),
        Color::from_u32(0x004b8c0f),
        Color::from_u32(0x00dbba00),
        Color::from_u32(0x001370d3),
        Color::from_u32(0x00c43ac3),
        Color::from_u32(0x00008eb0),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00162646), Color::from_u32(0x001C2C4C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00005AAC);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00AB8A00);
}
