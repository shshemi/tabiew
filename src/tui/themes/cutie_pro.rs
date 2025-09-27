
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct CutiePro;

impl SixColorsTwoRowsStyler for CutiePro {
    const BACKGROUND: Color = Color::from_u32(0x00181818);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00383838);
    const FOREGROUND: Color = Color::from_u32(0x00d5d0c9);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00e5a1a3),
        Color::from_u32(0x00e8d6a7),
        Color::from_u32(0x00f1bb79),
        Color::from_u32(0x0080c5de),
        Color::from_u32(0x00b294bb),
        Color::from_u32(0x009dccbb),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f56e7f),
        Color::from_u32(0x00bec975),
        Color::from_u32(0x00f58669),
        Color::from_u32(0x0042d9c5),
        Color::from_u32(0x00d286b7),
        Color::from_u32(0x0037cb8a),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001E1E1E), Color::from_u32(0x00242424)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00CFA4AD);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C55639);

    fn id(&self) -> &str {
        "cutie_pro"
    }

    fn title(&self) -> &str {
        "CutiePro"
    }
}
