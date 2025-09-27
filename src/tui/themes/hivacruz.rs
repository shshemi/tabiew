
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Hivacruz;

impl SixColorsTwoRowsStyler for Hivacruz {
    const BACKGROUND: Color = Color::from_u32(0x00132638);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00334658);
    const FOREGROUND: Color = Color::from_u32(0x00ede4e4);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00202746);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00c76b29),
        Color::from_u32(0x0073ad43),
        Color::from_u32(0x005e6687),
        Color::from_u32(0x00898ea4),
        Color::from_u32(0x00dfe2f1),
        Color::from_u32(0x009c637a),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00c94922),
        Color::from_u32(0x00ac9739),
        Color::from_u32(0x00c08b30),
        Color::from_u32(0x003d8fd1),
        Color::from_u32(0x006679cc),
        Color::from_u32(0x0022a2c9),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00192C3E), Color::from_u32(0x001F3244)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00777D94);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00991900);

    fn id(&self) -> &str {
        "hivacruz"
    }

    fn title(&self) -> &str {
        "Hivacruz"
    }
}
