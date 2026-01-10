use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Wryan;

impl SixColorsTwoRowsStyler for Wryan {
    const BACKGROUND: Color = Color::from_u32(0x00101010);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00303030);
    const FOREGROUND: Color = Color::from_u32(0x00999993);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00333333);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00bf4d80),
        Color::from_u32(0x0053a6a6),
        Color::from_u32(0x009e9ecb),
        Color::from_u32(0x00477ab3),
        Color::from_u32(0x007e62b3),
        Color::from_u32(0x006096bf),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x008c4665),
        Color::from_u32(0x00287373),
        Color::from_u32(0x007c7c99),
        Color::from_u32(0x00395573),
        Color::from_u32(0x005e468c),
        Color::from_u32(0x0031658c),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00161616), Color::from_u32(0x001C1C1C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x007E7EAB);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x005C1635);
}
