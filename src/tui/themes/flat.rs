use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Flat;

impl SixColorsTwoRowsStyler for Flat {
    const BACKGROUND: Color = Color::from_u32(0x00002240);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00204260);
    const FOREGROUND: Color = Color::from_u32(0x002cc55d);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00222d3f);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00d4312e),
        Color::from_u32(0x002d9440),
        Color::from_u32(0x00e5be0c),
        Color::from_u32(0x003c7dd2),
        Color::from_u32(0x008230a7),
        Color::from_u32(0x0035b387),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00a82320),
        Color::from_u32(0x0032a548),
        Color::from_u32(0x00e58d11),
        Color::from_u32(0x003167ac),
        Color::from_u32(0x00781aa0),
        Color::from_u32(0x002c9370),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00062846), Color::from_u32(0x000C2E4C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00C59E00);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B55D00);
}
