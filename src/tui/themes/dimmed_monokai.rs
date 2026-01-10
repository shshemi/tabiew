use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct DimmedMonokai;

impl SixColorsTwoRowsStyler for DimmedMonokai {
    const BACKGROUND: Color = Color::from_u32(0x001f1f1f);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003F3F3F);
    const FOREGROUND: Color = Color::from_u32(0x00b9bcba);
    const DARK_FOREGROUND: Color = Color::from_u32(0x003a3d43);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00fb001f),
        Color::from_u32(0x000f722f),
        Color::from_u32(0x00c47033),
        Color::from_u32(0x00186de3),
        Color::from_u32(0x00fb0067),
        Color::from_u32(0x002e706d),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00be3f48),
        Color::from_u32(0x00879a3b),
        Color::from_u32(0x00c5a635),
        Color::from_u32(0x004f76a1),
        Color::from_u32(0x00855c8d),
        Color::from_u32(0x00578fa4),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00252525), Color::from_u32(0x002B2B2B)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00D81E00);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00957605);
}
