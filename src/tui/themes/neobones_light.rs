use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct NeobonesLight;

impl SixColorsTwoRowsStyler for NeobonesLight {
    const BACKGROUND: Color = Color::from_u32(0x00e5ede6);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x00202e18);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00e5ede6);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x0094253e),
        Color::from_u32(0x003f5a22),
        Color::from_u32(0x00803d1c),
        Color::from_u32(0x001d5573),
        Color::from_u32(0x007b3b70),
        Color::from_u32(0x002b747c),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00a8334c),
        Color::from_u32(0x00567a30),
        Color::from_u32(0x00944927),
        Color::from_u32(0x00286486),
        Color::from_u32(0x0088507d),
        Color::from_u32(0x003b8992),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00EBF3EC), Color::from_u32(0x00F1F9F2)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00000E00);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x0078031C);
}
