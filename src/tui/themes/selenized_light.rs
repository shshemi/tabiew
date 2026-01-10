use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct SelenizedLight;

impl SixColorsTwoRowsStyler for SelenizedLight {
    const BACKGROUND: Color = Color::from_u32(0x00fbf3db);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFB);
    const FOREGROUND: Color = Color::from_u32(0x0053676d);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00ece3cc);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00cc1729),
        Color::from_u32(0x00428b00),
        Color::from_u32(0x00a78300),
        Color::from_u32(0x00006dce),
        Color::from_u32(0x00c44392),
        Color::from_u32(0x0000978a),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00d2212d),
        Color::from_u32(0x00489100),
        Color::from_u32(0x00ad8900),
        Color::from_u32(0x000072d4),
        Color::from_u32(0x00ca4898),
        Color::from_u32(0x00009c8f),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FFF9E1), Color::from_u32(0x00FFFFE7)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0033474D);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00A20000);
}
