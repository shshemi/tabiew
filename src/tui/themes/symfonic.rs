use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Symfonic;

impl SixColorsTwoRowsStyler for Symfonic {
    const BACKGROUND: Color = Color::from_u32(0x00000000);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00202020);
    const FOREGROUND: Color = Color::from_u32(0x00ffffff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00dc322f),
        Color::from_u32(0x0056db3a),
        Color::from_u32(0x00ff8400),
        Color::from_u32(0x000084d4),
        Color::from_u32(0x00b729d9),
        Color::from_u32(0x00ccccff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00dc322f),
        Color::from_u32(0x0056db3a),
        Color::from_u32(0x00ff8400),
        Color::from_u32(0x000084d4),
        Color::from_u32(0x00b729d9),
        Color::from_u32(0x00ccccff),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00060606), Color::from_u32(0x000C0C0C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00BC120F);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF5400);
}
