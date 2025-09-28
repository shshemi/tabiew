
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Dracula;

impl SixColorsTwoRowsStyler for Dracula {
    const BACKGROUND: Color = Color::from_u32(0x00282a36);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00484A56);
    const FOREGROUND: Color = Color::from_u32(0x00f8f8f2);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0021222c);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff6e6e),
        Color::from_u32(0x0069ff94),
        Color::from_u32(0x00ffffa5),
        Color::from_u32(0x00d6acff),
        Color::from_u32(0x00ff92df),
        Color::from_u32(0x00a4ffff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff5555),
        Color::from_u32(0x0050fa7b),
        Color::from_u32(0x00f1fa8c),
        Color::from_u32(0x00bd93f9),
        Color::from_u32(0x00ff79c6),
        Color::from_u32(0x008be9fd),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002E303C), Color::from_u32(0x00343642)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00D8D8D2);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF2525);
}
