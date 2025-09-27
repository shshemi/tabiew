
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Mellow;

impl SixColorsTwoRowsStyler for Mellow {
    const BACKGROUND: Color = Color::from_u32(0x00161617);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00363637);
    const FOREGROUND: Color = Color::from_u32(0x00c9c7cd);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0027272a);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ffae9f),
        Color::from_u32(0x009dc6ac),
        Color::from_u32(0x00f0c5a9),
        Color::from_u32(0x00b9aeda),
        Color::from_u32(0x00ecaad6),
        Color::from_u32(0x00f591b2),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f5a191),
        Color::from_u32(0x0090b99f),
        Color::from_u32(0x00e6b99d),
        Color::from_u32(0x00aca1cf),
        Color::from_u32(0x00e29eca),
        Color::from_u32(0x00ea83a5),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001C1C1D), Color::from_u32(0x00222223)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00AAA9BD);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C57161);

    fn id(&self) -> &str {
        "mellow"
    }

    fn title(&self) -> &str {
        "Mellow"
    }
}
