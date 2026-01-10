use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct MonokaiProSpectrum;

impl SixColorsTwoRowsStyler for MonokaiProSpectrum {
    const BACKGROUND: Color = Color::from_u32(0x00222222);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00424242);
    const FOREGROUND: Color = Color::from_u32(0x00f7f1ff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00222222);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00fc618d),
        Color::from_u32(0x007bd88f),
        Color::from_u32(0x00fce566),
        Color::from_u32(0x00fd9353),
        Color::from_u32(0x00948ae3),
        Color::from_u32(0x005ad4e6),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00fc618d),
        Color::from_u32(0x007bd88f),
        Color::from_u32(0x00fce566),
        Color::from_u32(0x00fd9353),
        Color::from_u32(0x00948ae3),
        Color::from_u32(0x005ad4e6),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00282828), Color::from_u32(0x002E2E2E)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x009A96A0);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CD6323);
}
