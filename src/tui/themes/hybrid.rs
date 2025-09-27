
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Hybrid;

impl SixColorsTwoRowsStyler for Hybrid {
    const BACKGROUND: Color = Color::from_u32(0x00161719);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00363739);
    const FOREGROUND: Color = Color::from_u32(0x00b7bcba);
    const DARK_FOREGROUND: Color = Color::from_u32(0x002a2e33);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x008d2e32),
        Color::from_u32(0x00798431),
        Color::from_u32(0x00e58a50),
        Color::from_u32(0x004b6b88),
        Color::from_u32(0x006e5079),
        Color::from_u32(0x004d7b74),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00b84d51),
        Color::from_u32(0x00b3bf5a),
        Color::from_u32(0x00e4b55e),
        Color::from_u32(0x006e90b0),
        Color::from_u32(0x00a17eac),
        Color::from_u32(0x007fbfb4),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001C1D1F), Color::from_u32(0x00222325)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00979C9A);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B4852E);

    fn id(&self) -> &str {
        "hybrid"
    }

    fn title(&self) -> &str {
        "Hybrid"
    }
}
