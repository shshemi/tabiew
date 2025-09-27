
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Lavandula;

impl SixColorsTwoRowsStyler for Lavandula {
    const BACKGROUND: Color = Color::from_u32(0x00050014);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00252034);
    const FOREGROUND: Color = Color::from_u32(0x00736e7d);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00230046);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00e05167),
        Color::from_u32(0x0052e0c4),
        Color::from_u32(0x00e0c386),
        Color::from_u32(0x008e87e0),
        Color::from_u32(0x00a776e0),
        Color::from_u32(0x009ad4e0),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x007d1625),
        Color::from_u32(0x00337e6f),
        Color::from_u32(0x007f6f49),
        Color::from_u32(0x004f4a7f),
        Color::from_u32(0x005a3f7f),
        Color::from_u32(0x0058777f),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x000B061A), Color::from_u32(0x00110C20)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x006C71DA);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x004F3F19);

    fn id(&self) -> &str {
        "lavandula"
    }

    fn title(&self) -> &str {
        "Lavandula"
    }
}
