use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct TokyoNightMoon;

impl SixColorsTwoRowsStyler for TokyoNightMoon {
    const BACKGROUND: Color = Color::from_u32(0x00222436);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00424456);
    const FOREGROUND: Color = Color::from_u32(0x00c8d3f5);
    const DARK_FOREGROUND: Color = Color::from_u32(0x001b1d2b);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff757f),
        Color::from_u32(0x00c3e88d),
        Color::from_u32(0x00ffc777),
        Color::from_u32(0x0082aaff),
        Color::from_u32(0x00c099ff),
        Color::from_u32(0x0086e1fc),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff757f),
        Color::from_u32(0x00c3e88d),
        Color::from_u32(0x00ffc777),
        Color::from_u32(0x0082aaff),
        Color::from_u32(0x00c099ff),
        Color::from_u32(0x0086e1fc),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00282A3C), Color::from_u32(0x002E3042)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A8B3D5);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF9747);

    fn id(&self) -> &str {
        "tokyo_night_moon"
    }

    fn title(&self) -> &str {
        "TokyoNightMoon"
    }
}
