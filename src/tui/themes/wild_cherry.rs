
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct WildCherry;

impl SixColorsTwoRowsStyler for WildCherry {
    const BACKGROUND: Color = Color::from_u32(0x001f1726);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003F3746);
    const FOREGROUND: Color = Color::from_u32(0x00dafaff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000507);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00da6bac),
        Color::from_u32(0x00f4dca5),
        Color::from_u32(0x00eac066),
        Color::from_u32(0x00308cba),
        Color::from_u32(0x00ae636b),
        Color::from_u32(0x00ff919d),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00d94085),
        Color::from_u32(0x002ab250),
        Color::from_u32(0x00ffd16f),
        Color::from_u32(0x00883cdc),
        Color::from_u32(0x00ececec),
        Color::from_u32(0x00c1b8b7),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00251D2C), Color::from_u32(0x002B2332)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00BD00DF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CFA13F);
}
