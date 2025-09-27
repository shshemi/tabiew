
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct RedSands;

impl SixColorsTwoRowsStyler for RedSands {
    const BACKGROUND: Color = Color::from_u32(0x007a251e);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x009A453E);
    const FOREGROUND: Color = Color::from_u32(0x00d7c9a7);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00bb0000),
        Color::from_u32(0x0000bb00),
        Color::from_u32(0x00e7b000),
        Color::from_u32(0x000072ae),
        Color::from_u32(0x00ff55ff),
        Color::from_u32(0x0055ffff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff3f00),
        Color::from_u32(0x0000bb00),
        Color::from_u32(0x00e7b000),
        Color::from_u32(0x000072ff),
        Color::from_u32(0x00bb00bb),
        Color::from_u32(0x0000bbbb),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00802B24), Color::from_u32(0x0086312A)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DFDFDF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF0F00);

    fn id(&self) -> &str {
        "red_sands"
    }

    fn title(&self) -> &str {
        "RedSands"
    }
}
