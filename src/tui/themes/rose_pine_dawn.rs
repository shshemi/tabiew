use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct RosePineDawn;

impl SixColorsTwoRowsStyler for RosePineDawn {
    const BACKGROUND: Color = Color::from_u32(0x00faf4ed);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x00575279);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00f2e9e1);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00b4637a),
        Color::from_u32(0x00286983),
        Color::from_u32(0x00ea9d34),
        Color::from_u32(0x0056949f),
        Color::from_u32(0x00907aa9),
        Color::from_u32(0x00d7827e),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00b4637a),
        Color::from_u32(0x00286983),
        Color::from_u32(0x00ea9d34),
        Color::from_u32(0x0056949f),
        Color::from_u32(0x00907aa9),
        Color::from_u32(0x00d7827e),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FFFAF3), Color::from_u32(0x00FFFFF9)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00373259);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00BA6D04);
}
