use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Nightfox;

impl SixColorsTwoRowsStyler for Nightfox {
    const BACKGROUND: Color = Color::from_u32(0x00192330);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00394350);
    const FOREGROUND: Color = Color::from_u32(0x00cdcecf);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00393b44);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00d16983),
        Color::from_u32(0x008ebaa4),
        Color::from_u32(0x00e0c989),
        Color::from_u32(0x0086abdc),
        Color::from_u32(0x00baa1e2),
        Color::from_u32(0x007ad5d6),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00c94f6d),
        Color::from_u32(0x0081b29a),
        Color::from_u32(0x00dbc074),
        Color::from_u32(0x00719cd6),
        Color::from_u32(0x009d79d6),
        Color::from_u32(0x0063cdcf),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001F2936), Color::from_u32(0x00252F3C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00ADAEAF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00AB9044);
}
