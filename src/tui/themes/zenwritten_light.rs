use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct ZenwrittenLight;

impl SixColorsTwoRowsStyler for ZenwrittenLight {
    const BACKGROUND: Color = Color::from_u32(0x00eeeeee);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x00353535);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00eeeeee);

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
        Color::from_u32(0x004f6c31),
        Color::from_u32(0x00944927),
        Color::from_u32(0x00286486),
        Color::from_u32(0x0088507d),
        Color::from_u32(0x003b8992),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00F4F4F4), Color::from_u32(0x00FAFAFA)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00151515);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x0078031C);
}
