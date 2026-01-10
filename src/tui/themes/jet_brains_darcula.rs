use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct JetbrainsDarcula;

impl SixColorsTwoRowsStyler for JetbrainsDarcula {
    const BACKGROUND: Color = Color::from_u32(0x00202020);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00404040);
    const FOREGROUND: Color = Color::from_u32(0x00adadad);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00fb7172),
        Color::from_u32(0x0067ff4f),
        Color::from_u32(0x00ffff00),
        Color::from_u32(0x006d9df1),
        Color::from_u32(0x00fb82ff),
        Color::from_u32(0x0060d3d1),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00fa5355),
        Color::from_u32(0x00126e00),
        Color::from_u32(0x00c2c300),
        Color::from_u32(0x004581eb),
        Color::from_u32(0x00fa54ff),
        Color::from_u32(0x0033c2c1),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00262626), Color::from_u32(0x002C2C2C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DFDFDF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CA2325);
}
