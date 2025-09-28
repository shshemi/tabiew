
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct XcodeLight;

impl SixColorsTwoRowsStyler for XcodeLight {
    const BACKGROUND: Color = Color::from_u32(0x00ffffff);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x00262626);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00b4d8fd);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00d12f1b),
        Color::from_u32(0x0023575c),
        Color::from_u32(0x0078492a),
        Color::from_u32(0x000b4f79),
        Color::from_u32(0x00ad3da4),
        Color::from_u32(0x004b21b0),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00d12f1b),
        Color::from_u32(0x003e8087),
        Color::from_u32(0x0078492a),
        Color::from_u32(0x000f68a0),
        Color::from_u32(0x00ad3da4),
        Color::from_u32(0x00804fb8),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FFFFFF), Color::from_u32(0x00FFFFFF)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00060606);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00A10000);
}
