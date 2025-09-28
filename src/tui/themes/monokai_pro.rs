
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct MonokaiPro;

impl SixColorsTwoRowsStyler for MonokaiPro {
    const BACKGROUND: Color = Color::from_u32(0x002d2a2e);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x004D4A4E);
    const FOREGROUND: Color = Color::from_u32(0x00fcfcfa);
    const DARK_FOREGROUND: Color = Color::from_u32(0x002d2a2e);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff6188),
        Color::from_u32(0x00a9dc76),
        Color::from_u32(0x00ffd866),
        Color::from_u32(0x00fc9867),
        Color::from_u32(0x00ab9df2),
        Color::from_u32(0x0078dce8),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff6188),
        Color::from_u32(0x00a9dc76),
        Color::from_u32(0x00ffd866),
        Color::from_u32(0x00fc9867),
        Color::from_u32(0x00ab9df2),
        Color::from_u32(0x0078dce8),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00333034), Color::from_u32(0x0039363A)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A1A0A0);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CFA836);
}
