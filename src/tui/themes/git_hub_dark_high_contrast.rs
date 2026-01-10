use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct GithubDarkHighContrast;

impl SixColorsTwoRowsStyler for GithubDarkHighContrast {
    const BACKGROUND: Color = Color::from_u32(0x000a0c10);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x002A2C30);
    const FOREGROUND: Color = Color::from_u32(0x00f0f3f6);
    const DARK_FOREGROUND: Color = Color::from_u32(0x007a828e);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ffb1af),
        Color::from_u32(0x004ae168),
        Color::from_u32(0x00f7c843),
        Color::from_u32(0x0091cbff),
        Color::from_u32(0x00dbb7ff),
        Color::from_u32(0x0056d4dd),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff9492),
        Color::from_u32(0x0026cd4d),
        Color::from_u32(0x00f0b72f),
        Color::from_u32(0x0071b7ff),
        Color::from_u32(0x00cb9eff),
        Color::from_u32(0x0039c5cf),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00101216), Color::from_u32(0x0016181C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x005197DF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF6462);
}
