
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Whimsy;

impl SixColorsTwoRowsStyler for Whimsy {
    const BACKGROUND: Color = Color::from_u32(0x0029283b);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x0049485B);
    const FOREGROUND: Color = Color::from_u32(0x00b3b0d6);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00535178);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ef6487),
        Color::from_u32(0x005eca89),
        Color::from_u32(0x00fdd877),
        Color::from_u32(0x0065aef7),
        Color::from_u32(0x00aa7ff0),
        Color::from_u32(0x0043c1be),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ef6487),
        Color::from_u32(0x005eca89),
        Color::from_u32(0x00fdd877),
        Color::from_u32(0x0065aef7),
        Color::from_u32(0x00aa7ff0),
        Color::from_u32(0x0043c1be),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002F2E41), Color::from_u32(0x00353447)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x009390B6);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CDA847);
}
