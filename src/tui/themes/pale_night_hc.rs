use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct PaleNightHc;

impl SixColorsTwoRowsStyler for PaleNightHc {
    const BACKGROUND: Color = Color::from_u32(0x003e4251);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x005E6271);
    const FOREGROUND: Color = Color::from_u32(0x00cccccc);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f6a9ae),
        Color::from_u32(0x00dbf1ba),
        Color::from_u32(0x00ffdfa6),
        Color::from_u32(0x00b4ccff),
        Color::from_u32(0x00ddbdf2),
        Color::from_u32(0x00b8eaff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f07178),
        Color::from_u32(0x00c3e88d),
        Color::from_u32(0x00ffcb6b),
        Color::from_u32(0x0082aaff),
        Color::from_u32(0x00c792ea),
        Color::from_u32(0x0089ddff),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00444857), Color::from_u32(0x004A4E5D)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DFAB4B);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF9B3B);
}
