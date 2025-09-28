
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct CursorDark;

impl SixColorsTwoRowsStyler for CursorDark {
    const BACKGROUND: Color = Color::from_u32(0x00141414);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00343434);
    const FOREGROUND: Color = Color::from_u32(0x00ffffff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x002a2a2a);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00bf616a),
        Color::from_u32(0x00a3be8c),
        Color::from_u32(0x00ebcb8b),
        Color::from_u32(0x0081a1c1),
        Color::from_u32(0x00b48ead),
        Color::from_u32(0x0088c0d0),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00bf616a),
        Color::from_u32(0x00a3be8c),
        Color::from_u32(0x00ebcb8b),
        Color::from_u32(0x0081a1c1),
        Color::from_u32(0x00b48ead),
        Color::from_u32(0x0088c0d0),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001A1A1A), Color::from_u32(0x00202020)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DFDFDF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00BB9B5B);
}
