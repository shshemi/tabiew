
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct TangoAdapted;

impl SixColorsTwoRowsStyler for TangoAdapted {
    const BACKGROUND: Color = Color::from_u32(0x00ffffff);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x00000000);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff0013),
        Color::from_u32(0x0093ff00),
        Color::from_u32(0x00fff121),
        Color::from_u32(0x0088c9ff),
        Color::from_u32(0x00e9a7e1),
        Color::from_u32(0x0000feff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff0000),
        Color::from_u32(0x0059d600),
        Color::from_u32(0x00f0cb00),
        Color::from_u32(0x0000a2ff),
        Color::from_u32(0x00c17ecc),
        Color::from_u32(0x0000d0d6),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FFFFFF), Color::from_u32(0x00FFFFFF)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00000000);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF0000);

    fn id(&self) -> &str {
        "tango_adapted"
    }

    fn title(&self) -> &str {
        "TangoAdapted"
    }
}
