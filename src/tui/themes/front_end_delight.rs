use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct FrontEndDelight;

impl SixColorsTwoRowsStyler for FrontEndDelight {
    const BACKGROUND: Color = Color::from_u32(0x001b1c1d);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003B3C3D);
    const FOREGROUND: Color = Color::from_u32(0x00adadad);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00242526);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f74319),
        Color::from_u32(0x0074ec4c),
        Color::from_u32(0x00fdc325),
        Color::from_u32(0x003393ca),
        Color::from_u32(0x00e75e4f),
        Color::from_u32(0x004fbce6),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f8511b),
        Color::from_u32(0x00565747),
        Color::from_u32(0x00fa771d),
        Color::from_u32(0x002c70b7),
        Color::from_u32(0x00f02e4f),
        Color::from_u32(0x003ca1a6),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00212223), Color::from_u32(0x00272829)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00ADADAD);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CA4700);
}
