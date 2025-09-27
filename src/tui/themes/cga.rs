
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Cga;

impl SixColorsTwoRowsStyler for Cga {
    const BACKGROUND: Color = Color::from_u32(0x00000000);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00202020);
    const FOREGROUND: Color = Color::from_u32(0x00aaaaaa);
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
        Color::from_u32(0x00aa0000),
        Color::from_u32(0x0000aa00),
        Color::from_u32(0x00aa5500),
        Color::from_u32(0x000000aa),
        Color::from_u32(0x00aa00aa),
        Color::from_u32(0x0000aaaa),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00060606), Color::from_u32(0x000C0C0C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00989898);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x007A0000);

    fn id(&self) -> &str {
        "cga"
    }

    fn title(&self) -> &str {
        "Cga"
    }
}
