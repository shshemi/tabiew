
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Wez;

impl SixColorsTwoRowsStyler for Wez {
    const BACKGROUND: Color = Color::from_u32(0x00000000);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00202020);
    const FOREGROUND: Color = Color::from_u32(0x00b3b3b3);
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
        Color::from_u32(0x00cc5555),
        Color::from_u32(0x0055cc55),
        Color::from_u32(0x00cdcd55),
        Color::from_u32(0x005555cc),
        Color::from_u32(0x00cc55cc),
        Color::from_u32(0x007acaca),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00060606), Color::from_u32(0x000C0C0C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00338E51);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x009D9D25);

    fn id(&self) -> &str {
        "wez"
    }

    fn title(&self) -> &str {
        "Wez"
    }
}
