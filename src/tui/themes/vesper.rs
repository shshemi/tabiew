
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Vesper;

impl SixColorsTwoRowsStyler for Vesper {
    const BACKGROUND: Color = Color::from_u32(0x00101010);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00303030);
    const FOREGROUND: Color = Color::from_u32(0x00ffffff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00101010);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff8080),
        Color::from_u32(0x0099ffe4),
        Color::from_u32(0x00ffc799),
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

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00161616), Color::from_u32(0x001C1C1C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x008C918B);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C57161);

    fn id(&self) -> &str {
        "vesper"
    }

    fn title(&self) -> &str {
        "Vesper"
    }
}
