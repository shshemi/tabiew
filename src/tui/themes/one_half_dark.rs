
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct OneHalfDark;

impl SixColorsTwoRowsStyler for OneHalfDark {
    const BACKGROUND: Color = Color::from_u32(0x00282c34);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00484C54);
    const FOREGROUND: Color = Color::from_u32(0x00dcdfe4);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00282c34);

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

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002E323A), Color::from_u32(0x00343840)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x008393AC);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B5904B);

    fn id(&self) -> &str {
        "one_half_dark"
    }

    fn title(&self) -> &str {
        "OneHalfDark"
    }
}
