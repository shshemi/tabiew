
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct DoomPeacock;

impl SixColorsTwoRowsStyler for DoomPeacock {
    const BACKGROUND: Color = Color::from_u32(0x002b2a27);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x004B4A47);
    const FOREGROUND: Color = Color::from_u32(0x00ede0ce);
    const DARK_FOREGROUND: Color = Color::from_u32(0x001c1f24);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff5d38),
        Color::from_u32(0x0098be65),
        Color::from_u32(0x00e6f972),
        Color::from_u32(0x0051afef),
        Color::from_u32(0x00c678dd),
        Color::from_u32(0x0046d9ff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00cb4b16),
        Color::from_u32(0x0026a6a6),
        Color::from_u32(0x00bcd42a),
        Color::from_u32(0x002a6cc6),
        Color::from_u32(0x00a9a1e1),
        Color::from_u32(0x005699af),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0031302D), Color::from_u32(0x00373633)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x007C7C7D);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x009B1B00);

    fn id(&self) -> &str {
        "doom_peacock"
    }

    fn title(&self) -> &str {
        "DoomPeacock"
    }
}
