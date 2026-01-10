use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct AyuMirage;

impl SixColorsTwoRowsStyler for AyuMirage {
    const BACKGROUND: Color = Color::from_u32(0x001f2430);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003F4450);
    const FOREGROUND: Color = Color::from_u32(0x00cbccc6);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00191e2a);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f28779),
        Color::from_u32(0x00bae67e),
        Color::from_u32(0x00ffd580),
        Color::from_u32(0x0073d0ff),
        Color::from_u32(0x00d4bfff),
        Color::from_u32(0x0095e6cb),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ed8274),
        Color::from_u32(0x00a6cc70),
        Color::from_u32(0x00fad07b),
        Color::from_u32(0x006dcbfa),
        Color::from_u32(0x00cfbafa),
        Color::from_u32(0x0090e1c6),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00252A36), Color::from_u32(0x002B303C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DFAC46);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CAA04B);
}
