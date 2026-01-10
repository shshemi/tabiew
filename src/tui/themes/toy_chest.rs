use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct ToyChest;

impl SixColorsTwoRowsStyler for ToyChest {
    const BACKGROUND: Color = Color::from_u32(0x0024364b);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x0044566B);
    const FOREGROUND: Color = Color::from_u32(0x0031d07b);
    const DARK_FOREGROUND: Color = Color::from_u32(0x002c3f58);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00dd5944),
        Color::from_u32(0x0031d07b),
        Color::from_u32(0x00e7d84b),
        Color::from_u32(0x0034a6da),
        Color::from_u32(0x00ae6bdc),
        Color::from_u32(0x0042c3ae),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00be2d26),
        Color::from_u32(0x001a9172),
        Color::from_u32(0x00db8e27),
        Color::from_u32(0x00325d96),
        Color::from_u32(0x008a5edc),
        Color::from_u32(0x0035a08f),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002A3C51), Color::from_u32(0x00304257)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00B5B5B5);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00AB5E00);
}
