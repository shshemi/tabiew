
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct ForestBlue;

impl SixColorsTwoRowsStyler for ForestBlue {
    const BACKGROUND: Color = Color::from_u32(0x00051519);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00253539);
    const FOREGROUND: Color = Color::from_u32(0x00e2d8cd);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00333333);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00fb3d66),
        Color::from_u32(0x006bb48d),
        Color::from_u32(0x0030c85a),
        Color::from_u32(0x0039a7a2),
        Color::from_u32(0x007e62b3),
        Color::from_u32(0x006096bf),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f8818e),
        Color::from_u32(0x0092d3a2),
        Color::from_u32(0x001a8e63),
        Color::from_u32(0x008ed0ce),
        Color::from_u32(0x005e468c),
        Color::from_u32(0x0031658c),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x000B1B1F), Color::from_u32(0x00112125)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x007E7EAB);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C8515E);
}
