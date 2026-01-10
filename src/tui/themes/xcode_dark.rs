use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct XcodeDark;

impl SixColorsTwoRowsStyler for XcodeDark {
    const BACKGROUND: Color = Color::from_u32(0x00292a30);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00494A50);
    const FOREGROUND: Color = Color::from_u32(0x00dfdfe0);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00414453);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff8170),
        Color::from_u32(0x00acf2e4),
        Color::from_u32(0x00ffa14f),
        Color::from_u32(0x006bdfff),
        Color::from_u32(0x00ff7ab2),
        Color::from_u32(0x00dabaff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff8170),
        Color::from_u32(0x0078c2b3),
        Color::from_u32(0x00d9c97c),
        Color::from_u32(0x004eb0cc),
        Color::from_u32(0x00ff7ab2),
        Color::from_u32(0x00b281eb),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002F3036), Color::from_u32(0x0035363C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00BFBFC0);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF5140);
}
