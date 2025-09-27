
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Breeze;

impl SixColorsTwoRowsStyler for Breeze {
    const BACKGROUND: Color = Color::from_u32(0x0031363b);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x0051565B);
    const FOREGROUND: Color = Color::from_u32(0x00eff0f1);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0031363b);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00c0392b),
        Color::from_u32(0x001cdc9a),
        Color::from_u32(0x00fdbc4b),
        Color::from_u32(0x003daee9),
        Color::from_u32(0x008e44ad),
        Color::from_u32(0x0016a085),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ed1515),
        Color::from_u32(0x0011d116),
        Color::from_u32(0x00f67400),
        Color::from_u32(0x001d99f3),
        Color::from_u32(0x009b59b6),
        Color::from_u32(0x001abc9c),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00373C41), Color::from_u32(0x003D4247)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00CFD0D1);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C64400);

    fn id(&self) -> &str {
        "breeze"
    }

    fn title(&self) -> &str {
        "Breeze"
    }
}
