
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct GruvboxLight;

impl SixColorsTwoRowsStyler for GruvboxLight {
    const BACKGROUND: Color = Color::from_u32(0x00fbf1c7);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFE7);
    const FOREGROUND: Color = Color::from_u32(0x003c3836);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00fbf1c7);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x009d0006),
        Color::from_u32(0x0079740e),
        Color::from_u32(0x00b57614),
        Color::from_u32(0x00076678),
        Color::from_u32(0x008f3f71),
        Color::from_u32(0x00427b58),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00cc241d),
        Color::from_u32(0x0098971a),
        Color::from_u32(0x00d79921),
        Color::from_u32(0x00458588),
        Color::from_u32(0x00b16286),
        Color::from_u32(0x00689d6a),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FFF7CD), Color::from_u32(0x00FFFDD3)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x001C1816);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00A76900);

    fn id(&self) -> &str {
        "gruvbox_light"
    }

    fn title(&self) -> &str {
        "GruvboxLight"
    }
}
