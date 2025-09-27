
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct GreyGreen;

impl SixColorsTwoRowsStyler for GreyGreen {
    const BACKGROUND: Color = Color::from_u32(0x00002a1a);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00204A3A);
    const FOREGROUND: Color = Color::from_u32(0x00ffffff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff3939),
        Color::from_u32(0x0000ff44),
        Color::from_u32(0x00ffd100),
        Color::from_u32(0x0000afff),
        Color::from_u32(0x00ff008a),
        Color::from_u32(0x0000ffd3),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00fe1414),
        Color::from_u32(0x0074ff00),
        Color::from_u32(0x00f1ff01),
        Color::from_u32(0x0000deff),
        Color::from_u32(0x00ff00f0),
        Color::from_u32(0x0000ffbc),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00063020), Color::from_u32(0x000C3626)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DFD400);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF00C0);

    fn id(&self) -> &str {
        "grey_green"
    }

    fn title(&self) -> &str {
        "GreyGreen"
    }
}
