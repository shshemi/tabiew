use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Batman;

impl SixColorsTwoRowsStyler for Batman {
    const BACKGROUND: Color = Color::from_u32(0x001b1d1e);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003B3D3E);
    const FOREGROUND: Color = Color::from_u32(0x006f6f6f);
    const DARK_FOREGROUND: Color = Color::from_u32(0x001b1d1e);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00fff78e),
        Color::from_u32(0x00fff27d),
        Color::from_u32(0x00feed6c),
        Color::from_u32(0x00919495),
        Color::from_u32(0x009a9a9d),
        Color::from_u32(0x00a3a3a6),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00e6dc44),
        Color::from_u32(0x00c8be46),
        Color::from_u32(0x00f4fd22),
        Color::from_u32(0x00737174),
        Color::from_u32(0x00747271),
        Color::from_u32(0x0062605f),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00212324), Color::from_u32(0x0027292A)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DCCF00);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C4CD00);
}
