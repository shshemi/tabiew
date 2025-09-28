
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Nord;

impl SixColorsTwoRowsStyler for Nord {
    const BACKGROUND: Color = Color::from_u32(0x002e3440);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x004E5460);
    const FOREGROUND: Color = Color::from_u32(0x00d8dee9);
    const DARK_FOREGROUND: Color = Color::from_u32(0x003b4252);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00bf616a),
        Color::from_u32(0x00a3be8c),
        Color::from_u32(0x00ebcb8b),
        Color::from_u32(0x0081a1c1),
        Color::from_u32(0x00b48ead),
        Color::from_u32(0x008fbcbb),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00bf616a),
        Color::from_u32(0x00a3be8c),
        Color::from_u32(0x00ebcb8b),
        Color::from_u32(0x0081a1c1),
        Color::from_u32(0x00b48ead),
        Color::from_u32(0x0088c0d0),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00343A46), Color::from_u32(0x003A404C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00CCCFD4);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00BB9B5B);
}
