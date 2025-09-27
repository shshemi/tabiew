
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct TangoHalfAdapted;

impl SixColorsTwoRowsStyler for TangoHalfAdapted {
    const BACKGROUND: Color = Color::from_u32(0x00ffffff);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x00000000);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff0013),
        Color::from_u32(0x008af600),
        Color::from_u32(0x00ffec00),
        Color::from_u32(0x0076bfff),
        Color::from_u32(0x00d898d1),
        Color::from_u32(0x0000f6fa),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff0000),
        Color::from_u32(0x004cc300),
        Color::from_u32(0x00e2c000),
        Color::from_u32(0x00008ef6),
        Color::from_u32(0x00a96cb3),
        Color::from_u32(0x0000bdc3),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FFFFFF), Color::from_u32(0x00FFFFFF)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00000000);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF0000);

    fn id(&self) -> &str {
        "tango_half_adapted"
    }

    fn title(&self) -> &str {
        "TangoHalfAdapted"
    }
}
