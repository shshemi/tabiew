
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Seti;

impl SixColorsTwoRowsStyler for Seti {
    const BACKGROUND: Color = Color::from_u32(0x00111213);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00313233);
    const FOREGROUND: Color = Color::from_u32(0x00cacecd);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00323232);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00c22832),
        Color::from_u32(0x008ec43d),
        Color::from_u32(0x00e0c64f),
        Color::from_u32(0x0043a5d5),
        Color::from_u32(0x008b57b5),
        Color::from_u32(0x008ec43d),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00c22832),
        Color::from_u32(0x008ec43d),
        Color::from_u32(0x00e0c64f),
        Color::from_u32(0x0043a5d5),
        Color::from_u32(0x008b57b5),
        Color::from_u32(0x008ec43d),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00171819), Color::from_u32(0x001D1E1F)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00C39F01);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B0961F);

    fn id(&self) -> &str {
        "seti"
    }

    fn title(&self) -> &str {
        "Seti"
    }
}
