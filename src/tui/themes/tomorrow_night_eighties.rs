use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct TomorrowNightEighties;

impl SixColorsTwoRowsStyler for TomorrowNightEighties {
    const BACKGROUND: Color = Color::from_u32(0x002d2d2d);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x004D4D4D);
    const FOREGROUND: Color = Color::from_u32(0x00cccccc);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f2777a),
        Color::from_u32(0x0099cc99),
        Color::from_u32(0x00ffcc66),
        Color::from_u32(0x006699cc),
        Color::from_u32(0x00cc99cc),
        Color::from_u32(0x0066cccc),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f2777a),
        Color::from_u32(0x0099cc99),
        Color::from_u32(0x00ffcc66),
        Color::from_u32(0x006699cc),
        Color::from_u32(0x00cc99cc),
        Color::from_u32(0x0066cccc),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00333333), Color::from_u32(0x00393939)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00ACACAC);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF9C36);
}
