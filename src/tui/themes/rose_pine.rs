use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct RosePine;

impl SixColorsTwoRowsStyler for RosePine {
    const BACKGROUND: Color = Color::from_u32(0x00191724);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00393744);
    const FOREGROUND: Color = Color::from_u32(0x00e0def4);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0026233a);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00eb6f92),
        Color::from_u32(0x0031748f),
        Color::from_u32(0x00f6c177),
        Color::from_u32(0x009ccfd8),
        Color::from_u32(0x00c4a7e7),
        Color::from_u32(0x00ebbcba),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00eb6f92),
        Color::from_u32(0x0031748f),
        Color::from_u32(0x00f6c177),
        Color::from_u32(0x009ccfd8),
        Color::from_u32(0x00c4a7e7),
        Color::from_u32(0x00ebbcba),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001F1D2A), Color::from_u32(0x00252330)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00C0BED4);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C69147);
}
