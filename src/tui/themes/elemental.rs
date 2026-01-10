use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Elemental;

impl SixColorsTwoRowsStyler for Elemental {
    const BACKGROUND: Color = Color::from_u32(0x0022211d);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x0042413D);
    const FOREGROUND: Color = Color::from_u32(0x00807a74);
    const DARK_FOREGROUND: Color = Color::from_u32(0x003c3c30);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00e0502a),
        Color::from_u32(0x0061e070),
        Color::from_u32(0x00d69927),
        Color::from_u32(0x0079d9d9),
        Color::from_u32(0x00cd7c54),
        Color::from_u32(0x0059d599),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x0098290f),
        Color::from_u32(0x00479a43),
        Color::from_u32(0x007f7111),
        Color::from_u32(0x00497f7d),
        Color::from_u32(0x007f4e2f),
        Color::from_u32(0x00387f58),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00282723), Color::from_u32(0x002E2D29)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DAAB60);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00680000);
}
