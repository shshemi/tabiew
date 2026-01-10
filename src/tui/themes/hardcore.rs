use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Hardcore;

impl SixColorsTwoRowsStyler for Hardcore {
    const BACKGROUND: Color = Color::from_u32(0x00121212);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00323232);
    const FOREGROUND: Color = Color::from_u32(0x00a0a0a0);
    const DARK_FOREGROUND: Color = Color::from_u32(0x001b1d1e);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff669d),
        Color::from_u32(0x00beed5f),
        Color::from_u32(0x00e6db74),
        Color::from_u32(0x0066d9ef),
        Color::from_u32(0x009e6ffe),
        Color::from_u32(0x00a3babf),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f92672),
        Color::from_u32(0x00a6e22e),
        Color::from_u32(0x00fd971f),
        Color::from_u32(0x0066d9ef),
        Color::from_u32(0x009e6ffe),
        Color::from_u32(0x005e7175),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00181818), Color::from_u32(0x001E1E1E)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x009B9B9B);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CD6700);
}
