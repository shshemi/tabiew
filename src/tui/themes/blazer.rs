
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Blazer;

impl SixColorsTwoRowsStyler for Blazer {
    const BACKGROUND: Color = Color::from_u32(0x000d1926);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x002D3946);
    const FOREGROUND: Color = Color::from_u32(0x00d9e6f2);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00dbbdbd),
        Color::from_u32(0x00bddbbd),
        Color::from_u32(0x00dbdbbd),
        Color::from_u32(0x00bdbddb),
        Color::from_u32(0x00dbbddb),
        Color::from_u32(0x00bddbdb),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00b87a7a),
        Color::from_u32(0x007ab87a),
        Color::from_u32(0x00b8b87a),
        Color::from_u32(0x007a7ab8),
        Color::from_u32(0x00b87ab8),
        Color::from_u32(0x007ab8b8),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00131F2C), Color::from_u32(0x00192532)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00B9C6D2);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00884A4A);
}
