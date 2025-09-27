
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Highway;

impl SixColorsTwoRowsStyler for Highway {
    const BACKGROUND: Color = Color::from_u32(0x00222225);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00424245);
    const FOREGROUND: Color = Color::from_u32(0x00ededed);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f07e18),
        Color::from_u32(0x00b1d130),
        Color::from_u32(0x00fff120),
        Color::from_u32(0x004fc2fd),
        Color::from_u32(0x00de0071),
        Color::from_u32(0x005d504a),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00d00e18),
        Color::from_u32(0x00138034),
        Color::from_u32(0x00ffcb3e),
        Color::from_u32(0x00006bb3),
        Color::from_u32(0x006b2775),
        Color::from_u32(0x00384564),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0028282B), Color::from_u32(0x002E2E31)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00C0B999);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF9B0E);

    fn id(&self) -> &str {
        "highway"
    }

    fn title(&self) -> &str {
        "Highway"
    }
}
