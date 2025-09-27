
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct AtomOneDark;

impl SixColorsTwoRowsStyler for AtomOneDark {
    const BACKGROUND: Color = Color::from_u32(0x0021252b);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x0041454B);
    const FOREGROUND: Color = Color::from_u32(0x00abb2bf);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0021252b);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00e06c75),
        Color::from_u32(0x0098c379),
        Color::from_u32(0x00e5c07b),
        Color::from_u32(0x0061afef),
        Color::from_u32(0x00c678dd),
        Color::from_u32(0x0056b6c2),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00e06c75),
        Color::from_u32(0x0098c379),
        Color::from_u32(0x00e5c07b),
        Color::from_u32(0x0061afef),
        Color::from_u32(0x00c678dd),
        Color::from_u32(0x0056b6c2),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00272B31), Color::from_u32(0x002D3137)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x008B929F);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B5904B);

    fn id(&self) -> &str {
        "atom_one_dark"
    }

    fn title(&self) -> &str {
        "AtomOneDark"
    }
}
