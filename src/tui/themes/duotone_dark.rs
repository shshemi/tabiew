use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct DuotoneDark;

impl SixColorsTwoRowsStyler for DuotoneDark {
    const BACKGROUND: Color = Color::from_u32(0x001f1d27);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003F3D47);
    const FOREGROUND: Color = Color::from_u32(0x00b7a1ff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x001f1d27);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00d9393e),
        Color::from_u32(0x002dcd73),
        Color::from_u32(0x00d9b76e),
        Color::from_u32(0x00ffc284),
        Color::from_u32(0x00de8d40),
        Color::from_u32(0x002488ff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00d9393e),
        Color::from_u32(0x002dcd73),
        Color::from_u32(0x00d9b76e),
        Color::from_u32(0x00ffc284),
        Color::from_u32(0x00de8d40),
        Color::from_u32(0x002488ff),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0025232D), Color::from_u32(0x002B2933)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DF7819);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF9254);
}
