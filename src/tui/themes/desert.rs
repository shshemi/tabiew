use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Desert;

impl SixColorsTwoRowsStyler for Desert {
    const BACKGROUND: Color = Color::from_u32(0x00333333);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00535353);
    const FOREGROUND: Color = Color::from_u32(0x00ffffff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x004d4d4d);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff5555),
        Color::from_u32(0x0055ff55),
        Color::from_u32(0x00ffff55),
        Color::from_u32(0x0087ceff),
        Color::from_u32(0x00ff55ff),
        Color::from_u32(0x00ffd700),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff2b2b),
        Color::from_u32(0x0098fb98),
        Color::from_u32(0x00f0e68c),
        Color::from_u32(0x00cd853f),
        Color::from_u32(0x00ffdead),
        Color::from_u32(0x00ffa0a0),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00393939), Color::from_u32(0x003F3F3F)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0000DF00);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF0000);
}
