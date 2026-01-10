use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Hopscotch;

impl SixColorsTwoRowsStyler for Hopscotch {
    const BACKGROUND: Color = Color::from_u32(0x00322931);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00524951);
    const FOREGROUND: Color = Color::from_u32(0x00b9b5b8);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00322931);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00fd8b19),
        Color::from_u32(0x00433b42),
        Color::from_u32(0x005c545b),
        Color::from_u32(0x00989498),
        Color::from_u32(0x00d5d3d5),
        Color::from_u32(0x00b33508),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00dd464c),
        Color::from_u32(0x008fc13e),
        Color::from_u32(0x00fdcc59),
        Color::from_u32(0x001290bf),
        Color::from_u32(0x00c85e7c),
        Color::from_u32(0x00149b93),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00382F37), Color::from_u32(0x003E353D)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00999598);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CD9C29);
}
