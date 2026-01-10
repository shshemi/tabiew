use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct BlueMatrix;

impl SixColorsTwoRowsStyler for BlueMatrix {
    const BACKGROUND: Color = Color::from_u32(0x00101116);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00303136);
    const FOREGROUND: Color = Color::from_u32(0x0000a2ff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00101116);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff6e67),
        Color::from_u32(0x005ffa68),
        Color::from_u32(0x00fffc67),
        Color::from_u32(0x006871ff),
        Color::from_u32(0x00d682ec),
        Color::from_u32(0x0060fdff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff5680),
        Color::from_u32(0x0000ff9c),
        Color::from_u32(0x00fffc58),
        Color::from_u32(0x0000b0ff),
        Color::from_u32(0x00d57bff),
        Color::from_u32(0x0076c1ff),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0016171C), Color::from_u32(0x001C1D22)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0056DF7F);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CFCC28);
}
