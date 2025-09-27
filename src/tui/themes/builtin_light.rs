
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct BuiltinLight;

impl SixColorsTwoRowsStyler for BuiltinLight {
    const BACKGROUND: Color = Color::from_u32(0x00ffffff);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x00000000);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff5555),
        Color::from_u32(0x0055ff55),
        Color::from_u32(0x00ffff55),
        Color::from_u32(0x005555ff),
        Color::from_u32(0x00ff55ff),
        Color::from_u32(0x0055ffff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00bb0000),
        Color::from_u32(0x0000bb00),
        Color::from_u32(0x00bbbb00),
        Color::from_u32(0x000000bb),
        Color::from_u32(0x00bb00bb),
        Color::from_u32(0x0000bbbb),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FFFFFF), Color::from_u32(0x00FFFFFF)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00000000);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x008B0000);

    fn id(&self) -> &str {
        "builtin_light"
    }

    fn title(&self) -> &str {
        "BuiltinLight"
    }
}
