
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct IcGreenPpl;

impl SixColorsTwoRowsStyler for IcGreenPpl {
    const BACKGROUND: Color = Color::from_u32(0x002c2c2c);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x004C4C4C);
    const FOREGROUND: Color = Color::from_u32(0x00e0f1dc);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00014401);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00b4fa5c),
        Color::from_u32(0x00aefb86),
        Color::from_u32(0x00dafa87),
        Color::from_u32(0x002efaeb),
        Color::from_u32(0x0050fafa),
        Color::from_u32(0x003cfac8),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff2736),
        Color::from_u32(0x0041a638),
        Color::from_u32(0x0076a831),
        Color::from_u32(0x002ec3b9),
        Color::from_u32(0x0050a096),
        Color::from_u32(0x003ca078),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00323232), Color::from_u32(0x00383838)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0027DA4B);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF0006);
}
