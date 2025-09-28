
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Spacegray;

impl SixColorsTwoRowsStyler for Spacegray {
    const BACKGROUND: Color = Color::from_u32(0x0020242d);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x0040444D);
    const FOREGROUND: Color = Color::from_u32(0x00b3b8c3);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00b04b57),
        Color::from_u32(0x0087b379),
        Color::from_u32(0x00e5c179),
        Color::from_u32(0x007d8fa4),
        Color::from_u32(0x00a47996),
        Color::from_u32(0x0085a7a5),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00b04b57),
        Color::from_u32(0x0087b379),
        Color::from_u32(0x00e5c179),
        Color::from_u32(0x007d8fa4),
        Color::from_u32(0x00a47996),
        Color::from_u32(0x0085a7a5),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00262A33), Color::from_u32(0x002C3039)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x009398A3);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B59149);
}
