
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Idea;

impl SixColorsTwoRowsStyler for Idea {
    const BACKGROUND: Color = Color::from_u32(0x00202020);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00404040);
    const FOREGROUND: Color = Color::from_u32(0x00adadad);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00adadad);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00fc7072),
        Color::from_u32(0x0098b61c),
        Color::from_u32(0x00ffff0b),
        Color::from_u32(0x006c9ced),
        Color::from_u32(0x00fc7eff),
        Color::from_u32(0x00248887),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00fc5256),
        Color::from_u32(0x0098b61c),
        Color::from_u32(0x00ccb444),
        Color::from_u32(0x00437ee7),
        Color::from_u32(0x009d74b0),
        Color::from_u32(0x00248887),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00262626), Color::from_u32(0x002C2C2C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x009B9B9B);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CC2226);

    fn id(&self) -> &str {
        "idea"
    }

    fn title(&self) -> &str {
        "Idea"
    }
}
