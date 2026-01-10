use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Flatland;

impl SixColorsTwoRowsStyler for Flatland {
    const BACKGROUND: Color = Color::from_u32(0x001d1f21);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003D3F41);
    const FOREGROUND: Color = Color::from_u32(0x00b8dbef);
    const DARK_FOREGROUND: Color = Color::from_u32(0x001d1d19);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00d22a24),
        Color::from_u32(0x00a7d42c),
        Color::from_u32(0x00ff8949),
        Color::from_u32(0x0061b9d0),
        Color::from_u32(0x00695abc),
        Color::from_u32(0x00d63865),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f18339),
        Color::from_u32(0x009fd364),
        Color::from_u32(0x00f4ef6d),
        Color::from_u32(0x005096be),
        Color::from_u32(0x00695abc),
        Color::from_u32(0x00d63865),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00232527), Color::from_u32(0x00292B2D)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00506264);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C4BF3D);
}
