
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct RosePineMoon;

impl SixColorsTwoRowsStyler for RosePineMoon {
    const BACKGROUND: Color = Color::from_u32(0x00232136);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00434156);
    const FOREGROUND: Color = Color::from_u32(0x00e0def4);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00393552);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00eb6f92),
        Color::from_u32(0x003e8fb0),
        Color::from_u32(0x00f6c177),
        Color::from_u32(0x009ccfd8),
        Color::from_u32(0x00c4a7e7),
        Color::from_u32(0x00ea9a97),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00eb6f92),
        Color::from_u32(0x003e8fb0),
        Color::from_u32(0x00f6c177),
        Color::from_u32(0x009ccfd8),
        Color::from_u32(0x00c4a7e7),
        Color::from_u32(0x00ea9a97),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0029273C), Color::from_u32(0x002F2D42)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00C0BED4);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C69147);
}
