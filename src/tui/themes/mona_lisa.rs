
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct MonaLisa;

impl SixColorsTwoRowsStyler for MonaLisa {
    const BACKGROUND: Color = Color::from_u32(0x00120b0d);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00322B2D);
    const FOREGROUND: Color = Color::from_u32(0x00f7d66a);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00351b0e);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff4331),
        Color::from_u32(0x00b4b264),
        Color::from_u32(0x00ff9566),
        Color::from_u32(0x009eb2b4),
        Color::from_u32(0x00ff5b6a),
        Color::from_u32(0x008acd8f),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x009b291c),
        Color::from_u32(0x00636232),
        Color::from_u32(0x00c36e28),
        Color::from_u32(0x00515c5d),
        Color::from_u32(0x009b1d29),
        Color::from_u32(0x00588056),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00181113), Color::from_u32(0x001E1719)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A44C12);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00933E00);
}
