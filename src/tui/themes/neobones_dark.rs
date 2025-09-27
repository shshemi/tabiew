
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct NeobonesDark;

impl SixColorsTwoRowsStyler for NeobonesDark {
    const BACKGROUND: Color = Color::from_u32(0x000f191f);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x002F393F);
    const FOREGROUND: Color = Color::from_u32(0x00c6d5cf);
    const DARK_FOREGROUND: Color = Color::from_u32(0x000f191f);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00e8838f),
        Color::from_u32(0x00a0ff85),
        Color::from_u32(0x00d68c67),
        Color::from_u32(0x0092a0e2),
        Color::from_u32(0x00cf86c1),
        Color::from_u32(0x0065b8c1),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00de6e7c),
        Color::from_u32(0x0090ff6b),
        Color::from_u32(0x00b77e64),
        Color::from_u32(0x008190d4),
        Color::from_u32(0x00b279a7),
        Color::from_u32(0x0066a5ad),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00151F25), Color::from_u32(0x001B252B)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00AEBDB7);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00AE3E4C);

    fn id(&self) -> &str {
        "neobones_dark"
    }

    fn title(&self) -> &str {
        "NeobonesDark"
    }
}
