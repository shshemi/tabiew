
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Wilmersdorf;

impl SixColorsTwoRowsStyler for Wilmersdorf {
    const BACKGROUND: Color = Color::from_u32(0x00282b33);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00484B53);
    const FOREGROUND: Color = Color::from_u32(0x00c6c6c6);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0034373e);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00fa7193),
        Color::from_u32(0x008fd7d6),
        Color::from_u32(0x00d1dfff),
        Color::from_u32(0x00b2cff0),
        Color::from_u32(0x00efccfd),
        Color::from_u32(0x0069abc5),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00e06383),
        Color::from_u32(0x007ebebd),
        Color::from_u32(0x00cccccc),
        Color::from_u32(0x00a6c1e0),
        Color::from_u32(0x00e1c1ee),
        Color::from_u32(0x005b94ab),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002E3139), Color::from_u32(0x0034373F)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x005E9E9D);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B191BE);
}
